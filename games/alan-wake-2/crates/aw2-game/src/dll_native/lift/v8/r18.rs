#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000885f14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000885f14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000885f18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000885f19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000885f1a); }
            let r0 = c.sub(c.r64(4), 0xc0u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000885f22); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000885f26); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 215; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(11, c.r64(7).wrapping_add(0x18));
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(12).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000885f3e); };
            c.w64(12, t2);
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000885f46); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000885f50); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000885f56); };
            c.w64(7, t4);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(7)) { return RegfileExit::fault(0x7000885f5d); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885f6a); };
            c.w64(11, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000885f70); };
            c.w64(11, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(11)) { return RegfileExit::fault(0x7000885f77); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 217; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885f88); };
            c.w32(0, t7);
            let r8 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r8);
            let Some(t9) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000885f90); };
            c.cmp(t9, 0xbau64, 16);
            if c.cc_b() { blk = 217; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x7000885fa6); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885faf); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000885fb7); };
            c.cmp(t12, 0x834u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000885fbf); };
            c.w32(0, t13);
            let r14 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r14);
            blk = 35; continue;
        }
        12 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 14; continue;
        }
        13 => {
            c.w32(3, 0x1u64);
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t15) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885fda); };
            c.w32(3, t15);
            let r16 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000885fe0); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r18);
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885fe6); };
            c.w32(2, t19);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t20) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885fec); };
            c.w32(1, t20);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xb0u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x7000885ffd);
        }
        17 => {
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x7000885ffe); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 221; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t22) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000886013); };
            c.w32(0, t22);
            let r23 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r23);
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000886019); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088601f); };
            c.w32(3, t26);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t27) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000886025); };
            c.w32(2, t27);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x44u64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x7000886033);
        }
        21 => {
            let Some(t28) = c.ld32(c.r64(0).wrapping_add(0x117)) else { return RegfileExit::fault(0x7000886034); };
            c.cmp(t28, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(3), 0xaeu64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x7000886045);
        }
        24 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x7000886046); };
            c.cmp(t29, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r64(3), 0xafu64, 64);
            if c.cc_a() { blk = 220; continue; }
            blk = 26; continue;
        }
        26 => {
            return RegfileExit::fault(0x700088605b);
        }
        27 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088605c); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            blk = 28; continue;
        }
        28 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            return c.call(0x70008a17f4, 0x700088606d, 0x7000886068);
        }
        30 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x700088606d); };
            c.w64(11, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886074); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886078); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088607c); };
            c.w64(15, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000886080); };
            c.w64(7, t36);
            blk = 35; continue;
        }
        31 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(2, c.r64(0));
            blk = 34; continue;
        }
        33 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        34 => {
            c.w64(0, c.r64(2));
            blk = 35; continue;
        }
        35 => {
            c.w32(3, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(0)) { return RegfileExit::fault(0x70008860a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008860a9); }
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(2)) { return RegfileExit::fault(0x70008860ad); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008860b4); };
            c.w64(1, t37);
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x13b)) else { return RegfileExit::fault(0x70008860bb); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008860c9); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008860d3); }
            blk = 41; continue;
        }
        38 => {
            c.w64(1, c.r64(0));
            let r40 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r40);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 41; continue;
        }
        39 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 40; continue;
        }
        40 => {
            let r41 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008860f0); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 41; continue;
        }
        41 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 213; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 44; continue;
        }
        43 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_l() { blk = 213; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000886111); };
            c.w64(1, t42);
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000886118); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000886121); };
            c.cmp(t45, c.r32(1), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 223; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t46) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000886133); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088613a); };
            c.cmp(t48, 0x40u64, 16);
            if c.cc_ae() { blk = 223; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000886146); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088614f); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 74; continue;
        }
        50 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_le() { blk = 74; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088616c); };
            c.w32(8, t49);
            let r50 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000886173); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088617a); };
            c.w32(9, t53);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2cu64, 64);
            if c.cc_a() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            return RegfileExit::fault(0x700088618a);
        }
        53 => {
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0xb7)) else { return RegfileExit::fault(0x700088618b); };
            c.w32(8, t54);
            let r55 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r55);
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000886195); };
            c.cmp(t56, c.r32(8), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008861a2); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008861a9); };
            c.w64(9, t58);
            c.w64(7, c.r64(9).wrapping_add(0x10));
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008861b0); };
            c.w64(11, t59);
            let Some(t60) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008861b7); };
            c.cmp(t60, c.r64(7), 64);
            if c.cc_be() { blk = 225; continue; }
            blk = 55; continue;
        }
        55 => {
            if !c.st64(c.r64(8), c.r64(7)) { return RegfileExit::fault(0x70008861c0); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 56; continue;
        }
        56 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008861c7); };
            c.w64(11, t61);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008861ce); }
            c.w32(11, 0x4u64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008861d8); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008861dc); };
            c.w64(11, t62);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008861e3); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008861e7); }
            let Some(t63) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008861eb); };
            c.w32(11, t63);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            return RegfileExit::fault(0x70008861fb);
        }
        58 => {
            c.w64(11, c.r64(12));
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008861ff); }
            let Some(t64) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000886203); };
            c.w32(12, t64);
            let r65 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r65);
            let Some(t66) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088620a); };
            c.w32(12, t66);
            let r67 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r67);
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000886212); };
            c.w32(8, t68);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2cu64, 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x7000886223);
        }
        60 => {
            if !c.st32(c.r64(12).wrapping_add(0xb7), c.r32(7)) { return RegfileExit::fault(0x7000886224); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r69 = c.r64(8) & c.r64(12);
            c.w64(8, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088623c); };
            c.logic(t70 & 0x4u64, 8);
            if c.cc_ne() { blk = 387; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 72; continue;
        }
        63 => {
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088624c); };
            c.w32(9, t71);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x700088625c);
        }
        65 => {
            let Some(t72) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088625d); };
            c.w32(9, t72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000886261); };
            c.cmp(t73, c.r32(9), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(12));
            return c.call(0x7000883ff4, 0x7000886275, 0x7000886270);
        }
        67 => {
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000886275); };
            c.cmp(t74, c.r32(0), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886282); };
            c.w64(6, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886286); };
            c.w64(12, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088628a); };
            c.w64(15, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088628e); };
            c.w64(1, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000886292); };
            c.w64(11, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000886299); };
            c.w64(0, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008862a0); };
            c.w64(2, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008862a7); };
            c.w64(3, t82);
            blk = 71; continue;
        }
        69 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008862ad); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r83 = c.r64(9) & c.r64(8);
            c.w64(9, r83);
            c.logic(r83, 64);
            let Some(t84) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008862c1); };
            c.logic(t84 & 0x4u64, 8);
            if c.cc_ne() { blk = 384; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(11, c.r64(12));
            blk = 72; continue;
        }
        72 => {
            c.w32(7, 0x1u64);
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(7, c.r64(15));
            let r85 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008862e0); };
            c.w64(0, t86);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008862ea); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008862eb); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008862f1); }
            return c.ret(0x0, 0x70008862f3);
        }
        75 => {
            let Some(t87) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008862f4); };
            c.w32(7, t87);
            let r88 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r88);
            let Some(t89) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008862fb); };
            c.w32(12, t89);
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(7)) { return RegfileExit::fault(0x7000886300); }
            let Some(t90) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000886307); };
            c.cmp(t90, 0x834u64, 16);
            if c.cc_ne() { blk = 99; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t91) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000886313); };
            c.w32(8, t91);
            let r92 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r92);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000886320); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088632b); }
            blk = 82; continue;
        }
        79 => {
            c.w64(8, c.r64(0));
            let r93 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r93);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 82; continue;
        }
        80 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            let r94 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r94);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088634b); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 82; continue;
        }
        82 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            if c.cc_e() { blk = 87; continue; }
            blk = 84; continue;
        }
        84 => {
            blk = 86; continue;
        }
        85 => {
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 88; continue;
        }
        87 => {
            c.w32(8, 0x1u64);
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t95) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088637d); };
            c.w32(8, t95);
            let r96 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r96);
            let Some(t97) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000886384); };
            c.w32(8, t97);
            let r98 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r98);
            let Some(t99) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088638b); };
            c.w32(9, t99);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            return RegfileExit::fault(0x700088639b);
        }
        91 => {
            let Some(t100) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x700088639c); };
            c.w32(8, t100);
            let Some(t101) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008863a3); };
            c.cmp(t101, c.r32(8), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008863a9); };
            c.w64(8, t102);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t103) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008863b3); };
            c.cmp(t103, c.r32(9), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(8, c.r64(12));
            let r104 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r104);
            let r105 = c.r32(8) & 0x3fu64;
            c.w32(8, r105);
            c.logic(r105, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 98; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_le() { blk = 97; continue; }
            blk = 95; continue;
        }
        95 => {
            let r106 = c.r32(12) & 0xfcu64;
            c.w32(12, r106);
            c.logic(r106, 32);
            c.cmp(c.r32(12), 0x30u64, 32);
            if c.cc_e() { blk = 228; continue; }
            blk = 96; continue;
        }
        96 => {
            blk = 99; continue;
        }
        97 => {
            c.w32(12, 0x54cu64);
            blk = 100; continue;
        }
        98 => {
            c.w32(12, 0x54au64);
            blk = 100; continue;
        }
        99 => {
            c.w32(12, 0x8a4u64);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc8), c.r64(12)) { return RegfileExit::fault(0x70008863fc); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            blk = 105; continue;
        }
        102 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000886410); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 105; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_s() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(10, 0x8000000000000000u64);
            let r107 = c.r64(3) | c.r64(10);
            c.w64(3, r107);
            c.logic(r107, 64);
            blk = 105; continue;
        }
        105 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(3)) { return RegfileExit::fault(0x700088644a); }
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_a() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(8, c.r64(3));
            blk = 108; continue;
        }
        107 => {
            c.w32(8, 0x7ffeu64);
            blk = 108; continue;
        }
        108 => {
            let Some(t108) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000886465); };
            c.w32(7, t108);
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(7)) { return RegfileExit::fault(0x7000886468); }
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 362; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088647c); };
            c.w64(12, t109);
            let Some(t110) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000886483); };
            c.w64(12, t110);
            c.w64(0, c.r64(8).wrapping_mul(4));
            c.w64(3, c.r64(0).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000886493); }
            c.w64(0, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088649b); };
            c.w64(2, t111);
            let Some(t112) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008864a2); };
            c.cmp(t112, c.r64(0), 64);
            if c.cc_be() { blk = 256; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 256; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008864b8); };
            c.w64(3, t113);
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x70008864bf); }
            let r114 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r114);
            blk = 112; continue;
        }
        112 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008864ca); };
            c.w64(3, t115);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008864d1); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008864d6); }
            c.w64(0, c.r64(12).wrapping_add(0x7));
            c.cmp(c.r64(7), 0x0u64, 64);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.w64(10, c.r64(4));
            let r116 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r116);
            let r117 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r117);
            c.logic(r117, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008864f5); }
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(12)) { return RegfileExit::fault(0x70008864fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000886501); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(9));
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088650b); };
            c.w64(8, t118);
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x700088650f); };
            c.w64(0, t119);
            c.w64(10, 0x700088651du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700088651d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000886521); }
            return c.call(c.r64(0), 0x7000886527, 0x7000886525);
        }
        113 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000886527); }
            let Some(t120) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088652f); };
            c.w64(4, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000886534); };
            c.w64(1, t121);
            let Some(t122) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000886538); };
            c.w32(7, t122);
            let r123 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r123);
            let Some(t124) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088653e); };
            c.w32(7, t124);
            let r125 = c.r32(7) & 0x8u64;
            c.w32(7, r125);
            c.logic(r125, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000886550); };
            c.w64(0, t126);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(3));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700088655c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000886560); }
            blk = 115; continue;
        }
        114 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000886566); };
            c.w64(3, t127);
            let r128 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r128);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700088656e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000886572); }
            c.w64(0, c.r64(1));
            blk = 115; continue;
        }
        115 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000886579); };
            c.w64(2, t129);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000886580); }
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886584); };
            c.w64(6, t130);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700088658a); };
            c.w64(9, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000886591); };
            c.w64(8, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000886598); };
            c.cmp(c.r64(3), t133, 64);
            if c.cc_ae() { blk = 155; continue; }
            blk = 116; continue;
        }
        116 => {
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(0)) { return RegfileExit::fault(0x70008865a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x70008865ac); }
            c.cmp(c.r32(2), 0x8a4u64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t134) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008865bc); };
            c.cmp(t134, c.r32(8), 32);
            if c.cc_ne() { blk = 132; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t135) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008865c6); };
            c.w32(7, t135);
            let r136 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r136);
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t137) = c.ld32(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008865d8); };
            c.cmp(t137, 0x0u64, 32);
            if c.cc_ne() { blk = 122; continue; }
            blk = 119; continue;
        }
        119 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008865e1); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008865ed); }
            blk = 124; continue;
        }
        121 => {
            let r138 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r138);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 124; continue;
        }
        122 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(11, c.r64(9));
            let r139 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r139);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088660c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 124; continue;
        }
        124 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 128; continue; }
            blk = 125; continue;
        }
        125 => {
            if c.cc_e() { blk = 129; continue; }
            blk = 126; continue;
        }
        126 => {
            blk = 128; continue;
        }
        127 => {
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 130; continue;
        }
        129 => {
            c.w32(7, 0x1u64);
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000886637); };
            c.w64(7, t140);
            let Some(t141) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088663e); };
            c.cmp(t141, c.r32(1), 32);
            if c.cc_ne() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(2, 0x8a4u64);
            c.w64(7, c.r64(2));
            blk = 133; continue;
        }
        133 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc8), c.r64(2)) { return RegfileExit::fault(0x700088664b); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r142 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r142);
            blk = 135; continue;
        }
        135 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700088665c); }
            c.w64(0, c.r64(6));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886663); };
            c.w64(6, t143);
            c.w64(12, c.r64(2));
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700088666a); };
            return c.call(t144, 0x7000886672, 0x700088666a);
        }
        136 => {
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886672); };
            c.w64(6, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000886676); };
            c.w64(3, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x700088667d); };
            c.w64(1, t147);
            return c.call(0x7000883e94, 0x7000886689, 0x7000886684);
        }
        137 => {
            c.w64(7, c.r64(0));
            let Some(t148) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700088668c); };
            c.cmp(t148, c.r32(7), 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000886699); };
            c.cmp(t149, 0x0u64, 64);
            if c.cc_e() { blk = 146; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t150) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008866a0); };
            c.cmp(t150, 0x0u64, 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x70008866a6); };
            c.w64(11, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008866ad); };
            let r153 = c.imul2(c.r64(11), t152, 64);
            c.w64(11, r153);
            c.w64(0, c.r64(11));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x70008866b5); };
            c.w64(12, t154);
            c.cqo();
            c.idiv(c.r64(12), 64);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008866c1); };
            c.cmp(c.r64(0), t155, 64);
            if c.cc_ne() { blk = 350; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008866cb); };
            let r157 = c.add(c.r64(11), t156, 64);
            c.w64(11, r157);
            if c.cc_o() { blk = 348; continue; }
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r64(11), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 348; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008866e2); };
            c.cmp(t158, 0x1u64, 64);
            c.w8(15, (c.cc_g() as u64));
            c.w32(15, c.r8(15));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008866ef); };
            c.cmp(t159, 0x0u64, 64);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let r160 = c.r32(15) | c.r32(0);
            c.w32(15, r160);
            c.logic(r160, 32);
            if c.cc_ne() { blk = 258; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000886703); };
            c.w64(0, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088670a); };
            c.w64(15, t162);
            blk = 145; continue;
        }
        145 => {
            c.w64(6, c.r64(15));
            c.w64(1, c.r64(0));
            blk = 147; continue;
        }
        146 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000886716); };
            c.w64(6, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088671a); };
            c.w64(11, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x700088671e); };
            c.w64(1, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000886725); };
            c.w64(12, t166);
            blk = 147; continue;
        }
        147 => {
            let Some(t167) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088672c); };
            c.w32(15, t167);
            let r168 = c.add(c.r64(15), c.r64(11), 64);
            c.w64(15, r168);
            if c.cc_o() { blk = 360; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r64(15), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 360; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t169) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000886746); };
            c.w32(11, t169);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_ge() { blk = 282; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_ae() { blk = 212; continue; }
            blk = 151; continue;
        }
        151 => {
            c.w64(11, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(11)), c.r32(7)) { return RegfileExit::fault(0x700088676d); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 153; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r170 = c.r64(0) & c.r64(1);
            c.w64(0, r170);
            c.logic(r170, 64);
            let Some(t171) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000886781); };
            c.logic(t171 & 0x4u64, 8);
            if c.cc_ne() { blk = 381; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t172) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088678b); };
            c.w32(7, t172);
            let r173 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r173);
            let Some(t174) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000886791); };
            c.w32(7, t174);
            let r175 = c.r32(7) & 0x8u64;
            c.w32(7, r175);
            c.logic(r175, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.w64(11, c.r64(6).wrapping_add(0x1));
            let Some(t176) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008867a7); };
            let r177 = c.r32(7) & t176;
            c.w32(7, r177);
            c.logic(r177, 32);
            c.w64(6, c.r64(11));
            c.w64(11, c.r64(15));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 114; continue;
        }
        154 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008867b8); };
            c.w64(7, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x70008867bc); };
            c.w64(1, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008867c3); };
            c.w64(6, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008867c7); };
            c.w64(11, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008867cb); };
            c.w64(15, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x70008867cf); };
            c.w64(12, t183);
            blk = 114; continue;
        }
        155 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 165; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t184) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008867e5); };
            c.cmp(t184, 0x0u64, 32);
            if c.cc_ne() { blk = 165; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x70008867eb); };
            c.w64(12, t185);
            let r186 = c.imul2(c.r64(12), c.r64(15), 64);
            c.w64(12, r186);
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(12));
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x70008867fc); };
            c.w64(7, t187);
            c.cqo();
            c.idiv(c.r64(7), 64);
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_ne() { blk = 333; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000886811); };
            let r189 = c.add(c.r64(12), t188, 64);
            c.w64(12, r189);
            if c.cc_o() { blk = 331; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r64(12), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 331; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t190) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000886828); };
            c.w32(7, t190);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(7));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000886835); };
            c.w64(0, t191);
            c.cmp(c.r64(0), c.r64(3), 64);
            if c.cc_ge() { blk = 304; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r64(0), c.r64(3), 64);
            if c.cc_b() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            return RegfileExit::fault(0x7000886847);
        }
        163 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700088684c); }
            blk = 164; continue;
        }
        164 => {
            c.w64(7, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 166; continue;
        }
        165 => {
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088685a); };
            c.w64(12, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088685e); };
            c.w64(7, t193);
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 198; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 189; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w32(11, c.r32(12));
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let Some(t194) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000886885); };
            c.cmp(t194, 0x0u64, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 178; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w64(15, c.r64(11));
            let r195 = c.add(c.r32(15), c.r32(11), 32);
            c.w32(15, r195);
            if c.cc_o() { blk = 177; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r196 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r196);
            if c.cc_o() { blk = 176; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t197) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008868ab); };
            c.w64(15, t197);
            let Some(t198) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008868b2); };
            c.w64(15, t198);
            let r199 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r199);
            let r200 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r200);
            c.logic(r200, 64);
            c.w64(3, c.r64(15).wrapping_add(c.r64(11)));
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008868c1); };
            c.w64(2, t201);
            let Some(t202) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008868c8); };
            c.cmp(t202, c.r64(3), 64);
            if c.cc_be() { blk = 335; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 335; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008868de); };
            c.w64(11, t203);
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008868e5); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 175; continue;
        }
        175 => {
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008868ec); };
            c.w64(15, t204);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008868f3); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008868f7); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008868ff); }
            blk = 187; continue;
        }
        176 => {
            return RegfileExit::fault(0x7000886908);
        }
        177 => {
            return RegfileExit::fault(0x7000886909);
        }
        178 => {
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700088690a); };
            c.w64(11, t205);
            blk = 187; continue;
        }
        179 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r206 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r206);
            if c.cc_o() { blk = 185; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088692a); };
            c.w64(15, t207);
            let Some(t208) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000886931); };
            c.w64(15, t208);
            let r209 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r209);
            let r210 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r210);
            c.logic(r210, 64);
            c.w64(3, c.r64(15).wrapping_add(c.r64(11)));
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000886940); };
            c.w64(2, t211);
            let Some(t212) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000886947); };
            c.cmp(t212, c.r64(3), 64);
            if c.cc_be() { blk = 337; continue; }
            blk = 182; continue;
        }
        182 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 337; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088695d); };
            c.w64(11, t213);
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000886964); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 184; continue;
        }
        184 => {
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700088696b); };
            c.w64(15, t214);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000886972); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000886976); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700088697e); }
            blk = 187; continue;
        }
        185 => {
            return RegfileExit::fault(0x7000886984);
        }
        186 => {
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000886985); };
            c.w64(11, t215);
            blk = 187; continue;
        }
        187 => {
            c.w64(10, c.r64(4));
            let r216 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r216);
            let r217 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r217);
            c.logic(r217, 64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000886997); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x700088699c); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(7));
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008869aa); };
            c.w64(9, t218);
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0x13a8)) else { return RegfileExit::fault(0x70008869ae); };
            c.w64(0, t219);
            c.w64(10, 0x70008869bcu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008869bc); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008869c0); }
            return c.call(c.r64(0), 0x70008869c6, 0x70008869c4);
        }
        188 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008869c6); }
            let Some(t220) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008869ce); };
            c.w64(4, t220);
            c.w64(3, c.r64(0));
            blk = 199; continue;
        }
        189 => {
            let Some(t221) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008869d8); };
            c.w32(7, t221);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            return RegfileExit::fault(0x70008869e7);
        }
        191 => {
            let Some(t222) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008869e8); };
            c.w32(3, t222);
            let r223 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r223);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t224) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008869fe); };
            c.w32(11, t224);
            let r225 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r225);
            let Some(t226) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000886a05); };
            c.cmp(t226, 0x3fu64, 16);
            if c.cc_be() { blk = 199; continue; }
            blk = 193; continue;
        }
        193 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t227) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000886a12); };
            c.w32(7, t227);
            let Some(t228) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000886a15); };
            c.cmp(t228, c.r32(7), 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000886a1e); };
            c.w64(0, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886a22); };
            c.w64(6, t230);
            return c.call(0x70008d2374, 0x7000886a2b, 0x7000886a26);
        }
        196 => {
            c.w64(3, c.r64(0));
            blk = 199; continue;
        }
        197 => {
            return RegfileExit::fault(0x7000886a30);
        }
        198 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000886a31); };
            c.w64(0, t231);
            c.w64(3, c.r64(0));
            blk = 199; continue;
        }
        199 => {
            c.w64(0, c.r64(3));
            blk = 200; continue;
        }
        200 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886a3e); };
            c.w64(6, t232);
            let Some(t233) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000886a42); };
            c.w32(7, t233);
            let r234 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r234);
            let Some(t235) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000886a48); };
            c.w32(7, t235);
            let r236 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r236);
            let Some(t237) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000886a4e); };
            c.w32(11, t237);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000886a58); }
            c.cmp(c.r64(11), 0x2cu64, 64);
            if c.cc_a() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            return RegfileExit::fault(0x7000886a62);
        }
        202 => {
            let Some(t238) = c.ld32(c.r64(7).wrapping_add(0xb7)) else { return RegfileExit::fault(0x7000886a63); };
            c.w32(7, t238);
            let r239 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r239);
            let Some(t240) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000886a6c); };
            c.cmp(t240, c.r32(7), 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            return RegfileExit::fault(0x7000886a75);
        }
        204 => {
            let Some(t241) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000886a76); };
            c.w32(11, t241);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            return RegfileExit::fault(0x7000886a86);
        }
        206 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886a87); };
            c.w64(3, t242);
            let Some(t243) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000886a8b); };
            c.cmp(t243, c.r32(3), 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_e() { blk = 210; continue; }
            blk = 208; continue;
        }
        208 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000884314, 0x7000886a9e, 0x7000886a99);
        }
        209 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000886a9e); };
            c.w64(0, t244);
            blk = 211; continue;
        }
        210 => {
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000886aa4); };
            c.w64(11, t245);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000886aab); }
            blk = 211; continue;
        }
        211 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000886aaf); };
            c.w64(7, t246);
            let r247 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r247);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000886aba); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000886abb); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000886ac1); }
            return c.ret(0x0, 0x7000886ac3);
        }
        212 => {
            return RegfileExit::fault(0x7000886ac4);
        }
        213 => {
            c.w32(7, 0x192u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000886aca); }
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000886acb); };
            c.w64(3, t248);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000886adc, 0x7000886ad7);
        }
        214 => {
            return RegfileExit::fault(0x7000886adc);
        }
        215 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000886add); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000886ae3); };
            c.w64(3, t249);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x7000886af2, 0x7000886aed);
        }
        216 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886af2); };
            c.w64(6, t250);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000886af6); };
            c.w64(0, t251);
            blk = 1; continue;
        }
        217 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a2054, 0x7000886b07, 0x7000886b02);
        }
        218 => {
            c.w64(12, c.r64(0));
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886b0a); };
            c.w64(6, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000886b0e); };
            c.w64(15, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000886b12); };
            c.w64(7, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000886b19); };
            c.w64(11, t255);
            blk = 9; continue;
        }
        219 => {
            let Some(t256) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000886b25); };
            c.w32(0, t256);
            let r257 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r257);
            blk = 28; continue;
        }
        220 => {
            let Some(t258) = c.ld32(c.r64(0).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000886b32); };
            c.cmp(t258, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000886b3e); };
            c.w64(3, t259);
            c.w64(0, c.r64(12));
            return c.call(0x7000870914, 0x7000886b4d, 0x7000886b48);
        }
        222 => {
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000886b4d); };
            c.w64(11, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886b54); };
            c.w64(6, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886b58); };
            c.w64(12, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000886b5c); };
            c.w64(15, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000886b60); };
            c.w64(7, t264);
            blk = 28; continue;
        }
        223 => {
            c.w64(0, c.r64(1));
            return c.call(0x700089c794, 0x7000886b74, 0x7000886b6f);
        }
        224 => {
            c.w64(1, c.r64(0));
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886b77); };
            c.w64(6, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886b7b); };
            c.w64(12, t266);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000886b7f); };
            c.w64(15, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000886b83); };
            c.w64(7, t268);
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000886b8a); };
            c.w64(11, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000886b91); };
            c.w64(0, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000886b98); };
            c.w64(2, t271);
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000886b9f); };
            c.w64(3, t272);
            blk = 47; continue;
        }
        225 => {
            c.w32(2, 0x10u64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x7000886bb5, 0x7000886bb0);
        }
        226 => {
            c.w64(7, c.r64(0));
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886bb8); };
            c.w64(6, t273);
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886bbc); };
            c.w64(12, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000886bc0); };
            c.w64(15, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000886bc4); };
            c.w64(1, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000886bc8); };
            c.w64(0, t277);
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000886bcf); };
            c.w64(2, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000886bd6); };
            c.w64(3, t279);
            blk = 56; continue;
        }
        227 => {
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886be1); };
            c.w64(6, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886be5); };
            c.w64(11, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000886be9); };
            c.w64(15, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000886bed); };
            c.w64(1, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000886bf1); };
            c.w64(0, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000886bf8); };
            c.w64(2, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000886bff); };
            c.w64(3, t286);
            blk = 73; continue;
        }
        228 => {
            let Some(t287) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000886c08); };
            c.w32(12, t287);
            let r288 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r288);
            let Some(t289) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000886c0f); };
            c.w32(8, t289);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            return RegfileExit::fault(0x7000886c20);
        }
        230 => {
            let Some(t290) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000886c21); };
            c.w32(12, t290);
            c.w32(12, c.r32(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w32(12, 0x548u64);
            blk = 100; continue;
        }
        232 => {
            let Some(t291) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000886c3a); };
            c.cmp(t291, c.r32(1), 32);
            if c.cc_e() { blk = 255; continue; }
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            let r292 = c.sub(c.r32(3), 0x2u64, 32);
            c.w32(3, r292);
            if c.cc_o() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 250; continue;
        }
        236 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000886c61); }
            blk = 239; continue;
        }
        238 => {
            let r293 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r293);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 239; continue;
        }
        239 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x7000886c90); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 245; continue; }
            blk = 240; continue;
        }
        240 => {
            if c.cc_ne() { blk = 245; continue; }
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            c.movq_rx(12, 0);
            if let Some(sr) = c.shr(c.r64(12), 0x20u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 245; continue; }
            blk = 243; continue;
        }
        243 => {
            c.w64(12, c.r64(7));
            let r294 = c.add(c.r32(12), c.r32(7), 32);
            c.w32(12, r294);
            if c.cc_o() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 250; continue;
        }
        245 => {
            let Some(t295) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000886cc1); };
            c.w64(7, t295);
            let Some(t296) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000886cc8); };
            c.w64(12, t296);
            c.w64(0, c.r64(12).wrapping_add(0xc));
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000886cd0); };
            c.w64(3, t297);
            let Some(t298) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000886cd7); };
            c.cmp(t298, c.r64(0), 64);
            if c.cc_a() { blk = 248; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000886ce6, 0x7000886ce1);
        }
        247 => {
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000886ce6); };
            c.w64(1, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000886cea); };
            c.w64(6, t300);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000886cee); };
            c.w64(11, t301);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000886cf2); };
            c.w64(15, t302);
            blk = 249; continue;
        }
        248 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x7000886cf8); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 249; continue;
        }
        249 => {
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000886d03); };
            c.w64(7, t303);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000886d0a); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x7000886d0d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000886d12); }
            blk = 250; continue;
        }
        250 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 253; continue; }
            blk = 251; continue;
        }
        251 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            return c.call(0x70008d2374, 0x7000886d26, 0x7000886d21);
        }
        252 => {
            blk = 200; continue;
        }
        253 => {
            let Some(t304) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000886d2b); };
            c.w64(3, t304);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000886d39, 0x7000886d34);
        }
        254 => {
            return RegfileExit::fault(0x7000886d39);
        }
        255 => {
            let Some(t305) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000886d3a); };
            c.w64(0, t305);
            blk = 200; continue;
        }
        256 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000886d46); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x7000886d52, 0x7000886d4d);
        }
        257 => {
            c.w64(12, c.r64(0));
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000886d58); };
            c.w64(8, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000886d5c); };
            c.w64(7, t307);
            blk = 112; continue;
        }
        258 => {
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000886d68); };
            c.w64(0, t308);
            let Some(t309) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000886d6f); };
            c.w32(15, t309);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x7000886d7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000886d80); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(15)) { return RegfileExit::fault(0x7000886d84); }
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000886d88); };
            c.w64(2, t310);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_l() { blk = 279; continue; }
            blk = 259; continue;
        }
        259 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_b() { blk = 339; continue; }
            blk = 260; continue;
        }
        260 => {
            c.w64(1, c.r64(3));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r311 = c.add(c.r64(3), c.r64(1), 64);
            c.w64(3, r311);
            let r312 = c.add(c.r64(3), 0x10u64, 64);
            c.w64(3, r312);
            if c.cc_e() { blk = 276; continue; }
            blk = 261; continue;
        }
        261 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 344; continue; }
            blk = 262; continue;
        }
        262 => {
            let r313 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r313);
            c.w64(1, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(1));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w32(6, c.r32(6));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(3)) { return RegfileExit::fault(0x7000886dcc); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(1)) { return RegfileExit::fault(0x7000886dd3); }
            let r314 = c.imul2(c.r32(6), 0x4u64, 32);
            c.w32(6, r314);
            if c.cc_o() { blk = 343; continue; }
            blk = 263; continue;
        }
        263 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r315 = c.add(c.r64(6), c.r64(8), 64);
            c.w64(6, r315);
            if c.cc_o() { blk = 342; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t316) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000886df2); };
            c.w64(9, t316);
            let Some(t317) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000886df9); };
            c.w64(9, t317);
            c.w64(7, c.r64(9).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(9)) { return RegfileExit::fault(0x7000886e00); }
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000886e07); };
            c.w64(9, t318);
            let Some(t319) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000886e0e); };
            c.cmp(t319, c.r64(7), 64);
            if c.cc_be() { blk = 266; continue; }
            blk = 265; continue;
        }
        265 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_b() { blk = 268; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x7000886e24, 0x7000886e1f);
        }
        267 => {
            c.w64(7, c.r64(0));
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000886e27); };
            c.w64(0, t320);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000886e2e); };
            c.w64(2, t321);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000886e32); };
            c.w64(11, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000886e36); };
            c.w64(15, t323);
            c.w32(8, 0x8u64);
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000886e40); };
            c.w64(1, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000886e44); };
            c.w64(3, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000886e4b); };
            c.w64(12, t326);
            blk = 269; continue;
        }
        268 => {
            let Some(t327) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000886e54); };
            c.w64(6, t327);
            if !c.st64(c.r64(6), c.r64(7)) { return RegfileExit::fault(0x7000886e5b); }
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000886e5e); };
            c.w64(7, t328);
            let r329 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r329);
            blk = 269; continue;
        }
        269 => {
            let Some(t330) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000886e69); };
            c.w64(6, t330);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000886e70); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000886e73); }
            c.w64(3, c.r64(1));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(8));
            blk = 270; continue;
        }
        270 => {
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_e() { blk = 275; continue; }
            blk = 271; continue;
        }
        271 => {
            c.w64(6, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_e() { blk = 273; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t331) = c.ld32(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000886e9c); };
            c.w32(6, t331);
            let r332 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r332);
            blk = 274; continue;
        }
        273 => {
            let Some(t333) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000886ea7); };
            c.w64(6, t333);
            blk = 274; continue;
        }
        274 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000886eae); }
            let r334 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r334);
            blk = 270; continue;
        }
        275 => {
            c.w64(0, c.r64(7));
            blk = 277; continue;
        }
        276 => {
            let Some(t335) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000886ebe); };
            c.w64(0, t335);
            blk = 277; continue;
        }
        277 => {
            let Some(t336) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000886ec5); };
            c.w32(7, t336);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(6, c.r64(2));
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_ae() { blk = 346; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000886eda); };
            c.w64(15, t337);
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st32(c.r64(0).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000886ee2); }
            c.w64(1, c.r64(0));
            blk = 281; continue;
        }
        279 => {
            c.w64(1, c.r64(0));
            c.w64(6, c.r64(2));
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_ae() { blk = 347; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000886efa); };
            c.w64(15, t338);
            let r339 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r339);
            if !c.st32(c.r64(1).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000886f01); }
            blk = 281; continue;
        }
        281 => {
            c.w64(7, c.r64(6).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            c.w64(15, c.r64(7));
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000886f10); };
            c.w64(7, t340);
            blk = 145; continue;
        }
        282 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000886f19); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000886f1d); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x7000886f21); }
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(1)) { return RegfileExit::fault(0x7000886f25); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000886f2c); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_b() { blk = 352; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w64(3, c.r64(0));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r341 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r341);
            let r342 = c.add(c.r64(0), 0x10u64, 64);
            c.w64(0, r342);
            if c.cc_e() { blk = 299; continue; }
            blk = 284; continue;
        }
        284 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 357; continue; }
            blk = 285; continue;
        }
        285 => {
            let r343 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r343);
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(2, c.r32(2));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(0)) { return RegfileExit::fault(0x7000886f66); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x7000886f6d); }
            let r344 = c.imul2(c.r32(2), 0x4u64, 32);
            c.w32(2, r344);
            if c.cc_o() { blk = 356; continue; }
            blk = 286; continue;
        }
        286 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r345 = c.add(c.r64(2), c.r64(8), 64);
            c.w64(2, r345);
            if c.cc_o() { blk = 355; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000886f8c); };
            c.w64(9, t346);
            let Some(t347) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000886f93); };
            c.w64(9, t347);
            c.w64(7, c.r64(9).wrapping_add(c.r64(2)));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(9)) { return RegfileExit::fault(0x7000886f9a); }
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000886f9e); };
            c.w64(9, t348);
            let Some(t349) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000886fa5); };
            c.cmp(t349, c.r64(7), 64);
            if c.cc_be() { blk = 289; continue; }
            blk = 288; continue;
        }
        288 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_b() { blk = 291; continue; }
            blk = 289; continue;
        }
        289 => {
            return c.call(0x70007febb4, 0x7000886fb8, 0x7000886fb3);
        }
        290 => {
            c.w64(7, c.r64(0));
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000886fbb); };
            c.w64(15, t350);
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000886fbf); };
            c.w64(6, t351);
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000886fc3); };
            c.w64(1, t352);
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000886fca); };
            c.w64(11, t353);
            c.w32(8, 0x8u64);
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000886fd4); };
            c.w64(3, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000886fd8); };
            c.w64(12, t355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000886fdf); };
            c.w64(0, t356);
            blk = 292; continue;
        }
        291 => {
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000886fe8); };
            c.w64(2, t357);
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x7000886fef); }
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000886ff2); };
            c.w64(7, t358);
            let r359 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r359);
            blk = 292; continue;
        }
        292 => {
            let Some(t360) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000886ffa); };
            c.w64(2, t360);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000887001); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000887004); }
            c.w64(0, c.r64(3));
            c.w64(0, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(8));
            blk = 293; continue;
        }
        293 => {
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_e() { blk = 298; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w64(2, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            let Some(t361) = c.ld32(c.r64(1).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887027); };
            c.w32(2, t361);
            let r362 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r362);
            blk = 297; continue;
        }
        296 => {
            let Some(t363) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000887031); };
            c.w64(2, t363);
            blk = 297; continue;
        }
        297 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000887038); }
            let r364 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r364);
            blk = 293; continue;
        }
        298 => {
            c.w64(0, c.r64(7));
            blk = 300; continue;
        }
        299 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000887048); };
            c.w64(0, t365);
            blk = 300; continue;
        }
        300 => {
            let Some(t366) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088704f); };
            c.w32(7, t366);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_ae() { blk = 359; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(7, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000887069); };
            c.w64(11, t367);
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(11)) { return RegfileExit::fault(0x700088706d); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r368 = c.r64(3) & c.r64(0);
            c.w64(3, r368);
            c.logic(r368, 64);
            let Some(t369) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000887081); };
            c.logic(t369 & 0x4u64, 8);
            if c.cc_ne() { blk = 378; continue; }
            blk = 303; continue;
        }
        303 => {
            c.w64(1, c.r64(0));
            c.w64(7, c.r64(11));
            blk = 153; continue;
        }
        304 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000887096); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700088709a); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x700088709e); }
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(11)) { return RegfileExit::fault(0x70008870a2); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_ae() { blk = 308; continue; }
            blk = 305; continue;
        }
        305 => {
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008870af); };
            c.w64(2, t370);
            let Some(t371) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008870b6); };
            c.w32(2, t371);
            let r372 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r372);
            return c.call(0x7000824234, 0x70008870c4, 0x70008870bf);
        }
        306 => {
            return RegfileExit::fault(0x70008870c4);
        }
        307 => {
            return RegfileExit::fault(0x70008870c5);
        }
        308 => {
            c.w64(2, c.r64(3));
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r373 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r373);
            let r374 = c.add(c.r64(3), 0x10u64, 64);
            c.w64(3, r374);
            if c.cc_e() { blk = 327; continue; }
            blk = 309; continue;
        }
        309 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 325; continue; }
            blk = 310; continue;
        }
        310 => {
            let r375 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r375);
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(1, c.r32(1));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x70008870f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(2)) { return RegfileExit::fault(0x70008870f7); }
            let r376 = c.imul2(c.r32(1), 0x4u64, 32);
            c.w32(1, r376);
            if c.cc_o() { blk = 324; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r377 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r377);
            if c.cc_o() { blk = 323; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t378) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000887116); };
            c.w64(9, t378);
            let Some(t379) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088711d); };
            c.w64(9, t379);
            c.w64(7, c.r64(9).wrapping_add(c.r64(1)));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x7000887124); }
            let Some(t380) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000887128); };
            c.w64(9, t380);
            let Some(t381) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088712f); };
            c.cmp(t381, c.r64(7), 64);
            if c.cc_be() { blk = 314; continue; }
            blk = 313; continue;
        }
        313 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_b() { blk = 316; continue; }
            blk = 314; continue;
        }
        314 => {
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000887145, 0x7000887140);
        }
        315 => {
            c.w64(7, c.r64(0));
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000887148); };
            c.w64(6, t382);
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x700088714c); };
            c.w64(11, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000887153); };
            c.w64(0, t384);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000887157); };
            c.w64(12, t385);
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088715b); };
            c.w64(15, t386);
            c.w32(8, 0x8u64);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000887165); };
            c.w64(2, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000887169); };
            c.w64(3, t388);
            blk = 317; continue;
        }
        316 => {
            let Some(t389) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088716f); };
            c.w64(1, t389);
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000887176); }
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000887179); };
            c.w64(7, t390);
            let r391 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r391);
            blk = 317; continue;
        }
        317 => {
            let Some(t392) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000887181); };
            c.w64(1, t392);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000887188); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700088718b); }
            c.w64(3, c.r64(2));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(11));
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088719c); };
            c.w64(11, t393);
            c.w64(2, c.r64(8));
            blk = 318; continue;
        }
        318 => {
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_e() { blk = 328; continue; }
            blk = 319; continue;
        }
        319 => {
            c.w64(9, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_e() { blk = 321; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t394) = c.ld32(c.r64(1).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008871b5); };
            c.w32(9, t394);
            let r395 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r395);
            blk = 322; continue;
        }
        321 => {
            let Some(t396) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008871c0); };
            c.w64(9, t396);
            blk = 322; continue;
        }
        322 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008871c7); }
            let r397 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r397);
            blk = 318; continue;
        }
        323 => {
            return RegfileExit::fault(0x70008871d2);
        }
        324 => {
            return RegfileExit::fault(0x70008871d3);
        }
        325 => {
            let Some(t398) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008871d4); };
            c.w64(3, t398);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x70008871e5, 0x70008871e0);
        }
        326 => {
            return RegfileExit::fault(0x70008871e5);
        }
        327 => {
            let Some(t399) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008871e6); };
            c.w64(7, t399);
            blk = 328; continue;
        }
        328 => {
            let Some(t400) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008871ed); };
            c.w32(11, t400);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_b() { blk = 330; continue; }
            blk = 329; continue;
        }
        329 => {
            return RegfileExit::fault(0x70008871fc);
        }
        330 => {
            c.w32(11, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000887201); }
            c.w64(11, c.r64(7));
            blk = 164; continue;
        }
        331 => {
            let Some(t401) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x700088720e); };
            c.w64(3, t401);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000887217); };
            c.w64(6, t402);
            return c.call(0x700086ed74, 0x7000887220, 0x700088721b);
        }
        332 => {
            return RegfileExit::fault(0x7000887220);
        }
        333 => {
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000887221); };
            c.w64(3, t403);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088722a); };
            c.w64(6, t404);
            return c.call(0x700086ed74, 0x7000887233, 0x700088722e);
        }
        334 => {
            return RegfileExit::fault(0x7000887233);
        }
        335 => {
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(0)) { return RegfileExit::fault(0x7000887234); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700088723b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700088723f); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x700088724b, 0x7000887246);
        }
        336 => {
            c.w64(11, c.r64(0));
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088724e); };
            c.w64(6, t405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000887252); };
            c.w64(0, t406);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000887259); };
            c.w64(7, t407);
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088725d); };
            c.w64(12, t408);
            blk = 175; continue;
        }
        337 => {
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(0)) { return RegfileExit::fault(0x7000887266); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700088726d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000887271); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x700088727d, 0x7000887278);
        }
        338 => {
            c.w64(11, c.r64(0));
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000887280); };
            c.w64(6, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000887284); };
            c.w64(0, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088728b); };
            c.w64(7, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088728f); };
            c.w64(12, t412);
            blk = 184; continue;
        }
        339 => {
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000887298); };
            c.w64(2, t413);
            let Some(t414) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x700088729f); };
            c.w32(2, t414);
            let r415 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r415);
            return c.call(0x7000824234, 0x70008872ad, 0x70008872a8);
        }
        340 => {
            return RegfileExit::fault(0x70008872ad);
        }
        341 => {
            return RegfileExit::fault(0x70008872ae);
        }
        342 => {
            return RegfileExit::fault(0x70008872af);
        }
        343 => {
            return RegfileExit::fault(0x70008872b0);
        }
        344 => {
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008872b1); };
            c.w64(3, t416);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008872c1, 0x70008872bc);
        }
        345 => {
            return RegfileExit::fault(0x70008872c1);
        }
        346 => {
            return RegfileExit::fault(0x70008872c2);
        }
        347 => {
            return RegfileExit::fault(0x70008872c3);
        }
        348 => {
            let Some(t417) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x70008872c4); };
            c.w64(3, t417);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008872cd); };
            c.w64(6, t418);
            return c.call(0x700086ed74, 0x70008872d6, 0x70008872d1);
        }
        349 => {
            return RegfileExit::fault(0x70008872d6);
        }
        350 => {
            let Some(t419) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x70008872d7); };
            c.w64(3, t419);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008872e0); };
            c.w64(6, t420);
            return c.call(0x700086ed74, 0x70008872e9, 0x70008872e4);
        }
        351 => {
            return RegfileExit::fault(0x70008872e9);
        }
        352 => {
            let Some(t421) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008872ea); };
            c.w64(2, t421);
            let Some(t422) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008872f1); };
            c.w32(2, t422);
            let r423 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r423);
            return c.call(0x7000824234, 0x70008872ff, 0x70008872fa);
        }
        353 => {
            return RegfileExit::fault(0x70008872ff);
        }
        354 => {
            return RegfileExit::fault(0x7000887300);
        }
        355 => {
            return RegfileExit::fault(0x7000887301);
        }
        356 => {
            return RegfileExit::fault(0x7000887302);
        }
        357 => {
            let Some(t424) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000887303); };
            c.w64(3, t424);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000887313, 0x700088730e);
        }
        358 => {
            return RegfileExit::fault(0x7000887313);
        }
        359 => {
            return RegfileExit::fault(0x7000887314);
        }
        360 => {
            let Some(t425) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000887315); };
            c.w64(3, t425);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088731e); };
            c.w64(6, t426);
            return c.call(0x700086ed74, 0x7000887327, 0x7000887322);
        }
        361 => {
            return RegfileExit::fault(0x7000887327);
        }
        362 => {
            let Some(t427) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000887328); };
            c.w64(3, t427);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(9));
            return c.call(0x700086ed74, 0x7000887339, 0x7000887334);
        }
        363 => {
            blk = 377; continue;
        }
        364 => {
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088733e); };
            c.w64(6, t428);
            let Some(t429) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887342); };
            c.w32(7, t429);
            let r430 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r430);
            let Some(t431) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000887348); };
            c.w32(7, t431);
            let r432 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r432);
            let Some(t433) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088734e); };
            c.w32(11, t433);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000887358); }
            c.cmp(c.r64(11), 0x2cu64, 64);
            if c.cc_a() { blk = 366; continue; }
            blk = 365; continue;
        }
        365 => {
            return RegfileExit::fault(0x7000887362);
        }
        366 => {
            let Some(t434) = c.ld32(c.r64(7).wrapping_add(0xb7)) else { return RegfileExit::fault(0x7000887363); };
            c.w32(7, t434);
            let r435 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r435);
            let Some(t436) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088736c); };
            c.cmp(t436, c.r32(7), 32);
            if c.cc_ne() { blk = 368; continue; }
            blk = 367; continue;
        }
        367 => {
            return RegfileExit::fault(0x7000887375);
        }
        368 => {
            let Some(t437) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000887376); };
            c.w32(11, t437);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 370; continue; }
            blk = 369; continue;
        }
        369 => {
            return RegfileExit::fault(0x7000887386);
        }
        370 => {
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000887387); };
            c.w64(3, t438);
            let Some(t439) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088738b); };
            c.cmp(t439, c.r32(3), 32);
            if c.cc_ne() { blk = 372; continue; }
            blk = 371; continue;
        }
        371 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_e() { blk = 374; continue; }
            blk = 372; continue;
        }
        372 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000884314, 0x700088739e, 0x7000887399);
        }
        373 => {
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088739e); };
            c.w64(6, t440);
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008873a2); };
            c.w64(0, t441);
            blk = 375; continue;
        }
        374 => {
            let Some(t442) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008873a8); };
            c.w64(11, t442);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008873af); }
            blk = 375; continue;
        }
        375 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008873b3); }
            let Some(t443) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x70008873b4); };
            c.w64(3, t443);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008873c5, 0x70008873c0);
        }
        376 => {
            return RegfileExit::fault(0x70008873c5);
        }
        377 => {
            return RegfileExit::fault(0x70008873c6);
        }
        378 => {
            c.w32(11, c.r32(11));
            let r444 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r444);
            c.w64(3, 0xfffffffffffc0000u64);
            let r445 = c.r64(3) & c.r64(11);
            c.w64(3, r445);
            c.logic(r445, 64);
            let Some(t446) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008873d7); };
            c.logic(t446 & 0x2u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 379; continue;
        }
        379 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008873e5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008873e6); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008873f3, 0x70008873ee);
        }
        380 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008873f3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008873f5); }
            blk = 303; continue;
        }
        381 => {
            c.w32(7, c.r32(7));
            let r447 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r447);
            c.w64(0, 0xfffffffffffc0000u64);
            let r448 = c.r64(0) & c.r64(7);
            c.w64(0, r448);
            c.logic(r448, 64);
            let Some(t449) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088740a); };
            c.logic(t449 & 0x2u64, 8);
            if c.cc_e() { blk = 153; continue; }
            blk = 382; continue;
        }
        382 => {
            c.w64(3, c.r64(1).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000887418); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000887419); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x7000887426, 0x7000887421);
        }
        383 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000887426); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000887428); }
            blk = 153; continue;
        }
        384 => {
            c.w32(12, c.r32(12));
            let r450 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r450);
            c.w64(9, 0xfffffffffffc0000u64);
            let r451 = c.r64(9) & c.r64(12);
            c.w64(9, r451);
            c.logic(r451, 64);
            let Some(t452) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088743e); };
            c.logic(t452 & 0x2u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 385; continue;
        }
        385 => {
            c.w64(7, c.r64(8).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088744d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088744e); }
            let (x453, x454) = (c.r64(8), c.r64(7));
            c.w64(8, x454);
            c.w64(7, x453);
            return c.call(0x70007f0e54, 0x7000887458, 0x7000887453);
        }
        386 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000887458); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700088745a); }
            blk = 71; continue;
        }
        387 => {
            c.w32(7, c.r32(7));
            let r455 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r455);
            c.w64(8, 0xfffffffffffc0000u64);
            let r456 = c.r64(8) & c.r64(7);
            c.w64(8, r456);
            c.logic(r456, 64);
            let Some(t457) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088746f); };
            c.logic(t457 & 0x2u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 388; continue;
        }
        388 => {
            c.w64(9, c.r64(12).wrapping_add(0xb7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000887482); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000887483); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000887490, 0x700088748b);
        }
        389 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000887490); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000887492); }
            blk = 62; continue;
        }
        390 => {
            return RegfileExit::jump(0x700088749c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008874b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008874b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008874b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008874b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008874ba); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008874bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008874c3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008874c7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008874db); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008874e9); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008874ef); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008874fb); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008874ff); };
            c.w64(3, t6);
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x13f)) else { return RegfileExit::fault(0x7000887506); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            c.w64(0, c.r64(2));
            return c.call(0x7000870914, 0x7000887517, 0x7000887512);
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000887517); };
            c.w64(2, (t9 as u32 as i32 as i64 as u64));
            let r10 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r10);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700088751f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887527); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088752d); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000887533); };
            if !c.push64(t14) { return RegfileExit::fault(0x7000887533); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088753b); };
            c.w64(6, t15);
            return c.call(0x70007f2274, 0x7000887544, 0x700088753f);
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000887544); };
            c.w64(2, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088754b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088754c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000887552); }
            return c.ret(0x0, 0x7000887554);
        }
        8 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000887555); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000887559); };
            c.w64(6, t18);
            return c.call(0x7000856b94, 0x7000887562, 0x700088755d);
        }
        9 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000887562); };
            c.w64(2, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000887569); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088756a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000887570); }
            return c.ret(0x0, 0x7000887572);
        }
        10 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000887575); };
            c.w64(3, t20);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x7000887584, 0x700088757f);
        }
        11 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000887584); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000887588); };
            c.w64(0, t22);
            blk = 1; continue;
        }
        12 => {
            c.w64(0, c.r64(2));
            return c.call(0x70008a2054, 0x7000887599, 0x7000887594);
        }
        13 => {
            c.w64(2, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088759c); };
            c.w64(6, t23);
            blk = 3; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008875a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008875b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008875b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008875b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008875b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008875ba); }
            let r0 = c.sub(c.r64(4), 0xb8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008875c2); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008875c6); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 215; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008875da); };
            c.w64(11, t2);
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008875e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008875e6); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008875f0); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008875f6); };
            c.w64(7, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 383; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887607); };
            c.w32(15, t5);
            let r6 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r6);
            let Some(t7) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088760e); };
            c.cmp(t7, 0x82eu64, 16);
            if c.cc_ne() { blk = 383; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088761b); };
            c.w32(15, t8);
            let r9 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r9);
            let Some(t10) = c.ld8(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000887622); };
            c.logic(t10 & 0x4u64, 8);
            if c.cc_ne() { blk = 381; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700088762d); };
            c.w64(15, t11);
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 217; continue; }
            blk = 8; continue;
        }
        8 => {
            let r12 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r12);
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x7000887641); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000887645); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 226; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t14) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887658); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            let Some(t16) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088765e); };
            c.cmp(t16, 0x40u64, 16);
            if c.cc_b() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 226; continue;
        }
        13 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088766a); };
            c.w64(7, t17);
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000887671); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(7)) { return RegfileExit::fault(0x7000887684); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700088768b); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000887694); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 40; continue;
        }
        17 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_le() { blk = 40; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008876b3); };
            c.w32(3, t20);
            let r21 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r21);
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008876b9); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008876bf); };
            c.w32(2, t24);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2cu64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008876ce);
        }
        20 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0xb7)) else { return RegfileExit::fault(0x70008876cf); };
            c.w32(3, t25);
            let r26 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r26);
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008876d8); };
            c.cmp(t27, c.r32(3), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008876e5); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008876ec); };
            c.w64(2, t29);
            c.w64(1, c.r64(2).wrapping_add(0x10));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008876f3); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008876fa); };
            c.cmp(t31, c.r64(1), 64);
            if c.cc_be() { blk = 228; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x7000887703); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 23; continue;
        }
        23 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700088770a); };
            c.w64(2, t32);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000887711); }
            c.w32(2, 0x4u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x7000887719); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088771c); };
            c.w64(2, t33);
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x7000887723); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x7000887726); }
            let Some(t34) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000887729); };
            c.w32(2, t34);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return RegfileExit::fault(0x7000887738);
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000887739); }
            let Some(t35) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088773d); };
            c.w32(2, t35);
            let r36 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r36);
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000887743); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000887749); };
            c.w32(1, t39);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2cu64, 64);
            if c.cc_a() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            return RegfileExit::fault(0x7000887758);
        }
        27 => {
            if !c.st32(c.r64(2).wrapping_add(0xb7), c.r32(3)) { return RegfileExit::fault(0x7000887759); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r40 = c.r64(1) & c.r64(2);
            c.w64(1, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088776e); };
            c.logic(t41 & 0x4u64, 8);
            if c.cc_ne() { blk = 400; continue; }
            blk = 29; continue;
        }
        29 => {
            blk = 38; continue;
        }
        30 => {
            let Some(t42) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088777a); };
            c.w32(2, t42);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x7000887789);
        }
        32 => {
            let Some(t43) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088778a); };
            c.w32(2, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088778d); };
            c.cmp(t44, c.r32(2), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x7000883ff4, 0x70008877a1, 0x700088779c);
        }
        34 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008877a1); };
            c.cmp(t45, c.r32(0), 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008877ae); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008877b2); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008877b6); };
            c.w64(12, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x70008877ba); };
            c.w64(7, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008877c1); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008877c5); };
            c.w64(0, t51);
            blk = 38; continue;
        }
        36 => {
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008877cb); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r52 = c.r64(2) & c.r64(3);
            c.w64(2, r52);
            c.logic(r52, 64);
            let Some(t53) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008877df); };
            c.logic(t53 & 0x4u64, 8);
            if c.cc_ne() { blk = 397; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0x1u64);
            blk = 39; continue;
        }
        39 => {
            c.w32(3, 0x1u64);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(7, c.r64(12).wrapping_add(0x1));
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008877fd); };
            c.w64(0, t54);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000887807); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000887808); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088780e); }
            return c.ret(0x0, 0x7000887810);
        }
        41 => {
            let Some(t55) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887811); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000887818); };
            c.w32(2, t57);
            c.w64(1, c.r64(2));
            let r58 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r58);
            let r59 = c.r32(2) & 0xfcu64;
            c.w32(2, r59);
            c.logic(r59, 32);
            let r60 = c.r32(1) & 0x3fu64;
            c.w32(1, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(1), 0x15u64, 32);
            if c.cc_g() { blk = 53; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(2), 0x44u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(2), 0x48u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(2), 0x4cu64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(2), 0x50u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(2), 0x54u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            return RegfileExit::fault(0x7000887849);
        }
        48 => {
            c.w32(2, 0x8bau64);
            blk = 66; continue;
        }
        49 => {
            c.w32(2, 0x8b8u64);
            blk = 66; continue;
        }
        50 => {
            c.w32(2, 0x8b6u64);
            blk = 66; continue;
        }
        51 => {
            c.w32(2, 0x8b4u64);
            blk = 66; continue;
        }
        52 => {
            c.w32(2, 0x8b2u64);
            blk = 66; continue;
        }
        53 => {
            c.cmp(c.r32(2), 0x58u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(2), 0x60u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(2), 0x64u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(2), 0x68u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(2), 0x6cu64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x7000887894);
        }
        60 => {
            c.w32(2, 0x8b0u64);
            blk = 66; continue;
        }
        61 => {
            c.w32(2, 0x8aeu64);
            blk = 66; continue;
        }
        62 => {
            c.w32(2, 0x8acu64);
            blk = 66; continue;
        }
        63 => {
            c.w32(2, 0x8aau64);
            blk = 66; continue;
        }
        64 => {
            c.w32(2, 0x8a8u64);
            blk = 66; continue;
        }
        65 => {
            c.w32(2, 0x8a6u64);
            blk = 66; continue;
        }
        66 => {
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(2)) { return RegfileExit::fault(0x70008878bd); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            blk = 71; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008878d1); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_s() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(10, 0x8000000000000000u64);
            let r61 = c.r64(15) | c.r64(10);
            c.w64(15, r61);
            c.logic(r61, 64);
            blk = 71; continue;
        }
        71 => {
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(15)) { return RegfileExit::fault(0x700088790c); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_a() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(0, c.r64(15));
            blk = 74; continue;
        }
        73 => {
            c.w32(0, 0x7ffeu64);
            blk = 74; continue;
        }
        74 => {
            let Some(t62) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000887926); };
            c.w32(1, t62);
            if !c.st64(c.r64(5).wrapping_sub(0xc8), c.r64(1)) { return RegfileExit::fault(0x7000887929); }
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 365; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088793c); };
            c.w64(8, t63);
            let Some(t64) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000887943); };
            c.w64(8, t64);
            c.w64(9, c.r64(0).wrapping_mul(4));
            c.w64(7, c.r64(9).wrapping_add(0x8));
            c.w64(11, c.r64(7).wrapping_add(c.r64(8)));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000887956); };
            c.w64(12, t65);
            let Some(t66) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700088795d); };
            c.cmp(t66, c.r64(11), 64);
            if c.cc_be() { blk = 231; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 231; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000887974); };
            c.w64(7, t67);
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700088797b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 78; continue;
        }
        78 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000887986); };
            c.w64(12, t68);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700088798d); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000887991); }
            c.w64(11, c.r64(7).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x0u64, 64);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.w64(10, c.r64(4));
            let r69 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r69);
            let r70 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r70);
            c.logic(r70, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008879b0); }
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(7)) { return RegfileExit::fault(0x70008879b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008879bc); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(11));
            c.w64(8, c.r64(9));
            c.w64(2, c.r64(0));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008879cb); };
            c.w64(0, t71);
            c.w64(10, 0x70008879d9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008879d9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008879dd); }
            return c.call(c.r64(0), 0x70008879e3, 0x70008879e1);
        }
        79 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008879e3); }
            let Some(t72) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008879eb); };
            c.w64(4, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x70008879f0); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008879f7); };
            c.w32(11, t74);
            let r75 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r75);
            let Some(t76) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008879fe); };
            c.w32(11, t76);
            let r77 = c.r32(11) & 0x8u64;
            c.w32(11, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000887a13); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(2));
            c.w64(12, c.r64(2));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000887a1f); }
            blk = 81; continue;
        }
        80 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000887a25); };
            c.w64(2, t78);
            let r79 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000887a2d); };
            c.w64(12, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000887a31); };
            c.w64(3, t81);
            blk = 81; continue;
        }
        81 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000887a35); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000887a3c); };
            c.w64(0, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000887a43); };
            c.w64(6, t84);
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000887a47); };
            c.w64(8, t85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000887a4e); };
            c.w64(9, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000887a55); };
            c.cmp(c.r64(2), t87, 64);
            if c.cc_ae() { blk = 155; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t88) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000887a62); };
            c.w32(7, t88);
            let r89 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r89);
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(0)) { return RegfileExit::fault(0x7000887a68); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000887a6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x7000887a73); }
            let Some(t90) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000887a77); };
            c.logic(t90 & 0x4u64, 8);
            if c.cc_ne() { blk = 233; continue; }
            blk = 83; continue;
        }
        83 => {
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(1)) { return RegfileExit::fault(0x7000887a81); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            let r91 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r91);
            blk = 85; continue;
        }
        85 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000887a92); }
            c.w64(0, c.r64(6));
            c.w64(3, c.r64(2));
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000887a9c); };
            c.w64(6, t92);
            c.w64(2, c.r64(1));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000887aa3); };
            return c.call(t93, 0x7000887aab, 0x7000887aa3);
        }
        86 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t94) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887aba); };
            c.w32(11, t94);
            let r95 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r95);
            let Some(t96) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000887ac1); };
            c.cmp(t96, 0x3fu64, 16);
            if c.cc_a() { blk = 90; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t97) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000887ac9); };
            c.cmp(t97, c.r32(0), 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 89; continue;
        }
        89 => {
            blk = 139; continue;
        }
        90 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t98) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887ae0); };
            c.w32(11, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000887ae4); };
            c.cmp(t99, c.r32(11), 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x10f0)) else { return RegfileExit::fault(0x7000887af1); };
            c.w64(11, t100);
            let Some(t101) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000887af8); };
            c.w32(12, t101);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(12));
            let r102 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r102);
            let r103 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r103);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 93; continue;
        }
        93 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000887b11); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 234; continue; }
            blk = 94; continue;
        }
        94 => {
            if c.cc_ne() { blk = 234; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            c.movq_rx(3, 0);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 234; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(3, c.r64(7));
            let r104 = c.add(c.r32(3), c.r32(7), 32);
            c.w32(3, r104);
            if c.cc_o() { blk = 234; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 100; continue;
        }
        99 => {
            c.w64(7, c.r64(0));
            blk = 100; continue;
        }
        100 => {
            c.w64(3, c.r64(7));
            let r105 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r105);
            c.w32(1, c.r32(7));
            let r106 = c.r32(15) & c.r32(3);
            c.w32(15, r106);
            c.logic(r106, 32);
            let r107 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r107);
            c.w64(6, c.r64(15).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(7)) { return RegfileExit::fault(0x7000887b71); }
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(15)) { return RegfileExit::fault(0x7000887b75); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(6)) { return RegfileExit::fault(0x7000887b7c); }
            let Some(t108) = c.ld32(c.r64(11).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000887b80); };
            c.cmp(t108, c.r32(7), 32);
            if c.cc_ne() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t109) = c.ld32(c.r64(11).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000887b87); };
            c.w32(0, t109);
            let r110 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r110);
            blk = 139; continue;
        }
        102 => {
            c.cmp(c.r64(12), 0x8000u64, 64);
            if c.cc_l() { blk = 254; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            c.w8(12, (c.cc_l() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w32(2, 0x80000000u64);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 105; continue;
        }
        105 => {
            let r111 = c.neg(c.r32(3), 32);
            c.w32(3, r111);
            blk = 107; continue;
        }
        106 => {
            c.cmp(c.r32(3), 0xau64, 32);
            if c.cc_l() { blk = 129; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 110; continue;
        }
        109 => {
            c.w32(0, 0x1u64);
            blk = 110; continue;
        }
        110 => {
            c.w64(9, c.r64(3));
            c.w64(8, c.r64(0));
            c.w32(0, 0x66666667u64);
            blk = 112; continue;
        }
        111 => {
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(9));
            c.imul1(c.r32(7), 32);
            let r112 = c.shr(c.r32(9), 0x1fu64, 32).unwrap_or(c.r32(9));
            c.w32(9, r112);
            let r113 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r113);
            let r114 = c.add(c.r32(9), c.r32(2), 32);
            c.w32(9, r114);
            let r115 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r115);
            c.w64(0, c.r64(7));
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_g() { blk = 111; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(7, c.r64(0));
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(2, 0xcu64);
            let r116 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r116);
            if c.cc_o() { blk = 213; continue; }
            blk = 114; continue;
        }
        114 => {
            let r117 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r117);
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000887c1f); };
            c.w64(9, t118);
            let Some(t119) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000887c26); };
            c.w64(9, t119);
            let r120 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r120);
            c.logic(r120, 64);
            c.w64(6, c.r64(9).wrapping_add(c.r64(2)));
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000887c31); };
            c.w64(15, t121);
            let Some(t122) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000887c38); };
            c.cmp(t122, c.r64(6), 64);
            if c.cc_be() { blk = 239; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 239; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000887c4e); };
            c.w64(15, t123);
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x7000887c55); }
            c.w64(15, c.r64(9).wrapping_add(0x1));
            blk = 117; continue;
        }
        117 => {
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000887c5c); };
            c.w64(6, t124);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000887c63); }
            if !c.st32(c.r64(15).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000887c67); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000887c6f); }
            let r125 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r125);
            c.w64(9, c.r64(3));
            c.w64(3, c.r64(0));
            blk = 119; continue;
        }
        118 => {
            if !c.st8(c.r64(15).wrapping_add(c.r64(3)).wrapping_add(0xb), c.r8(2)) { return RegfileExit::fault(0x7000887c7f); }
            let r126 = c.sub(c.r64(3), 0x1u64, 64);
            c.w64(3, r126);
            c.w64(9, c.r64(0));
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_le() { blk = 124; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t127) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000887c91); };
            c.w64(0, (t127 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(3), c.r64(0), 64);
            if c.cc_ae() { blk = 212; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(0, c.r64(7));
            c.imul1(c.r32(9), 32);
            c.w64(0, c.r64(9));
            let r128 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r128);
            let r129 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r129);
            let r130 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r130);
            let r131 = c.imul2(c.r32(0), 0xau64, 32);
            c.w32(2, r131);
            let r132 = c.sub(c.r32(9), c.r32(2), 32);
            c.w32(9, r132);
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_l() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w32(2, c.r64(9).wrapping_add(0x57));
            blk = 118; continue;
        }
        123 => {
            c.w32(2, c.r64(9).wrapping_add(0x30));
            blk = 118; continue;
        }
        124 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 126; continue; }
            blk = 125; continue;
        }
        125 => {
            let r133 = c.shl(c.r32(8), 0x1au64, 32).unwrap_or(c.r32(8));
            c.w32(8, r133);
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            let r134 = c.r32(8) | c.r32(7);
            c.w32(8, r134);
            c.logic(r134, 32);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000887cd7); }
            blk = 128; continue;
        }
        126 => {
            let Some(t135) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000887cdd); };
            c.w64(7, (t135 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 211; continue; }
            blk = 127; continue;
        }
        127 => {
            if !c.st8(c.r64(15).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x7000887ceb); }
            blk = 128; continue;
        }
        128 => {
            c.w64(2, c.r64(15));
            blk = 132; continue;
        }
        129 => {
            c.w32(12, c.r64(3).wrapping_add(0x30));
            c.cmp(c.r32(12), 0xffu64, 32);
            if c.cc_g() { blk = 241; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(0, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x7000887d0e); };
            c.w64(3, t136);
            let Some(t137) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000887d15); };
            c.w32(2, t137);
            let r138 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r138);
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000887d1d); };
            c.cmp(t139, c.r32(2), 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000887d2a); };
            c.w64(6, t140);
            blk = 132; continue;
        }
        132 => {
            let Some(t141) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000887d31); };
            c.w32(7, t141);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000887d3b); };
            c.w64(12, t142);
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_ae() { blk = 339; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(7, c.r64(12).wrapping_add(0x1));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000887d50); };
            c.w64(12, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000887d54); };
            c.w64(15, t144);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000887d58); }
            let Some(t145) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000887d5d); };
            c.w32(12, t145);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_ae() { blk = 340; continue; }
            blk = 134; continue;
        }
        134 => {
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(2)) { return RegfileExit::fault(0x7000887d78); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r146 = c.r64(12) & c.r64(11);
            c.w64(12, r146);
            c.logic(r146, 64);
            let Some(t147) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000887d8b); };
            c.logic(t147 & 0x4u64, 8);
            if c.cc_ne() { blk = 394; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(0, c.r64(2));
            blk = 139; continue;
        }
        137 => {
            let Some(t148) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000887d9c); };
            c.cmp(t148, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000887dab); };
            c.cmp(t149, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let r150 = c.r32(7) | c.r32(11);
            c.w32(7, r150);
            c.logic(r150, 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000887dc3); };
            c.w64(6, t151);
            return c.call(0x700089c794, 0x7000887dcc, 0x7000887dc7);
        }
        139 => {
            c.w64(7, c.r64(0));
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000887dcf); };
            c.cmp(t152, 0x0u64, 64);
            if c.cc_e() { blk = 147; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t153) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000887dda); };
            c.cmp(t153, 0x0u64, 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000887de0); };
            c.w64(11, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000887de7); };
            let r156 = c.imul2(c.r64(11), t155, 64);
            c.w64(11, r156);
            c.w64(0, c.r64(11));
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000887def); };
            c.w64(12, t157);
            c.cqo();
            c.idiv(c.r64(12), 64);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000887dfb); };
            c.cmp(c.r64(0), t158, 64);
            if c.cc_ne() { blk = 353; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000887e05); };
            let r160 = c.add(c.r64(11), t159, 64);
            c.w64(11, r160);
            if c.cc_o() { blk = 351; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r64(11), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 351; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000887e1c); };
            c.cmp(t161, 0x1u64, 64);
            c.w8(15, (c.cc_g() as u64));
            c.w32(15, c.r8(15));
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000887e29); };
            c.cmp(t162, 0x0u64, 64);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let r163 = c.r32(15) | c.r32(0);
            c.w32(15, r163);
            c.logic(r163, 32);
            if c.cc_ne() { blk = 256; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000887e3d); };
            c.w64(0, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000887e44); };
            c.w64(15, t165);
            blk = 146; continue;
        }
        146 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(10));
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 148; continue;
        }
        147 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000887e5c); };
            c.w64(11, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000887e60); };
            c.w64(0, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000887e64); };
            c.w64(15, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000887e6b); };
            c.w64(12, t169);
            blk = 148; continue;
        }
        148 => {
            let Some(t170) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000887e72); };
            c.w32(3, t170);
            let r171 = c.add(c.r64(3), c.r64(0), 64);
            c.w64(3, r171);
            if c.cc_o() { blk = 363; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r64(3), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 363; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t172) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000887e8b); };
            c.w32(0, t172);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(0));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ge() { blk = 280; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ae() { blk = 214; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(0, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(15).wrapping_add(c.r64(0)), c.r32(7)) { return RegfileExit::fault(0x7000887eb2); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r173 = c.r64(2) & c.r64(15);
            c.w64(2, r173);
            c.logic(r173, 64);
            let Some(t174) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000887ec6); };
            c.logic(t174 & 0x4u64, 8);
            if c.cc_ne() { blk = 391; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t175) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000887ed0); };
            c.w32(7, t175);
            let r176 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r176);
            let Some(t177) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000887ed6); };
            c.w32(7, t177);
            let r178 = c.r32(7) & 0x8u64;
            c.w32(7, r178);
            c.logic(r178, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let r179 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r179);
            let Some(t180) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000887eec); };
            let r181 = c.r32(7) & t180;
            c.w32(7, r181);
            c.logic(r181, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000887eef); }
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(15)) { return RegfileExit::fault(0x7000887ef3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000887efa); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000887efe); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), 0x0u64) { return RegfileExit::fault(0x7000887f02); }
            blk = 80; continue;
        }
        155 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 165; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t182) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000887f15); };
            c.cmp(t182, 0x0u64, 32);
            if c.cc_ne() { blk = 165; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000887f1b); };
            c.w64(7, t183);
            let r184 = c.imul2(c.r64(7), c.r64(3), 64);
            c.w64(7, r184);
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(7));
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000887f2c); };
            c.w64(11, t185);
            c.cqo();
            c.idiv(c.r64(11), 64);
            c.cmp(c.r64(0), c.r64(3), 64);
            if c.cc_ne() { blk = 332; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000887f41); };
            let r187 = c.add(c.r64(7), t186, 64);
            c.w64(7, r187);
            if c.cc_o() { blk = 330; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r64(7), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 330; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t188) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000887f58); };
            c.w32(11, t188);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(12), c.r64(0), 64);
            if c.cc_ge() { blk = 302; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r64(12), c.r64(0), 64);
            if c.cc_b() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            return RegfileExit::fault(0x7000887f73);
        }
        163 => {
            c.w32(11, c.r64(3).wrapping_add(c.r64(3)));
            if !c.st32(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000887f78); }
            blk = 164; continue;
        }
        164 => {
            let r189 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r189);
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(7));
            blk = 166; continue;
        }
        165 => {
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000887f89); };
            c.w64(15, t190);
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 198; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_e() { blk = 189; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w32(7, c.r32(15));
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t191) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000887faf); };
            c.cmp(t191, 0x0u64, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 178; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w64(11, c.r64(7));
            let r192 = c.add(c.r32(11), c.r32(7), 32);
            c.w32(11, r192);
            if c.cc_o() { blk = 177; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(11, 0xcu64);
            let r193 = c.add(c.r64(7), c.r64(11), 64);
            c.w64(7, r193);
            if c.cc_o() { blk = 176; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000887fd5); };
            c.w64(11, t194);
            let Some(t195) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000887fdc); };
            c.w64(11, t195);
            let r196 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r196);
            let r197 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r197);
            c.logic(r197, 64);
            c.w64(3, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000887feb); };
            c.w64(2, t198);
            let Some(t199) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000887ff2); };
            c.cmp(t199, c.r64(3), 64);
            if c.cc_be() { blk = 334; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 334; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000888008); };
            c.w64(7, t200);
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x700088800f); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 175; continue;
        }
        175 => {
            c.w64(11, c.r64(9));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000888019); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700088801d); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000888024); }
            blk = 187; continue;
        }
        176 => {
            return RegfileExit::fault(0x700088802d);
        }
        177 => {
            return RegfileExit::fault(0x700088802e);
        }
        178 => {
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700088802f); };
            c.w64(7, t201);
            blk = 187; continue;
        }
        179 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w64(7, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(11, 0xcu64);
            let r202 = c.add(c.r64(7), c.r64(11), 64);
            c.w64(7, r202);
            if c.cc_o() { blk = 185; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088804c); };
            c.w64(11, t203);
            let Some(t204) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000888053); };
            c.w64(11, t204);
            let r205 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r205);
            let r206 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r206);
            c.logic(r206, 64);
            c.w64(3, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000888062); };
            c.w64(2, t207);
            let Some(t208) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000888069); };
            c.cmp(t208, c.r64(3), 64);
            if c.cc_be() { blk = 336; continue; }
            blk = 182; continue;
        }
        182 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 336; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088807f); };
            c.w64(7, t209);
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x7000888086); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 184; continue;
        }
        184 => {
            c.w64(11, c.r64(8));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000888090); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000888094); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700088809b); }
            blk = 187; continue;
        }
        185 => {
            return RegfileExit::fault(0x70008880a1);
        }
        186 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008880a2); };
            c.w64(7, t210);
            blk = 187; continue;
        }
        187 => {
            c.w64(10, c.r64(4));
            let r211 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r211);
            let r212 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r212);
            c.logic(r212, 64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008880b4); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(7)) { return RegfileExit::fault(0x70008880b9); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(12));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x70008880c7); };
            c.w64(9, t213);
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x13a8)) else { return RegfileExit::fault(0x70008880ce); };
            c.w64(0, t214);
            c.w64(10, 0x70008880dcu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008880dc); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008880e0); }
            return c.call(c.r64(0), 0x70008880e6, 0x70008880e4);
        }
        188 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008880e6); }
            let Some(t215) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008880ee); };
            c.w64(4, t215);
            c.w64(3, c.r64(0));
            blk = 199; continue;
        }
        189 => {
            let Some(t216) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008880f8); };
            c.w32(7, t216);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            return RegfileExit::fault(0x7000888107);
        }
        191 => {
            let Some(t217) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000888108); };
            c.w32(3, t217);
            let r218 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r218);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t219) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088811e); };
            c.w32(11, t219);
            let r220 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r220);
            let Some(t221) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000888125); };
            c.cmp(t221, 0x3fu64, 16);
            if c.cc_be() { blk = 199; continue; }
            blk = 193; continue;
        }
        193 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t222) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888132); };
            c.w32(7, t222);
            let Some(t223) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000888135); };
            c.cmp(t223, c.r32(7), 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x700088813e); };
            c.w64(0, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888145); };
            c.w64(6, t225);
            return c.call(0x70008d2374, 0x700088814e, 0x7000888149);
        }
        196 => {
            c.w64(3, c.r64(0));
            blk = 199; continue;
        }
        197 => {
            return RegfileExit::fault(0x7000888153);
        }
        198 => {
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000888154); };
            c.w64(0, t226);
            c.w64(3, c.r64(0));
            blk = 199; continue;
        }
        199 => {
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088815e); };
            c.w64(6, t227);
            let Some(t228) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888162); };
            c.w32(7, t228);
            let r229 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r229);
            let Some(t230) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000888168); };
            c.w32(7, t230);
            let r231 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r231);
            let Some(t232) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088816e); };
            c.w32(11, t232);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000888178); }
            c.cmp(c.r64(11), 0x2cu64, 64);
            if c.cc_a() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            return RegfileExit::fault(0x7000888182);
        }
        201 => {
            let Some(t233) = c.ld32(c.r64(7).wrapping_add(0xb7)) else { return RegfileExit::fault(0x7000888183); };
            c.w32(0, t233);
            let r234 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r234);
            let Some(t235) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088818c); };
            c.cmp(t235, c.r32(0), 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            return RegfileExit::fault(0x7000888195);
        }
        203 => {
            let Some(t236) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888196); };
            c.w32(7, t236);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            return RegfileExit::fault(0x70008881a5);
        }
        205 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008881a6); };
            c.w64(11, t237);
            let Some(t238) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008881aa); };
            c.cmp(t238, c.r32(11), 32);
            if c.cc_ne() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r64(7), 0x2u64, 64);
            if c.cc_e() { blk = 209; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w64(3, c.r64(11));
            return c.call(0x7000884314, 0x70008881be, 0x70008881b9);
        }
        208 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008881be); };
            c.w64(3, t239);
            blk = 210; continue;
        }
        209 => {
            let Some(t240) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008881c4); };
            c.w64(7, t240);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008881cb); }
            blk = 210; continue;
        }
        210 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008881ce); };
            c.w64(7, t241);
            let r242 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r242);
            c.w64(0, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008881dc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008881dd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008881e3); }
            return c.ret(0x0, 0x70008881e5);
        }
        211 => {
            return RegfileExit::fault(0x70008881e6);
        }
        212 => {
            return RegfileExit::fault(0x70008881e7);
        }
        213 => {
            return RegfileExit::fault(0x70008881e8);
        }
        214 => {
            return RegfileExit::fault(0x70008881e9);
        }
        215 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008881ea); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008881f0); };
            c.w64(3, t243);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008881ff, 0x70008881fa);
        }
        216 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008881ff); };
            c.w64(6, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000888203); };
            c.w64(0, t245);
            blk = 1; continue;
        }
        217 => {
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088820c); };
            c.w64(0, t246);
            let Some(t247) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000888213); };
            c.w64(3, t247);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(7)) { return RegfileExit::fault(0x700088821a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x7000888221); }
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000888225); };
            c.w64(1, t248);
            let Some(t249) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700088822c); };
            c.cmp(t249, c.r64(2), 64);
            if c.cc_a() { blk = 220; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088823b, 0x7000888236);
        }
        219 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088823b); };
            c.w64(15, t250);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088823f); };
            c.w64(11, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888243); };
            c.w64(6, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000888247); };
            c.w64(12, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x700088824b); };
            c.w64(7, t254);
            blk = 221; continue;
        }
        220 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000888254); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 221; continue;
        }
        221 => {
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088825b); };
            c.w64(3, t255);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000888262); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 225; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            let r256 = c.r64(10) | 0x1u64;
            c.w64(10, r256);
            c.logic(r256, 64);
            blk = 224; continue;
        }
        224 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 225; continue;
        }
        225 => {
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088828c); }
            c.w64(15, c.r64(0));
            blk = 9; continue;
        }
        226 => {
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x70008882a1, 0x700088829c);
        }
        227 => {
            c.w64(7, c.r64(0));
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008882a4); };
            c.w64(11, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008882a8); };
            c.w64(6, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008882ac); };
            c.w64(12, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008882b0); };
            c.w64(15, t260);
            blk = 14; continue;
        }
        228 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008882c3, 0x70008882be);
        }
        229 => {
            c.w64(3, c.r64(0));
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008882c6); };
            c.w64(11, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008882ca); };
            c.w64(6, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008882ce); };
            c.w64(12, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x70008882d2); };
            c.w64(7, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008882d9); };
            c.w64(15, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008882dd); };
            c.w64(0, t266);
            blk = 23; continue;
        }
        230 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008882e8); };
            c.w64(11, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008882ec); };
            c.w64(6, t268);
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008882f0); };
            c.w64(0, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008882f4); };
            c.w64(12, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x70008882f8); };
            c.w64(7, t271);
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008882ff); };
            c.w64(15, t272);
            blk = 39; continue;
        }
        231 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000888308); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700088830c); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x7000888318, 0x7000888313);
        }
        232 => {
            c.w64(7, c.r64(0));
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088831b); };
            c.w64(9, t273);
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088831f); };
            c.w64(0, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000888323); };
            c.w64(1, t275);
            blk = 78; continue;
        }
        233 => {
            c.w32(1, 0x8a4u64);
            c.w64(7, c.r64(1));
            blk = 83; continue;
        }
        234 => {
            let Some(t276) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088833c); };
            c.w32(7, t276);
            let Some(t277) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088833f); };
            c.w32(12, t277);
            c.w64(3, c.r64(7));
            let r278 = c.r32(3) ^ c.r32(12);
            c.w32(3, r278);
            c.logic(r278, 32);
            let r279 = c.r32(3) & c.r32(15);
            c.w32(3, r279);
            c.logic(r279, 32);
            c.w32(15, c.r64(3).wrapping_add(c.r64(3)));
            if let Some(sr) = c.shl(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let Some(t280) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000888354); };
            c.w32(3, t280);
            let r281 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r281);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 254; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t282) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888365); };
            c.w32(2, t282);
            let Some(t283) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000888368); };
            c.cmp(t283, c.r32(2), 32);
            if c.cc_ne() { blk = 254; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t284) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000888375); };
            c.w32(2, t284);
            let Some(t285) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888378); };
            c.cmp(t285, c.r32(7), 32);
            if c.cc_ne() { blk = 254; continue; }
            blk = 237; continue;
        }
        237 => {
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_ne() { blk = 254; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t286) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088838a); };
            c.w32(0, t286);
            let r287 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r287);
            blk = 139; continue;
        }
        239 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x7000888397); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(1)) { return RegfileExit::fault(0x700088839b); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(8)) { return RegfileExit::fault(0x700088839f); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(12)) { return RegfileExit::fault(0x70008883a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(3)) { return RegfileExit::fault(0x70008883ad); }
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(0)) { return RegfileExit::fault(0x70008883b4); }
            return c.call(0x70007febb4, 0x70008883c0, 0x70008883bb);
        }
        240 => {
            c.w64(15, c.r64(0));
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008883c3); };
            c.w64(11, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008883c7); };
            c.w64(1, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008883cb); };
            c.w64(8, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008883d2); };
            c.w64(12, t291);
            c.w32(7, 0x66666667u64);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008883de); };
            c.w64(3, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008883e5); };
            c.w64(0, t293);
            blk = 117; continue;
        }
        241 => {
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008883f1); };
            c.w64(0, t294);
            let Some(t295) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008883f8); };
            c.w64(3, t295);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008883ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x7000888403); }
            let Some(t296) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088840a); };
            c.w64(1, t296);
            let Some(t297) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000888411); };
            c.cmp(t297, c.r64(2), 64);
            if c.cc_a() { blk = 244; continue; }
            blk = 242; continue;
        }
        242 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x7000888420, 0x700088841b);
        }
        243 => {
            blk = 245; continue;
        }
        244 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000888422); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 245; continue;
        }
        245 => {
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000888429); };
            c.w64(7, t298);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000888430); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x7000888433); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700088843a); }
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000888441); };
            c.w64(12, t299);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(12)) { return RegfileExit::fault(0x7000888448); }
            c.w64(2, c.r64(0));
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000888450); };
            c.w64(11, t300);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000888454); };
            c.w64(6, t301);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000888458); };
            c.w64(7, t302);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700088845c); };
            c.w64(15, t303);
            blk = 131; continue;
        }
        246 => {
            let Some(t304) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000888468); };
            c.w64(2, t304);
            let Some(t305) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700088846f); };
            c.w64(1, t305);
            c.w64(8, c.r64(1).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x7000888476); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(0)) { return RegfileExit::fault(0x700088847a); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x700088847e); }
            let Some(t306) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000888485); };
            c.w64(9, t306);
            let Some(t307) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088848c); };
            c.cmp(t307, c.r64(8), 64);
            if c.cc_a() { blk = 249; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700088849b, 0x7000888496);
        }
        248 => {
            blk = 250; continue;
        }
        249 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x700088849d); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(2));
            blk = 250; continue;
        }
        250 => {
            let Some(t308) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008884a7); };
            c.w64(7, t308);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008884ae); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008884b1); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008884b8); }
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008884bf); };
            c.w64(11, t309);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(11)) { return RegfileExit::fault(0x70008884c6); }
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008884ca); };
            c.w64(3, t310);
            let Some(t311) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008884d1); };
            c.w32(12, t311);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 338; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008884e4); };
            c.w64(11, t312);
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008884e8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r313 = c.r64(12) & c.r64(3);
            c.w64(12, r313);
            c.logic(r313, 64);
            let Some(t314) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008884fa); };
            c.logic(t314 & 0x4u64, 8);
            if c.cc_ne() { blk = 388; continue; }
            blk = 253; continue;
        }
        253 => {
            c.w64(2, c.r64(0));
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000888509); };
            c.w64(11, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088850d); };
            c.w64(6, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000888511); };
            c.w64(7, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000888515); };
            c.w64(15, t318);
            blk = 131; continue;
        }
        254 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000888521); }
            let Some(t319) = c.ld64(c.r64(13).wrapping_add(0x22d8)) else { return RegfileExit::fault(0x7000888522); };
            c.w64(3, t319);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000888535, 0x7000888530);
        }
        255 => {
            blk = 139; continue;
        }
        256 => {
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700088853a); };
            c.w64(0, t320);
            let Some(t321) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888541); };
            c.w32(15, t321);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700088854e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000888552); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x7000888556); }
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088855a); };
            c.w64(2, t322);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_l() { blk = 277; continue; }
            blk = 257; continue;
        }
        257 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_b() { blk = 342; continue; }
            blk = 258; continue;
        }
        258 => {
            c.w64(1, c.r64(3));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r323 = c.add(c.r64(3), c.r64(1), 64);
            c.w64(3, r323);
            let r324 = c.add(c.r64(3), 0x10u64, 64);
            c.w64(3, r324);
            if c.cc_e() { blk = 274; continue; }
            blk = 259; continue;
        }
        259 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 347; continue; }
            blk = 260; continue;
        }
        260 => {
            let r325 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r325);
            c.w64(1, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(1));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w32(6, c.r32(6));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(3)) { return RegfileExit::fault(0x700088859e); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(1)) { return RegfileExit::fault(0x70008885a5); }
            let r326 = c.imul2(c.r32(6), 0x4u64, 32);
            c.w32(6, r326);
            if c.cc_o() { blk = 346; continue; }
            blk = 261; continue;
        }
        261 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r327 = c.add(c.r64(6), c.r64(8), 64);
            c.w64(6, r327);
            if c.cc_o() { blk = 345; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008885c4); };
            c.w64(9, t328);
            let Some(t329) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008885cb); };
            c.w64(9, t329);
            c.w64(7, c.r64(9).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(9)) { return RegfileExit::fault(0x70008885d2); }
            let Some(t330) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008885d6); };
            c.w64(9, t330);
            let Some(t331) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008885dd); };
            c.cmp(t331, c.r64(7), 64);
            if c.cc_be() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_b() { blk = 266; continue; }
            blk = 264; continue;
        }
        264 => {
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x70008885f3, 0x70008885ee);
        }
        265 => {
            c.w64(7, c.r64(0));
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x70008885f6); };
            c.w64(0, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008885fd); };
            c.w64(2, t333);
            c.w32(8, 0x8u64);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000888607); };
            c.w64(11, t334);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088860b); };
            c.w64(15, t335);
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700088860f); };
            c.w64(1, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000888613); };
            c.w64(3, t337);
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x700088861a); };
            c.w64(12, t338);
            blk = 267; continue;
        }
        266 => {
            let Some(t339) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000888623); };
            c.w64(6, t339);
            if !c.st64(c.r64(6), c.r64(7)) { return RegfileExit::fault(0x700088862a); }
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700088862d); };
            c.w64(7, t340);
            let r341 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r341);
            blk = 267; continue;
        }
        267 => {
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000888635); };
            c.w64(6, t342);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700088863c); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700088863f); }
            c.w64(3, c.r64(1));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(15));
            c.w64(15, c.r64(8));
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(8));
            blk = 268; continue;
        }
        268 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 273; continue; }
            blk = 269; continue;
        }
        269 => {
            c.w64(9, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(9), c.r64(0), 64);
            if c.cc_e() { blk = 271; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t343) = c.ld32(c.r64(6).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088866b); };
            c.w32(9, t343);
            let r344 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r344);
            blk = 272; continue;
        }
        271 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000888676); };
            c.w64(9, t345);
            blk = 272; continue;
        }
        272 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700088867d); }
            let r346 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r346);
            blk = 268; continue;
        }
        273 => {
            c.w64(0, c.r64(7));
            blk = 275; continue;
        }
        274 => {
            let Some(t347) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088868d); };
            c.w64(0, t347);
            c.w32(8, 0x8u64);
            blk = 275; continue;
        }
        275 => {
            let Some(t348) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088869a); };
            c.w32(7, t348);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(15, c.r64(2));
            c.cmp(c.r64(15), c.r64(7), 64);
            if c.cc_ae() { blk = 349; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008886af); };
            c.w64(7, t349);
            let r350 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r350);
            if !c.st32(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008886b5); }
            c.w64(7, c.r64(15));
            c.w64(15, c.r64(0));
            blk = 279; continue;
        }
        277 => {
            c.w64(15, c.r64(0));
            c.w64(1, c.r64(2));
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_ae() { blk = 350; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008886d1); };
            c.w64(3, t351);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            if !c.st32(c.r64(15).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008886d8); }
            c.w32(8, 0x8u64);
            c.w64(7, c.r64(1));
            blk = 279; continue;
        }
        279 => {
            let r352 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r352);
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(7));
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008886f0); };
            c.w64(7, t353);
            blk = 146; continue;
        }
        280 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008886f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008886fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000888701); }
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(15)) { return RegfileExit::fault(0x7000888705); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x700088870c); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_b() { blk = 355; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w64(1, c.r64(2));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r354 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r354);
            let r355 = c.add(c.r64(2), 0x10u64, 64);
            c.w64(2, r355);
            if c.cc_e() { blk = 297; continue; }
            blk = 282; continue;
        }
        282 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 360; continue; }
            blk = 283; continue;
        }
        283 => {
            let r356 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r356);
            c.w64(1, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(1));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w32(6, c.r32(6));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(2)) { return RegfileExit::fault(0x7000888747); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(1)) { return RegfileExit::fault(0x700088874b); }
            let r357 = c.imul2(c.r32(6), 0x4u64, 32);
            c.w32(6, r357);
            if c.cc_o() { blk = 359; continue; }
            blk = 284; continue;
        }
        284 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r358 = c.add(c.r64(6), c.r64(8), 64);
            c.w64(6, r358);
            if c.cc_o() { blk = 358; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t359) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088876a); };
            c.w64(9, t359);
            let Some(t360) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000888771); };
            c.w64(9, t360);
            c.w64(7, c.r64(9).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x7000888778); }
            let Some(t361) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088877c); };
            c.w64(9, t361);
            let Some(t362) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000888783); };
            c.cmp(t362, c.r64(7), 64);
            if c.cc_be() { blk = 287; continue; }
            blk = 286; continue;
        }
        286 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_b() { blk = 289; continue; }
            blk = 287; continue;
        }
        287 => {
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x7000888799, 0x7000888794);
        }
        288 => {
            c.w64(7, c.r64(0));
            c.w32(8, 0x8u64);
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008887a2); };
            c.w64(3, t363);
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008887a6); };
            c.w64(11, t364);
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x70008887aa); };
            c.w64(15, t365);
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008887b1); };
            c.w64(0, t366);
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008887b5); };
            c.w64(1, t367);
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x70008887b9); };
            c.w64(12, t368);
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008887c0); };
            c.w64(2, t369);
            blk = 290; continue;
        }
        289 => {
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008887c6); };
            c.w64(6, t370);
            if !c.st64(c.r64(6), c.r64(7)) { return RegfileExit::fault(0x70008887cd); }
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008887d0); };
            c.w64(7, t371);
            let r372 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r372);
            blk = 290; continue;
        }
        290 => {
            let Some(t373) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008887d8); };
            c.w64(6, t373);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008887df); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008887e2); }
            c.w64(2, c.r64(1));
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(8));
            c.w64(6, c.r64(8));
            blk = 291; continue;
        }
        291 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_e() { blk = 296; continue; }
            blk = 292; continue;
        }
        292 => {
            c.w64(9, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(9), c.r64(6), 64);
            if c.cc_e() { blk = 294; continue; }
            blk = 293; continue;
        }
        293 => {
            let Some(t374) = c.ld32(c.r64(15).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888808); };
            c.w32(9, t374);
            let r375 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r375);
            blk = 295; continue;
        }
        294 => {
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000888813); };
            c.w64(9, t376);
            blk = 295; continue;
        }
        295 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700088881a); }
            let r377 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r377);
            blk = 291; continue;
        }
        296 => {
            c.w64(0, c.r64(7));
            blk = 298; continue;
        }
        297 => {
            let Some(t378) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088882a); };
            c.w64(0, t378);
            c.w32(8, 0x8u64);
            blk = 298; continue;
        }
        298 => {
            let Some(t379) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888837); };
            c.w32(7, t379);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 362; continue; }
            blk = 299; continue;
        }
        299 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000888851); };
            c.w64(1, t380);
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x7000888855); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r381 = c.r64(15) & c.r64(0);
            c.w64(15, r381);
            c.logic(r381, 64);
            let Some(t382) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888867); };
            c.logic(t382 & 0x4u64, 8);
            if c.cc_ne() { blk = 385; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(15, c.r64(0));
            c.w64(7, c.r64(1));
            blk = 154; continue;
        }
        302 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700088887d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000888881); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000888885); }
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(15)) { return RegfileExit::fault(0x7000888889); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000888890); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ae() { blk = 306; continue; }
            blk = 303; continue;
        }
        303 => {
            let Some(t383) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088889a); };
            c.w64(2, t383);
            let Some(t384) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008888a1); };
            c.w32(2, t384);
            let r385 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r385);
            return c.call(0x7000824234, 0x70008888af, 0x70008888aa);
        }
        304 => {
            return RegfileExit::fault(0x70008888af);
        }
        305 => {
            return RegfileExit::fault(0x70008888b0);
        }
        306 => {
            c.w64(2, c.r64(0));
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r386 = c.add(c.r64(0), c.r64(2), 64);
            c.w64(0, r386);
            let r387 = c.add(c.r64(0), 0x10u64, 64);
            c.w64(0, r387);
            if c.cc_e() { blk = 326; continue; }
            blk = 307; continue;
        }
        307 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 324; continue; }
            blk = 308; continue;
        }
        308 => {
            let r388 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r388);
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(1, c.r32(1));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x70008888dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x70008888e1); }
            let r389 = c.imul2(c.r32(1), 0x4u64, 32);
            c.w32(1, r389);
            if c.cc_o() { blk = 323; continue; }
            blk = 309; continue;
        }
        309 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r390 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r390);
            if c.cc_o() { blk = 322; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t391) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000888900); };
            c.w64(7, t391);
            let Some(t392) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000888907); };
            c.w64(7, t392);
            c.w64(11, c.r64(7).wrapping_add(c.r64(1)));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700088890e); }
            let Some(t393) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000888912); };
            c.w64(7, t393);
            let Some(t394) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000888919); };
            c.cmp(t394, c.r64(11), 64);
            if c.cc_be() { blk = 312; continue; }
            blk = 311; continue;
        }
        311 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_b() { blk = 314; continue; }
            blk = 312; continue;
        }
        312 => {
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x700088892f, 0x700088892a);
        }
        313 => {
            c.w64(7, c.r64(0));
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000888932); };
            c.w64(6, t395);
            let Some(t396) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000888936); };
            c.w64(9, t396);
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700088893d); };
            c.w64(15, t397);
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000888944); };
            c.w64(12, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000888948); };
            c.w64(3, t399);
            c.w32(8, 0x8u64);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000888952); };
            c.w64(2, t400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000888956); };
            c.w64(0, t401);
            blk = 315; continue;
        }
        314 => {
            let Some(t402) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088895c); };
            c.w64(7, t402);
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x7000888963); }
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000888966); };
            c.w64(7, t403);
            let r404 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r404);
            blk = 315; continue;
        }
        315 => {
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700088896e); };
            c.w64(11, t405);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000888975); }
            c.w64(11, c.r64(0));
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700088897c); }
            c.w64(11, c.r64(2));
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(15));
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088898e); };
            c.w64(15, t406);
            c.w64(0, c.r64(8));
            blk = 316; continue;
        }
        316 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 321; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w64(1, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_e() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            let Some(t407) = c.ld32(c.r64(2).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008889a7); };
            c.w32(1, t407);
            let r408 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r408);
            blk = 320; continue;
        }
        319 => {
            let Some(t409) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008889b1); };
            c.w64(1, t409);
            blk = 320; continue;
        }
        320 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008889b8); }
            let r410 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r410);
            blk = 316; continue;
        }
        321 => {
            c.w64(0, c.r64(7));
            let Some(t411) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008889c6); };
            c.w64(8, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008889cd); };
            c.w64(7, t412);
            blk = 327; continue;
        }
        322 => {
            return RegfileExit::fault(0x70008889d3);
        }
        323 => {
            return RegfileExit::fault(0x70008889d4);
        }
        324 => {
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008889d5); };
            c.w64(3, t413);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008889e5, 0x70008889e0);
        }
        325 => {
            return RegfileExit::fault(0x70008889e5);
        }
        326 => {
            let Some(t414) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008889e6); };
            c.w64(0, t414);
            blk = 327; continue;
        }
        327 => {
            let Some(t415) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008889ed); };
            c.w32(11, t415);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_b() { blk = 329; continue; }
            blk = 328; continue;
        }
        328 => {
            return RegfileExit::fault(0x70008889fc);
        }
        329 => {
            c.w32(11, c.r64(3).wrapping_add(c.r64(3)));
            if !c.st32(c.r64(0).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000888a01); }
            c.w64(15, c.r64(0));
            blk = 164; continue;
        }
        330 => {
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000888a0e); };
            c.w64(3, t416);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888a17); };
            c.w64(6, t417);
            return c.call(0x700086ed74, 0x7000888a20, 0x7000888a1b);
        }
        331 => {
            return RegfileExit::fault(0x7000888a20);
        }
        332 => {
            let Some(t418) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000888a21); };
            c.w64(3, t418);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888a2a); };
            c.w64(6, t419);
            return c.call(0x700086ed74, 0x7000888a33, 0x7000888a2e);
        }
        333 => {
            return RegfileExit::fault(0x7000888a33);
        }
        334 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(0)) { return RegfileExit::fault(0x7000888a34); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000888a3b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000888a3f); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x7000888a4b, 0x7000888a46);
        }
        335 => {
            c.w64(7, c.r64(0));
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000888a4e); };
            c.w64(6, t420);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000888a52); };
            c.w64(0, t421);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000888a59); };
            c.w64(12, t422);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000888a5d); };
            c.w64(15, t423);
            let Some(t424) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000888a61); };
            c.w64(9, t424);
            blk = 175; continue;
        }
        336 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(0)) { return RegfileExit::fault(0x7000888a6d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000888a74); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000888a78); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x7000888a84, 0x7000888a7f);
        }
        337 => {
            c.w64(7, c.r64(0));
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000888a87); };
            c.w64(6, t425);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000888a8b); };
            c.w64(0, t426);
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000888a92); };
            c.w64(12, t427);
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000888a96); };
            c.w64(15, t428);
            let Some(t429) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000888a9a); };
            c.w64(8, t429);
            blk = 184; continue;
        }
        338 => {
            return RegfileExit::fault(0x7000888aa6);
        }
        339 => {
            return RegfileExit::fault(0x7000888aa7);
        }
        340 => {
            return RegfileExit::fault(0x7000888aa8);
        }
        341 => {
            blk = 367; continue;
        }
        342 => {
            let Some(t430) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000888aae); };
            c.w64(2, t430);
            let Some(t431) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000888ab5); };
            c.w32(2, t431);
            let r432 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r432);
            return c.call(0x7000824234, 0x7000888ac3, 0x7000888abe);
        }
        343 => {
            return RegfileExit::fault(0x7000888ac3);
        }
        344 => {
            return RegfileExit::fault(0x7000888ac4);
        }
        345 => {
            return RegfileExit::fault(0x7000888ac5);
        }
        346 => {
            return RegfileExit::fault(0x7000888ac6);
        }
        347 => {
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000888ac7); };
            c.w64(3, t433);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000888ad7, 0x7000888ad2);
        }
        348 => {
            return RegfileExit::fault(0x7000888ad7);
        }
        349 => {
            return RegfileExit::fault(0x7000888ad8);
        }
        350 => {
            return RegfileExit::fault(0x7000888ad9);
        }
        351 => {
            let Some(t434) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000888ada); };
            c.w64(3, t434);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888ae3); };
            c.w64(6, t435);
            return c.call(0x700086ed74, 0x7000888aec, 0x7000888ae7);
        }
        352 => {
            return RegfileExit::fault(0x7000888aec);
        }
        353 => {
            let Some(t436) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000888aed); };
            c.w64(3, t436);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888af6); };
            c.w64(6, t437);
            return c.call(0x700086ed74, 0x7000888aff, 0x7000888afa);
        }
        354 => {
            return RegfileExit::fault(0x7000888aff);
        }
        355 => {
            let Some(t438) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000888b00); };
            c.w64(2, t438);
            let Some(t439) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000888b07); };
            c.w32(2, t439);
            let r440 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r440);
            return c.call(0x7000824234, 0x7000888b15, 0x7000888b10);
        }
        356 => {
            return RegfileExit::fault(0x7000888b15);
        }
        357 => {
            return RegfileExit::fault(0x7000888b16);
        }
        358 => {
            return RegfileExit::fault(0x7000888b17);
        }
        359 => {
            return RegfileExit::fault(0x7000888b18);
        }
        360 => {
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000888b19); };
            c.w64(3, t441);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000888b29, 0x7000888b24);
        }
        361 => {
            return RegfileExit::fault(0x7000888b29);
        }
        362 => {
            return RegfileExit::fault(0x7000888b2a);
        }
        363 => {
            let Some(t442) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000888b2b); };
            c.w64(3, t442);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888b34); };
            c.w64(6, t443);
            return c.call(0x700086ed74, 0x7000888b3d, 0x7000888b38);
        }
        364 => {
            return RegfileExit::fault(0x7000888b3d);
        }
        365 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t444) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000888b40); };
            c.w64(3, t444);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x7000888b51, 0x7000888b4c);
        }
        366 => {
            blk = 380; continue;
        }
        367 => {
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888b56); };
            c.w64(6, t445);
            let Some(t446) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888b5a); };
            c.w32(7, t446);
            let r447 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r447);
            let Some(t448) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000888b60); };
            c.w32(7, t448);
            let r449 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r449);
            let Some(t450) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888b66); };
            c.w32(11, t450);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000888b70); }
            c.cmp(c.r64(11), 0x2cu64, 64);
            if c.cc_a() { blk = 369; continue; }
            blk = 368; continue;
        }
        368 => {
            return RegfileExit::fault(0x7000888b7a);
        }
        369 => {
            let Some(t451) = c.ld32(c.r64(7).wrapping_add(0xb7)) else { return RegfileExit::fault(0x7000888b7b); };
            c.w32(7, t451);
            let r452 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r452);
            let Some(t453) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000888b84); };
            c.cmp(t453, c.r32(7), 32);
            if c.cc_ne() { blk = 371; continue; }
            blk = 370; continue;
        }
        370 => {
            return RegfileExit::fault(0x7000888b8d);
        }
        371 => {
            let Some(t454) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888b8e); };
            c.w32(11, t454);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 373; continue; }
            blk = 372; continue;
        }
        372 => {
            return RegfileExit::fault(0x7000888b9e);
        }
        373 => {
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000888b9f); };
            c.w64(3, t455);
            let Some(t456) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000888ba3); };
            c.cmp(t456, c.r32(3), 32);
            if c.cc_ne() { blk = 375; continue; }
            blk = 374; continue;
        }
        374 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_e() { blk = 377; continue; }
            blk = 375; continue;
        }
        375 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000884314, 0x7000888bb6, 0x7000888bb1);
        }
        376 => {
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888bb6); };
            c.w64(6, t457);
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000888bba); };
            c.w64(0, t458);
            blk = 378; continue;
        }
        377 => {
            let Some(t459) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000888bc0); };
            c.w64(11, t459);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000888bc7); }
            blk = 378; continue;
        }
        378 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000888bcb); }
            let Some(t460) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000888bcc); };
            c.w64(3, t460);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000888bdd, 0x7000888bd8);
        }
        379 => {
            return RegfileExit::fault(0x7000888bdd);
        }
        380 => {
            return RegfileExit::fault(0x7000888bde);
        }
        381 => {
            c.w32(7, 0x66u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888be4); }
            let Some(t461) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000888be5); };
            c.w64(7, t461);
            let Some(t462) = c.ld32(c.r64(7).wrapping_add(0x143)) else { return RegfileExit::fault(0x7000888bec); };
            c.w32(7, t462);
            let r463 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r463);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888bf5); }
            let Some(t464) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000888bf6); };
            c.w64(3, t464);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000888c07, 0x7000888c02);
        }
        382 => {
            return RegfileExit::fault(0x7000888c07);
        }
        383 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888c0d); }
            let Some(t465) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000888c0e); };
            c.w64(7, t465);
            let Some(t466) = c.ld32(c.r64(7).wrapping_add(0x143)) else { return RegfileExit::fault(0x7000888c15); };
            c.w32(7, t466);
            let r467 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r467);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888c1e); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000888c1f); }
            let Some(t468) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000888c21); };
            c.w64(3, t468);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000888c32, 0x7000888c2d);
        }
        384 => {
            return RegfileExit::fault(0x7000888c32);
        }
        385 => {
            c.w32(1, c.r32(1));
            let r469 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r469);
            c.w64(15, 0xfffffffffffc0000u64);
            let r470 = c.r64(15) & c.r64(1);
            c.w64(15, r470);
            c.logic(r470, 64);
            let Some(t471) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888c42); };
            c.logic(t471 & 0x2u64, 8);
            if c.cc_e() { blk = 301; continue; }
            blk = 386; continue;
        }
        386 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888c51); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000888c52); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000888c5f, 0x7000888c5a);
        }
        387 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000888c5f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000888c61); }
            blk = 301; continue;
        }
        388 => {
            c.w32(0, c.r32(0));
            let r472 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r472);
            c.w64(12, 0xfffffffffffc0000u64);
            let r473 = c.r64(12) & c.r64(0);
            c.w64(12, r473);
            c.logic(r473, 64);
            let Some(t474) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888c76); };
            c.logic(t474 & 0x2u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 389; continue;
        }
        389 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888c86); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000888c87); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000888c94, 0x7000888c8f);
        }
        390 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000888c94); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000888c96); }
            blk = 253; continue;
        }
        391 => {
            c.w32(7, c.r32(7));
            let r475 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r475);
            c.w64(2, 0xfffffffffffc0000u64);
            let r476 = c.r64(2) & c.r64(7);
            c.w64(2, r476);
            c.logic(r476, 64);
            let Some(t477) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888cab); };
            c.logic(t477 & 0x2u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 392; continue;
        }
        392 => {
            c.w64(1, c.r64(15).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888cb9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000888cba); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x7000888cc7, 0x7000888cc2);
        }
        393 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000888cc7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000888cc9); }
            blk = 154; continue;
        }
        394 => {
            c.w32(2, c.r32(2));
            let r478 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r478);
            c.w64(12, 0xfffffffffffc0000u64);
            let r479 = c.r64(12) & c.r64(2);
            c.w64(12, r479);
            c.logic(r479, 64);
            let Some(t480) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888cde); };
            c.logic(t480 & 0x2u64, 8);
            if c.cc_e() { blk = 136; continue; }
            blk = 395; continue;
        }
        395 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888cee); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000888cef); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000888cfc, 0x7000888cf7);
        }
        396 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000888cfc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000888cfe); }
            blk = 136; continue;
        }
        397 => {
            c.w32(11, c.r32(11));
            let r481 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r481);
            c.w64(2, 0xfffffffffffc0000u64);
            let r482 = c.r64(2) & c.r64(11);
            c.w64(2, r482);
            c.logic(r482, 64);
            let Some(t483) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888d14); };
            c.logic(t483 & 0x2u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 398; continue;
        }
        398 => {
            c.w64(1, c.r64(3).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888d22); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000888d23); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x7000888d30, 0x7000888d2b);
        }
        399 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000888d30); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000888d32); }
            blk = 38; continue;
        }
        400 => {
            c.w32(3, c.r32(3));
            let r484 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r484);
            c.w64(1, 0xfffffffffffc0000u64);
            let r485 = c.r64(1) & c.r64(3);
            c.w64(1, r485);
            c.logic(r485, 64);
            let Some(t486) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888d47); };
            c.logic(t486 & 0x2u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 401; continue;
        }
        401 => {
            c.w64(8, c.r64(2).wrapping_add(0xb7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888d58); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000888d61, 0x7000888d5c);
        }
        402 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000888d61); }
            blk = 29; continue;
        }
        403 => {
            return RegfileExit::jump(0x7000888d68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000888d74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000888d74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000888d78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000888d79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000888d7a); }
            let r0 = c.sub(c.r64(4), 0xa8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000888d82); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000888d86); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 163; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(11, c.r64(7).wrapping_add(0x18));
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(12).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000888d9e); };
            c.w64(12, t2);
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000888da6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x7000888daa); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000888db4); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000888dba); };
            c.w64(7, t4);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(7)) { return RegfileExit::fault(0x7000888dc1); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888dce); };
            c.w64(11, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000888dd4); };
            c.w64(11, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(11)) { return RegfileExit::fault(0x7000888ddb); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 305; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888dec); };
            c.w32(0, t7);
            let r8 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r8);
            let Some(t9) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000888df4); };
            c.cmp(t9, 0x82eu64, 16);
            if c.cc_ne() { blk = 305; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000888e00); };
            c.w32(0, t10);
            let r11 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r11);
            let Some(t12) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000888e08); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 303; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000888e12); };
            c.w64(0, t13);
            c.cmp(c.r64(0), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 165; continue; }
            blk = 11; continue;
        }
        11 => {
            let r14 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r14);
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000888e25); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000888e29); };
            c.w64(3, t15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000888e30); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000888e39); };
            c.cmp(t18, c.r32(3), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 174; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t19) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888e4b); };
            c.w32(2, t19);
            let r20 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r20);
            let Some(t21) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000888e51); };
            c.cmp(t21, 0x40u64, 16);
            if c.cc_ae() { blk = 174; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000888e64); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000888e68); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000888e71); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 41; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_le() { blk = 41; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t22) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888e8e); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            let Some(t24) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000888e94); };
            c.w32(1, t24);
            let r25 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r25);
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888e9a); };
            c.w32(8, t26);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2cu64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x7000888eaa);
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0xb7)) else { return RegfileExit::fault(0x7000888eab); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000888eb4); };
            c.cmp(t29, c.r32(1), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000888ec1); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000888ec8); };
            c.w64(8, t31);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000888ecf); };
            c.w64(7, t32);
            let Some(t33) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000888ed6); };
            c.cmp(t33, c.r64(9), 64);
            if c.cc_be() { blk = 176; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x7000888edf); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 24; continue;
        }
        24 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000888ee6); };
            c.w64(1, t34);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000888eed); }
            c.w32(1, 0x4u64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000888ef5); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000888ef8); };
            c.w64(1, t35);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000888eff); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000888f02); }
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888f05); };
            c.w32(1, t36);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x7000888f14);
        }
        26 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000888f15); }
            let Some(t37) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000888f19); };
            c.w32(1, t37);
            let r38 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r38);
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000888f1f); };
            c.w32(1, t39);
            let r40 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r40);
            let Some(t41) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888f25); };
            c.w32(8, t41);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2cu64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x7000888f35);
        }
        28 => {
            if !c.st32(c.r64(1).wrapping_add(0xb7), c.r32(7)) { return RegfileExit::fault(0x7000888f36); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r42 = c.r64(8) & c.r64(1);
            c.w64(8, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888f4c); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 316; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 39; continue;
        }
        31 => {
            let Some(t44) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000888f5c); };
            c.w32(8, t44);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x7000888f6c);
        }
        33 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000888f6d); };
            c.w32(8, t45);
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000888f71); };
            c.cmp(t46, c.r32(8), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(0, c.r64(1));
            c.w64(3, c.r64(12));
            return c.call(0x7000883ff4, 0x7000888f85, 0x7000888f80);
        }
        35 => {
            let Some(t47) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000888f85); };
            c.cmp(t47, c.r32(0), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000888f92); };
            c.w64(12, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000888f96); };
            c.w64(6, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000888f9a); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000888f9e); };
            c.w64(3, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000888fa2); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000888fa9); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000888fb0); };
            c.w64(0, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000888fb4); };
            c.w64(2, t55);
            blk = 39; continue;
        }
        37 => {
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000888fba); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r56 = c.r64(8) & c.r64(1);
            c.w64(8, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000888fce); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 313; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(1, 0x1u64);
            blk = 40; continue;
        }
        40 => {
            c.w32(7, 0x1u64);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, c.r64(15).wrapping_add(0x1));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000888fec); };
            c.w64(0, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000888ff6); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000888ff7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000888ffd); }
            return c.ret(0x0, 0x7000888fff);
        }
        42 => {
            let Some(t59) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000889000); };
            c.w32(1, t59);
            let r60 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r60);
            let Some(t61) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000889008); };
            c.w32(1, t61);
            c.w64(8, c.r64(1));
            let r62 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r62);
            let r63 = c.r32(1) & 0xfcu64;
            c.w32(1, r63);
            c.logic(r63, 32);
            let r64 = c.r32(8) & 0x3fu64;
            c.w32(8, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(8), 0x15u64, 32);
            if c.cc_g() { blk = 54; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(1), 0x44u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(1), 0x48u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(1), 0x4cu64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(1), 0x50u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(1), 0x54u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            return RegfileExit::fault(0x700088903c);
        }
        49 => {
            c.w32(1, 0x8bau64);
            blk = 67; continue;
        }
        50 => {
            c.w32(1, 0x8b8u64);
            blk = 67; continue;
        }
        51 => {
            c.w32(1, 0x8b6u64);
            blk = 67; continue;
        }
        52 => {
            c.w32(1, 0x8b4u64);
            blk = 67; continue;
        }
        53 => {
            c.w32(1, 0x8b2u64);
            blk = 67; continue;
        }
        54 => {
            c.cmp(c.r32(1), 0x58u64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(1), 0x5cu64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(1), 0x60u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(1), 0x64u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(1), 0x68u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(1), 0x6cu64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            return RegfileExit::fault(0x7000889087);
        }
        61 => {
            c.w32(1, 0x8b0u64);
            blk = 67; continue;
        }
        62 => {
            c.w32(1, 0x8aeu64);
            blk = 67; continue;
        }
        63 => {
            c.w32(1, 0x8acu64);
            blk = 67; continue;
        }
        64 => {
            c.w32(1, 0x8aau64);
            blk = 67; continue;
        }
        65 => {
            c.w32(1, 0x8a8u64);
            blk = 67; continue;
        }
        66 => {
            c.w32(1, 0x8a6u64);
            blk = 67; continue;
        }
        67 => {
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(1)) { return RegfileExit::fault(0x70008890b0); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            blk = 72; continue;
        }
        69 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008890c4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ns() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(0, cv);
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_s() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(10, 0x8000000000000000u64);
            let r65 = c.r64(0) | c.r64(10);
            c.w64(0, r65);
            c.logic(r65, 64);
            blk = 72; continue;
        }
        72 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(0)) { return RegfileExit::fault(0x70008890fe); }
            c.cmp(c.r64(0), 0x7ffeu64, 64);
            if c.cc_a() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(2, c.r64(0));
            blk = 75; continue;
        }
        74 => {
            c.w32(2, 0x7ffeu64);
            blk = 75; continue;
        }
        75 => {
            let Some(t66) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000889117); };
            c.w32(8, t66);
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(8)) { return RegfileExit::fault(0x700088911b); }
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 287; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088912f); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000889136); };
            c.w64(9, t68);
            c.w64(7, c.r64(2).wrapping_mul(4));
            c.w64(11, c.r64(7).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000889145); }
            c.w64(7, c.r64(11).wrapping_add(c.r64(9)));
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088914d); };
            c.w64(12, t69);
            let Some(t70) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000889154); };
            c.cmp(t70, c.r64(7), 64);
            if c.cc_be() { blk = 179; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 179; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088916b); };
            c.w64(11, t71);
            if !c.st64(c.r64(11), c.r64(7)) { return RegfileExit::fault(0x7000889172); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 79; continue;
        }
        79 => {
            c.w32(11, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700088917d); };
            c.w64(12, t72);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000889184); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000889188); }
            c.w64(11, c.r64(7).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x0u64, 64);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.w64(10, c.r64(4));
            let r73 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r73);
            let r74 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r74);
            c.logic(r74, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008891a7); }
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(7)) { return RegfileExit::fault(0x70008891ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x70008891b3); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(11));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008891bc); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008891c0); };
            c.w64(0, t76);
            c.w64(10, 0x70008891ceu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008891ce); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008891d2); }
            return c.call(c.r64(0), 0x70008891d8, 0x70008891d6);
        }
        80 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008891d8); }
            let Some(t77) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008891e0); };
            c.w64(4, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008891e5); };
            c.w64(3, t78);
            let Some(t79) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008891e9); };
            c.w32(7, t79);
            let r80 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r80);
            let Some(t81) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008891ef); };
            c.w32(7, t81);
            let r82 = c.r32(7) & 0x8u64;
            c.w32(7, r82);
            c.logic(r82, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000889201); };
            c.w64(8, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000889208); };
            c.w64(0, t84);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(2));
            c.w64(15, c.r64(2));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000889217); }
            blk = 82; continue;
        }
        81 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088921d); };
            c.w64(2, t85);
            let r86 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r86);
            c.w64(8, c.r64(12));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000889228); }
            c.w64(11, c.r64(6));
            c.w64(0, c.r64(1));
            blk = 82; continue;
        }
        82 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000889232); };
            c.w64(1, t87);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000889239); }
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088923d); };
            c.w64(6, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000889241); };
            c.cmp(c.r64(2), t89, 64);
            if c.cc_ae() { blk = 106; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t90) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088924e); };
            c.w32(7, t90);
            let r91 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r91);
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(0)) { return RegfileExit::fault(0x7000889254); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700088925b); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(2)) { return RegfileExit::fault(0x700088925f); }
            let Some(t92) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000889263); };
            c.logic(t92 & 0x4u64, 8);
            if c.cc_ne() { blk = 181; continue; }
            blk = 84; continue;
        }
        84 => {
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(1)) { return RegfileExit::fault(0x700088926d); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let r93 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r93);
            blk = 86; continue;
        }
        86 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700088927e); }
            c.w64(0, c.r64(6));
            c.w64(3, c.r64(2));
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889288); };
            c.w64(6, t94);
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700088928c); };
            return c.call(t95, 0x7000889294, 0x700088928c);
        }
        87 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889294); };
            c.w64(6, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000889298); };
            c.w64(3, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x700088929f); };
            c.w64(1, t98);
            return c.call(0x7000883e94, 0x70008892ab, 0x70008892a6);
        }
        88 => {
            c.w64(7, c.r64(0));
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008892ae); };
            c.cmp(t99, c.r32(7), 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008892bb); };
            c.cmp(t100, 0x0u64, 64);
            if c.cc_e() { blk = 97; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t101) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008892c2); };
            c.cmp(t101, 0x0u64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x70008892c8); };
            c.w64(11, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008892cf); };
            let r104 = c.imul2(c.r64(11), t103, 64);
            c.w64(11, r104);
            c.w64(0, c.r64(11));
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x70008892d7); };
            c.w64(12, t105);
            c.cqo();
            c.idiv(c.r64(12), 64);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008892e3); };
            c.cmp(c.r64(0), t106, 64);
            if c.cc_ne() { blk = 275; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008892ed); };
            let r108 = c.add(c.r64(11), t107, 64);
            c.w64(11, r108);
            if c.cc_o() { blk = 273; continue; }
            blk = 93; continue;
        }
        93 => {
            c.cmp(c.r64(11), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 273; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000889304); };
            c.cmp(t109, 0x1u64, 64);
            c.w8(15, (c.cc_g() as u64));
            c.w32(15, c.r8(15));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000889311); };
            c.cmp(t110, 0x0u64, 64);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let r111 = c.r32(15) | c.r32(0);
            c.w32(15, r111);
            c.logic(r111, 32);
            if c.cc_ne() { blk = 182; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000889325); };
            c.w64(0, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088932c); };
            c.w64(15, t113);
            blk = 96; continue;
        }
        96 => {
            c.w64(6, c.r64(15));
            c.w64(1, c.r64(0));
            blk = 98; continue;
        }
        97 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000889338); };
            c.w64(6, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088933c); };
            c.w64(11, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000889340); };
            c.w64(1, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000889347); };
            c.w64(12, t117);
            blk = 98; continue;
        }
        98 => {
            let Some(t118) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088934e); };
            c.w32(15, t118);
            let r119 = c.add(c.r64(15), c.r64(11), 64);
            c.w64(15, r119);
            if c.cc_o() { blk = 285; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r64(15), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 285; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t120) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000889368); };
            c.w32(11, t120);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_ge() { blk = 206; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_ae() { blk = 162; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(11, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(11)), c.r32(7)) { return RegfileExit::fault(0x700088938f); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r121 = c.r64(0) & c.r64(1);
            c.w64(0, r121);
            c.logic(r121, 64);
            let Some(t122) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008893a3); };
            c.logic(t122 & 0x4u64, 8);
            if c.cc_ne() { blk = 310; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t123) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008893ad); };
            c.w32(7, t123);
            let r124 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r124);
            let Some(t125) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008893b3); };
            c.w32(7, t125);
            let r126 = c.r32(7) & 0x8u64;
            c.w32(7, r126);
            c.logic(r126, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.w64(11, c.r64(6).wrapping_add(0x1));
            let Some(t127) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008893c9); };
            let r128 = c.r32(7) & t127;
            c.w32(7, r128);
            c.logic(r128, 32);
            c.w64(6, c.r64(11));
            c.w64(11, c.r64(15));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 81; continue;
        }
        105 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008893da); };
            c.w64(7, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x70008893de); };
            c.w64(1, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008893e5); };
            c.w64(6, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008893e9); };
            c.w64(11, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008893ed); };
            c.w64(15, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x70008893f1); };
            c.w64(12, t134);
            blk = 81; continue;
        }
        106 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 116; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t135) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000889403); };
            c.cmp(t135, 0x0u64, 32);
            if c.cc_ne() { blk = 116; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(7, c.r64(8));
            let r136 = c.imul2(c.r64(7), c.r64(15), 64);
            c.w64(7, r136);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(7));
            c.w64(12, c.r64(8));
            c.cqo();
            c.idiv(c.r64(12), 64);
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_ne() { blk = 258; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000889427); };
            let r138 = c.add(c.r64(7), t137, 64);
            c.w64(7, r138);
            if c.cc_o() { blk = 256; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r64(7), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 256; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t139) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088943e); };
            c.w32(12, t139);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(12));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ge() { blk = 228; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            return RegfileExit::fault(0x7000889459);
        }
        114 => {
            c.w32(12, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700088945e); }
            blk = 115; continue;
        }
        115 => {
            let r140 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r140);
            c.w64(0, c.r64(3));
            c.w64(12, c.r64(7));
            blk = 117; continue;
        }
        116 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088946f); };
            c.w64(12, t141);
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 149; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_e() { blk = 140; continue; }
            blk = 119; continue;
        }
        119 => {
            c.w32(7, c.r32(12));
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let Some(t142) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000889495); };
            c.cmp(t142, 0x0u64, 32);
            if c.cc_ne() { blk = 130; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(15, c.r64(7));
            let r143 = c.add(c.r32(15), c.r32(7), 32);
            c.w32(15, r143);
            if c.cc_o() { blk = 128; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w64(7, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r144 = c.add(c.r64(7), c.r64(15), 64);
            c.w64(7, r144);
            if c.cc_o() { blk = 127; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008894bb); };
            c.w64(15, t145);
            let Some(t146) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008894c2); };
            c.w64(15, t146);
            let r147 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r147);
            let r148 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r148);
            c.logic(r148, 64);
            c.w64(3, c.r64(15).wrapping_add(c.r64(7)));
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008894d1); };
            c.w64(2, t149);
            let Some(t150) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008894d8); };
            c.cmp(t150, c.r64(3), 64);
            if c.cc_be() { blk = 260; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 260; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008894ee); };
            c.w64(7, t151);
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x70008894f5); }
            c.w64(7, c.r64(15).wrapping_add(0x1));
            blk = 126; continue;
        }
        126 => {
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008894fc); };
            c.w64(15, t152);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000889503); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000889507); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700088950e); }
            blk = 138; continue;
        }
        127 => {
            return RegfileExit::fault(0x7000889517);
        }
        128 => {
            return RegfileExit::fault(0x7000889518);
        }
        129 => {
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000889519); };
            c.w64(7, t153);
            blk = 138; continue;
        }
        130 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r154 = c.add(c.r64(7), c.r64(15), 64);
            c.w64(7, r154);
            if c.cc_o() { blk = 136; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000889539); };
            c.w64(15, t155);
            let Some(t156) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000889540); };
            c.w64(15, t156);
            let r157 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r157);
            let r158 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r158);
            c.logic(r158, 64);
            c.w64(3, c.r64(15).wrapping_add(c.r64(7)));
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088954f); };
            c.w64(2, t159);
            let Some(t160) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000889556); };
            c.cmp(t160, c.r64(3), 64);
            if c.cc_be() { blk = 262; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 262; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088956c); };
            c.w64(7, t161);
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x7000889573); }
            c.w64(7, c.r64(15).wrapping_add(0x1));
            blk = 135; continue;
        }
        135 => {
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700088957a); };
            c.w64(15, t162);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000889581); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000889585); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700088958c); }
            blk = 138; continue;
        }
        136 => {
            return RegfileExit::fault(0x7000889592);
        }
        137 => {
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000889593); };
            c.w64(7, t163);
            blk = 138; continue;
        }
        138 => {
            c.w64(10, c.r64(4));
            let r164 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r164);
            let r165 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r165);
            c.logic(r165, 64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008895a5); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(7)) { return RegfileExit::fault(0x70008895aa); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(11));
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008895b8); };
            c.w64(9, t166);
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x13a8)) else { return RegfileExit::fault(0x70008895bc); };
            c.w64(0, t167);
            c.w64(10, 0x70008895cau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008895ca); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008895ce); }
            return c.call(c.r64(0), 0x70008895d4, 0x70008895d2);
        }
        139 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008895d4); }
            let Some(t168) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008895dc); };
            c.w64(4, t168);
            c.w64(3, c.r64(0));
            blk = 150; continue;
        }
        140 => {
            let Some(t169) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008895e6); };
            c.w32(7, t169);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            return RegfileExit::fault(0x70008895f5);
        }
        142 => {
            let Some(t170) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008895f6); };
            c.w32(3, t170);
            let r171 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r171);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t172) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088960c); };
            c.w32(11, t172);
            let r173 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r173);
            let Some(t174) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000889613); };
            c.cmp(t174, 0x3fu64, 16);
            if c.cc_be() { blk = 150; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t175) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000889620); };
            c.w32(7, t175);
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000889623); };
            c.cmp(t176, c.r32(7), 32);
            if c.cc_ne() { blk = 148; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088962c); };
            c.w64(0, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889630); };
            c.w64(6, t178);
            return c.call(0x70008d2374, 0x7000889639, 0x7000889634);
        }
        147 => {
            c.w64(3, c.r64(0));
            blk = 150; continue;
        }
        148 => {
            return RegfileExit::fault(0x700088963e);
        }
        149 => {
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700088963f); };
            c.w64(0, t179);
            c.w64(3, c.r64(0));
            blk = 150; continue;
        }
        150 => {
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889649); };
            c.w64(6, t180);
            let Some(t181) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088964d); };
            c.w32(7, t181);
            let r182 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r182);
            let Some(t183) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000889653); };
            c.w32(7, t183);
            let r184 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r184);
            let Some(t185) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000889659); };
            c.w32(11, t185);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000889663); }
            c.cmp(c.r64(11), 0x2cu64, 64);
            if c.cc_a() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            return RegfileExit::fault(0x700088966d);
        }
        152 => {
            let Some(t186) = c.ld32(c.r64(7).wrapping_add(0xb7)) else { return RegfileExit::fault(0x700088966e); };
            c.w32(0, t186);
            let r187 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r187);
            let Some(t188) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000889677); };
            c.cmp(t188, c.r32(0), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            return RegfileExit::fault(0x7000889680);
        }
        154 => {
            let Some(t189) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000889681); };
            c.w32(7, t189);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            return RegfileExit::fault(0x7000889690);
        }
        156 => {
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000889691); };
            c.w64(11, t190);
            let Some(t191) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000889695); };
            c.cmp(t191, c.r32(11), 32);
            if c.cc_ne() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r64(7), 0x2u64, 64);
            if c.cc_e() { blk = 160; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w64(3, c.r64(11));
            return c.call(0x7000884314, 0x70008896a9, 0x70008896a4);
        }
        159 => {
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008896a9); };
            c.w64(3, t192);
            blk = 161; continue;
        }
        160 => {
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008896af); };
            c.w64(7, t193);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008896b6); }
            blk = 161; continue;
        }
        161 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008896b9); };
            c.w64(7, t194);
            let r195 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r195);
            c.w64(0, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008896c7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008896c8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008896ce); }
            return c.ret(0x0, 0x70008896d0);
        }
        162 => {
            return RegfileExit::fault(0x70008896d1);
        }
        163 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008896d2); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008896d8); };
            c.w64(3, t196);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008896e7, 0x70008896e2);
        }
        164 => {
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008896e7); };
            c.w64(6, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008896eb); };
            c.w64(0, t198);
            blk = 1; continue;
        }
        165 => {
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008896f4); };
            c.w64(3, t199);
            let Some(t200) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008896fb); };
            c.w64(2, t200);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000889702); }
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000889706); };
            c.w64(8, t201);
            let Some(t202) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088970d); };
            c.cmp(t202, c.r64(1), 64);
            if c.cc_a() { blk = 168; continue; }
            blk = 166; continue;
        }
        166 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088971c, 0x7000889717);
        }
        167 => {
            c.w64(3, c.r64(0));
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088971f); };
            c.w64(12, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889723); };
            c.w64(6, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000889727); };
            c.w64(15, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x700088972b); };
            c.w64(7, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000889732); };
            c.w64(11, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000889739); };
            c.w64(0, t208);
            blk = 169; continue;
        }
        168 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x700088973f); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 169; continue;
        }
        169 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000889746); };
            c.w64(2, t209);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700088974d); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(0) as i64));
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ns() { blk = 173; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w64(10, c.r64(0));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            let r210 = c.r64(10) | 0x1u64;
            c.w64(10, r210);
            c.logic(r210, 64);
            blk = 172; continue;
        }
        172 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 173; continue;
        }
        173 => {
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000889777); }
            c.w64(0, c.r64(3));
            blk = 12; continue;
        }
        174 => {
            c.w64(0, c.r64(3));
            return c.call(0x700089c794, 0x700088978c, 0x7000889787);
        }
        175 => {
            c.w64(3, c.r64(0));
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088978f); };
            c.w64(12, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889793); };
            c.w64(6, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000889797); };
            c.w64(15, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x700088979b); };
            c.w64(7, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008897a2); };
            c.w64(11, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008897a9); };
            c.w64(0, t216);
            blk = 15; continue;
        }
        176 => {
            c.w32(2, 0x10u64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x70008897bf, 0x70008897ba);
        }
        177 => {
            c.w64(7, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008897c2); };
            c.w64(12, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008897c6); };
            c.w64(6, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008897ca); };
            c.w64(15, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008897ce); };
            c.w64(3, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008897d2); };
            c.w64(11, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008897d9); };
            c.w64(0, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008897dd); };
            c.w64(2, t223);
            blk = 24; continue;
        }
        178 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008897e8); };
            c.w64(12, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008897ec); };
            c.w64(6, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008897f0); };
            c.w64(2, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008897f4); };
            c.w64(15, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008897f8); };
            c.w64(3, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008897fc); };
            c.w64(11, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000889803); };
            c.w64(0, t230);
            blk = 40; continue;
        }
        179 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700088980c); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x7000889818, 0x7000889813);
        }
        180 => {
            c.w64(7, c.r64(0));
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088981b); };
            c.w64(2, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700088981f); };
            c.w64(8, t232);
            blk = 79; continue;
        }
        181 => {
            c.w32(1, 0x8a4u64);
            blk = 84; continue;
        }
        182 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000889835); };
            c.w64(0, t233);
            let Some(t234) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088983c); };
            c.w32(15, t234);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x7000889849); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700088984d); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(15)) { return RegfileExit::fault(0x7000889851); }
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000889855); };
            c.w64(2, t235);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_l() { blk = 203; continue; }
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_b() { blk = 264; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(1, c.r64(3));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r236 = c.add(c.r64(3), c.r64(1), 64);
            c.w64(3, r236);
            let r237 = c.add(c.r64(3), 0x10u64, 64);
            c.w64(3, r237);
            if c.cc_e() { blk = 200; continue; }
            blk = 185; continue;
        }
        185 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 269; continue; }
            blk = 186; continue;
        }
        186 => {
            let r238 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r238);
            c.w64(1, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(1));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w32(6, c.r32(6));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(3)) { return RegfileExit::fault(0x7000889899); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(1)) { return RegfileExit::fault(0x70008898a0); }
            let r239 = c.imul2(c.r32(6), 0x4u64, 32);
            c.w32(6, r239);
            if c.cc_o() { blk = 268; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r240 = c.add(c.r64(6), c.r64(8), 64);
            c.w64(6, r240);
            if c.cc_o() { blk = 267; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008898bf); };
            c.w64(9, t241);
            let Some(t242) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008898c6); };
            c.w64(9, t242);
            c.w64(7, c.r64(9).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(9)) { return RegfileExit::fault(0x70008898cd); }
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008898d4); };
            c.w64(9, t243);
            let Some(t244) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008898db); };
            c.cmp(t244, c.r64(7), 64);
            if c.cc_be() { blk = 190; continue; }
            blk = 189; continue;
        }
        189 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_b() { blk = 192; continue; }
            blk = 190; continue;
        }
        190 => {
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x70008898f1, 0x70008898ec);
        }
        191 => {
            c.w64(7, c.r64(0));
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x70008898f4); };
            c.w64(0, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008898fb); };
            c.w64(2, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008898ff); };
            c.w64(11, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000889903); };
            c.w64(15, t248);
            c.w32(8, 0x8u64);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700088990d); };
            c.w64(1, t249);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000889911); };
            c.w64(3, t250);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000889918); };
            c.w64(12, t251);
            blk = 193; continue;
        }
        192 => {
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000889921); };
            c.w64(6, t252);
            if !c.st64(c.r64(6), c.r64(7)) { return RegfileExit::fault(0x7000889928); }
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700088992b); };
            c.w64(7, t253);
            let r254 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r254);
            blk = 193; continue;
        }
        193 => {
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000889936); };
            c.w64(6, t255);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700088993d); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000889940); }
            c.w64(3, c.r64(1));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(8));
            blk = 194; continue;
        }
        194 => {
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_e() { blk = 199; continue; }
            blk = 195; continue;
        }
        195 => {
            c.w64(6, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_e() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t256) = c.ld32(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000889969); };
            c.w32(6, t256);
            let r257 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r257);
            blk = 198; continue;
        }
        197 => {
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000889974); };
            c.w64(6, t258);
            blk = 198; continue;
        }
        198 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700088997b); }
            let r259 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r259);
            blk = 194; continue;
        }
        199 => {
            c.w64(0, c.r64(7));
            blk = 201; continue;
        }
        200 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088998b); };
            c.w64(0, t260);
            blk = 201; continue;
        }
        201 => {
            let Some(t261) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000889992); };
            c.w32(7, t261);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(6, c.r64(2));
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_ae() { blk = 271; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008899a7); };
            c.w64(15, t262);
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st32(c.r64(0).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008899af); }
            c.w64(1, c.r64(0));
            blk = 205; continue;
        }
        203 => {
            c.w64(1, c.r64(0));
            c.w64(6, c.r64(2));
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_ae() { blk = 272; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008899c7); };
            c.w64(15, t263);
            let r264 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r264);
            if !c.st32(c.r64(1).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008899ce); }
            blk = 205; continue;
        }
        205 => {
            c.w64(7, c.r64(6).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            c.w64(15, c.r64(7));
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008899dd); };
            c.w64(7, t265);
            blk = 96; continue;
        }
        206 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008899e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008899ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x70008899ee); }
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(1)) { return RegfileExit::fault(0x70008899f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008899f9); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_b() { blk = 277; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w64(3, c.r64(0));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r266 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r266);
            let r267 = c.add(c.r64(0), 0x10u64, 64);
            c.w64(0, r267);
            if c.cc_e() { blk = 223; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 282; continue; }
            blk = 209; continue;
        }
        209 => {
            let r268 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r268);
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(2, c.r32(2));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(0)) { return RegfileExit::fault(0x7000889a33); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x7000889a3a); }
            let r269 = c.imul2(c.r32(2), 0x4u64, 32);
            c.w32(2, r269);
            if c.cc_o() { blk = 281; continue; }
            blk = 210; continue;
        }
        210 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r270 = c.add(c.r64(2), c.r64(8), 64);
            c.w64(2, r270);
            if c.cc_o() { blk = 280; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000889a59); };
            c.w64(9, t271);
            let Some(t272) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000889a60); };
            c.w64(9, t272);
            c.w64(7, c.r64(9).wrapping_add(c.r64(2)));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(9)) { return RegfileExit::fault(0x7000889a67); }
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000889a6b); };
            c.w64(9, t273);
            let Some(t274) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000889a72); };
            c.cmp(t274, c.r64(7), 64);
            if c.cc_be() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_b() { blk = 215; continue; }
            blk = 213; continue;
        }
        213 => {
            return c.call(0x70007febb4, 0x7000889a85, 0x7000889a80);
        }
        214 => {
            c.w64(7, c.r64(0));
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000889a88); };
            c.w64(15, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000889a8c); };
            c.w64(6, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000889a90); };
            c.w64(1, t277);
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000889a97); };
            c.w64(11, t278);
            c.w32(8, 0x8u64);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000889aa1); };
            c.w64(3, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000889aa5); };
            c.w64(12, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000889aac); };
            c.w64(0, t281);
            blk = 216; continue;
        }
        215 => {
            let Some(t282) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000889ab5); };
            c.w64(2, t282);
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x7000889abc); }
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000889abf); };
            c.w64(7, t283);
            let r284 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r284);
            blk = 216; continue;
        }
        216 => {
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000889ac7); };
            c.w64(2, t285);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000889ace); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000889ad1); }
            c.w64(0, c.r64(3));
            c.w64(0, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(8));
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_e() { blk = 222; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w64(2, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t286) = c.ld32(c.r64(1).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000889af4); };
            c.w32(2, t286);
            let r287 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r287);
            blk = 221; continue;
        }
        220 => {
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000889afe); };
            c.w64(2, t288);
            blk = 221; continue;
        }
        221 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000889b05); }
            let r289 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r289);
            blk = 217; continue;
        }
        222 => {
            c.w64(0, c.r64(7));
            blk = 224; continue;
        }
        223 => {
            let Some(t290) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000889b15); };
            c.w64(0, t290);
            blk = 224; continue;
        }
        224 => {
            let Some(t291) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000889b1c); };
            c.w32(7, t291);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_ae() { blk = 284; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w64(7, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000889b36); };
            c.w64(11, t292);
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(11)) { return RegfileExit::fault(0x7000889b3a); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r293 = c.r64(3) & c.r64(0);
            c.w64(3, r293);
            c.logic(r293, 64);
            let Some(t294) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000889b4e); };
            c.logic(t294 & 0x4u64, 8);
            if c.cc_ne() { blk = 307; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(1, c.r64(0));
            c.w64(7, c.r64(11));
            blk = 104; continue;
        }
        228 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000889b63); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000889b67); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x7000889b6b); }
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(3)) { return RegfileExit::fault(0x7000889b6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000889b76); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ae() { blk = 232; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t295) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000889b80); };
            c.w64(2, t295);
            let Some(t296) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000889b87); };
            c.w32(2, t296);
            let r297 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r297);
            return c.call(0x7000824234, 0x7000889b95, 0x7000889b90);
        }
        230 => {
            return RegfileExit::fault(0x7000889b95);
        }
        231 => {
            return RegfileExit::fault(0x7000889b96);
        }
        232 => {
            c.w64(2, c.r64(0));
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r298 = c.add(c.r64(0), c.r64(2), 64);
            c.w64(0, r298);
            let r299 = c.add(c.r64(0), 0x10u64, 64);
            c.w64(0, r299);
            if c.cc_e() { blk = 252; continue; }
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 250; continue; }
            blk = 234; continue;
        }
        234 => {
            let r300 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r300);
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(1, c.r32(1));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(0)) { return RegfileExit::fault(0x7000889bc3); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(2)) { return RegfileExit::fault(0x7000889bc7); }
            let r301 = c.imul2(c.r32(1), 0x4u64, 32);
            c.w32(1, r301);
            if c.cc_o() { blk = 249; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r302 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r302);
            if c.cc_o() { blk = 248; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000889be6); };
            c.w64(9, t303);
            let Some(t304) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000889bed); };
            c.w64(9, t304);
            c.w64(7, c.r64(9).wrapping_add(c.r64(1)));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x7000889bf4); }
            let Some(t305) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000889bf8); };
            c.w64(9, t305);
            let Some(t306) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000889bff); };
            c.cmp(t306, c.r64(7), 64);
            if c.cc_be() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_b() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000889c15, 0x7000889c10);
        }
        239 => {
            c.w64(7, c.r64(0));
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000889c18); };
            c.w64(6, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000889c1c); };
            c.w64(3, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000889c23); };
            c.w64(11, t309);
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000889c27); };
            c.w64(15, t310);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000889c2b); };
            c.w64(12, t311);
            c.w32(8, 0x8u64);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000889c35); };
            c.w64(2, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000889c39); };
            c.w64(0, t313);
            blk = 241; continue;
        }
        240 => {
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000889c3f); };
            c.w64(1, t314);
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000889c46); }
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000889c49); };
            c.w64(7, t315);
            let r316 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r316);
            blk = 241; continue;
        }
        241 => {
            let Some(t317) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000889c51); };
            c.w64(1, t317);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000889c58); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000889c5b); }
            c.w64(0, c.r64(2));
            c.w64(0, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(8));
            blk = 242; continue;
        }
        242 => {
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_e() { blk = 247; continue; }
            blk = 243; continue;
        }
        243 => {
            c.w64(1, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_e() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t318) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000889c81); };
            c.w32(1, t318);
            let r319 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r319);
            blk = 246; continue;
        }
        245 => {
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000889c8b); };
            c.w64(1, t320);
            blk = 246; continue;
        }
        246 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000889c92); }
            let r321 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r321);
            blk = 242; continue;
        }
        247 => {
            c.w64(0, c.r64(7));
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000889ca0); };
            c.w64(7, t322);
            blk = 253; continue;
        }
        248 => {
            return RegfileExit::fault(0x7000889ca6);
        }
        249 => {
            return RegfileExit::fault(0x7000889ca7);
        }
        250 => {
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000889ca8); };
            c.w64(3, t323);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000889cb8, 0x7000889cb3);
        }
        251 => {
            return RegfileExit::fault(0x7000889cb8);
        }
        252 => {
            let Some(t324) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000889cb9); };
            c.w64(0, t324);
            blk = 253; continue;
        }
        253 => {
            let Some(t325) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000889cc0); };
            c.w32(12, t325);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            return RegfileExit::fault(0x7000889ccf);
        }
        255 => {
            c.w32(12, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000889cd4); }
            c.w64(3, c.r64(0));
            blk = 115; continue;
        }
        256 => {
            let Some(t326) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000889ce1); };
            c.w64(3, t326);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889cea); };
            c.w64(6, t327);
            return c.call(0x700086ed74, 0x7000889cf3, 0x7000889cee);
        }
        257 => {
            return RegfileExit::fault(0x7000889cf3);
        }
        258 => {
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000889cf4); };
            c.w64(3, t328);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889cfd); };
            c.w64(6, t329);
            return c.call(0x700086ed74, 0x7000889d06, 0x7000889d01);
        }
        259 => {
            return RegfileExit::fault(0x7000889d06);
        }
        260 => {
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(0)) { return RegfileExit::fault(0x7000889d07); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000889d0e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000889d12); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x7000889d1e, 0x7000889d19);
        }
        261 => {
            c.w64(7, c.r64(0));
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000889d21); };
            c.w64(6, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000889d25); };
            c.w64(0, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000889d2c); };
            c.w64(11, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000889d30); };
            c.w64(12, t333);
            blk = 126; continue;
        }
        262 => {
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(0)) { return RegfileExit::fault(0x7000889d39); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000889d40); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000889d44); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x7000889d50, 0x7000889d4b);
        }
        263 => {
            c.w64(7, c.r64(0));
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000889d53); };
            c.w64(6, t334);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000889d57); };
            c.w64(0, t335);
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000889d5e); };
            c.w64(11, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000889d62); };
            c.w64(12, t337);
            blk = 135; continue;
        }
        264 => {
            let Some(t338) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000889d6b); };
            c.w64(2, t338);
            let Some(t339) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000889d72); };
            c.w32(2, t339);
            let r340 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r340);
            return c.call(0x7000824234, 0x7000889d80, 0x7000889d7b);
        }
        265 => {
            return RegfileExit::fault(0x7000889d80);
        }
        266 => {
            return RegfileExit::fault(0x7000889d81);
        }
        267 => {
            return RegfileExit::fault(0x7000889d82);
        }
        268 => {
            return RegfileExit::fault(0x7000889d83);
        }
        269 => {
            let Some(t341) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000889d84); };
            c.w64(3, t341);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000889d94, 0x7000889d8f);
        }
        270 => {
            return RegfileExit::fault(0x7000889d94);
        }
        271 => {
            return RegfileExit::fault(0x7000889d95);
        }
        272 => {
            return RegfileExit::fault(0x7000889d96);
        }
        273 => {
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000889d97); };
            c.w64(3, t342);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889da0); };
            c.w64(6, t343);
            return c.call(0x700086ed74, 0x7000889da9, 0x7000889da4);
        }
        274 => {
            return RegfileExit::fault(0x7000889da9);
        }
        275 => {
            let Some(t344) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000889daa); };
            c.w64(3, t344);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889db3); };
            c.w64(6, t345);
            return c.call(0x700086ed74, 0x7000889dbc, 0x7000889db7);
        }
        276 => {
            return RegfileExit::fault(0x7000889dbc);
        }
        277 => {
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000889dbd); };
            c.w64(2, t346);
            let Some(t347) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000889dc4); };
            c.w32(2, t347);
            let r348 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r348);
            return c.call(0x7000824234, 0x7000889dd2, 0x7000889dcd);
        }
        278 => {
            return RegfileExit::fault(0x7000889dd2);
        }
        279 => {
            return RegfileExit::fault(0x7000889dd3);
        }
        280 => {
            return RegfileExit::fault(0x7000889dd4);
        }
        281 => {
            return RegfileExit::fault(0x7000889dd5);
        }
        282 => {
            let Some(t349) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000889dd6); };
            c.w64(3, t349);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000889de6, 0x7000889de1);
        }
        283 => {
            return RegfileExit::fault(0x7000889de6);
        }
        284 => {
            return RegfileExit::fault(0x7000889de7);
        }
        285 => {
            let Some(t350) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000889de8); };
            c.w64(3, t350);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889df1); };
            c.w64(6, t351);
            return c.call(0x700086ed74, 0x7000889dfa, 0x7000889df5);
        }
        286 => {
            return RegfileExit::fault(0x7000889dfa);
        }
        287 => {
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000889dfb); };
            c.w64(3, t352);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000889e0b, 0x7000889e06);
        }
        288 => {
            blk = 302; continue;
        }
        289 => {
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889e10); };
            c.w64(6, t353);
            let Some(t354) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000889e14); };
            c.w32(7, t354);
            let r355 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r355);
            let Some(t356) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000889e1a); };
            c.w32(7, t356);
            let r357 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r357);
            let Some(t358) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000889e20); };
            c.w32(11, t358);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000889e2a); }
            c.cmp(c.r64(11), 0x2cu64, 64);
            if c.cc_a() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            return RegfileExit::fault(0x7000889e34);
        }
        291 => {
            let Some(t359) = c.ld32(c.r64(7).wrapping_add(0xb7)) else { return RegfileExit::fault(0x7000889e35); };
            c.w32(7, t359);
            let r360 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r360);
            let Some(t361) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000889e3e); };
            c.cmp(t361, c.r32(7), 32);
            if c.cc_ne() { blk = 293; continue; }
            blk = 292; continue;
        }
        292 => {
            return RegfileExit::fault(0x7000889e47);
        }
        293 => {
            let Some(t362) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000889e48); };
            c.w32(11, t362);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 295; continue; }
            blk = 294; continue;
        }
        294 => {
            return RegfileExit::fault(0x7000889e58);
        }
        295 => {
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000889e59); };
            c.w64(3, t363);
            let Some(t364) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000889e5d); };
            c.cmp(t364, c.r32(3), 32);
            if c.cc_ne() { blk = 297; continue; }
            blk = 296; continue;
        }
        296 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_e() { blk = 299; continue; }
            blk = 297; continue;
        }
        297 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000884314, 0x7000889e70, 0x7000889e6b);
        }
        298 => {
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000889e70); };
            c.w64(6, t365);
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000889e74); };
            c.w64(0, t366);
            blk = 300; continue;
        }
        299 => {
            let Some(t367) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000889e7a); };
            c.w64(11, t367);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000889e81); }
            blk = 300; continue;
        }
        300 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000889e85); }
            let Some(t368) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000889e86); };
            c.w64(3, t368);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000889e97, 0x7000889e92);
        }
        301 => {
            return RegfileExit::fault(0x7000889e97);
        }
        302 => {
            return RegfileExit::fault(0x7000889e98);
        }
        303 => {
            c.w32(7, 0x66u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889e9e); }
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000889e9f); };
            c.w64(7, t369);
            let Some(t370) = c.ld32(c.r64(7).wrapping_add(0x147)) else { return RegfileExit::fault(0x7000889ea6); };
            c.w32(7, t370);
            let r371 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r371);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889eaf); }
            let Some(t372) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000889eb0); };
            c.w64(3, t372);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000889ec1, 0x7000889ebc);
        }
        304 => {
            return RegfileExit::fault(0x7000889ec1);
        }
        305 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889ec7); }
            let Some(t373) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000889ec8); };
            c.w64(7, t373);
            let Some(t374) = c.ld32(c.r64(7).wrapping_add(0x147)) else { return RegfileExit::fault(0x7000889ecf); };
            c.w32(7, t374);
            let r375 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r375);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889ed8); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000889ed9); }
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000889edb); };
            c.w64(3, t376);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000889eec, 0x7000889ee7);
        }
        306 => {
            return RegfileExit::fault(0x7000889eec);
        }
        307 => {
            c.w32(11, c.r32(11));
            let r377 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r377);
            c.w64(3, 0xfffffffffffc0000u64);
            let r378 = c.r64(3) & c.r64(11);
            c.w64(3, r378);
            c.logic(r378, 64);
            let Some(t379) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000889efd); };
            c.logic(t379 & 0x2u64, 8);
            if c.cc_e() { blk = 227; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889f0b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000889f0c); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000889f19, 0x7000889f14);
        }
        309 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000889f19); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000889f1b); }
            blk = 227; continue;
        }
        310 => {
            c.w32(7, c.r32(7));
            let r380 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r380);
            c.w64(0, 0xfffffffffffc0000u64);
            let r381 = c.r64(0) & c.r64(7);
            c.w64(0, r381);
            c.logic(r381, 64);
            let Some(t382) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000889f30); };
            c.logic(t382 & 0x2u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(3, c.r64(1).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889f3e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000889f3f); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x7000889f4c, 0x7000889f47);
        }
        312 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000889f4c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000889f4e); }
            blk = 104; continue;
        }
        313 => {
            c.w32(12, c.r32(12));
            let r383 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r383);
            c.w64(8, 0xfffffffffffc0000u64);
            let r384 = c.r64(8) & c.r64(12);
            c.w64(8, r384);
            c.logic(r384, 64);
            let Some(t385) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000889f64); };
            c.logic(t385 & 0x2u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 314; continue;
        }
        314 => {
            c.w64(9, c.r64(1).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889f73); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000889f74); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000889f81, 0x7000889f7c);
        }
        315 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000889f81); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000889f83); }
            blk = 39; continue;
        }
        316 => {
            c.w32(7, c.r32(7));
            let r386 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r386);
            c.w64(8, 0xfffffffffffc0000u64);
            let r387 = c.r64(8) & c.r64(7);
            c.w64(8, r387);
            c.logic(r387, 64);
            let Some(t388) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000889f98); };
            c.logic(t388 & 0x2u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w64(9, c.r64(1).wrapping_add(0xb7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889faa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000889fab); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000889fb8, 0x7000889fb3);
        }
        318 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000889fb8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000889fba); }
            blk = 30; continue;
        }
        319 => {
            return RegfileExit::jump(0x7000889fc4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000889fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000889fd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000889fd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000889fd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000889fda); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000889fdf); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000889fe3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000889fe7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 215; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000889fff); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700088a004); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700088a008); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 217; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a016); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a01d); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 217; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700088a036); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a040); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let Some(t8) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a047); };
            c.cmp(t8, 0x834u64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088a050); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            blk = 27; continue;
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
            if c.cc_ne() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a06d); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088a074); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088a07b); };
            c.w32(12, t15);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t16) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a082); };
            c.w32(15, t16);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xb0u64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x700088a092);
        }
        11 => {
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700088a093); };
            c.cmp(t17, c.r32(15), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 221; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a0aa); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088a0b1); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088a0b8); };
            c.w32(11, t22);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t23) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a0bf); };
            c.w32(12, t23);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x44u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x700088a0cc);
        }
        15 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x117)) else { return RegfileExit::fault(0x700088a0cd); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(11), 0xaeu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x700088a0df);
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088a0e0); };
            c.cmp(t25, c.r32(12), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(11), 0xafu64, 64);
            if c.cc_a() { blk = 220; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x700088a0f6);
        }
        21 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088a0f7); };
            c.w32(9, t26);
            let r27 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r27);
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a17f4, 0x700088a10c, 0x700088a107);
        }
        24 => {
            c.w64(9, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088a10f); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088a113); };
            c.w64(6, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088a117); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088a11b); };
            c.w64(7, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088a11f); };
            c.w64(0, t32);
            blk = 27; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ge() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        27 => {
            c.w32(11, c.r32(9));
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088a140); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x700088a144); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x700088a148); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700088a14c); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a156); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 32; continue; }
            blk = 29; continue;
        }
        29 => {
            if c.cc_e() { blk = 214; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 214; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x700088a182); }
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_l() { blk = 39; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t33) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088a192); };
            c.w64(7, t33);
            blk = 36; continue;
        }
        35 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088a198); };
            c.w64(7, t34);
            blk = 36; continue;
        }
        36 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000899a14, 0x700088a1ad, 0x700088a1a8);
        }
        38 => {
            c.w64(7, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088a1b0); };
            c.w64(12, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088a1b4); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088a1b8); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088a1bc); };
            c.w64(1, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088a1c0); };
            c.w64(9, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088a1c4); };
            c.w64(15, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088a1c8); };
            c.w64(11, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088a1cc); };
            c.w64(0, t42);
            blk = 42; continue;
        }
        39 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 223; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(7, c.r64(11));
            let r43 = c.sub(c.r32(7), 0x2u64, 32);
            c.w32(7, r43);
            if c.cc_o() { blk = 223; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 42; continue;
        }
        42 => {
            c.w32(3, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a1fb); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 51; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 46; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ge() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 237; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 48; continue;
        }
        48 => {
            let r44 = c.add(c.r32(3), c.r32(11), 32);
            c.w32(3, r44);
            if c.cc_o() { blk = 237; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 50; continue;
        }
        50 => {
            c.w64(9, c.r64(7));
            blk = 74; continue;
        }
        51 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 255; continue; }
            blk = 52; continue;
        }
        52 => {
            let r45 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r45);
            if c.cc_o() { blk = 255; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 54; continue;
        }
        54 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 55; continue;
        }
        55 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a25e); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a269); }
            blk = 60; continue;
        }
        57 => {
            c.w64(15, c.r64(9));
            let r46 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r46);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            blk = 60; continue;
        }
        58 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(15, c.r64(7));
            let r47 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r47);
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a28e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 60; continue;
        }
        60 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 74; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 63; continue;
        }
        62 => {
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_le() { blk = 74; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a2b4); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a2bf); }
            blk = 69; continue;
        }
        66 => {
            c.w64(9, c.r64(7));
            let r48 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r48);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 69; continue;
        }
        67 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 68; continue;
        }
        68 => {
            let r49 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r49);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a2df); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 69; continue;
        }
        69 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 73; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 72; continue;
        }
        71 => {
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_le() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700088a2fa); };
            c.w64(7, t50);
            c.w64(9, c.r64(7));
            blk = 74; continue;
        }
        73 => {
            c.w64(9, c.r64(7));
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t51) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088a30f); };
            c.w64(1, t51);
            blk = 77; continue;
        }
        76 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088a315); };
            c.w64(1, t52);
            blk = 77; continue;
        }
        77 => {
            c.w64(0, c.r64(8));
            let Some(t53) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a31f); };
            c.w32(7, t53);
            let r54 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r54);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700088a325); }
            let Some(t55) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a329); };
            c.cmp(t55, 0x834u64, 16);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w32(8, 0x1u64);
            blk = 80; continue;
        }
        79 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t56) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088a346); };
            c.w32(8, t56);
            c.w64(11, c.r64(8));
            let r57 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r57);
            let r58 = c.r32(11) & 0x3fu64;
            c.w32(11, r58);
            c.logic(r58, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_a() { blk = 170; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t59) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a360); };
            c.w32(11, t59);
            let r60 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r60);
            let Some(t61) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088a367); };
            c.w32(11, t61);
            let r62 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r62);
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088a36e); };
            c.w32(15, t63);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4cu64, 64);
            if c.cc_a() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            return RegfileExit::fault(0x700088a37e);
        }
        84 => {
            let Some(t64) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x700088a37f); };
            c.w32(11, t64);
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088a386); };
            c.cmp(t65, c.r32(11), 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088a390); };
            c.w64(7, t66);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088a39a); };
            c.cmp(t67, c.r32(11), 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t68) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088a3a4); };
            c.w32(7, t68);
            c.w32(7, c.r32(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 87; continue;
        }
        87 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 172; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w32(7, c.r8(8));
            let r69 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r69);
            let r70 = c.r32(7) & 0x3fu64;
            c.w32(7, r70);
            c.logic(r70, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_be() { blk = 130; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t71) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a3cf); };
            c.w32(7, t71);
            let r72 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r72);
            let Some(t73) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088a3d5); };
            c.w32(8, t73);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_g() { blk = 92; continue; }
            blk = 90; continue;
        }
        90 => {
            let r74 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r74);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            blk = 92; continue;
        }
        91 => {
            c.w32(8, c.r64(9).wrapping_sub(0x2));
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 127; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088a3fb); };
            c.w32(8, t75);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_ae() { blk = 128; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x700088a40b); }
            c.w64(15, 0xfff7fffffff7ffffu64);
            let Some(t76) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a41c); };
            c.cmp(t76, c.r64(15), 64);
            if c.cc_e() { blk = 91; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(8, c.r32(1));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088a426); };
            c.w64(15, t77);
            let Some(t78) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700088a42d); };
            c.w64(0, t78);
            let r79 = c.r32(8) & 0x1u64;
            c.w32(8, r79);
            c.logic(r79, 32);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088a438); };
            c.w64(2, t80);
            let Some(t81) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700088a43f); };
            c.cmp(t81, c.r64(3), 64);
            if c.cc_be() { blk = 270; continue; }
            blk = 96; continue;
        }
        96 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700088a448); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 97; continue;
        }
        97 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a44f); };
            c.w64(0, t82);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700088a456); }
            c.cmp(c.r32(8), 0x0u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088a466); }
            c.cmp(c.r32(1), c.r32(15), 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 113; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t83) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a489); };
            c.w32(8, t83);
            let r84 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a490); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t86) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a4a2); };
            c.w32(8, t86);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 274; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_ne() { blk = 123; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t87) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a4bd); };
            c.w32(8, t87);
            let r88 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r88);
            let Some(t89) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a4c4); };
            c.cmp(t89, 0x41u64, 16);
            if c.cc_e() { blk = 272; continue; }
            blk = 104; continue;
        }
        104 => {
            blk = 123; continue;
        }
        105 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t90) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a4da); };
            c.w32(8, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a4de); };
            c.cmp(t91, c.r32(8), 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 107; continue;
        }
        107 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a4eb); }
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a4f0); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 123; continue; }
            blk = 108; continue;
        }
        108 => {
            if c.cc_e() { blk = 124; continue; }
            blk = 109; continue;
        }
        109 => {
            blk = 123; continue;
        }
        110 => {
            let r92 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r92);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a50e); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 123; continue; }
            blk = 111; continue;
        }
        111 => {
            if c.cc_e() { blk = 124; continue; }
            blk = 112; continue;
        }
        112 => {
            blk = 123; continue;
        }
        113 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t93) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a533); };
            c.w32(8, t93);
            let r94 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a53a); };
            c.cmp(t95, c.r32(8), 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t96) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a543); };
            c.cmp(t96, 0x43u64, 16);
            blk = 123; continue;
        }
        116 => {
            c.w64(8, c.r64(1));
            let r97 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r97);
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a551); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 123; continue; }
            blk = 117; continue;
        }
        117 => {
            if c.cc_e() { blk = 124; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 123; continue;
        }
        119 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 124; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t98) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a570); };
            c.w32(8, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a574); };
            c.cmp(t99, c.r32(8), 32);
            if c.cc_ne() { blk = 124; continue; }
            blk = 121; continue;
        }
        121 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a57d); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            if c.cc_e() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700088a58a); };
            c.w64(0, t100);
            blk = 125; continue;
        }
        124 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088a593); };
            c.w64(0, t101);
            blk = 125; continue;
        }
        125 => {
            let Some(t102) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088a59a); };
            c.cmp(t102, c.r32(0), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 126; continue;
        }
        126 => {
            blk = 91; continue;
        }
        127 => {
            c.w64(7, c.r64(12));
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088a5b9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088a5ba); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088a5c0); }
            return c.ret(0x0, 0x700088a5c2);
        }
        128 => {
            return RegfileExit::fault(0x700088a5c3);
        }
        129 => {
            c.w64(1, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088a5cf); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088a5d0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088a5d6); }
            return c.ret(0x0, 0x700088a5d8);
        }
        130 => {
            let Some(t103) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a5d9); };
            c.w32(7, t103);
            let r104 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r104);
            let Some(t105) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088a5df); };
            c.w32(8, t105);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_g() { blk = 133; continue; }
            blk = 131; continue;
        }
        131 => {
            let r106 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r106);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            blk = 133; continue;
        }
        132 => {
            c.w32(8, c.r64(9).wrapping_sub(0x2));
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 166; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t107) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088a605); };
            c.w32(8, t107);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_ae() { blk = 167; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t108) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a615); };
            c.w32(0, t108);
            let r109 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r109);
            let Some(t110) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088a61d); };
            c.cmp(t110, c.r32(0), 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 136; continue;
        }
        136 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 137; continue;
        }
        137 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t111) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a64d); };
            c.w32(8, t111);
            let r112 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r112);
            let Some(t113) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a654); };
            c.cmp(t113, c.r32(8), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 162; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t114) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a667); };
            c.w32(8, t114);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 279; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_ne() { blk = 162; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t115) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a682); };
            c.w32(8, t115);
            let r116 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r116);
            let Some(t117) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a689); };
            c.cmp(t117, 0x41u64, 16);
            if c.cc_e() { blk = 277; continue; }
            blk = 143; continue;
        }
        143 => {
            blk = 162; continue;
        }
        144 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 149; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t118) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a6a0); };
            c.w32(8, t118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a6a4); };
            c.cmp(t119, c.r32(8), 32);
            if c.cc_ne() { blk = 162; continue; }
            blk = 146; continue;
        }
        146 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a6b1); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a6b6); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 162; continue; }
            blk = 147; continue;
        }
        147 => {
            if c.cc_e() { blk = 163; continue; }
            blk = 148; continue;
        }
        148 => {
            blk = 162; continue;
        }
        149 => {
            let r120 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r120);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a6d2); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 162; continue; }
            blk = 150; continue;
        }
        150 => {
            if c.cc_e() { blk = 163; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 162; continue;
        }
        152 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 162; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t121) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a6f7); };
            c.w32(8, t121);
            let r122 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r122);
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a6fe); };
            c.cmp(t123, c.r32(8), 32);
            if c.cc_e() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t124) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a707); };
            c.cmp(t124, 0x43u64, 16);
            blk = 162; continue;
        }
        155 => {
            c.w64(8, c.r64(1));
            let r125 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r125);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a715); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 162; continue; }
            blk = 156; continue;
        }
        156 => {
            if c.cc_e() { blk = 163; continue; }
            blk = 157; continue;
        }
        157 => {
            blk = 162; continue;
        }
        158 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t126) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a733); };
            c.w32(8, t126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a737); };
            c.cmp(t127, c.r32(8), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 160; continue;
        }
        160 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a740); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            if c.cc_e() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700088a74d); };
            c.w64(0, t128);
            blk = 164; continue;
        }
        163 => {
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088a756); };
            c.w64(0, t129);
            blk = 164; continue;
        }
        164 => {
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088a75d); };
            c.cmp(t130, c.r32(0), 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 165; continue;
        }
        165 => {
            blk = 132; continue;
        }
        166 => {
            c.w64(1, c.r64(12).wrapping_add(0x1));
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088a77a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088a77b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088a781); }
            return c.ret(0x0, 0x700088a783);
        }
        167 => {
            return RegfileExit::fault(0x700088a784);
        }
        168 => {
            c.w64(1, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088a790); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088a791); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088a797); }
            return c.ret(0x0, 0x700088a799);
        }
        169 => {
            c.w64(1, c.r64(12).wrapping_add(0x1));
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088a7a9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088a7aa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088a7b0); }
            return c.ret(0x0, 0x700088a7b2);
        }
        170 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 172; continue;
        }
        171 => {
            c.w64(9, c.r64(7));
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088a7bb); };
            c.w64(12, t131);
            c.w64(1, c.r64(2));
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088a7c2); };
            c.w64(0, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088a7c6); };
            c.w64(6, t133);
            blk = 172; continue;
        }
        172 => {
            c.w32(7, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700088a7d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088a7dd); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 175; continue; }
            blk = 173; continue;
        }
        173 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a7e7); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 176; continue; }
            blk = 174; continue;
        }
        174 => {
            blk = 212; continue;
        }
        175 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 212; continue; }
            blk = 176; continue;
        }
        176 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088a806); }
            c.w64(3, c.r64(9));
            return c.call(0x7000820354, 0x700088a812, 0x700088a80d);
        }
        177 => {
            let Some(t134) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088a812); };
            c.cmp(t134, c.r32(0), 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088a81b); };
            c.w64(2, t135);
            blk = 209; continue;
        }
        179 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088a824); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088a828); };
            c.w64(3, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088a82c); };
            c.w64(6, t138);
            return c.call(0x7000870914, 0x700088a835, 0x700088a830);
        }
        180 => {
            let Some(t139) = c.ld8(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088a835); };
            c.logic(t139 & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088a83f); };
            c.w64(2, t140);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 181; continue;
        }
        181 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 196; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t141) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a85e); };
            c.w32(1, t141);
            let r142 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r142);
            let Some(t143) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a864); };
            c.cmp(t143, c.r32(1), 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t144) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a876); };
            c.w32(1, t144);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 284; continue; }
            blk = 185; continue;
        }
        185 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 206; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t145) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a88e); };
            c.w32(1, t145);
            let r146 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r146);
            let Some(t147) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a894); };
            c.cmp(t147, 0x41u64, 16);
            if c.cc_e() { blk = 282; continue; }
            blk = 187; continue;
        }
        187 => {
            blk = 206; continue;
        }
        188 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t148) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a8a9); };
            c.w32(1, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a8ac); };
            c.cmp(t149, c.r32(1), 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 190; continue;
        }
        190 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a8b9); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a8be); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 206; continue; }
            blk = 191; continue;
        }
        191 => {
            if c.cc_e() { blk = 207; continue; }
            blk = 192; continue;
        }
        192 => {
            blk = 206; continue;
        }
        193 => {
            let r150 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r150);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a8da); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 206; continue; }
            blk = 194; continue;
        }
        194 => {
            if c.cc_e() { blk = 207; continue; }
            blk = 195; continue;
        }
        195 => {
            blk = 206; continue;
        }
        196 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t151) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a8fe); };
            c.w32(1, t151);
            let r152 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r152);
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a904); };
            c.cmp(t153, c.r32(1), 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t154) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088a90d); };
            c.cmp(t154, 0x43u64, 16);
            blk = 206; continue;
        }
        199 => {
            c.w64(1, c.r64(2));
            let r155 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r155);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a919); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 206; continue; }
            blk = 200; continue;
        }
        200 => {
            if c.cc_e() { blk = 207; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 206; continue;
        }
        202 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 207; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t156) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088a935); };
            c.w32(1, t156);
            let Some(t157) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088a938); };
            c.cmp(t157, c.r32(1), 32);
            if c.cc_ne() { blk = 207; continue; }
            blk = 204; continue;
        }
        204 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700088a941); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            if c.cc_e() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700088a94e); };
            c.w64(0, t158);
            blk = 208; continue;
        }
        207 => {
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088a957); };
            c.w64(0, t159);
            blk = 208; continue;
        }
        208 => {
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088a95e); };
            c.cmp(t160, c.r32(0), 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t161) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088a967); };
            c.cmp(t161, 0x0u64, 32);
            if c.cc_e() { blk = 287; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088a971); };
            c.w64(1, t162);
            let r163 = c.sub(c.r32(1), 0x2u64, 32);
            c.w32(1, r163);
            if c.cc_o() { blk = 287; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 171; continue;
        }
        212 => {
            c.w64(1, c.r64(12));
            let r164 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r164);
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088a997); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088a998); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088a99e); }
            return c.ret(0x0, 0x700088a9a0);
        }
        213 => {
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088a9a1); };
            c.w64(1, t165);
            let r166 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088a9a9); };
            c.w64(0, t167);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088a9b0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088a9b1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088a9b7); }
            return c.ret(0x0, 0x700088a9b9);
        }
        214 => {
            c.w64(1, c.r64(12).wrapping_add(0x1));
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088a9c9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088a9ca); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088a9d0); }
            return c.ret(0x0, 0x700088a9d2);
        }
        215 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088a9d5); };
            c.w64(3, t168);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700088a9e4, 0x700088a9df);
        }
        216 => {
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088a9e4); };
            c.w64(6, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088a9e8); };
            c.w64(0, t170);
            blk = 1; continue;
        }
        217 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a2054, 0x700088a9f9, 0x700088a9f4);
        }
        218 => {
            c.w64(8, c.r64(0));
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088a9fc); };
            c.w64(6, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088aa00); };
            c.w64(1, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088aa04); };
            c.w64(7, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088aa08); };
            c.w64(0, t174);
            blk = 3; continue;
        }
        219 => {
            let Some(t175) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088aa11); };
            c.w32(9, t175);
            let r176 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r176);
            blk = 22; continue;
        }
        220 => {
            let Some(t177) = c.ld32(c.r64(9).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700088aa1d); };
            c.cmp(t177, c.r32(12), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700088aa2a); };
            c.w64(3, t178);
            c.w64(0, c.r64(8));
            return c.call(0x7000870914, 0x700088aa39, 0x700088aa34);
        }
        222 => {
            c.w64(9, c.r64(0));
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088aa3c); };
            c.w64(8, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088aa40); };
            c.w64(6, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088aa44); };
            c.w64(1, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088aa48); };
            c.w64(7, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088aa4c); };
            c.w64(0, t183);
            blk = 22; continue;
        }
        223 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088aa5b); }
            blk = 226; continue;
        }
        225 => {
            c.w64(7, c.r64(9));
            let r184 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r184);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 226; continue;
        }
        226 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x700088aa8e); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 232; continue; }
            blk = 227; continue;
        }
        227 => {
            if c.cc_ne() { blk = 232; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            c.movq_rx(3, 0);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            c.w64(3, c.r64(7));
            let r185 = c.add(c.r32(3), c.r32(7), 32);
            c.w32(3, r185);
            if c.cc_o() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 42; continue;
        }
        232 => {
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088aabd); };
            c.w64(7, t186);
            let Some(t187) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700088aac4); };
            c.w64(3, t187);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700088aacb); }
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088aacf); };
            c.w64(3, t188);
            let Some(t189) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700088aad6); };
            c.cmp(t189, c.r64(2), 64);
            if c.cc_a() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088aae5, 0x700088aae0);
        }
        234 => {
            c.w64(7, c.r64(0));
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088aae8); };
            c.w64(12, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088aaec); };
            c.w64(8, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088aaf0); };
            c.w64(6, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088aaf4); };
            c.w64(1, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088aaf8); };
            c.w64(9, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088aafc); };
            c.w64(15, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088ab00); };
            c.w64(11, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088ab04); };
            c.w64(0, t197);
            blk = 236; continue;
        }
        235 => {
            if !c.st64(c.r64(7), c.r64(2)) { return RegfileExit::fault(0x700088ab0a); }
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ab0d); };
            c.w64(7, t198);
            let r199 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r199);
            blk = 236; continue;
        }
        236 => {
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088ab15); };
            c.w64(3, t200);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700088ab1c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700088ab1f); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088ab24); }
            blk = 42; continue;
        }
        237 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 239; continue; }
            blk = 238; continue;
        }
        238 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088ab33); }
            blk = 240; continue;
        }
        239 => {
            let r201 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r201);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 240; continue;
        }
        240 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088ab4a); }
            blk = 243; continue;
        }
        242 => {
            let r202 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r202);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 243; continue;
        }
        243 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x700088ab6e); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 249; continue; }
            blk = 244; continue;
        }
        244 => {
            if c.cc_ne() { blk = 249; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 249; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w64(9, c.r64(7));
            let r203 = c.add(c.r32(9), c.r32(7), 32);
            c.w32(9, r203);
            if c.cc_o() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 254; continue;
        }
        249 => {
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088ab9f); };
            c.w64(7, t204);
            let Some(t205) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700088aba6); };
            c.w64(9, t205);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088abad); };
            c.w64(15, t206);
            let Some(t207) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700088abb4); };
            c.cmp(t207, c.r64(11), 64);
            if c.cc_a() { blk = 252; continue; }
            blk = 250; continue;
        }
        250 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088abc3, 0x700088abbe);
        }
        251 => {
            blk = 253; continue;
        }
        252 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700088abc5); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 253; continue;
        }
        253 => {
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088abcf); };
            c.w64(1, t208);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700088abd6); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700088abd9); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088abde); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088abe3); };
            c.w64(1, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088abe7); };
            c.w64(12, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088abeb); };
            c.w64(8, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088abef); };
            c.w64(6, t212);
            blk = 254; continue;
        }
        254 => {
            c.w64(7, c.r64(0));
            blk = 50; continue;
        }
        255 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700088abfb); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700088abff); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 257; continue; }
            blk = 256; continue;
        }
        256 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088ac09); }
            blk = 258; continue;
        }
        257 => {
            let r213 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r213);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 258; continue;
        }
        258 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x700088ac3d); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 264; continue; }
            blk = 259; continue;
        }
        259 => {
            if c.cc_ne() { blk = 264; continue; }
            blk = 260; continue;
        }
        260 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 264; continue; }
            blk = 262; continue;
        }
        262 => {
            c.w64(11, c.r64(9));
            let r214 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r214);
            if c.cc_o() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 269; continue;
        }
        264 => {
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088ac6f); };
            c.w64(9, t215);
            let Some(t216) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088ac76); };
            c.w64(11, t216);
            c.w64(15, c.r64(11).wrapping_add(0xc));
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088ac7d); };
            c.w64(0, t217);
            let Some(t218) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700088ac84); };
            c.cmp(t218, c.r64(15), 64);
            if c.cc_a() { blk = 267; continue; }
            blk = 265; continue;
        }
        265 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700088ac96, 0x700088ac91);
        }
        266 => {
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ac96); };
            c.w64(2, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ac9a); };
            c.w64(12, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088ac9e); };
            c.w64(8, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088aca2); };
            c.w64(6, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088aca6); };
            c.w64(1, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088acaa); };
            c.w64(7, t224);
            blk = 268; continue;
        }
        267 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700088acb0); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 268; continue;
        }
        268 => {
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088acba); };
            c.w64(9, t225);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700088acc1); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x700088acc5); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088acca); }
            blk = 269; continue;
        }
        269 => {
            c.w64(9, c.r64(0));
            blk = 54; continue;
        }
        270 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088acdc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088ace0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x700088ace4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700088ace9); }
            return c.call(0x70007febf4, 0x700088acf2, 0x700088aced);
        }
        271 => {
            c.w64(15, c.r64(0));
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088acf5); };
            c.w64(12, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088acf9); };
            c.w64(1, t227);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ad00); };
            c.w64(9, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088ad04); };
            c.w64(8, t229);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700088ad08); }
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088ad0d); };
            c.w64(7, t230);
            blk = 97; continue;
        }
        272 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700088ad16); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700088ad17); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088ad19); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700088ad1d); }
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700088ad21); };
            c.w64(3, t231);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700088ad35, 0x700088ad30);
        }
        273 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088ad35); };
            c.w64(7, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ad39); };
            c.w64(12, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ad3d); };
            c.w64(1, t234);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ad44); };
            c.w64(9, t235);
            blk = 125; continue;
        }
        274 => {
            let Some(t236) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ad4d); };
            c.w32(8, t236);
            let r237 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r237);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088ad54); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700088ad58); }
            let Some(t238) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088ad5c); };
            c.cmp(t238, 0x40u64, 16);
            if c.cc_ae() { blk = 123; continue; }
            blk = 275; continue;
        }
        275 => {
            c.w64(2, c.r64(1));
            c.w64(0, c.r64(15));
            c.w64(6, c.r64(11));
            return c.call(0x70007fb534, 0x700088ad76, 0x700088ad71);
        }
        276 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088ad76); };
            c.w64(7, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ad7a); };
            c.w64(12, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ad7e); };
            c.w64(1, t241);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ad85); };
            c.w64(9, t242);
            blk = 125; continue;
        }
        277 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700088ad8e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088ad8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088ad90); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700088ad94); }
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700088ad98); };
            c.w64(3, t243);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700088adac, 0x700088ada7);
        }
        278 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088adac); };
            c.w64(7, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088adb0); };
            c.w64(12, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088adb4); };
            c.w64(1, t246);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088adbb); };
            c.w64(9, t247);
            blk = 164; continue;
        }
        279 => {
            let Some(t248) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088adc4); };
            c.w32(8, t248);
            let r249 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r249);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088adcb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700088adcf); }
            let Some(t250) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088add3); };
            c.cmp(t250, 0x40u64, 16);
            if c.cc_ae() { blk = 162; continue; }
            blk = 280; continue;
        }
        280 => {
            c.w64(2, c.r64(1));
            c.w64(6, c.r64(11));
            return c.call(0x70007fb534, 0x700088adea, 0x700088ade5);
        }
        281 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088adea); };
            c.w64(7, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088adee); };
            c.w64(12, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088adf2); };
            c.w64(1, t253);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088adf9); };
            c.w64(9, t254);
            blk = 164; continue;
        }
        282 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700088ae02); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088ae03); }
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700088ae04); };
            c.w64(3, t255);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088ae17, 0x700088ae12);
        }
        283 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ae17); };
            c.w64(2, t256);
            blk = 208; continue;
        }
        284 => {
            let Some(t257) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ae20); };
            c.w32(1, t257);
            let r258 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r258);
            let Some(t259) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088ae26); };
            c.cmp(t259, 0x40u64, 16);
            if c.cc_ae() { blk = 206; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x700088ae38, 0x700088ae33);
        }
        286 => {
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ae38); };
            c.w64(2, t260);
            blk = 208; continue;
        }
        287 => {
            let Some(t261) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088ae41); };
            c.cmp(t261, 0x0u64, 32);
            if c.cc_ne() { blk = 289; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ae47); };
            c.w64(0, t262);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088ae4b); }
            blk = 290; continue;
        }
        289 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ae52); };
            c.w64(0, t263);
            let r264 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r264);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 290; continue;
        }
        290 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700088ae7e); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 296; continue; }
            blk = 291; continue;
        }
        291 => {
            if c.cc_ne() { blk = 296; continue; }
            blk = 292; continue;
        }
        292 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 294; continue; }
            blk = 293; continue;
        }
        293 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 296; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w64(7, c.r64(1));
            let r265 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r265);
            if c.cc_o() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 301; continue;
        }
        296 => {
            let Some(t266) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088aead); };
            c.w64(1, t266);
            let Some(t267) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700088aeb4); };
            c.w64(7, t267);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088aebb); };
            c.w64(9, t268);
            let Some(t269) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088aec2); };
            c.cmp(t269, c.r64(8), 64);
            if c.cc_a() { blk = 299; continue; }
            blk = 297; continue;
        }
        297 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x700088aed4, 0x700088aecf);
        }
        298 => {
            blk = 300; continue;
        }
        299 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700088aed6); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 300; continue;
        }
        300 => {
            let Some(t270) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088aee0); };
            c.w64(1, t270);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700088aee7); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700088aeea); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088aeef); }
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088aef4); };
            c.w64(2, t271);
            blk = 301; continue;
        }
        301 => {
            c.w64(7, c.r64(0));
            blk = 171; continue;
        }
        302 => {
            return RegfileExit::jump(0x700088af04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088af14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088af14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088af18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088af19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088af1a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088af1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088af29); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088af37); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088af3e); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088af57); };
            c.w64(2, t7);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088af64); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088af6b); };
            c.w32(8, t10);
            let r11 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r11);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700088af84); };
            c.w64(8, t12);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088af8e); };
            c.w32(9, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088af92); };
            c.cmp(t14, c.r32(9), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088af9b); };
            c.w64(3, t15);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088afa4); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088afab); };
            c.logic(t18 & 0x2u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700088afb5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088afb7); }
            c.w64(0, c.r64(7));
            let Some(t19) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088afbc); };
            c.w64(1, t19);
            return c.call(0x700088b394, 0x700088afc5, 0x700088afc0);
        }
        10 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088afc5); };
            c.w64(1, t20);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088afcc); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x700088afd3);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088afd6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088afdd); }
            return c.ret(0x0, 0x700088afdf);
        }
        13 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088afe6); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700088afe8); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088afe9); };
            c.w64(3, t21);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700088affa, 0x700088aff5);
        }
        14 => {
            return RegfileExit::fault(0x700088affa);
        }
        15 => {
            return RegfileExit::fault(0x700088affb);
        }
        16 => {
            return RegfileExit::fault(0x700088affc);
        }
        17 => {
            return RegfileExit::fault(0x700088affd);
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088affe); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088b002); };
            c.w64(3, t22);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088b010, 0x700088b00b);
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088b010); };
            c.w64(6, t23);
            blk = 1; continue;
        }
        20 => {
            return RegfileExit::jump(0x700088b01c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088b034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088b034); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088b038); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088b039); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088b03a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088b03f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088b049); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b057); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b05e); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 19; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088b077); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b084); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088b08b); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700088b096); };
            c.w64(2, t11);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b0a3); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b0aa); };
            c.w32(8, t14);
            let r15 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r15);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 17; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088b0bf); };
            c.w64(8, t16);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b0c9); };
            c.w32(9, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088b0cd); };
            c.cmp(t18, c.r32(9), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700088b0d6); };
            c.w64(9, t19);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b0e0); };
            c.w32(11, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088b0e4); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088b0ed); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700088b0ef); }
            c.w64(0, c.r64(7));
            let Some(t22) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088b0f4); };
            c.w64(1, t22);
            return c.call(0x700088b394, 0x700088b0fd, 0x700088b0f8);
        }
        12 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088b0fd); };
            c.w64(1, t23);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088b104); }
            c.cmp(c.r64(1), 0x5u64, 64);
            if c.cc_g() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.ret(0x30, 0x700088b10b);
        }
        14 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088b10e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088b115); }
            return c.ret(0x0, 0x700088b117);
        }
        15 => {
            return RegfileExit::fault(0x700088b118);
        }
        16 => {
            return RegfileExit::fault(0x700088b119);
        }
        17 => {
            return RegfileExit::fault(0x700088b11a);
        }
        18 => {
            return RegfileExit::fault(0x700088b11b);
        }
        19 => {
            return RegfileExit::fault(0x700088b11c);
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088b11d); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088b121); };
            c.w64(3, t24);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088b12f, 0x700088b12a);
        }
        21 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088b12f); };
            c.w64(6, t25);
            blk = 1; continue;
        }
        22 => {
            return RegfileExit::jump(0x700088b13c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088b154(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088b154); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088b158); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088b159); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088b15a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088b15f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088b163); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 24; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088b16d); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b17b); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b182); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 23; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088b19b); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b1a8); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088b1af); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700088b1ba); };
            c.w64(0, t11);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b1c6); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b1cd); };
            c.w32(8, t14);
            let r15 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r15);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088b1e6); };
            c.w64(8, t16);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w32(11, c.r32(8));
            c.cmp(c.r32(9), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700088b205); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700088b209); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700088b20d); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b217); };
            c.w32(15, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088b21b); };
            c.cmp(t18, c.r32(15), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700088b228); };
            c.w64(15, t19);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b232); };
            c.w32(2, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088b236); };
            c.cmp(t21, c.r32(2), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_add(0x40)) else { return RegfileExit::fault(0x700088b23f); };
            c.w64(1, t22);
            c.w64(3, c.r64(8));
            return c.call(0x7000899bb4, 0x700088b24b, 0x700088b246);
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088b24b); };
            c.cmp(t23, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088b251); };
            c.w64(8, t24);
            let r25 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r25);
            if c.cc_o() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        15 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088b25e); }
            let Some(t26) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700088b260); };
            if !c.push64(t26) { return RegfileExit::fault(0x700088b260); }
            let Some(t27) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088b263); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088b267); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088b26b); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700088b26f); };
            c.w64(2, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088b273); };
            c.w64(6, t31);
            c.w64(7, c.r64(0));
            return c.call(0x700088b394, 0x700088b27f, 0x700088b27a);
        }
        16 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088b27f); };
            c.w64(1, t32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088b286); }
            c.cmp(c.r64(1), 0x6u64, 64);
            if c.cc_g() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x38, 0x700088b28d);
        }
        18 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088b290); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088b297); }
            return c.ret(0x0, 0x700088b299);
        }
        19 => {
            return RegfileExit::fault(0x700088b29a);
        }
        20 => {
            return RegfileExit::fault(0x700088b29b);
        }
        21 => {
            return RegfileExit::fault(0x700088b29c);
        }
        22 => {
            return RegfileExit::fault(0x700088b29d);
        }
        23 => {
            return RegfileExit::fault(0x700088b29e);
        }
        24 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088b29f); };
            c.w64(3, t33);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088b2ad, 0x700088b2a8);
        }
        25 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088b2ad); };
            c.w64(6, t34);
            blk = 1; continue;
        }
        26 => {
            let Some(t35) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088b2b6); };
            c.cmp(t35, 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088b2bc); };
            c.w64(8, t36);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088b2c0); }
            blk = 29; continue;
        }
        28 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088b2c8); };
            c.w64(8, t37);
            let r38 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r38);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 29; continue;
        }
        29 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700088b2f8); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            if c.cc_ne() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, c.r64(8));
            let r39 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r39);
            if c.cc_o() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 40; continue;
        }
        35 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088b32a); };
            c.w64(8, t40);
            let Some(t41) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088b331); };
            c.w64(9, t41);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088b338); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700088b33f); };
            c.cmp(t43, c.r64(11), 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088b34f, 0x700088b34a);
        }
        37 => {
            blk = 39; continue;
        }
        38 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700088b351); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 39; continue;
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088b35b); };
            c.w64(8, t44);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700088b362); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700088b366); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088b36b); }
            blk = 40; continue;
        }
        40 => {
            c.w64(8, c.r64(0));
            blk = 15; continue;
        }
        41 => {
            return RegfileExit::jump(0x700088b37c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088b394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088b394); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700088b398); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088b39e); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700088b3ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700088b3b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700088b3b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700088b3ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700088b3be); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x700088b3c2); }
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088b3c6); };
            c.w64(0, t2);
            blk = 2; continue;
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088b3cc); };
            c.w64(8, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088b3d0); };
            c.w64(7, t4);
            blk = 2; continue;
        }
        2 => {
            c.w32(15, c.r32(0));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x700088b3d7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700088b3e5); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088b3ef); }
            let Some(t5) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088b3f4); };
            c.cmp(t5, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088b3fa); }
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
            let Some(t7) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088b411); };
            c.cmp(t7, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(3, c.r64(0));
            let r8 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r8);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088b41c); }
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
            blk = 249; continue;
        }
        10 => {
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ge() { blk = 249; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700088b43e); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 273; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b44c); };
            c.w32(3, t9);
            let r10 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r10);
            let Some(t11) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b452); };
            c.w32(8, t11);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 273; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 266; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b479); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b480); };
            c.w32(9, t14);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088b48c); }
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
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088b4c7); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088b4cb); };
            c.w64(1, t17);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 252; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b4e1); };
            c.w32(2, t18);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 273; continue; }
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
            if c.cc_ne() { blk = 273; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(9, c.r64(0));
            blk = 30; continue;
        }
        27 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088b512); };
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
            let Some(t22) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b533); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b539); };
            c.w32(8, t24);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088b54b); };
            c.w32(1, t25);
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x700088b55a); };
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
            let Some(t30) = c.ld16(c.r64(2).wrapping_add(0x5)) else { return RegfileExit::fault(0x700088b57e); };
            c.w32(6, (t30 as u16 as i16 as i64 as u64));
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b582); };
            c.w32(7, t31);
            let r32 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r32);
            let r33 = c.sub(c.r32(6), 0x1u64, 32);
            c.w32(6, r33);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700088b58c); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700088b595); }
            blk = 38; continue;
        }
        38 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088b599); };
            c.w64(0, t34);
            let r35 = c.sub(c.r32(0), c.r32(15), 32);
            c.w32(0, r35);
            let r36 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r36);
            let r37 = c.add(c.r32(0), c.r32(15), 32);
            c.w32(0, r37);
            c.w32(12, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088b5ac); };
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
            let Some(t42) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088b5c6); };
            c.w32(12, t42);
            let r43 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r43);
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b5ce); };
            c.w32(12, t44);
            let r45 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r45);
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_be() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(15, c.r64(0).wrapping_add(0x1));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088b5e0); };
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
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088b5ff); };
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
            let Some(t52) = c.ld32(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088b619); };
            c.w32(0, t52);
            let r53 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r53);
            let Some(t54) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b620); };
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
            let Some(t58) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088b659); };
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
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b66f); };
            c.w32(11, t59);
            let r60 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r60);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700088b67c); };
            c.w64(11, t61);
            blk = 54; continue;
        }
        54 => {
            let Some(t62) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088b683); };
            c.w32(12, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r63 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r63);
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b691); };
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
            let Some(t70) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b6b5); };
            c.w32(2, t70);
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088b6ba); };
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
            if c.cc_ne() { blk = 260; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t72) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088b6df); };
            c.logic(t72 & 0x24u64, 8);
            if c.cc_ne() { blk = 265; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t73) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b6e9); };
            c.w32(11, t73);
            let r74 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r74);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700088b6f6); };
            c.w64(11, t75);
            blk = 63; continue;
        }
        63 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088b6fd); };
            c.w32(12, t76);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r77 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r77);
            let Some(t78) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b70b); };
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
            let Some(t84) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b72e); };
            c.w32(1, t84);
            let r85 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r85);
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088b736); };
            c.cmp(t86, c.r32(1), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t87) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088b73f); };
            c.cmp(t87, c.r32(1), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t88) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b748); };
            c.cmp(t88, c.r32(9), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t89) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088b74e); };
            c.w32(11, t89);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 289; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t90) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088b761); };
            c.w32(11, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088b765); };
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
            let Some(t92) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b77f); };
            c.w32(8, t92);
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            let Some(t94) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b786); };
            c.w32(8, t94);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 101; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b796); };
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
            blk = 254; continue;
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
            blk = 254; continue;
        }
        84 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088b803); };
            c.w32(11, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700088b808); };
            c.cmp(t99, c.r32(11), 32);
            if c.cc_ne() { blk = 255; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088b815); };
            c.w32(12, t100);
            let r101 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r101);
            let Some(t102) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b81d); };
            c.w32(11, t102);
            let r103 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r103);
            let Some(t104) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b825); };
            c.w32(11, t104);
            blk = 76; continue;
        }
        86 => {
            let Some(t105) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088b82c); };
            c.w32(11, t105);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r106 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r106);
            let Some(t107) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088b83a); };
            c.w32(12, t107);
            let r108 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b842); };
            c.w32(11, t109);
            let r110 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r110);
            let Some(t111) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b84a); };
            c.w32(11, t111);
            blk = 76; continue;
        }
        87 => {
            let Some(t112) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088b854); };
            c.w32(12, t112);
            let r113 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r113);
            let Some(t114) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b85c); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b864); };
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
            if c.cc_ne() { blk = 256; continue; }
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
            let Some(t119) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x700088b89e); };
            c.w32(8, t119);
            blk = 96; continue;
        }
        95 => {
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700088b8a5); };
            c.w32(8, t120);
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 264; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 262; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 261; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088b8d8); };
            c.w64(0, t121);
            let Some(t122) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b8dc); };
            c.w32(8, t122);
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700088b8e0); };
            c.cmp(t123, c.r32(8), 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t124) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b8e9); };
            c.logic(t124 & 0x1u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t125) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088b8f3); };
            c.w32(2, t125);
            let r126 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088b8f9); };
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
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088b914); };
            c.w64(1, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088b918); };
            c.w64(6, t129);
            blk = 107; continue;
        }
        107 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x700088b91c); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            blk = 109; continue;
        }
        108 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088b928); };
            c.w32(3, t130);
            let r131 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r131);
            let Some(t132) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088b92f); };
            c.w32(9, t132);
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 273; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t133) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088b941); };
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
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
            blk = 273; continue;
        }
        179 => {
            blk = 184; continue;
        }
        180 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bb6a); };
            c.w32(8, t136);
            let r137 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r137);
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088bb71); };
            c.w32(9, t138);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_ae() { blk = 185; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t139) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bb8a); };
            c.cmp(t139, c.r64(9), 64);
            if c.cc_e() { blk = 203; continue; }
            blk = 182; continue;
        }
        182 => {
            blk = 240; continue;
        }
        183 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088bb9a); };
            c.w32(9, t140);
            let r141 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r141);
            let Some(t142) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bba1); };
            c.w32(9, t142);
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 240; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bbae); };
            c.w32(8, t143);
            let r144 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r144);
            let Some(t145) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088bbb5); };
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
            if c.cc_b() { blk = 273; continue; }
            blk = 186; continue;
        }
        186 => {
            blk = 203; continue;
        }
        187 => {
            let Some(t146) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bbd8); };
            c.w32(8, t146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088bbdd); };
            c.cmp(t147, c.r32(8), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 188; continue;
        }
        188 => {
            blk = 240; continue;
        }
        189 => {
            let Some(t148) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088bbef); };
            c.w32(9, t148);
            let r149 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r149);
            let Some(t150) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bbf6); };
            c.w32(9, t150);
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 240; continue; }
            blk = 190; continue;
        }
        190 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x700088bc03); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_b() { blk = 273; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bc16); };
            c.w32(8, t151);
            let r152 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r152);
            let Some(t153) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088bc1d); };
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700088bc39); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700088bc3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700088bc42); }
            c.w64(2, c.r64(12));
            c.w64(1, c.r64(13));
            c.w64(15, c.r64(1));
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700088bc4f); };
            c.w64(0, t157);
            c.w64(10, 0x700088bc5du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700088bc5d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700088bc61); }
            return c.call(c.r64(0), 0x700088bc67, 0x700088bc65);
        }
        192 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700088bc67); }
            let Some(t158) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088bc6f); };
            c.w64(4, t158);
            c.w32(8, c.r32(0));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088bc77); };
            let r160 = c.r64(8) & t159;
            c.w64(8, r160);
            c.logic(r160, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088bc7e); };
            c.w64(11, t161);
            blk = 194; continue;
        }
        193 => {
            let r162 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r162);
            let r163 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088bc8b); };
            c.w64(12, t164);
            let r165 = c.r64(12) & c.r64(8);
            c.w64(12, r165);
            c.logic(r165, 64);
            c.w64(8, c.r64(12));
            blk = 194; continue;
        }
        194 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t166) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bc9a); };
            c.w32(12, t166);
            let r167 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088bca2); };
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
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088bcb1); };
            c.cmp(t169, c.r32(12), 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 197; continue;
        }
        197 => {
            c.xorpd_rr(0, 0);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088bcbe); };
            c.cvtsi2sd(0, (t170 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700088bcc4); }
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
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088bcdf); };
            c.cmp(c.r64(12), t171, 64);
            if c.cc_e() { blk = 240; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 193; continue;
        }
        202 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bceb); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088bcef); };
            c.w64(3, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088bcf3); };
            c.w64(2, t174);
            blk = 203; continue;
        }
        203 => {
            let Some(t175) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088bcf7); };
            c.w32(8, t175);
            let r176 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088bcfe); };
            c.w64(6, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088bd02); };
            c.w64(1, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088bd06); };
            c.w64(7, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088bd0a); };
            c.cmp(t180, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 204; continue;
        }
        204 => {
            blk = 108; continue;
        }
        205 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088bd1c); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700088bd23); };
            c.logic(t183 & 0x4u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(9, c.r64(2));
            let Some(t184) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700088bd31); };
            c.cmp(t184, c.r64(9), 64);
            if c.cc_a() { blk = 240; continue; }
            blk = 207; continue;
        }
        207 => {
            blk = 241; continue;
        }
        208 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088bd40); };
            c.w32(11, t185);
            let r186 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r186);
            let Some(t187) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700088bd47); };
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
            let Some(t188) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088bd62); };
            c.w64(11, t188);
            let Some(t189) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088bd66); };
            c.w64(12, t189);
            let Some(t190) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700088bd6a); };
            c.logic(t190 & 0x1u64, 8);
            if c.cc_ne() { blk = 213; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t191) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700088bd71); };
            c.w64(15, t191);
            let r192 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r192);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 241; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t193) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700088bd81); };
            c.w64(8, t193);
            blk = 239; continue;
        }
        213 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 241; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t194) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088bd93); };
            c.w32(8, t194);
            let r195 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r195);
            let Some(t196) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088bd9a); };
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
            blk = 250; continue;
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
            blk = 250; continue;
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
            blk = 250; continue;
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
            blk = 250; continue;
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700088be5d); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700088be68); };
            c.w64(0, t202);
            c.w64(10, 0x700088be76u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700088be76); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700088be7a); }
            return c.call(c.r64(0), 0x700088be80, 0x700088be7e);
        }
        238 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700088be80); }
            let Some(t203) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088be88); };
            c.w64(4, t203);
            c.w64(8, c.r64(0));
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088be90); };
            c.w64(0, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088be94); };
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
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088bea0); };
            c.w64(8, t206);
            blk = 242; continue;
        }
        241 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700088bea9); };
            c.w64(8, t207);
            blk = 242; continue;
        }
        242 => {
            let Some(t208) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088beb0); };
            c.cmp(t208, c.r32(8), 32);
            if c.cc_ne() { blk = 246; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088beb9); };
            c.w64(0, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bebd); };
            c.w64(3, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088bec1); };
            c.w64(6, t211);
            return c.call(0x7000870914, 0x700088beca, 0x700088bec5);
        }
        244 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088beca); };
            if !c.push64(t212) { return RegfileExit::fault(0x700088beca); }
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088becd); };
            if !c.push64(t213) { return RegfileExit::fault(0x700088becd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088bed0); }
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088bed1); };
            if !c.push64(t214) { return RegfileExit::fault(0x700088bed1); }
            c.w32(0, 0x3u64);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088bed9); };
            c.w64(7, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088bedd); };
            c.w64(6, t216);
            return c.call(0x70007f2274, 0x700088bee6, 0x700088bee1);
        }
        245 => {
            c.w64(1, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088bee9); };
            c.w64(0, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088beed); };
            c.w64(3, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088bef1); };
            c.w64(6, t219);
            return c.call(0x7000899bb4, 0x700088befa, 0x700088bef5);
        }
        246 => {
            let Some(t220) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088befa); };
            c.cmp(t220, 0x0u64, 32);
            if c.cc_e() { blk = 275; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088bf04); };
            c.w64(8, t221);
            let r222 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r222);
            if c.cc_o() { blk = 275; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        249 => {
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088bf1a); };
            c.w64(0, t223);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088bf21); }
            return c.ret(0x10, 0x700088bf22);
        }
        250 => {
            return RegfileExit::fault(0x700088bf25);
        }
        251 => {
            blk = 273; continue;
        }
        252 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 273; continue; }
            blk = 253; continue;
        }
        253 => {
            let Some(t224) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088bf36); };
            c.w32(9, t224);
            let r225 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r225);
            blk = 30; continue;
        }
        254 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x700088bf42); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088bf46); }
            blk = 258; continue;
        }
        255 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x700088bf4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088bf50); }
            blk = 258; continue;
        }
        256 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x700088bf56); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088bf5a); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 258; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t226) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088bf64); };
            c.w64(12, t226);
            blk = 93; continue;
        }
        258 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700088bf6e); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088bf70); }
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700088bf71); };
            c.w64(3, t227);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700088bf85, 0x700088bf80);
        }
        259 => {
            let r228 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r228);
            c.w64(8, c.r64(0));
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088bf8a); };
            c.w64(3, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088bf8e); };
            c.w64(9, t230);
            blk = 96; continue;
        }
        260 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bf97); };
            c.w64(0, t231);
            blk = 273; continue;
        }
        261 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bf9d); };
            c.w64(0, t232);
            blk = 273; continue;
        }
        262 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bfa3); };
            c.w64(0, t233);
            blk = 273; continue;
        }
        263 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bfa9); };
            c.w64(0, t234);
            blk = 273; continue;
        }
        264 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bfaf); };
            c.w64(0, t235);
            blk = 273; continue;
        }
        265 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bfb5); };
            c.w64(0, t236);
            blk = 273; continue;
        }
        266 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088bfbb); };
            c.w64(6, t237);
            return c.call(0x70008a1af4, 0x700088bfc4, 0x700088bfbf);
        }
        267 => {
            let Some(t238) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088bfc4); };
            c.w32(8, t238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700088bfc8); };
            c.cmp(t239, c.r32(8), 32);
            if c.cc_ne() { blk = 270; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t240) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088bfd1); };
            c.logic(t240 & 0x1u64, 8);
            if c.cc_e() { blk = 271; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bfd7); };
            c.w64(0, t241);
            blk = 241; continue;
        }
        270 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088bfe0); };
            c.w64(3, t242);
            blk = 271; continue;
        }
        271 => {
            c.w64(3, c.r64(0));
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088bfe7); };
            c.w64(0, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088bfeb); };
            c.w64(6, t244);
            return c.call(0x70008bced4, 0x700088bff4, 0x700088bfef);
        }
        272 => {
            c.w64(8, c.r64(0));
            blk = 242; continue;
        }
        273 => {
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088bffc); };
            if !c.push64(t245) { return RegfileExit::fault(0x700088bffc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088bfff); }
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x700088c000); };
            c.w64(3, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088c007); };
            c.w64(6, t247);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700088c018, 0x700088c013);
        }
        274 => {
            c.w64(8, c.r64(0));
            blk = 242; continue;
        }
        275 => {
            let Some(t248) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088c020); };
            c.cmp(t248, 0x0u64, 32);
            if c.cc_ne() { blk = 277; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088c026); };
            c.w64(0, t249);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088c02a); }
            blk = 278; continue;
        }
        277 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088c031); };
            c.w64(0, t250);
            let r251 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r251);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 278; continue;
        }
        278 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 1) { return RegfileExit::fault(0x700088c05f); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 284; continue; }
            blk = 279; continue;
        }
        279 => {
            if c.cc_ne() { blk = 284; continue; }
            blk = 280; continue;
        }
        280 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 282; continue; }
            blk = 281; continue;
        }
        281 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 284; continue; }
            blk = 282; continue;
        }
        282 => {
            c.w64(9, c.r64(8));
            let r252 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r252);
            if c.cc_o() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        284 => {
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c091); };
            c.w64(8, t253);
            let Some(t254) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088c098); };
            c.w64(9, t254);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088c09f); };
            c.w64(12, t255);
            let Some(t256) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700088c0a6); };
            c.cmp(t256, c.r64(11), 64);
            if c.cc_a() { blk = 287; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088c0b6, 0x700088c0b1);
        }
        286 => {
            blk = 288; continue;
        }
        287 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700088c0b8); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 288; continue;
        }
        288 => {
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088c0c2); };
            c.w64(8, t257);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700088c0c9); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x700088c0cd); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088c0d2); }
            blk = 1; continue;
        }
        289 => {
            return RegfileExit::fault(0x700088c0dc);
        }
        290 => {
            return RegfileExit::jump(0x700088c0e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088c0f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088c0f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088c0f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088c0f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088c0fa); }
            let r0 = c.sub(c.r64(4), 0x78u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088c0ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700088c103); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088c107); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 210; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088c11f); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088c124); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088c133); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088c142); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700088c146); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x700088c14a); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x700088c163); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 212; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c171); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            let Some(t8) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c178); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 212; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(15, c.r64(11));
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x700088c192); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c19b); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c1a2); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088c1aa); };
            c.w32(3, t12);
            let r13 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r13);
            c.w64(2, c.r64(3));
            blk = 30; continue;
        }
        8 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            c.w32(2, 0x1u64);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c1c7); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088c1cd); };
            c.w32(2, t16);
            let r17 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r17);
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c1d3); };
            c.w32(1, t18);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c1d9); };
            c.w32(7, t19);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xb0u64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700088c1e9);
        }
        13 => {
            let Some(t20) = c.ld32(c.r64(2).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700088c1ea); };
            c.cmp(t20, c.r32(7), 32);
            if c.cc_e() { blk = 214; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 216; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c1ff); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088c205); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c20b); };
            c.w32(2, t25);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c211); };
            c.w32(1, t26);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x44u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700088c21e);
        }
        17 => {
            let Some(t27) = c.ld32(c.r64(3).wrapping_add(0x117)) else { return RegfileExit::fault(0x700088c21f); };
            c.cmp(t27, c.r32(1), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(2), 0xaeu64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x700088c230);
        }
        20 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088c231); };
            c.cmp(t28, c.r32(1), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r64(2), 0xafu64, 64);
            if c.cc_a() { blk = 215; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x700088c246);
        }
        23 => {
            let Some(t29) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088c247); };
            c.w32(3, t29);
            let r30 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r30);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a17f4, 0x700088c25b, 0x700088c256);
        }
        26 => {
            c.w64(2, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088c25e); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088c262); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088c266); };
            c.w64(15, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088c26a); };
            c.w64(11, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088c26e); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088c272); };
            c.w64(12, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088c276); };
            c.w64(9, t37);
            blk = 30; continue;
        }
        27 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(2, c.r64(3));
            blk = 30; continue;
        }
        29 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 30; continue;
        }
        30 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(0), 0x0u64, 64);
            c.w8(1, (c.cc_be() as u64));
            c.w32(1, c.r8(1));
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x700088c297); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700088c29b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700088c29f); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 247; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t38) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088c2b2); };
            c.w64(7, t38);
            c.w64(8, c.r64(7));
            blk = 34; continue;
        }
        33 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088c2bb); };
            c.w64(8, t39);
            blk = 34; continue;
        }
        34 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088c2c2); };
            c.w64(7, t40);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700088c2c9); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 248; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c2d7); };
            c.w32(1, t41);
            let r42 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r42);
            let Some(t43) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088c2de); };
            c.logic(t43 & 0x2u64, 8);
            if c.cc_e() { blk = 218; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t44) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088c2ee); };
            c.w64(9, t44);
            blk = 39; continue;
        }
        38 => {
            c.w64(9, c.r64(7));
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700088c2f7); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xfc0)) else { return RegfileExit::fault(0x700088c2fb); };
            c.w64(1, t45);
            let Some(t46) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088c302); };
            c.cmp(t46, c.r32(3), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t47) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c315); };
            c.w32(12, t47);
            let r48 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r48);
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(12)) { return RegfileExit::fault(0x700088c31c); }
            let Some(t49) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c323); };
            c.cmp(t49, 0x834u64, 16);
            if c.cc_ne() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(1, 0x1u64);
            blk = 44; continue;
        }
        43 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t50) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088c33f); };
            c.w32(1, t50);
            c.w64(7, c.r64(1));
            let r51 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r51);
            let r52 = c.r32(7) & 0x3fu64;
            c.w32(7, r52);
            c.logic(r52, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(1)) { return RegfileExit::fault(0x700088c34e); }
            c.cmp(c.r32(7), 0xbu64, 32);
            if c.cc_g() { blk = 203; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t53) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c35e); };
            c.w32(7, t53);
            let r54 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r54);
            let Some(t55) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088c364); };
            c.w32(7, t55);
            let r56 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r56);
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c36a); };
            c.w32(9, t57);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            return RegfileExit::fault(0x700088c37a);
        }
        48 => {
            let Some(t58) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x700088c37b); };
            c.w32(9, t58);
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088c382); };
            c.cmp(t59, c.r32(9), 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088c38d); };
            c.w64(9, t60);
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088c394); };
            c.cmp(t61, c.r32(3), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 50; continue;
        }
        50 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 203; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_g() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088c3ac); };
            c.w64(7, t62);
            blk = 61; continue;
        }
        53 => {
            c.w64(7, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(7));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 244; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c3ce); };
            c.w64(0, t63);
            let Some(t64) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700088c3d5); };
            c.w64(0, t64);
            c.w64(6, c.r64(7).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(6).wrapping_add(c.r64(0)));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088c3e4); };
            c.w64(9, t65);
            let Some(t66) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088c3eb); };
            c.cmp(t66, c.r64(1), 64);
            if c.cc_be() { blk = 219; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_ae() { blk = 219; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c401); };
            c.w64(9, t67);
            if !c.st64(c.r64(9), c.r64(1)) { return RegfileExit::fault(0x700088c408); }
            c.w64(9, c.r64(0).wrapping_add(0x1));
            blk = 57; continue;
        }
        57 => {
            c.w64(0, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let r68 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700088c41a); };
            c.w64(1, t69);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700088c421); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700088c425); }
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088c42f); };
            c.w64(15, t70);
            blk = 59; continue;
        }
        59 => {
            let r71 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r71);
            if !c.st32(c.r64(0).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x700088c43a); }
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_ne() { blk = 59; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(7, c.r64(9));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088c447); };
            c.w64(6, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088c44b); };
            c.w64(0, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088c44f); };
            c.w64(15, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700088c453); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088c45a); };
            c.w64(9, t76);
            blk = 61; continue;
        }
        61 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x700088c461); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), 0x1u64) { return RegfileExit::fault(0x700088c465); }
            c.w64(8, c.r64(3));
            if !c.st64(c.r64(5).wrapping_sub(0x70), 0x1u64) { return RegfileExit::fault(0x700088c470); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), 0x0u64) { return RegfileExit::fault(0x700088c478); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088c480); };
            c.w64(0, t77);
            c.w64(15, c.r64(9));
            blk = 63; continue;
        }
        62 => {
            let r78 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r78);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(1)) { return RegfileExit::fault(0x700088c493); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(12)) { return RegfileExit::fault(0x700088c497); }
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700088c49b); };
            c.w64(12, t79);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700088c4a2); }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_ge() { blk = 105; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t80) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c4af); };
            c.cmp(t80, c.r32(12), 32);
            if c.cc_ne() { blk = 159; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t81) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088c4b9); };
            c.cmp(t81, c.r32(3), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t82) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088c4c3); };
            c.w32(9, t82);
            let r83 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r83);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 67; continue;
        }
        67 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088c4dc); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088c4e8); }
            blk = 72; continue;
        }
        69 => {
            c.w64(9, c.r64(8));
            let r84 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r84);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 72; continue;
        }
        70 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 71; continue;
        }
        71 => {
            let r85 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r85);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088c50a); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 72; continue;
        }
        72 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 159; continue; }
            blk = 73; continue;
        }
        73 => {
            blk = 75; continue;
        }
        74 => {
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_le() { blk = 159; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t86) = c.ld8(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700088c52e); };
            c.w32(9, t86);
            let r87 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r87);
            let r88 = c.r32(9) & 0x3fu64;
            c.w32(9, r88);
            c.logic(r88, 32);
            let r89 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r89);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t90) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c547); };
            c.w32(9, t90);
            let r91 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r91);
            let Some(t92) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c54e); };
            c.w32(12, t92);
            let r93 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r93);
            let Some(t94) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c555); };
            c.w32(12, t94);
            let r95 = c.sub(c.r32(12), 0x77u64, 32);
            c.w32(12, r95);
            c.cmp(c.r32(12), 0xcu64, 32);
            if c.cc_a() { blk = 153; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t96) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c569); };
            c.w32(12, t96);
            c.cmp(c.r32(8), c.r32(12), 32);
            if c.cc_ae() { blk = 152; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t97) = c.ld32(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c579); };
            c.w32(9, t97);
            let r98 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088c581); };
            c.cmp(t99, c.r32(9), 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 79; continue;
        }
        79 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700088c58e); }
            blk = 86; continue;
        }
        80 => {
            let Some(t100) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c597); };
            c.w32(9, t100);
            let r101 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r101);
            let Some(t102) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c59e); };
            c.w32(12, t102);
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700088c5a2); };
            c.cmp(t103, c.r32(12), 32);
            if c.cc_ne() { blk = 158; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t104) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c5af); };
            c.w32(12, t104);
            c.cmp(c.r32(8), c.r32(12), 32);
            if c.cc_ae() { blk = 154; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x700088c5bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700088c5c6); }
            c.w64(8, 0xfff7fffffff7ffffu64);
            let Some(t105) = c.ld64(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c5d4); };
            c.cmp(t105, c.r64(8), 64);
            if c.cc_e() { blk = 103; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c5df); };
            c.w64(8, t106);
            let Some(t107) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088c5e6); };
            c.w64(9, t107);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088c5ed); };
            c.w64(11, t108);
            let Some(t109) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700088c5f4); };
            c.cmp(t109, c.r64(12), 64);
            if c.cc_be() { blk = 221; continue; }
            blk = 84; continue;
        }
        84 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700088c5fd); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 85; continue;
        }
        85 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700088c604); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088c608); }
            c.w64(9, c.r64(8));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088c611); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088c615); };
            c.w64(11, t111);
            blk = 86; continue;
        }
        86 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700088c619); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088c61b); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700088c61d); }
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088c61f); };
            if !c.push64(t112) { return RegfileExit::fault(0x700088c61f); }
            c.w64(9, c.r64(0));
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088c625); };
            c.w64(7, t113);
            c.w32(0, 0x3u64);
            return c.call(0x70007f2274, 0x700088c633, 0x700088c62e);
        }
        87 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088c637); };
            c.w64(7, t114);
            let Some(t115) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c63b); };
            c.w32(8, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088c63f); };
            c.w64(9, t116);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_ae() { blk = 155; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700088c657); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r117 = c.r64(11) & c.r64(7);
            c.w64(11, r117);
            c.logic(r117, 64);
            let Some(t118) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088c669); };
            c.logic(t118 & 0x4u64, 8);
            if c.cc_ne() { blk = 258; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(8, c.r64(9));
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088c677); };
            c.w64(9, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700088c67b); };
            c.w64(11, t120);
            blk = 102; continue;
        }
        92 => {
            let Some(t121) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c684); };
            c.w32(8, t121);
            let Some(t122) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088c688); };
            c.cmp(t122, c.r32(8), 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088c691); };
            c.w64(7, t123);
            let Some(t124) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c695); };
            c.w32(9, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088c699); };
            c.w64(8, t125);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_ae() { blk = 156; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x700088c6b1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r126 = c.r64(11) & c.r64(7);
            c.w64(11, r126);
            c.logic(r126, 64);
            let Some(t127) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088c6c3); };
            c.logic(t127 & 0x4u64, 8);
            if c.cc_ne() { blk = 255; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088c6ce); };
            c.w64(9, t128);
            blk = 101; continue;
        }
        97 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088c6d4); };
            c.w64(7, t129);
            let Some(t130) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c6d8); };
            c.w32(9, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088c6dc); };
            c.w64(8, t131);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_ae() { blk = 157; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x700088c6f4); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r132 = c.r64(11) & c.r64(7);
            c.w64(11, r132);
            c.logic(r132, 64);
            let Some(t133) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088c706); };
            c.logic(t133 & 0x4u64, 8);
            if c.cc_ne() { blk = 252; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 101; continue;
        }
        101 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 102; continue;
        }
        102 => {
            c.w64(12, c.r64(9));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088c71a); };
            c.w64(9, t134);
            c.w64(1, c.r64(11));
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088c721); };
            c.w64(6, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088c725); };
            c.w64(2, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088c729); };
            c.w64(11, t137);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088c72f); };
            c.w64(0, t138);
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088c736); };
            c.w64(15, t139);
            blk = 62; continue;
        }
        103 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088c742); };
            c.w64(8, t140);
            blk = 104; continue;
        }
        104 => {
            c.w32(9, 0x1u64);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088c74c); };
            c.w64(12, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700088c750); };
            c.w64(1, t142);
            blk = 62; continue;
        }
        105 => {
            let Some(t143) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700088c759); };
            c.cmp(t143, 0x0u64, 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 111; continue;
        }
        107 => {
            let Some(t144) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088c764); };
            c.cmp(t144, 0x0u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w32(8, 0x2u64);
            c.w64(9, c.r64(8));
            blk = 110; continue;
        }
        109 => {
            c.w32(9, 0x4u64);
            blk = 110; continue;
        }
        110 => {
            c.w64(8, c.r64(9));
            blk = 111; continue;
        }
        111 => {
            let Some(t145) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c77e); };
            c.w32(9, t145);
            let Some(t146) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088c782); };
            c.cmp(t146, 0x0u64, 32);
            if c.cc_ne() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(11, c.r64(2));
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_l() { blk = 115; continue; }
            blk = 113; continue;
        }
        113 => {
            blk = 120; continue;
        }
        114 => {
            c.w64(11, c.r64(2));
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w32(8, 0x3u64);
            blk = 120; continue;
        }
        118 => {
            c.w32(8, 0x5u64);
            c.w64(12, c.r64(8));
            blk = 120; continue;
        }
        119 => {
            c.w32(8, 0x1u64);
            blk = 120; continue;
        }
        120 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t147) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c7bd); };
            c.w32(15, t147);
            let r148 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r148);
            let r149 = c.add(c.r64(12), 0x69u64, 64);
            c.w64(12, r149);
            let Some(t150) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088c7c8); };
            c.w32(15, t150);
            let r151 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r151);
            let Some(t152) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c7cf); };
            c.w32(0, t152);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(12), c.r64(0), 64);
            if c.cc_b() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            return RegfileExit::fault(0x700088c7de);
        }
        122 => {
            let Some(t153) = c.ld32(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c7df); };
            c.w32(12, t153);
            let r154 = c.shr(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r154);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x700088c7e7); }
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c7f1); };
            c.w64(8, t155);
            let Some(t156) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088c7f8); };
            c.w64(11, t156);
            c.w64(15, c.r64(11).wrapping_add(0x10));
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088c7ff); };
            c.w64(0, t157);
            let Some(t158) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700088c806); };
            c.cmp(t158, c.r64(15), 64);
            if c.cc_be() { blk = 223; continue; }
            blk = 124; continue;
        }
        124 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x700088c80f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 125; continue;
        }
        125 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700088c816); }
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088c81a); };
            c.w64(11, t159);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700088c821); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700088c825); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700088c829); }
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088c82d); };
            c.w64(15, t160);
            c.w64(9, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088c83b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088c83c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088c842); }
            return c.ret(0x0, 0x700088c844);
        }
        126 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088c84e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700088c852); }
            c.cmp(c.r64(9), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 231; continue; }
            blk = 127; continue;
        }
        127 => {
            c.w64(15, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 225; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c878); };
            c.w64(0, t161);
            let Some(t162) = c.ld64(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088c87f); };
            c.w64(0, t162);
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c883); };
            c.w64(2, t163);
            let Some(t164) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700088c88a); };
            c.w64(2, t164);
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 131; continue; }
            blk = 129; continue;
        }
        129 => {
            let r165 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r165);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700088c899); }
            c.w32(1, 0x4u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700088c8a0); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x2u64);
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700088c8a9); };
            c.w64(3, t166);
            return c.call(0x700086ed74, 0x700088c8b5, 0x700088c8b0);
        }
        130 => {
            blk = 133; continue;
        }
        131 => {
            c.w64(1, c.r64(15).wrapping_add(c.r64(2)));
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_be() { blk = 227; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c8c4); };
            c.w64(15, t167);
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x700088c8cb); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 133; continue;
        }
        133 => {
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088c8d5); };
            c.w64(8, t168);
            let r169 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r169);
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700088c8dc); };
            c.w64(3, t170);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700088c8e3); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700088c8e6); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088c8ea); };
            c.w64(8, t171);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088c8f1); }
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088c8f7); };
            c.w64(8, t172);
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r173 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r173);
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(0)), 0) { return RegfileExit::fault(0x700088c90d); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 134; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088c919); };
            c.w64(7, t174);
            let Some(t175) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c91d); };
            c.w32(8, t175);
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088c921); };
            c.w64(9, t176);
            let Some(t177) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088c928); };
            c.w64(11, t177);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088c92f); };
            c.w64(15, t178);
            let Some(t179) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700088c936); };
            c.cmp(t179, c.r64(12), 64);
            if c.cc_be() { blk = 229; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700088c93f); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 137; continue;
        }
        137 => {
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088c946); };
            c.w64(11, t180);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700088c94a); }
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088c94e); };
            c.w64(11, t181);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700088c955); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700088c959); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700088c95d); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 139; continue;
        }
        138 => {
            c.w32(8, c.r64(3).wrapping_add(0x2));
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 139; continue;
        }
        139 => {
            let Some(t182) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088c96c); };
            c.cmp(c.r32(3), t182, 32);
            if c.cc_ge() { blk = 148; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t183) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c975); };
            c.w32(8, t183);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_ae() { blk = 149; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w64(8, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t184) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088c985); };
            c.w32(11, t184);
            let r185 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r185);
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t186) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088c99f); };
            c.w32(15, t186);
            let Some(t187) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088c9a3); };
            c.cmp(t187, c.r32(15), 32);
            if c.cc_ne() { blk = 138; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t188) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088c9ac); };
            c.w32(15, t188);
            c.cmp(c.r32(3), c.r32(15), 32);
            if c.cc_ae() { blk = 150; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700088c9bb); }
            blk = 147; continue;
        }
        146 => {
            let r189 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r189);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 147; continue;
        }
        147 => {
            if let Some(sr) = c.shl(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(0).wrapping_add(c.r64(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700088c9dd); }
            blk = 138; continue;
        }
        148 => {
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700088c9e9); }
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088c9ed); };
            c.w64(2, t190);
            c.w64(8, c.r64(2).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088c9fb); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088c9fc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088ca02); }
            return c.ret(0x0, 0x700088ca04);
        }
        149 => {
            return RegfileExit::fault(0x700088ca05);
        }
        150 => {
            return RegfileExit::fault(0x700088ca06);
        }
        151 => {
            blk = 159; continue;
        }
        152 => {
            return RegfileExit::fault(0x700088ca09);
        }
        153 => {
            return RegfileExit::fault(0x700088ca0a);
        }
        154 => {
            return RegfileExit::fault(0x700088ca0b);
        }
        155 => {
            return RegfileExit::fault(0x700088ca0c);
        }
        156 => {
            return RegfileExit::fault(0x700088ca0d);
        }
        157 => {
            return RegfileExit::fault(0x700088ca0e);
        }
        158 => {
            return RegfileExit::fault(0x700088ca0f);
        }
        159 => {
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700088ca10); };
            c.w64(9, t191);
            let Some(t192) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700088ca17); };
            c.cmp(t192, 0x0u64, 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 164; continue;
        }
        161 => {
            let Some(t193) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088ca22); };
            c.cmp(t193, 0x0u64, 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            c.w32(11, 0x2u64);
            blk = 164; continue;
        }
        163 => {
            c.w32(11, 0x4u64);
            blk = 164; continue;
        }
        164 => {
            let Some(t194) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088ca36); };
            c.w32(12, t194);
            let Some(t195) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088ca3a); };
            c.cmp(t195, 0x0u64, 32);
            if c.cc_ne() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(8), c.r32(12), 32);
            if c.cc_ge() { blk = 171; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w32(11, 0x3u64);
            blk = 171; continue;
        }
        169 => {
            c.w32(11, 0x5u64);
            blk = 171; continue;
        }
        170 => {
            c.w32(11, 0x1u64);
            blk = 171; continue;
        }
        171 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t196) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ca6a); };
            c.w32(0, t196);
            let r197 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r197);
            let r198 = c.add(c.r64(15), 0x69u64, 64);
            c.w64(15, r198);
            let Some(t199) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088ca74); };
            c.w32(0, t199);
            let r200 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r200);
            let Some(t201) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088ca7a); };
            c.w32(1, t201);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(15), c.r64(1), 64);
            if c.cc_b() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            return RegfileExit::fault(0x700088ca88);
        }
        173 => {
            let Some(t202) = c.ld32(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088ca89); };
            c.w32(15, t202);
            let r203 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r203);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x700088ca91); }
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088ca9b); };
            c.w64(9, t204);
            let Some(t205) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088caa2); };
            c.w64(11, t205);
            c.w64(0, c.r64(11).wrapping_add(0x10));
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088caa9); };
            c.w64(3, t206);
            let Some(t207) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700088cab0); };
            c.cmp(t207, c.r64(0), 64);
            if c.cc_be() { blk = 234; continue; }
            blk = 175; continue;
        }
        175 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x700088cab9); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 176; continue;
        }
        176 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700088cac0); }
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088cac4); };
            c.w64(11, t208);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700088cacb); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700088cacf); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700088cad3); }
            blk = 200; continue;
        }
        177 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700088cae5); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700088cae9); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(12)) { return RegfileExit::fault(0x700088caed); }
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 242; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(0, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(0), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 236; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088cb12); };
            c.w64(1, t209);
            let Some(t210) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088cb19); };
            c.w64(1, t210);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700088cb1d); }
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088cb21); };
            c.w64(1, t211);
            let Some(t212) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700088cb28); };
            c.w64(1, t212);
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_be() { blk = 182; continue; }
            blk = 180; continue;
        }
        180 => {
            let r213 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r213);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088cb35); }
            c.w32(0, 0x4u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088cb3b); }
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(9));
            return c.call(0x700086ed74, 0x700088cb4c, 0x700088cb47);
        }
        181 => {
            blk = 184; continue;
        }
        182 => {
            c.w64(6, c.r64(0).wrapping_add(c.r64(1)));
            c.w32(2, 0x4u64);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088cb57); };
            c.w64(8, t214);
            c.cmp(c.r64(8), c.r64(6), 64);
            if c.cc_be() { blk = 238; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088cb64); };
            c.w64(8, t215);
            if !c.st64(c.r64(8), c.r64(6)) { return RegfileExit::fault(0x700088cb6b); }
            c.w64(8, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 184; continue;
        }
        184 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700088cb75); };
            c.w64(8, t216);
            let r217 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r217);
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700088cb7c); };
            c.w64(9, t218);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700088cb83); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700088cb87); }
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088cb8b); };
            c.w64(8, t219);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088cb92); }
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088cb98); };
            c.w64(8, t220);
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            let r221 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r221);
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(0)), 0) { return RegfileExit::fault(0x700088cbae); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 185; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088cbba); };
            c.w64(7, t222);
            let Some(t223) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088cbbe); };
            c.w32(8, t223);
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088cbc2); };
            c.w64(9, t224);
            let Some(t225) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088cbc9); };
            c.w64(11, t225);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088cbd0); };
            c.w64(15, t226);
            let Some(t227) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700088cbd7); };
            c.cmp(t227, c.r64(12), 64);
            if c.cc_be() { blk = 240; continue; }
            blk = 187; continue;
        }
        187 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700088cbe0); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 188; continue;
        }
        188 => {
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088cbe7); };
            c.w64(11, t228);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700088cbeb); }
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088cbef); };
            c.w64(11, t229);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700088cbf6); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700088cbfa); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700088cbfe); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 190; continue;
        }
        189 => {
            c.w32(8, c.r64(3).wrapping_add(0x2));
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 190; continue;
        }
        190 => {
            let Some(t230) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088cc0d); };
            c.cmp(c.r32(3), t230, 32);
            if c.cc_ge() { blk = 199; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t231) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088cc16); };
            c.w32(8, t231);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_ae() { blk = 201; continue; }
            blk = 192; continue;
        }
        192 => {
            c.w64(8, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t232) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088cc26); };
            c.w32(11, t232);
            let r233 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r233);
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t234) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088cc40); };
            c.w32(15, t234);
            let Some(t235) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088cc44); };
            c.cmp(t235, c.r32(15), 32);
            if c.cc_ne() { blk = 189; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t236) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088cc4d); };
            c.w32(15, t236);
            c.cmp(c.r32(3), c.r32(15), 32);
            if c.cc_ae() { blk = 202; continue; }
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700088cc5c); }
            blk = 198; continue;
        }
        197 => {
            let r237 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r237);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 198; continue;
        }
        198 => {
            if let Some(sr) = c.shl(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(0).wrapping_add(c.r64(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700088cc7e); }
            blk = 189; continue;
        }
        199 => {
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700088cc8a); }
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088cc8e); };
            c.w64(8, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088cc92); };
            c.w64(6, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088cc96); };
            c.w64(2, t240);
            blk = 200; continue;
        }
        200 => {
            c.w64(0, c.r64(9));
            blk = 206; continue;
        }
        201 => {
            return RegfileExit::fault(0x700088cc9f);
        }
        202 => {
            return RegfileExit::fault(0x700088cca0);
        }
        203 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700088cca1); }
            c.w32(0, 0x1u64);
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x1c80)) else { return RegfileExit::fault(0x700088cca8); };
            c.w64(3, t241);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700088ccb7, 0x700088ccb2);
        }
        204 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ccb7); };
            if !c.push64(t242) { return RegfileExit::fault(0x700088ccb7); }
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088ccba); };
            c.w64(7, t243);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088ccc1); }
            c.w64(7, c.r64(0));
            c.w64(2, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088cccd); };
            c.w64(6, t244);
            return c.call(0x70007f7214, 0x700088ccd6, 0x700088ccd1);
        }
        205 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ccd9); };
            c.w64(2, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ccdd); };
            c.w64(6, t246);
            blk = 206; continue;
        }
        206 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088cce1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700088cce3); }
            c.w64(2, c.r64(0));
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088cce7); };
            c.w64(0, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088cceb); };
            c.w64(3, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088ccef); };
            c.w64(1, t249);
            c.w64(7, c.r64(0));
            return c.call(0x700088b394, 0x700088ccfb, 0x700088ccf6);
        }
        207 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ccfb); };
            c.w64(8, t250);
            let r251 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r251);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088cd06); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088cd07); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088cd0d); }
            return c.ret(0x0, 0x700088cd0f);
        }
        208 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088cd16); }
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088cd18); };
            c.w64(8, t252);
            let Some(t253) = c.ld32(c.r64(8).wrapping_add(0x14b)) else { return RegfileExit::fault(0x700088cd1f); };
            c.w32(8, t253);
            let r254 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r254);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088cd29); }
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088cd2b); };
            c.w64(3, t255);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700088cd3c, 0x700088cd37);
        }
        209 => {
            return RegfileExit::fault(0x700088cd3c);
        }
        210 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088cd3d); };
            c.w64(3, t256);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700088cd4e, 0x700088cd49);
        }
        211 => {
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088cd4e); };
            c.w64(6, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088cd52); };
            c.w64(0, t258);
            blk = 1; continue;
        }
        212 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x700088cd63, 0x700088cd5e);
        }
        213 => {
            c.w64(15, c.r64(0));
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088cd66); };
            c.w64(6, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088cd6a); };
            c.w64(8, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088cd6e); };
            c.w64(11, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088cd72); };
            c.w64(12, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088cd76); };
            c.w64(9, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088cd7a); };
            c.w64(0, t264);
            blk = 5; continue;
        }
        214 => {
            let Some(t265) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088cd83); };
            c.w32(3, t265);
            let r266 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r266);
            blk = 24; continue;
        }
        215 => {
            let Some(t267) = c.ld32(c.r64(3).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700088cd8f); };
            c.cmp(t267, c.r32(1), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700088cd9b); };
            c.w64(3, t268);
            c.w64(0, c.r64(15));
            return c.call(0x7000870914, 0x700088cdaa, 0x700088cda5);
        }
        217 => {
            c.w64(3, c.r64(0));
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088cdad); };
            c.w64(6, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088cdb1); };
            c.w64(8, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088cdb5); };
            c.w64(15, t271);
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088cdb9); };
            c.w64(11, t272);
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088cdbd); };
            c.w64(12, t273);
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088cdc1); };
            c.w64(9, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088cdc5); };
            c.w64(0, t275);
            blk = 24; continue;
        }
        218 => {
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088cdce); };
            c.w64(1, t276);
            blk = 248; continue;
        }
        219 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x700088cdd7); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x700088cddb); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x700088cde7, 0x700088cde2);
        }
        220 => {
            c.w64(9, c.r64(0));
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088cdea); };
            c.w64(8, t277);
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088cdee); };
            c.w64(2, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088cdf2); };
            c.w64(11, t279);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700088cdf8); };
            c.w64(12, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088cdff); };
            c.w64(15, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088ce03); };
            c.w64(7, t282);
            blk = 57; continue;
        }
        221 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 0) { return RegfileExit::fault(0x700088ce11); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700088ce1e, 0x700088ce19);
        }
        222 => {
            c.w64(8, c.r64(0));
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ce21); };
            c.w64(6, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ce25); };
            c.w64(2, t284);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088ce2b); };
            c.w64(7, t285);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x700088ce2f); }
            let Some(t286) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088ce34); };
            c.w64(0, t286);
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088ce3b); };
            c.w64(15, t287);
            blk = 85; continue;
        }
        223 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700088ce4c); }
            return c.call(0x70007febb4, 0x700088ce55, 0x700088ce50);
        }
        224 => {
            c.w64(8, c.r64(0));
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088ce58); };
            c.w64(7, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088ce5c); };
            c.w64(12, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088ce60); };
            c.w64(9, t290);
            blk = 125; continue;
        }
        225 => {
            c.w64(6, c.r64(3));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t291) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700088ce6e); };
            c.w64(3, t291);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700088ce7d, 0x700088ce78);
        }
        226 => {
            return RegfileExit::fault(0x700088ce7d);
        }
        227 => {
            let r292 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r292);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700088ce81); }
            c.w32(15, 0x4u64);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700088ce89); }
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(3));
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700088ce93); };
            c.w64(3, t293);
            return c.call(0x700086ed74, 0x700088ce9f, 0x700088ce9a);
        }
        228 => {
            blk = 133; continue;
        }
        229 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700088cea4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088cea8); }
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700088ceb6, 0x700088ceb1);
        }
        230 => {
            c.w64(9, c.r64(0));
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088ceb9); };
            c.w64(7, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088cebd); };
            c.w64(0, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088cec1); };
            c.w64(8, t296);
            blk = 137; continue;
        }
        231 => {
            c.w64(6, c.r64(3));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700088cecf); };
            c.w64(3, t297);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700088cede, 0x700088ced9);
        }
        232 => {
            return RegfileExit::fault(0x700088cede);
        }
        233 => {
            blk = 159; continue;
        }
        234 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700088cee4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700088cee8); }
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700088cef6, 0x700088cef1);
        }
        235 => {
            c.w64(9, c.r64(0));
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088cef9); };
            c.w64(6, t298);
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088cefd); };
            c.w64(2, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088cf01); };
            c.w64(8, t300);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088cf05); };
            c.w64(7, t301);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088cf09); };
            c.w64(15, t302);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088cf0d); };
            c.w64(12, t303);
            blk = 176; continue;
        }
        236 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(3));
            let Some(t304) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700088cf1b); };
            c.w64(3, t304);
            return c.call(0x700086ed74, 0x700088cf27, 0x700088cf22);
        }
        237 => {
            return RegfileExit::fault(0x700088cf27);
        }
        238 => {
            c.w32(8, c.r64(0).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088cf2c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700088cf2e); }
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(9));
            return c.call(0x700086ed74, 0x700088cf3f, 0x700088cf3a);
        }
        239 => {
            blk = 184; continue;
        }
        240 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700088cf44); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x700088cf48); }
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700088cf56, 0x700088cf51);
        }
        241 => {
            c.w64(9, c.r64(0));
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088cf59); };
            c.w64(0, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088cf5d); };
            c.w64(7, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088cf61); };
            c.w64(8, t307);
            blk = 188; continue;
        }
        242 => {
            c.w64(6, c.r64(3));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t308) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700088cf6f); };
            c.w64(3, t308);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700088cf7e, 0x700088cf79);
        }
        243 => {
            return RegfileExit::fault(0x700088cf7e);
        }
        244 => {
            c.w64(6, c.r64(3));
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700088cf82); };
            c.w64(3, t309);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700088cf93, 0x700088cf8e);
        }
        245 => {
            return RegfileExit::fault(0x700088cf93);
        }
        246 => {
            blk = 248; continue;
        }
        247 => {
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088cf96); };
            c.w64(7, t310);
            blk = 248; continue;
        }
        248 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 250; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088cfa2); };
            c.w64(8, t311);
            let Some(t312) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088cfa6); };
            c.w64(8, t312);
            c.w64(7, c.r64(8));
            blk = 250; continue;
        }
        250 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088cfb3); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088cfb5); }
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088cfb6); };
            c.w64(3, t313);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700088cfc7, 0x700088cfc2);
        }
        251 => {
            return RegfileExit::fault(0x700088cfc7);
        }
        252 => {
            c.w32(0, c.r32(0));
            let r314 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r314);
            c.w64(11, 0xfffffffffffc0000u64);
            let r315 = c.r64(11) & c.r64(0);
            c.w64(11, r315);
            c.logic(r315, 64);
            let Some(t316) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088cfd7); };
            c.logic(t316 & 0x2u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 253; continue;
        }
        253 => {
            c.w64(12, c.r64(7).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088cfe6); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700088cff0, 0x700088cfeb);
        }
        254 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700088cff0); }
            blk = 100; continue;
        }
        255 => {
            c.w32(0, c.r32(0));
            let r317 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r317);
            c.w64(11, 0xfffffffffffc0000u64);
            let r318 = c.r64(11) & c.r64(0);
            c.w64(11, r318);
            c.logic(r318, 64);
            let Some(t319) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088d006); };
            c.logic(t319 & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w64(12, c.r64(7).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088d015); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700088d01f, 0x700088d01a);
        }
        257 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700088d01f); }
            blk = 96; continue;
        }
        258 => {
            c.w32(0, c.r32(0));
            let r320 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r320);
            c.w64(11, 0xfffffffffffc0000u64);
            let r321 = c.r64(11) & c.r64(0);
            c.w64(11, r321);
            c.logic(r321, 64);
            let Some(t322) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088d035); };
            c.logic(t322 & 0x2u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 259; continue;
        }
        259 => {
            c.w64(12, c.r64(7).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088d044); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700088d04e, 0x700088d049);
        }
        260 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700088d04e); }
            blk = 91; continue;
        }
        261 => {
            return RegfileExit::jump(0x700088d058);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088d074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088d074); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088d078); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088d079); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088d07a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700088d07f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700088d083); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088d087); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088d09f); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700088d0a4); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x11c0)) else { return RegfileExit::fault(0x700088d0a8); };
            c.w64(11, t3);
            let Some(t4) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700088d0af); };
            c.logic(t4 & 0xffffffffffffffffu64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d0b5); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088d0bc); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088d0c3); };
            c.w32(11, t9);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2bu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x700088d0d3);
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0xb3)) else { return RegfileExit::fault(0x700088d0d4); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            blk = 5; continue;
        }
        5 => {
            c.w32(1, c.r64(0).wrapping_add(c.r64(0)));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x700088d0e1); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d0eb); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088d0f2); };
            c.logic(t14 & 0x40u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700088d0f9); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088d0fa); };
            c.w64(11, t15);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700088d101); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(8));
            c.w64(2, c.r64(8));
            c.w64(11, c.r64(0));
            return c.call(0x70007f7214, 0x700088d116, 0x700088d111);
        }
        8 => {
            c.w64(8, c.r64(0));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088d119); };
            c.w64(6, t16);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088d11f); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088d123); };
            c.w64(1, t18);
            blk = 27; continue;
        }
        9 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700088d131); }
            c.w64(10, 0x40cffe0000000000u64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 58; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 13; continue;
        }
        12 => {
            c.cmp(c.r32(1), 0x7ff8u64, 32);
            if c.cc_ge() { blk = 58; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088d15d); };
            c.w32(8, t19);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x69u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x700088d16d);
        }
        15 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t20) = c.ld32(c.r64(6).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x700088d171); };
            c.w32(11, t20);
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088d184); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700088d18b); };
            c.w64(3, t22);
            c.w64(2, c.r64(8).wrapping_mul(2).wrapping_add(0x18));
            c.w64(7, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088d19a); };
            c.w64(9, t23);
            let Some(t24) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088d1a1); };
            c.cmp(t24, c.r64(7), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x700088d1aa); }
            c.w64(9, c.r64(3).wrapping_add(0x1));
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700088d1b1); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x700088d1b7); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088d1bb); };
            c.w64(2, t25);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700088d1c2); }
            c.w64(11, c.r64(9).wrapping_add(0x10));
            let r26 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r26);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700088d1cd); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700088d1d1); };
            c.w64(15, t27);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700088d1d8); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700088d1dc); }
            c.w64(12, c.r64(9));
            c.w64(2, c.r64(11));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 22; continue;
        }
        19 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088d1eb); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700088d1f2); };
            c.w64(15, t29);
            c.w64(2, c.r64(15).wrapping_add(0x10));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088d1fa); };
            c.w64(3, t30);
            let Some(t31) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700088d201); };
            c.cmp(t31, c.r64(2), 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x700088d20a); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 21; continue;
        }
        21 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700088d212); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700088d21a); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088d21f); };
            c.w64(2, t32);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700088d226); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700088d22b); }
            c.w64(3, c.r64(15));
            c.w32(9, 0x1u64);
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088d24d); };
            c.w64(9, t33);
            blk = 25; continue;
        }
        25 => {
            let r34 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r34);
            if !c.st32(c.r64(8).wrapping_add(c.r64(2)), c.r32(9)) { return RegfileExit::fault(0x700088d258); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 25; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(8, c.r64(12));
            blk = 27; continue;
        }
        27 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700088d268); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700088d26c); }
            c.w64(11, c.r64(3));
            blk = 30; continue;
        }
        28 => {
            c.w32(0, c.r64(11).wrapping_add(0x2));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700088d27c); }
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(11));
            c.w64(1, c.r64(15));
            return c.call(0x7000899bb4, 0x700088d28e, 0x700088d289);
        }
        29 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088d28e); };
            c.w64(11, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088d292); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088d296); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088d29a); };
            c.w64(6, t38);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 30; continue;
        }
        30 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088d2a0); };
            c.w64(12, t39);
            let Some(t40) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088d2a4); };
            c.cmp(c.r32(11), t40, 32);
            if c.cc_ge() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(9), c.r64(0), 64);
            if c.cc_be() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t41) = c.ld64(c.r64(12).wrapping_add(c.r64(15).wrapping_mul(4))) else { return RegfileExit::fault(0x700088d2b8); };
            c.w64(15, t41);
            blk = 28; continue;
        }
        33 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088d2be); };
            c.w64(15, t42);
            blk = 28; continue;
        }
        34 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(0, c.r64(8));
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d2d4); };
            c.w32(8, t43);
            let r44 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r44);
            let Some(t45) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088d2db); };
            c.cmp(t45, 0x834u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(11, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t46) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088d2f5); };
            c.w32(11, t46);
            let r47 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r47);
            let r48 = c.r32(11) & 0x3fu64;
            c.w32(11, r48);
            c.logic(r48, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_a() { blk = 46; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d308); };
            c.w32(11, t49);
            let r50 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088d30f); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088d316); };
            c.w32(12, t53);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4cu64, 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x700088d326);
        }
        42 => {
            let Some(t54) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x700088d327); };
            c.w32(11, t54);
            let Some(t55) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088d32e); };
            c.cmp(t55, c.r32(11), 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088d334); };
            c.w64(8, t56);
            c.w64(11, c.r64(3));
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088d33e); };
            c.cmp(t57, c.r32(11), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w32(8, 0x1u64);
            blk = 47; continue;
        }
        45 => {
            c.w64(0, c.r64(8));
            blk = 46; continue;
        }
        46 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t58) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088d358); };
            c.w32(8, t58);
            let Some(t59) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088d35c); };
            c.cmp(c.r32(8), t59, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700088d362); };
            c.w64(3, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088d369); };
            c.w64(1, t61);
            return c.call(0x7000871cb4, 0x700088d372, 0x700088d36d);
        }
        50 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088d372); };
            c.w64(0, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088d376); };
            c.w64(9, t63);
            blk = 51; continue;
        }
        51 => {
            c.w64(8, c.r64(9));
            let r64 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088d384); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088d385); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088d38b); }
            return c.ret(0x0, 0x700088d38d);
        }
        52 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088d390); };
            c.w64(3, t65);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700088d39f, 0x700088d39a);
        }
        53 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088d39f); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088d3a3); };
            c.w64(0, t67);
            blk = 1; continue;
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700088d3ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700088d3b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700088d3b4); }
            return c.call(0x70007febf4, 0x700088d3bd, 0x700088d3b8);
        }
        55 => {
            c.w64(9, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088d3c0); };
            c.w64(6, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088d3c4); };
            c.w64(0, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088d3c8); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088d3cc); };
            c.w64(11, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088d3d0); };
            c.w64(12, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088d3d4); };
            c.w64(1, t73);
            blk = 18; continue;
        }
        56 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700088d3e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700088d3e6); }
            return c.call(0x70007febf4, 0x700088d3ef, 0x700088d3ea);
        }
        57 => {
            c.w64(12, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088d3f2); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088d3f6); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088d3fa); };
            c.w64(9, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088d3fe); };
            c.w64(0, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088d402); };
            c.w64(8, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088d406); };
            c.w64(11, t79);
            blk = 21; continue;
        }
        58 => {
            let Some(t80) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d40f); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088d416); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            let Some(t84) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088d41d); };
            c.w32(11, t84);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2bu64, 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x700088d42d);
        }
        60 => {
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0xb3)) else { return RegfileExit::fault(0x700088d42e); };
            c.w32(8, t85);
            let r86 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r86);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700088d438); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088d439); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088d43b); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088d43d); };
            c.w64(8, t87);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088d444); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x700088d446); };
            c.w64(3, t88);
            c.w32(0, 0x4u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700088d45a, 0x700088d455);
        }
        61 => {
            c.w64(12, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088d45d); };
            c.w64(6, t89);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088d463); };
            c.w64(0, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088d467); };
            c.w64(1, t91);
            blk = 26; continue;
        }
        62 => {
            return RegfileExit::jump(0x700088d474);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088d494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088d494); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088d498); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088d499); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088d49a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088d49f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088d4a9); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d4b6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088d4bd); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 18; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088d4d6); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d4e3); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088d4ea); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088d4f5); };
            c.w64(8, t11);
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d512); };
            c.w32(12, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088d516); };
            c.cmp(t13, c.r32(12), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(0, 0x1u64);
            c.w64(12, c.r64(0));
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 10; continue;
        }
        10 => {
            let r14 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r14);
            if c.cc_o() { blk = 21; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        12 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088d535); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088d537); };
            c.w64(1, t15);
            c.w64(0, c.r64(2));
            c.w64(7, c.r64(9));
            return c.call(0x700088d834, 0x700088d549, 0x700088d544);
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088d549); };
            c.w64(1, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088d550); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return c.ret(0x18, 0x700088d557);
        }
        15 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088d55a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088d561); }
            return c.ret(0x0, 0x700088d563);
        }
        16 => {
            return RegfileExit::fault(0x700088d564);
        }
        17 => {
            return RegfileExit::fault(0x700088d565);
        }
        18 => {
            return RegfileExit::fault(0x700088d566);
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088d567); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088d56d); };
            c.w64(3, t17);
            return c.call(0x700086ed74, 0x700088d579, 0x700088d574);
        }
        20 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088d579); };
            c.w64(6, t18);
            blk = 1; continue;
        }
        21 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088d582); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088d58c); }
            blk = 24; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r19 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r19);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700088d5c3); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            if c.cc_ne() { blk = 30; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(11, c.r64(9));
            let r20 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r20);
            if c.cc_o() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 35; continue;
        }
        30 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088d5f5); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088d5fc); };
            c.w64(11, t22);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088d603); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700088d60a); };
            c.cmp(t24, c.r64(12), 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700088d61c, 0x700088d617);
        }
        32 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088d61c); };
            c.w64(6, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088d620); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088d624); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088d628); };
            c.w64(8, t28);
            blk = 34; continue;
        }
        33 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700088d62e); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 34; continue;
        }
        34 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088d638); };
            c.w64(9, t29);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700088d63f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700088d643); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088d648); }
            blk = 35; continue;
        }
        35 => {
            c.w64(9, c.r64(0));
            blk = 12; continue;
        }
        36 => {
            return RegfileExit::jump(0x700088d658);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088d674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088d674); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088d678); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088d679); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088d67a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088d67f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088d689); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d696); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088d69d); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088d6b6); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d6bf); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088d6c6); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088d6cd); };
            c.w64(7, t11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d6d7); };
            c.w32(8, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088d6db); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700088d6e4); };
            c.w64(8, t14);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d6ee); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088d6f2); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088d6fb); }
            c.w64(0, c.r64(2));
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088d700); };
            c.w64(1, t17);
            return c.call(0x700088d834, 0x700088d709, 0x700088d704);
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088d709); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088d710); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x700088d717);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088d71a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088d721); }
            return c.ret(0x0, 0x700088d723);
        }
        13 => {
            return RegfileExit::fault(0x700088d724);
        }
        14 => {
            return RegfileExit::fault(0x700088d725);
        }
        15 => {
            return RegfileExit::fault(0x700088d726);
        }
        16 => {
            return RegfileExit::fault(0x700088d727);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088d728); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088d72c); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088d73a, 0x700088d735);
        }
        18 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088d73a); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x700088d744);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088d754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088d754); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088d758); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088d759); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088d75a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088d75f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088d769); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d776); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088d77d); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088d796); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d79f); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088d7a6); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088d7ad); };
            c.w64(7, t11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d7b7); };
            c.w32(8, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088d7bb); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700088d7c4); };
            c.w64(8, t14);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d7ce); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088d7d2); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088d7db); }
            c.w64(0, c.r64(2));
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088d7e0); };
            c.w64(1, t17);
            return c.call(0x700088d834, 0x700088d7e9, 0x700088d7e4);
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088d7e9); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088d7f0); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x700088d7f7);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088d7fa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088d801); }
            return c.ret(0x0, 0x700088d803);
        }
        13 => {
            return RegfileExit::fault(0x700088d804);
        }
        14 => {
            return RegfileExit::fault(0x700088d805);
        }
        15 => {
            return RegfileExit::fault(0x700088d806);
        }
        16 => {
            return RegfileExit::fault(0x700088d807);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088d808); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088d80c); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088d81a, 0x700088d815);
        }
        18 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088d81a); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x700088d824);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088d834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088d834); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700088d838); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700088d83e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700088d842); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700088d846); }
            blk = 2; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088d84c); };
            c.w64(2, t1);
            blk = 2; continue;
        }
        2 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700088d85f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700088d863); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700088d867); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088d871); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            blk = 249; continue;
        }
        5 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 249; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700088d88e); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 275; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d89b); };
            c.w32(15, t2);
            let r3 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r3);
            let Some(t4) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088d8a2); };
            c.w32(11, t4);
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_b() { blk = 275; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(11), 0xbcu64, 16);
            if c.cc_e() { blk = 269; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d8c9); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088d8d0); };
            c.w32(12, t7);
            c.cmp(c.r16(12), 0x42u64, 16);
            if c.cc_ne() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088d8dd); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(3) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(2, 0x1fffffffffffffu64);
            let r8 = c.add(c.r64(2), c.r64(3), 64);
            c.w64(2, r8);
            c.w64(1, 0x3ffffffffffffeu64);
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_be() { blk = 103; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088d918); };
            c.w64(2, t9);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_ae() { blk = 255; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088d92e); };
            c.w32(3, t10);
            c.logic(c.r32(3) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 18; continue;
        }
        18 => {
            c.logic(c.r8(3) & 0x2u64, 8);
            if c.cc_e() { blk = 275; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r16(12), 0x25u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r16(12), 0x2du64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 275; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(12, c.r64(7));
            blk = 26; continue;
        }
        23 => {
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088d95f); };
            c.w32(12, t11);
            let r12 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r12);
            blk = 26; continue;
        }
        24 => {
            if let Some(sr) = c.shr(c.r64(3), 0x2u64, 64) { c.w64(3, sr); }
            let r13 = c.r64(3) & 0xffffffu64;
            c.w64(3, r13);
            c.logic(r13, 64);
            blk = 105; continue;
        }
        25 => {
            c.w64(12, c.r64(7));
            blk = 26; continue;
        }
        26 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700088d97b); };
            c.w64(1, t14);
            c.w64(3, c.r64(2));
            blk = 28; continue;
        }
        27 => {
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088d987); };
            c.w32(15, t15);
            let r16 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r16);
            let Some(t17) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088d98e); };
            c.w32(11, t17);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(11), 0x410u64, 32);
            if c.cc_le() { blk = 57; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088d9a0); };
            c.w32(2, t18);
            c.logic(c.r32(2) & 0x200000u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t19) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x700088d9b0); };
            c.w32(3, t19);
            let r20 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r20);
            c.logic(c.r32(2) & 0xffc00u64, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 31; continue;
        }
        31 => {
            let r21 = c.shr(c.r32(2), 0xau64, 32).unwrap_or(c.r32(2));
            c.w32(2, r21);
            let r22 = c.r32(2) & 0x3ffu64;
            c.w32(2, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(2), 0x20u64, 32);
            if c.cc_be() { blk = 46; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t23) = c.ld16(c.r64(3).wrapping_add(0x5)) else { return RegfileExit::fault(0x700088d9d5); };
            c.w32(6, (t23 as u16 as i16 as i64 as u64));
            let Some(t24) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088d9d9); };
            c.w32(7, t24);
            let r25 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r25);
            let r26 = c.sub(c.r32(6), 0x1u64, 32);
            c.w32(6, r26);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700088d9e4); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088d9ec); }
            blk = 34; continue;
        }
        34 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088d9f0); };
            c.w64(9, t27);
            let r28 = c.sub(c.r32(9), c.r32(0), 32);
            c.w32(9, r28);
            let r29 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r29);
            let r30 = c.add(c.r32(9), c.r32(0), 32);
            c.w32(9, r30);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t31) = c.ld32(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088da04); };
            c.w32(8, t31);
            let r32 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r32);
            let r33 = c.shr(c.r32(8), 0x9u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r33);
            let r34 = c.r32(8) & 0x3ffu64;
            c.w32(8, r34);
            c.logic(r34, 32);
            c.w32(8, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t35) = c.ld32(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088da1e); };
            c.w32(8, t35);
            let r36 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r36);
            let Some(t37) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088da26); };
            c.w32(8, t37);
            let r38 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r38);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_be() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(0, c.r64(9).wrapping_add(0x1));
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088da37); };
            c.w64(9, t39);
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700088da40); }
            blk = 39; continue;
        }
        38 => {
            let r40 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r40);
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(6), c.r32(0), 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(8, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t41) = c.ld32(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088da54); };
            c.w32(8, t41);
            let r42 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r42);
            let r43 = c.shr(c.r32(8), 0x9u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r43);
            let r44 = c.r32(8) & 0x3ffu64;
            c.w32(8, r44);
            c.logic(r44, 32);
            c.w32(9, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t45) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088da6e); };
            c.w32(9, t45);
            let r46 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r46);
            let Some(t47) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088da76); };
            c.w32(15, t47);
            let r48 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r48);
            c.cmp(c.r32(7), c.r32(15), 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(2), c.r32(8), 32);
            if c.cc_a() { blk = 238; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088da91); };
            c.w64(15, t49);
            blk = 68; continue;
        }
        44 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088da9a); };
            c.w64(15, t50);
            blk = 68; continue;
        }
        45 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088daa3); };
            c.w64(15, t51);
            blk = 68; continue;
        }
        46 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            let r52 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(8, r52);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 47; continue;
        }
        47 => {
            let r53 = c.sub(c.r64(8), 0x3u64, 64);
            c.w64(8, r53);
            let Some(t54) = c.ld32(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088dac1); };
            c.cmp(t54, c.r32(12), 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ne() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 68; continue;
        }
        50 => {
            let Some(t55) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088dad7); };
            c.w32(8, t55);
            let r56 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r56);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(8, c.r64(1));
            blk = 52; continue;
        }
        52 => {
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088dae7); };
            c.w32(9, t57);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r58 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r58);
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088daf5); };
            c.w32(0, t59);
            let r60 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r60);
            let r61 = c.r64(0) & c.r64(9);
            c.w64(0, r61);
            c.logic(r61, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 54; continue;
        }
        53 => {
            let r62 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r62);
            let r63 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r63);
            c.w64(2, c.r64(9));
            let r64 = c.r64(2) & c.r64(0);
            c.w64(2, r64);
            c.logic(r64, 64);
            c.w64(0, c.r64(2));
            blk = 54; continue;
        }
        54 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088db19); };
            c.w32(2, t65);
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088db1e); };
            c.cmp(t66, c.r32(2), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 53; continue;
        }
        57 => {
            c.cmp(c.r32(11), 0xbdu64, 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t67) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088db43); };
            c.logic(t67 & 0x24u64, 8);
            if c.cc_ne() { blk = 268; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t68) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088db4e); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(8, c.r64(1));
            blk = 61; continue;
        }
        61 => {
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088db5e); };
            c.w32(9, t70);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r71 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r71);
            let Some(t72) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088db6c); };
            c.w32(0, t72);
            let r73 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r73);
            let r74 = c.r64(0) & c.r64(9);
            c.w64(0, r74);
            c.logic(r74, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 63; continue;
        }
        62 => {
            let r75 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r75);
            let r76 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r76);
            c.w64(2, c.r64(9));
            let r77 = c.r64(2) & c.r64(0);
            c.w64(2, r77);
            c.logic(r77, 64);
            c.w64(0, c.r64(2));
            blk = 63; continue;
        }
        63 => {
            c.w64(2, c.r64(0).wrapping_add(0x5));
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088db8f); };
            c.w32(6, t78);
            let r79 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r79);
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088db97); };
            c.cmp(t80, c.r32(6), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088dba0); };
            c.cmp(t81, c.r32(6), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t82) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088dba9); };
            c.cmp(t82, c.r32(12), 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t83) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088dbaf); };
            c.w32(8, t83);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_ae() { blk = 292; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t84) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088dbc2); };
            c.w32(8, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088dbc6); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_ne() { blk = 238; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(11), 0x82eu64, 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t86) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088dbe0); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            let Some(t88) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dbe8); };
            c.w32(8, t88);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 99; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t89) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dbf8); };
            c.w32(9, t89);
            let r90 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r90);
            c.w32(11, c.r32(9));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(9), 0x30u64, 32);
            if c.cc_g() { blk = 99; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w32(9, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(12));
            c.w64(8, c.r64(6));
            blk = 74; continue;
        }
        74 => {
            c.w64(0, c.r64(9));
            let r91 = c.r32(0) & 0x7u64;
            c.w32(0, r91);
            c.logic(r91, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 79; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 78; continue;
        }
        78 => {
            blk = 257; continue;
        }
        79 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            blk = 257; continue;
        }
        82 => {
            let Some(t92) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088dc5f); };
            c.w32(9, t92);
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700088dc63); };
            c.cmp(t93, c.r32(9), 32);
            if c.cc_ne() { blk = 258; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t94) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088dc70); };
            c.w32(11, t94);
            let r95 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r95);
            let Some(t96) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088dc77); };
            c.w32(9, t96);
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            let Some(t98) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dc7e); };
            c.w32(9, t98);
            blk = 74; continue;
        }
        84 => {
            let Some(t99) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088dc85); };
            c.w32(9, t99);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r100 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r100);
            let Some(t101) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088dc92); };
            c.w32(11, t101);
            let r102 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r102);
            let Some(t103) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088dc99); };
            c.w32(9, t103);
            let r104 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r104);
            let Some(t105) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dca0); };
            c.w32(9, t105);
            blk = 74; continue;
        }
        85 => {
            let Some(t106) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088dca7); };
            c.w32(11, t106);
            let r107 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r107);
            let Some(t108) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088dcae); };
            c.w32(9, t108);
            let r109 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r109);
            let Some(t110) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dcb5); };
            c.w32(9, t110);
            blk = 74; continue;
        }
        86 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 89; continue;
        }
        87 => {
            c.w32(0, 0x1u64);
            blk = 89; continue;
        }
        88 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(12));
            c.w64(8, c.r64(6));
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 259; continue; }
            blk = 90; continue;
        }
        90 => {
            let r111 = c.add(c.r64(11), 0xbu64, 64);
            c.w64(11, r111);
            blk = 91; continue;
        }
        91 => {
            let r112 = c.r32(9) & 0x8u64;
            c.w32(9, r112);
            c.logic(r112, 32);
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t113) = c.ld16(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x700088dceb); };
            c.w32(8, t113);
            blk = 94; continue;
        }
        93 => {
            let Some(t114) = c.ld8(c.r64(11).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700088dcf2); };
            c.w32(8, t114);
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 267; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 265; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 264; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088dd25); };
            c.w64(7, t115);
            let Some(t116) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088dd29); };
            c.w32(8, t116);
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700088dd2d); };
            c.cmp(t117, c.r32(8), 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t118) = c.ld8(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dd36); };
            c.logic(t118 & 0x1u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t119) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088dd40); };
            c.w32(3, t119);
            let r120 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r120);
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088dd47); };
            c.cmp(t121, c.r32(3), 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 27; continue;
        }
        103 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088dd59); };
            c.w64(2, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088dd5d); };
            c.w64(12, t123);
            blk = 105; continue;
        }
        104 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(3, c.r64(8));
            blk = 105; continue;
        }
        105 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700088dd6c); }
            c.w64(8, c.r64(2));
            blk = 107; continue;
        }
        106 => {
            let Some(t124) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088dd75); };
            c.w32(15, t124);
            let r125 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r125);
            let Some(t126) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dd7c); };
            c.w32(11, t126);
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(11), 0x410u64, 32);
            if c.cc_le() { blk = 275; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t127) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088dd8e); };
            c.w32(9, t127);
            let r128 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r128);
            let r129 = c.r32(9) & 0x3fu64;
            c.w32(9, r129);
            c.logic(r129, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 143; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 127; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 119; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 116; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(9), 0x28u64, 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 275; continue;
        }
        116 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 275; continue;
        }
        119 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_l() { blk = 124; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 123; continue;
        }
        123 => {
            blk = 275; continue;
        }
        124 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 126; continue;
        }
        126 => {
            blk = 275; continue;
        }
        127 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 136; continue; }
            blk = 128; continue;
        }
        128 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 133; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 132; continue;
        }
        132 => {
            blk = 275; continue;
        }
        133 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 135; continue;
        }
        135 => {
            blk = 275; continue;
        }
        136 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 140; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 139; continue;
        }
        139 => {
            blk = 275; continue;
        }
        140 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 142; continue;
        }
        142 => {
            blk = 275; continue;
        }
        143 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_l() { blk = 161; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_l() { blk = 153; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_l() { blk = 150; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 149; continue;
        }
        149 => {
            blk = 275; continue;
        }
        150 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 152; continue;
        }
        152 => {
            blk = 275; continue;
        }
        153 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_l() { blk = 158; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(9), 0xfu64, 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 157; continue;
        }
        157 => {
            blk = 275; continue;
        }
        158 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 160; continue;
        }
        160 => {
            blk = 275; continue;
        }
        161 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 170; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 167; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 166; continue;
        }
        166 => {
            blk = 275; continue;
        }
        167 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 168; continue;
        }
        168 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 169; continue;
        }
        169 => {
            blk = 275; continue;
        }
        170 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 174; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 173; continue;
        }
        173 => {
            blk = 275; continue;
        }
        174 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 176; continue;
        }
        176 => {
            blk = 275; continue;
        }
        177 => {
            blk = 182; continue;
        }
        178 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dfb7); };
            c.w32(8, t130);
            let r131 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r131);
            let Some(t132) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088dfbe); };
            c.w32(9, t132);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_ae() { blk = 183; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t133) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dfd7); };
            c.cmp(t133, c.r64(9), 64);
            if c.cc_e() { blk = 201; continue; }
            blk = 180; continue;
        }
        180 => {
            blk = 238; continue;
        }
        181 => {
            let Some(t134) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088dfe7); };
            c.w32(9, t134);
            let r135 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r135);
            let Some(t136) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dfee); };
            c.w32(9, t136);
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_b() { blk = 238; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088dffb); };
            c.w32(8, t137);
            let r138 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r138);
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088e002); };
            c.w32(9, t139);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_b() { blk = 185; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_b() { blk = 275; continue; }
            blk = 184; continue;
        }
        184 => {
            blk = 201; continue;
        }
        185 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e025); };
            c.w32(8, t140);
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088e02a); };
            c.cmp(t141, c.r32(8), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 186; continue;
        }
        186 => {
            blk = 238; continue;
        }
        187 => {
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e03c); };
            c.w32(9, t142);
            let r143 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r143);
            let Some(t144) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e043); };
            c.w32(9, t144);
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_b() { blk = 238; continue; }
            blk = 188; continue;
        }
        188 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700088e050); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_b() { blk = 275; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t145) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e063); };
            c.w32(8, t145);
            let r146 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r146);
            let Some(t147) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088e06a); };
            c.w32(9, t147);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r148 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r148);
            c.w32(11, c.r32(3));
            c.w64(10, c.r64(4));
            let r149 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r149);
            let r150 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r150);
            c.logic(r150, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700088e086); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700088e08b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088e08f); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700088e099); };
            c.w64(0, t151);
            c.w64(10, 0x700088e0a7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700088e0a7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700088e0ab); }
            return c.call(c.r64(0), 0x700088e0b1, 0x700088e0af);
        }
        190 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700088e0b1); }
            let Some(t152) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088e0b9); };
            c.w64(4, t152);
            c.w32(8, c.r32(0));
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088e0c1); };
            let r154 = c.r64(8) & t153;
            c.w64(8, r154);
            c.logic(r154, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088e0c8); };
            c.w64(11, t155);
            blk = 192; continue;
        }
        191 => {
            let r156 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r156);
            let r157 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088e0d5); };
            c.w64(12, t158);
            let r159 = c.r64(12) & c.r64(8);
            c.w64(12, r159);
            c.logic(r159, 64);
            c.w64(8, c.r64(12));
            blk = 192; continue;
        }
        192 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t160) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e0e4); };
            c.w32(12, t160);
            let r161 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r161);
            let Some(t162) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088e0ec); };
            c.cmp(t162, c.r32(12), 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 193; continue;
        }
        193 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 198; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t163) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088e0fb); };
            c.cmp(t163, c.r32(12), 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 195; continue;
        }
        195 => {
            c.xorpd_rr(0, 0);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088e108); };
            c.cvtsi2sd(0, (t164 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700088e10e); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 191; continue; }
            blk = 196; continue;
        }
        196 => {
            if c.cc_e() { blk = 238; continue; }
            blk = 197; continue;
        }
        197 => {
            blk = 191; continue;
        }
        198 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088e129); };
            c.cmp(c.r64(12), t165, 64);
            if c.cc_e() { blk = 238; continue; }
            blk = 199; continue;
        }
        199 => {
            blk = 191; continue;
        }
        200 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e135); };
            c.w64(7, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088e139); };
            c.w64(15, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088e13d); };
            c.w64(3, t168);
            blk = 201; continue;
        }
        201 => {
            let Some(t169) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088e141); };
            c.w32(8, t169);
            let r170 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088e148); };
            c.w64(2, t171);
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088e14c); };
            c.cmp(t172, c.r32(8), 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 202; continue;
        }
        202 => {
            blk = 106; continue;
        }
        203 => {
            let Some(t173) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e15e); };
            c.w32(9, t173);
            let r174 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r174);
            let Some(t175) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700088e165); };
            c.logic(t175 & 0x4u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t176) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700088e170); };
            c.cmp(t176, c.r64(3), 64);
            if c.cc_a() { blk = 238; continue; }
            blk = 205; continue;
        }
        205 => {
            blk = 239; continue;
        }
        206 => {
            let Some(t177) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e17f); };
            c.w32(9, t177);
            let r178 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r178);
            let Some(t179) = c.ld32(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700088e186); };
            c.w32(11, t179);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 235; continue; }
            blk = 207; continue;
        }
        207 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t180) = c.ld64(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e19e); };
            c.w64(9, t180);
            let Some(t181) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088e1a2); };
            c.w64(11, t181);
            let Some(t182) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700088e1a6); };
            c.logic(t182 & 0x1u64, 8);
            if c.cc_ne() { blk = 211; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t183) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700088e1ad); };
            c.w64(12, t183);
            let r184 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r184);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 239; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t185) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700088e1bd); };
            c.w64(8, t185);
            c.w64(0, c.r64(8));
            blk = 237; continue;
        }
        211 => {
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 239; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t186) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e1d2); };
            c.w32(8, t186);
            let r187 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r187);
            let Some(t188) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088e1d9); };
            c.w32(8, t188);
            let r189 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r189);
            let r190 = c.r32(8) & 0x3fu64;
            c.w32(8, r190);
            c.logic(r190, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 222; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 218; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 217; continue;
        }
        217 => {
            blk = 253; continue;
        }
        218 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 221; continue;
        }
        221 => {
            blk = 253; continue;
        }
        222 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 227; continue; }
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 231; continue; }
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 231; continue; }
            blk = 225; continue;
        }
        225 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 226; continue;
        }
        226 => {
            blk = 253; continue;
        }
        227 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            blk = 253; continue;
        }
        230 => {
            c.w32(8, 0x1u64);
            blk = 234; continue;
        }
        231 => {
            c.w32(8, 0x2u64);
            blk = 234; continue;
        }
        232 => {
            c.w32(8, 0x4u64);
            blk = 234; continue;
        }
        233 => {
            c.w32(8, 0x8u64);
            blk = 234; continue;
        }
        234 => {
            let r191 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r191);
            c.w64(0, c.r64(9));
            c.cqo();
            c.idiv(c.r64(8), 64);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088e283); };
            c.w64(9, t192);
            blk = 237; continue;
        }
        235 => {
            c.w64(10, c.r64(4));
            let r193 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r193);
            let r194 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r194);
            c.logic(r194, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700088e294); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700088e29f); };
            c.w64(0, t195);
            c.w64(10, 0x700088e2adu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700088e2ad); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700088e2b1); }
            return c.call(c.r64(0), 0x700088e2b7, 0x700088e2b5);
        }
        236 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700088e2b7); }
            let Some(t196) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088e2bf); };
            c.w64(4, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088e2c4); };
            c.w64(3, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e2c8); };
            c.w64(7, t198);
            blk = 237; continue;
        }
        237 => {
            c.cmp(c.r64(3), c.r64(0), 64);
            if c.cc_ae() { blk = 239; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088e2d1); };
            c.w64(8, t199);
            blk = 240; continue;
        }
        239 => {
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700088e2da); };
            c.w64(8, t200);
            blk = 240; continue;
        }
        240 => {
            let Some(t201) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088e2e1); };
            c.cmp(t201, c.r32(8), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088e2ea); };
            c.w64(1, t202);
            blk = 246; continue;
        }
        242 => {
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088e2f0); };
            c.w64(0, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e2f4); };
            c.w64(3, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088e2f8); };
            c.w64(6, t205);
            return c.call(0x7000870914, 0x700088e301, 0x700088e2fc);
        }
        243 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088e301); };
            c.w64(8, t206);
            let Some(t207) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088e305); };
            c.cmp(t207, c.r32(8), 32);
            if c.cc_e() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088e30e); };
            if !c.push64(t208) { return RegfileExit::fault(0x700088e30e); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e311); };
            if !c.push64(t209) { return RegfileExit::fault(0x700088e311); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088e314); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088e315); }
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088e317); };
            c.w64(8, t210);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088e31e); }
            c.w32(0, 0x4u64);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088e325); };
            c.w64(7, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088e329); };
            c.w64(6, t212);
            return c.call(0x70007f2134, 0x700088e332, 0x700088e32d);
        }
        245 => {
            c.w64(1, c.r64(0));
            blk = 246; continue;
        }
        246 => {
            let Some(t213) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088e335); };
            c.cmp(t213, 0x0u64, 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088e33f); };
            c.w64(8, t214);
            let r215 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r215);
            if c.cc_o() { blk = 277; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        249 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088e355); };
            c.w64(0, t216);
            let Some(t217) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088e359); };
            c.cmp(t217, c.r32(0), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088e365); }
            return c.ret(0x8, 0x700088e366);
        }
        251 => {
            c.w32(8, 0x140u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088e36f); }
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088e371); };
            c.w64(8, t218);
            let Some(t219) = c.ld32(c.r64(8).wrapping_add(0x14f)) else { return RegfileExit::fault(0x700088e378); };
            c.w32(8, t219);
            let r220 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r220);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088e382); }
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088e384); };
            c.w64(3, t221);
            c.w32(0, 0x2u64);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088e390); };
            c.w64(6, t222);
            return c.call(0x700086ed74, 0x700088e399, 0x700088e394);
        }
        252 => {
            return RegfileExit::fault(0x700088e399);
        }
        253 => {
            return RegfileExit::fault(0x700088e39a);
        }
        254 => {
            blk = 275; continue;
        }
        255 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_ne() { blk = 275; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t223) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e3ab); };
            c.w32(12, t223);
            let r224 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r224);
            blk = 26; continue;
        }
        257 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700088e3b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700088e3bb); }
            blk = 261; continue;
        }
        258 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700088e3c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700088e3c5); }
            blk = 261; continue;
        }
        259 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700088e3cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700088e3cf); }
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 261; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t225) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088e3d9); };
            c.w64(11, t225);
            blk = 91; continue;
        }
        261 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700088e3e2); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088e3e4); }
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700088e3e5); };
            c.w64(3, t226);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700088e3f6, 0x700088e3f1);
        }
        262 => {
            let r227 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r227);
            c.w64(8, c.r64(0));
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088e3fb); };
            c.w64(15, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088e3ff); };
            c.w64(12, t229);
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700088e403); };
            c.w64(1, t230);
            blk = 94; continue;
        }
        263 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e40f); };
            c.w64(7, t231);
            blk = 275; continue;
        }
        264 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e415); };
            c.w64(7, t232);
            blk = 275; continue;
        }
        265 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e41b); };
            c.w64(7, t233);
            blk = 275; continue;
        }
        266 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e421); };
            c.w64(7, t234);
            blk = 275; continue;
        }
        267 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e427); };
            c.w64(7, t235);
            blk = 275; continue;
        }
        268 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e42d); };
            c.w64(7, t236);
            blk = 275; continue;
        }
        269 => {
            c.w64(0, c.r64(7));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088e436); };
            c.w64(6, t237);
            return c.call(0x70008a1af4, 0x700088e43f, 0x700088e43a);
        }
        270 => {
            let Some(t238) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e43f); };
            c.w32(8, t238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700088e443); };
            c.cmp(t239, c.r32(8), 32);
            if c.cc_ne() { blk = 273; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t240) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e44c); };
            c.logic(t240 & 0x1u64, 8);
            if c.cc_e() { blk = 273; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e452); };
            c.w64(7, t241);
            blk = 239; continue;
        }
        273 => {
            c.w64(3, c.r64(0));
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088e45e); };
            c.w64(0, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088e462); };
            c.w64(6, t243);
            return c.call(0x70008bced4, 0x700088e46b, 0x700088e466);
        }
        274 => {
            c.w64(8, c.r64(0));
            blk = 240; continue;
        }
        275 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088e473); };
            if !c.push64(t244) { return RegfileExit::fault(0x700088e473); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088e476); }
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x700088e477); };
            c.w64(3, t245);
            c.w32(0, 0x2u64);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088e483); };
            c.w64(6, t246);
            return c.call(0x700086ed74, 0x700088e48c, 0x700088e487);
        }
        276 => {
            c.w64(8, c.r64(0));
            blk = 240; continue;
        }
        277 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700088e494); }
            let Some(t247) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088e498); };
            c.cmp(t247, 0x0u64, 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e49e); };
            c.w64(7, t248);
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088e4a2); }
            blk = 280; continue;
        }
        279 => {
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e4a9); };
            c.w64(7, t249);
            let r250 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r250);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 280; continue;
        }
        280 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700088e4d7); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 286; continue; }
            blk = 281; continue;
        }
        281 => {
            if c.cc_ne() { blk = 286; continue; }
            blk = 282; continue;
        }
        282 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 286; continue; }
            blk = 284; continue;
        }
        284 => {
            c.w64(9, c.r64(8));
            let r251 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r251);
            if c.cc_o() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 291; continue;
        }
        286 => {
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088e509); };
            c.w64(8, t252);
            let Some(t253) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088e510); };
            c.w64(9, t253);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088e517); };
            c.w64(12, t254);
            let Some(t255) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700088e51e); };
            c.cmp(t255, c.r64(11), 64);
            if c.cc_a() { blk = 289; continue; }
            blk = 287; continue;
        }
        287 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088e52e, 0x700088e529);
        }
        288 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088e52e); };
            c.w64(1, t256);
            blk = 290; continue;
        }
        289 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700088e534); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 290; continue;
        }
        290 => {
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088e53e); };
            c.w64(8, t257);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700088e545); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700088e549); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088e54e); }
            blk = 291; continue;
        }
        291 => {
            c.w64(7, c.r64(0));
            blk = 1; continue;
        }
        292 => {
            return RegfileExit::fault(0x700088e55b);
        }
        293 => {
            return RegfileExit::jump(0x700088e560);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088e574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088e574); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088e578); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088e579); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088e57a); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088e57f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700088e583); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088e587); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088e59f); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088e5a4); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088e5b3); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088e5c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088e5c6); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e5dd); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e5e4); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 108; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700088e5fe); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e608); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e60f); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e618); };
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
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e638); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088e63f); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088e646); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e64d); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700088e65d);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700088e65e); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e675); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088e67c); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088e684); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e68c); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x700088e699);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x700088e69a); };
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
            return RegfileExit::fault(0x700088e6ad);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088e6ae); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 111; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x700088e6c5);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e6c6); };
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
            return c.call(0x70008a17f4, 0x700088e6db, 0x700088e6d6);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e6de); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088e6e2); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088e6e6); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088e6ea); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088e6ee); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700088e712); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700088e716); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 136; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088e729); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088e732); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088e739); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700088e740); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e74d); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088e753); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_g() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088e763); };
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
            let Some(t43) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088e775); };
            c.w64(8, t43);
            blk = 40; continue;
        }
        39 => {
            c.w64(8, c.r64(2));
            blk = 40; continue;
        }
        40 => {
            c.w32(9, c.r32(15));
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700088e78b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700088e78f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700088e793); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e7a0); };
            c.w32(7, t44);
            let r45 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r45);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700088e7a7); }
            let Some(t46) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e7ab); };
            c.cmp(t46, 0x834u64, 16);
            if c.cc_ne() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), 0x1u64) { return RegfileExit::fault(0x700088e7b3); }
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), 0x0u64) { return RegfileExit::fault(0x700088e7bd); }
            blk = 44; continue;
        }
        44 => {
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088e7c5); };
            c.cmp(t47, 0x0u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t48) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088e7cf); };
            c.w32(8, t48);
            c.w64(3, c.r64(8));
            let r49 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r49);
            let r50 = c.r32(3) & 0x3fu64;
            c.w32(3, r50);
            c.logic(r50, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700088e7dd); }
            c.cmp(c.r32(3), 0xbu64, 32);
            if c.cc_g() { blk = 90; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t51) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e7ea); };
            c.w32(3, t51);
            let r52 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r52);
            let Some(t53) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088e7f0); };
            c.w32(3, t53);
            let r54 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088e7f6); };
            c.w32(9, t55);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            return RegfileExit::fault(0x700088e806);
        }
        48 => {
            let Some(t56) = c.ld32(c.r64(3).wrapping_add(0x137)) else { return RegfileExit::fault(0x700088e807); };
            c.w32(9, t56);
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088e80e); };
            c.cmp(t57, c.r32(9), 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088e818); };
            c.w64(9, t58);
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e81f); };
            c.cmp(t59, c.r32(12), 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(3, c.r64(15).wrapping_sub(0x2));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088e830); };
            c.w64(8, t60);
            blk = 52; continue;
        }
        51 => {
            c.w32(9, c.r64(3).wrapping_sub(0x2));
            c.w64(3, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088e840); };
            c.w64(9, t61);
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 80; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t62) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e850); };
            c.cmp(t62, c.r32(7), 32);
            if c.cc_ne() { blk = 102; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e85a); };
            c.cmp(t63, c.r32(12), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t64) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088e864); };
            c.w32(9, t64);
            let r65 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r65);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700088e87c); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700088e888); }
            blk = 61; continue;
        }
        58 => {
            c.w64(9, c.r64(3));
            let r66 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r66);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 61; continue;
        }
        59 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 60; continue;
        }
        60 => {
            let r67 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r67);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700088e8a9); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 61; continue;
        }
        61 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 102; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 64; continue;
        }
        63 => {
            c.cmp(c.r32(9), c.r32(3), 32);
            if c.cc_le() { blk = 102; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t68) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088e8cc); };
            c.w32(9, t68);
            let r69 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r69);
            let r70 = c.r32(9) & 0x3fu64;
            c.w32(9, r70);
            c.logic(r70, 32);
            let r71 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r71);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e8e2); };
            c.w32(9, t72);
            let r73 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r73);
            let Some(t74) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e8e9); };
            c.w32(12, t74);
            let r75 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r75);
            let Some(t76) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e8f0); };
            c.w32(12, t76);
            let r77 = c.sub(c.r32(12), 0x77u64, 32);
            c.w32(12, r77);
            c.cmp(c.r32(12), 0xcu64, 32);
            if c.cc_a() { blk = 86; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t78) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088e904); };
            c.w32(12, t78);
            c.cmp(c.r32(3), c.r32(12), 32);
            if c.cc_ae() { blk = 85; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e914); };
            c.w32(9, t79);
            let r80 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r80);
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088e91c); };
            c.cmp(t81, c.r32(9), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 68; continue;
        }
        68 => {
            blk = 75; continue;
        }
        69 => {
            let Some(t82) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e92b); };
            c.w32(9, t82);
            let r83 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r83);
            let Some(t84) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088e932); };
            c.w32(12, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700088e936); };
            c.cmp(t85, c.r32(12), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t86) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088e943); };
            c.w32(12, t86);
            c.cmp(c.r32(3), c.r32(12), 32);
            if c.cc_ae() { blk = 87; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x700088e953); }
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t87) = c.ld64(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088e964); };
            c.cmp(t87, c.r64(11), 64);
            if c.cc_e() { blk = 79; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088e96f); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088e976); };
            c.w64(11, t89);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088e97d); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700088e984); };
            c.cmp(t91, c.r64(12), 64);
            if c.cc_be() { blk = 114; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700088e98d); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 74; continue;
        }
        74 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088e994); };
            c.w64(11, t92);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700088e99b); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088e99f); }
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088e9a5); };
            c.w64(11, t93);
            blk = 75; continue;
        }
        75 => {
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088e9a9); };
            c.cmp(t94, c.r32(8), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700088e9b2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700088e9b4); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700088e9b5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088e9b7); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700088e9b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700088e9ba); }
            c.w32(0, 0x4u64);
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x700088e9ce, 0x700088e9c9);
        }
        77 => {
            c.w64(8, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088e9d1); };
            c.w64(6, t95);
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088e9d5); };
            c.w64(2, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088e9dc); };
            c.w64(0, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088e9e0); };
            c.w64(11, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088e9e4); };
            c.w64(1, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088e9e8); };
            c.w64(3, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088e9ec); };
            c.w64(7, t101);
            blk = 51; continue;
        }
        78 => {
            c.w64(8, c.r64(9));
            blk = 51; continue;
        }
        79 => {
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088e9fd); };
            c.w64(11, t102);
            blk = 51; continue;
        }
        80 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088ea06); };
            c.cmp(t103, c.r32(8), 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(9, c.r64(0));
            let r104 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r104);
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088ea1c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088ea1d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088ea23); }
            return c.ret(0x0, 0x700088ea25);
        }
        82 => {
            c.w32(8, 0x140u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088ea2c); }
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088ea2e); };
            c.w64(8, t105);
            let Some(t106) = c.ld32(c.r64(8).wrapping_add(0x14f)) else { return RegfileExit::fault(0x700088ea35); };
            c.w32(8, t106);
            let r107 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r107);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088ea3f); }
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088ea41); };
            c.w64(3, t108);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700088ea52, 0x700088ea4d);
        }
        83 => {
            return RegfileExit::fault(0x700088ea52);
        }
        84 => {
            blk = 102; continue;
        }
        85 => {
            return RegfileExit::fault(0x700088ea58);
        }
        86 => {
            return RegfileExit::fault(0x700088ea59);
        }
        87 => {
            return RegfileExit::fault(0x700088ea5a);
        }
        88 => {
            return RegfileExit::fault(0x700088ea5b);
        }
        89 => {
            blk = 102; continue;
        }
        90 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088ea61); };
            c.w64(9, t109);
            let r110 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r110);
            if c.cc_o() { blk = 116; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(3, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 92; continue;
        }
        92 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ea72); };
            c.w64(8, t111);
            blk = 102; continue;
        }
        93 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(12, c.r64(15));
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700088ea83); }
            c.w64(15, c.r64(12));
            blk = 96; continue;
        }
        95 => {
            c.w64(9, c.r64(15));
            let r112 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r112);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 96; continue;
        }
        96 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 131; continue; }
            blk = 97; continue;
        }
        97 => {
            if c.cc_ne() { blk = 131; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            c.movq_rx(12, 0);
            if let Some(sr) = c.shr(c.r64(12), 0x20u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 131; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(12, c.r64(9));
            let r113 = c.add(c.r32(12), c.r32(9), 32);
            c.w32(12, r113);
            if c.cc_o() { blk = 131; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(3, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 102; continue;
        }
        102 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088eaf6); };
            if !c.push64(t114) { return RegfileExit::fault(0x700088eaf6); }
            c.w64(0, c.r64(11));
            c.w64(7, c.r64(3));
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(2, c.r64(11));
            return c.call(0x700088d834, 0x700088eb0d, 0x700088eb08);
        }
        103 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088eb0d); };
            c.w64(8, t115);
            let r116 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r116);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088eb18); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088eb19); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088eb1f); }
            return c.ret(0x0, 0x700088eb21);
        }
        104 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088eb28); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088eb2a); };
            c.w64(8, t117);
            let Some(t118) = c.ld32(c.r64(8).wrapping_add(0x14f)) else { return RegfileExit::fault(0x700088eb31); };
            c.w32(8, t118);
            let r119 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r119);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088eb3b); }
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088eb3d); };
            c.w64(3, t120);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700088eb4e, 0x700088eb49);
        }
        105 => {
            return RegfileExit::fault(0x700088eb4e);
        }
        106 => {
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088eb4f); };
            c.w64(3, t121);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700088eb60, 0x700088eb5b);
        }
        107 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088eb60); };
            c.w64(6, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088eb64); };
            c.w64(0, t123);
            blk = 1; continue;
        }
        108 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x700088eb75, 0x700088eb70);
        }
        109 => {
            c.w64(11, c.r64(0));
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088eb78); };
            c.w64(6, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088eb7c); };
            c.w64(8, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088eb80); };
            c.w64(9, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088eb84); };
            c.w64(0, t127);
            blk = 4; continue;
        }
        110 => {
            let Some(t128) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088eb8d); };
            c.w32(12, t128);
            let r129 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r129);
            blk = 23; continue;
        }
        111 => {
            let Some(t130) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700088eb99); };
            c.cmp(t130, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700088eba7); };
            c.w64(3, t131);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x700088ebb6, 0x700088ebb1);
        }
        113 => {
            c.w64(12, c.r64(0));
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ebb9); };
            c.w64(6, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088ebbd); };
            c.w64(8, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ebc1); };
            c.w64(11, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ebc5); };
            c.w64(9, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088ebc9); };
            c.w64(0, t136);
            blk = 23; continue;
        }
        114 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700088ebd7); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700088ebdb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700088ebe0); }
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700088ebec, 0x700088ebe7);
        }
        115 => {
            c.w64(9, c.r64(0));
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ebef); };
            c.w64(6, t137);
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088ebf3); };
            c.w64(2, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088ebfa); };
            c.w64(0, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088ebfe); };
            c.w64(1, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088ec02); };
            c.w64(3, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ec06); };
            c.w64(8, t142);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700088ec0a); }
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088ec0f); };
            c.w64(7, t143);
            blk = 74; continue;
        }
        116 => {
            let Some(t144) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088ec18); };
            c.cmp(t144, 0x0u64, 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(12, c.r64(15));
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700088ec21); }
            c.w64(15, c.r64(12));
            blk = 119; continue;
        }
        118 => {
            c.w64(8, c.r64(15));
            let r145 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r145);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 119; continue;
        }
        119 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700088ec5c); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 125; continue; }
            blk = 120; continue;
        }
        120 => {
            if c.cc_ne() { blk = 125; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(9, c.r64(8));
            let r146 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r146);
            if c.cc_o() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            c.w64(3, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(3));
            blk = 130; continue;
        }
        125 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088ec8e); };
            c.w64(8, t147);
            let Some(t148) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088ec95); };
            c.w64(9, t148);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088ec9c); };
            c.w64(3, t149);
            let Some(t150) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700088eca3); };
            c.cmp(t150, c.r64(12), 64);
            if c.cc_a() { blk = 128; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088ecb2, 0x700088ecad);
        }
        127 => {
            blk = 129; continue;
        }
        128 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700088ecb4); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 129; continue;
        }
        129 => {
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088ecbe); };
            c.w64(8, t151);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700088ecc5); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700088ecc9); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088ecce); }
            c.w64(8, c.r64(0));
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ecd6); };
            c.w64(6, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088ecda); };
            c.w64(0, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ecde); };
            c.w64(11, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088ece2); };
            c.w64(1, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088ece6); };
            c.w64(15, t156);
            blk = 130; continue;
        }
        130 => {
            c.w64(3, c.r64(8));
            blk = 92; continue;
        }
        131 => {
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088ecf2); };
            c.w64(9, t157);
            let Some(t158) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088ecf9); };
            c.w64(12, t158);
            c.w64(3, c.r64(12).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700088ed01); }
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088ed06); };
            c.w64(2, t159);
            let Some(t160) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700088ed0d); };
            c.cmp(t160, c.r64(3), 64);
            if c.cc_a() { blk = 134; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088ed1c, 0x700088ed17);
        }
        133 => {
            c.w64(9, c.r64(0));
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ed1f); };
            c.w64(6, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088ed23); };
            c.w64(0, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ed27); };
            c.w64(11, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088ed2b); };
            c.w64(1, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088ed2f); };
            c.w64(15, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ed33); };
            c.w64(8, t166);
            blk = 135; continue;
        }
        134 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x700088ed39); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 135; continue;
        }
        135 => {
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088ed41); };
            c.w64(12, t167);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700088ed48); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700088ed4c); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088ed51); }
            c.w64(3, c.r64(9));
            blk = 102; continue;
        }
        136 => {
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088ed5f); };
            c.w64(2, t168);
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t169) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088ed6b); };
            c.w64(8, t169);
            c.w64(2, c.r64(8));
            blk = 139; continue;
        }
        139 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088ed78); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700088ed7a); }
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088ed7b); };
            c.w64(3, t170);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700088ed8c, 0x700088ed87);
        }
        140 => {
            return RegfileExit::fault(0x700088ed8c);
        }
        141 => {
            return RegfileExit::jump(0x700088ed90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088ed94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088ed94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088ed98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088ed99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088ed9a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088ed9f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088eda9); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088edb6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088edbd); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 13; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088edd2); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088eddb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088ede2); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088ede9); };
            c.w64(8, t11);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088edf3); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088edf7); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088ee00); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088ee02); };
            c.w64(1, t14);
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(2));
            return c.call(0x700088f014, 0x700088ee13, 0x700088ee0e);
        }
        8 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088ee13); };
            c.w64(1, t15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088ee1a); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return c.ret(0x18, 0x700088ee21);
        }
        10 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088ee24); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088ee2b); }
            return c.ret(0x0, 0x700088ee2d);
        }
        11 => {
            return RegfileExit::fault(0x700088ee2e);
        }
        12 => {
            return RegfileExit::fault(0x700088ee2f);
        }
        13 => {
            return RegfileExit::fault(0x700088ee30);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088ee31); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088ee35); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088ee43, 0x700088ee3e);
        }
        15 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ee43); };
            c.w64(6, t17);
            blk = 1; continue;
        }
        16 => {
            return RegfileExit::jump(0x700088ee50);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088ee54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088ee54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088ee58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088ee59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088ee5a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088ee5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088ee69); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ee76); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088ee7d); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088ee96); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ee9f); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088eea6); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088eead); };
            c.w64(7, t11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088eeb7); };
            c.w32(8, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088eebb); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700088eec4); };
            c.w64(8, t14);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088eece); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088eed2); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088eedb); }
            c.w64(0, c.r64(2));
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088eee0); };
            c.w64(1, t17);
            return c.call(0x700088f014, 0x700088eee9, 0x700088eee4);
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088eee9); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088eef0); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x700088eef7);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088eefa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088ef01); }
            return c.ret(0x0, 0x700088ef03);
        }
        13 => {
            return RegfileExit::fault(0x700088ef04);
        }
        14 => {
            return RegfileExit::fault(0x700088ef05);
        }
        15 => {
            return RegfileExit::fault(0x700088ef06);
        }
        16 => {
            return RegfileExit::fault(0x700088ef07);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088ef08); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088ef0c); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088ef1a, 0x700088ef15);
        }
        18 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ef1a); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x700088ef24);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088ef34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088ef34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088ef38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088ef39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088ef3a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088ef3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088ef49); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ef56); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088ef5d); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088ef76); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ef7f); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088ef86); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088ef8d); };
            c.w64(7, t11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ef97); };
            c.w32(8, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088ef9b); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700088efa4); };
            c.w64(8, t14);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088efae); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088efb2); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088efbb); }
            c.w64(0, c.r64(2));
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700088efc0); };
            c.w64(1, t17);
            return c.call(0x700088f014, 0x700088efc9, 0x700088efc4);
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088efc9); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088efd0); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x700088efd7);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088efda); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088efe1); }
            return c.ret(0x0, 0x700088efe3);
        }
        13 => {
            return RegfileExit::fault(0x700088efe4);
        }
        14 => {
            return RegfileExit::fault(0x700088efe5);
        }
        15 => {
            return RegfileExit::fault(0x700088efe6);
        }
        16 => {
            return RegfileExit::fault(0x700088efe7);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088efe8); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088efec); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700088effa, 0x700088eff5);
        }
        18 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088effa); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x700088f004);
        }
            _ => unreachable!(),
        }
    }
}
