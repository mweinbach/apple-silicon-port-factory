#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000847d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000847d14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000847d18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000847d19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000847d1a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000847d1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000847d23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000847d2d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847d39); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847d3f); };
            c.cmp(t5, 0x82fu64, 16);
            if c.cc_ne() { blk = 103; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000847d4b); };
            c.w64(7, t6);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847d55); };
            c.w32(8, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000847d59); };
            c.cmp(t8, c.r32(8), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000847d62); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(7));
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000847d78); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000847d7f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000847d83); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847d91); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847d98); };
            c.w32(11, t13);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 57; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000847da8); };
            c.cmp(t14, c.r32(9), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(9, c.r16(11));
            c.cmp(c.r32(9), 0xbau64, 32);
            if c.cc_ge() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(10, c.r64(4));
            let r15 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r15);
            let r16 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r16);
            c.logic(r16, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000847dd8); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x7000847de3); };
            c.w64(0, t17);
            c.w64(10, 0x7000847df1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000847df1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000847df5); }
            return c.call(c.r64(0), 0x7000847dfb, 0x7000847df9);
        }
        14 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000847dfb); }
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000847e03); };
            c.w64(4, t18);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847e0e); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000847e12); };
            c.w64(0, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847e16); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000847e1a); };
            c.w64(2, t22);
            blk = 23; continue;
        }
        15 => {
            c.w64(2, c.r64(7));
            let Some(t23) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847e23); };
            c.w32(7, t23);
            let r24 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r24);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t25) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847e2f); };
            c.w32(9, t25);
            let r26 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r26);
            let Some(t27) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847e36); };
            c.w32(9, t27);
            c.cmp(c.r16(9), 0xb0u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(9), 0x7bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847e4e); };
            c.w32(9, t28);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_be() { blk = 101; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000847e62); };
            c.w32(7, t29);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 23; continue;
        }
        21 => {
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847e6d); };
            c.w32(7, t30);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.shr(c.r64(7), 0xau64, 64) { c.w64(7, sr); }
            let r31 = c.r64(7) & 0xfffffu64;
            c.w64(7, r31);
            c.logic(r31, 64);
            blk = 23; continue;
        }
        22 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 23; continue;
        }
        23 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000847e89); };
            c.w32(9, t32);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r33 = c.r64(11) & c.r64(7);
            c.w64(11, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000847e9a); };
            c.w32(11, t34);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 25; continue;
        }
        24 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000847ea7); };
            c.w32(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let r36 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r36);
            let r37 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r37);
            c.w64(12, c.r64(11).wrapping_mul(4));
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000847ec7); };
            c.cmp(t38, c.r32(2), 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 24; continue;
        }
        28 => {
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(2));
            blk = 87; continue;
        }
        29 => {
            c.w64(10, c.r64(4));
            let r39 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r39);
            let r40 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r40);
            c.logic(r40, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000847eea); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x7000847ef5); };
            c.w64(0, t41);
            c.w64(10, 0x7000847f03u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000847f03); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000847f07); }
            return c.call(c.r64(0), 0x7000847f0d, 0x7000847f0b);
        }
        30 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000847f0d); }
            let Some(t42) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000847f15); };
            c.w64(4, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847f1a); };
            c.w64(8, t43);
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000847f1e); };
            c.w32(7, t44);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(7).wrapping_sub(0x1));
            let r45 = c.r64(11) & c.r64(9);
            c.w64(11, r45);
            c.logic(r45, 64);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000847f35); };
            c.w32(11, t46);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000847f40); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000847f44); }
            blk = 32; continue;
        }
        31 => {
            c.w64(9, c.r64(12));
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000847f4d); };
            c.w32(9, t47);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 39; continue; }
            blk = 33; continue;
        }
        33 => {
            let r48 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r48);
            let r49 = c.add(c.r64(11), c.r64(7), 64);
            c.w64(11, r49);
            c.w64(12, c.r64(11).wrapping_mul(4));
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000847f70); };
            c.w32(15, t50);
            let r51 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000847f78); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000847f7c); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t52) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847f86); };
            c.w32(0, t52);
            let r53 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r53);
            let Some(t54) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847f8d); };
            c.cmp(t54, 0x41u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000847f94); };
            if !c.push64(t55) { return RegfileExit::fault(0x7000847f94); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000847f97); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x7000847f99); };
            c.w64(3, t56);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000847fac, 0x7000847fa7);
        }
        36 => {
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000847fac); };
            c.cmp(t57, c.r32(0), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847fb5); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000847fb9); };
            c.w64(12, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000847fbd); };
            c.w64(7, t60);
            blk = 31; continue;
        }
        38 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000847fc3); };
            c.w64(11, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847fc7); };
            c.w64(8, t62);
            blk = 80; continue;
        }
        39 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000847fd0); };
            c.w64(9, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000847fd4); };
            c.w64(7, t64);
            blk = 87; continue;
        }
        40 => {
            c.w64(10, c.r64(4));
            let r65 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r65);
            let r66 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r66);
            c.logic(r66, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000847fe8); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x7000847ff3); };
            c.w64(0, t67);
            c.w64(10, 0x7000848001u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000848001); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000848005); }
            return c.call(c.r64(0), 0x700084800b, 0x7000848009);
        }
        41 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084800b); }
            let Some(t68) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000848013); };
            c.w64(4, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000848018); };
            c.w64(8, t69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084801c); };
            c.w32(7, t70);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000848023); };
            c.w64(2, t71);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000848027); }
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(7).wrapping_sub(0x1));
            let r72 = c.r64(11) & c.r64(9);
            c.w64(11, r72);
            c.logic(r72, 64);
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084803c); };
            c.w32(11, t73);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 43; continue;
        }
        42 => {
            let Some(t74) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000848049); };
            c.w32(11, t74);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            let r75 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r75);
            let r76 = c.add(c.r64(11), c.r64(7), 64);
            c.w64(11, r76);
            c.w64(12, c.r64(11).wrapping_mul(4));
            let Some(t77) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848069); };
            c.w32(15, t77);
            let r78 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r78);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t79) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848077); };
            c.w32(0, t79);
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084807b); };
            c.cmp(t80, c.r32(0), 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000848084); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            if c.cc_e() { blk = 80; continue; }
            blk = 48; continue;
        }
        48 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            if c.cc_e() { blk = 42; continue; }
            blk = 50; continue;
        }
        50 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 80; continue; }
            blk = 51; continue;
        }
        51 => {
            if c.cc_e() { blk = 42; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 80; continue;
        }
        53 => {
            let r81 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r81);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 42; continue; }
            blk = 54; continue;
        }
        54 => {
            if c.cc_e() { blk = 80; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 42; continue;
        }
        56 => {
            c.w64(7, c.r64(2));
            blk = 87; continue;
        }
        57 => {
            let Some(t82) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008480d1); };
            c.w32(9, t82);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            let r83 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r83);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 61; continue;
        }
        59 => {
            c.w64(10, c.r64(4));
            let r84 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r84);
            let r85 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r85);
            c.logic(r85, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008480ef); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x70008480fa); };
            c.w64(0, t86);
            c.w64(10, 0x7000848108u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000848108); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084810c); }
            return c.call(c.r64(0), 0x7000848112, 0x7000848110);
        }
        60 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000848112); }
            let Some(t87) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084811a); };
            c.w64(4, t87);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            let Some(t88) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848128); };
            c.w64(0, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084812c); };
            c.w64(6, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000848130); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000848134); };
            c.w64(8, t91);
            blk = 61; continue;
        }
        61 => {
            let Some(t92) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000848138); };
            c.w32(11, t92);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            let r93 = c.r64(12) & c.r64(9);
            c.w64(12, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848149); };
            c.w32(12, t94);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000848154); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000848158); }
            blk = 63; continue;
        }
        62 => {
            c.w64(9, c.r64(15));
            let Some(t95) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000848161); };
            c.w32(9, t95);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(9));
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r64(12), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 70; continue; }
            blk = 64; continue;
        }
        64 => {
            let r96 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r96);
            let r97 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r97);
            c.w64(15, c.r64(12).wrapping_mul(4));
            let Some(t98) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848184); };
            c.w32(3, t98);
            let r99 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r99);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700084818c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000848190); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t100) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848199); };
            c.w32(2, t100);
            let r101 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r101);
            let Some(t102) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084819f); };
            c.cmp(t102, 0x40u64, 16);
            if c.cc_ae() { blk = 62; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(2, c.r64(7));
            c.w64(0, c.r64(3));
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x70008481b3, 0x70008481ae);
        }
        67 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008481b3); };
            c.cmp(t103, c.r32(0), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008481bc); };
            c.w64(7, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008481c0); };
            c.w64(8, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008481c4); };
            c.w64(15, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008481c8); };
            c.w64(11, t107);
            blk = 62; continue;
        }
        69 => {
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008481ce); };
            c.w64(11, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008481d2); };
            c.w64(8, t109);
            blk = 80; continue;
        }
        70 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008481db); };
            c.w64(9, t110);
            blk = 87; continue;
        }
        71 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t111) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008481ea); };
            c.w32(11, t111);
            c.w32(12, c.r32(9));
            let r112 = c.shl(c.r32(9), 0xfu64, 32).unwrap_or(c.r32(9));
            c.w32(9, r112);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r113 = !c.r32(12);
            c.w32(12, r113);
            let r114 = c.add(c.r32(9), c.r32(12), 32);
            c.w32(9, r114);
            c.w64(12, c.r64(9));
            let r115 = c.shr(c.r32(12), 0xcu64, 32).unwrap_or(c.r32(12));
            c.w32(12, r115);
            let r116 = c.r32(12) ^ c.r32(9);
            c.w32(12, r116);
            c.logic(r116, 32);
            c.w32(9, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(4)));
            c.w64(12, c.r64(9));
            let r117 = c.shr(c.r32(12), 0x4u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r117);
            let r118 = c.r32(12) ^ c.r32(9);
            c.w32(12, r118);
            c.logic(r118, 32);
            let r119 = c.imul2(c.r32(12), 0x809u64, 32);
            c.w32(9, r119);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            c.w64(15, c.r64(9));
            let r120 = c.shr(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r120);
            let r121 = c.r32(15) ^ c.r32(9);
            c.w32(15, r121);
            c.logic(r121, 32);
            let r122 = c.r32(15) & 0x3fffffffu64;
            c.w32(15, r122);
            c.logic(r122, 32);
            c.w64(9, (c.r32(15) as u32 as i32 as i64 as u64));
            let r123 = c.r64(12) & c.r64(9);
            c.w64(12, r123);
            c.logic(r123, 64);
            let Some(t124) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084823b); };
            c.w32(12, t124);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 73; continue;
        }
        72 => {
            let Some(t125) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000848248); };
            c.w32(12, t125);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r64(12), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 87; continue; }
            blk = 74; continue;
        }
        74 => {
            let r126 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r126);
            let r127 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r127);
            c.w64(15, c.r64(12).wrapping_mul(4));
            let Some(t128) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084826c); };
            c.w32(3, t128);
            let r129 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r129);
            c.cmp(c.r32(3), c.r32(7), 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 75; continue;
        }
        75 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t130) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084827d); };
            c.w32(2, t130);
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000848280); };
            c.cmp(t131, c.r32(2), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(2, c.r64(7));
            let r132 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r132);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084828e); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 72; continue; }
            blk = 78; continue;
        }
        78 => {
            if c.cc_ne() { blk = 72; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(11, c.r64(12));
            blk = 80; continue;
        }
        80 => {
            let Some(t133) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008482a6); };
            c.w32(7, t133);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(11).wrapping_add(0x4));
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_ae() { blk = 102; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x17));
            let Some(t134) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008482c5); };
            c.w64(11, t134);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(11)) { return RegfileExit::fault(0x70008482c9); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r135 = c.r64(9) & c.r64(8);
            c.w64(9, r135);
            c.logic(r135, 64);
            let Some(t136) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008482dd); };
            c.logic(t136 & 0x4u64, 8);
            if c.cc_ne() { blk = 111; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008482e8); };
            c.w64(0, t137);
            blk = 84; continue;
        }
        84 => {
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008482ec); };
            c.w64(1, t138);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008482f3); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            return c.ret(0x18, 0x70008482fa);
        }
        86 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008482fd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000848304); }
            return c.ret(0x0, 0x7000848306);
        }
        87 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_g() { blk = 90; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(10, c.r64(4));
            let r139 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r139);
            let r140 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r140);
            c.logic(r140, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000848318); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x12d8)) else { return RegfileExit::fault(0x7000848323); };
            c.w64(0, t141);
            c.w64(10, 0x7000848331u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000848331); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000848335); }
            return c.call(c.r64(0), 0x700084833b, 0x7000848339);
        }
        89 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084833b); }
            let Some(t142) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000848343); };
            c.w64(4, t142);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            blk = 90; continue;
        }
        90 => {
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000848351); };
            c.w64(8, t143);
            let Some(t144) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848355); };
            c.w32(7, t144);
            let Some(t145) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000848359); };
            c.w32(11, t145);
            let Some(t146) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084835d); };
            c.w32(12, t146);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r147 = c.add(c.r64(7), c.r64(11), 64);
            c.w64(7, r147);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000848373); }
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_l() { blk = 93; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t148) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700084837c); };
            if !c.push64(t148) { return RegfileExit::fault(0x700084837c); }
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x1dd0)) else { return RegfileExit::fault(0x700084837f); };
            c.w64(3, t149);
            c.w32(0, 0x1u64);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084838b); };
            c.w64(6, t150);
            return c.call(0x700086ed74, 0x7000848394, 0x700084838f);
        }
        92 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848394); };
            c.w64(0, t151);
            let Some(t152) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000848398); };
            c.w32(8, t152);
            let r153 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r153);
            let Some(t154) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084839f); };
            c.w32(7, t154);
            let Some(t155) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008483a3); };
            c.w32(9, t155);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t156) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008483ad); };
            c.w32(11, t156);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r157 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r157);
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(9));
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008483c3); };
            c.w64(9, t158);
            blk = 94; continue;
        }
        93 => {
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 94; continue;
        }
        94 => {
            let r159 = c.imul2(c.r64(7), 0x3u64, 64);
            c.w64(11, r159);
            c.w64(15, c.r64(12).wrapping_sub(0x1));
            let r160 = c.r64(15) & c.r64(9);
            c.w64(15, r160);
            c.logic(r160, 64);
            c.w64(9, c.r64(12).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(9).wrapping_mul(4));
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008483e4); };
            c.w32(12, t161);
            c.w64(0, c.r64(9).wrapping_mul(4).wrapping_add(0x13));
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008483f1); };
            c.w64(3, t162);
            if !c.st32(c.r64(8).wrapping_add(c.r64(0)), c.r32(3)) { return RegfileExit::fault(0x70008483f5); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r163 = c.r64(2) & c.r64(8);
            c.w64(2, r163);
            c.logic(r163, 64);
            let Some(t164) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000848408); };
            c.logic(t164 & 0x4u64, 8);
            if c.cc_ne() { blk = 108; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x17));
            let r165 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084841c); };
            c.w64(0, t166);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000848420); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r167 = c.r64(3) & c.r64(8);
            c.w64(3, r167);
            c.logic(r167, 64);
            let Some(t168) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000848432); };
            c.logic(t168 & 0x4u64, 8);
            if c.cc_ne() { blk = 105; continue; }
            blk = 98; continue;
        }
        98 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x1b), c.r32(12)) { return RegfileExit::fault(0x7000848438); }
            if !c.st32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x700084843d); }
            let Some(t169) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848442); };
            c.w32(7, t169);
            let r170 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r170);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700084844c); }
            let Some(t171) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848450); };
            c.w64(0, t171);
            blk = 84; continue;
        }
        99 => {
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000848459); };
            c.w64(3, t172);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000848467, 0x7000848462);
        }
        100 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848467); };
            c.w64(6, t173);
            blk = 1; continue;
        }
        101 => {
            return RegfileExit::fault(0x7000848470);
        }
        102 => {
            return RegfileExit::fault(0x7000848471);
        }
        103 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848477); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000848478); };
            c.w64(7, t174);
            let Some(t175) = c.ld32(c.r64(7).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700084847f); };
            c.w32(7, t175);
            let r176 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r176);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848485); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000848486); }
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000848487); };
            c.w64(3, t177);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000848498, 0x7000848493);
        }
        104 => {
            return RegfileExit::fault(0x7000848498);
        }
        105 => {
            c.w32(0, c.r32(0));
            let r178 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r178);
            c.w64(3, 0xfffffffffffc0000u64);
            let r179 = c.r64(3) & c.r64(0);
            c.w64(3, r179);
            c.logic(r179, 64);
            let Some(t180) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008484a8); };
            c.logic(t180 & 0x2u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(2, c.r64(8).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008484b2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008484b3); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008484c0, 0x70008484bb);
        }
        107 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008484c0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008484c2); }
            blk = 98; continue;
        }
        108 => {
            c.w32(3, c.r32(3));
            let r181 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r181);
            c.w64(2, 0xfffffffffffc0000u64);
            let r182 = c.r64(2) & c.r64(3);
            c.w64(2, r182);
            c.logic(r182, 64);
            let Some(t183) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008484d7); };
            c.logic(t183 & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(1, c.r64(8).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008484e5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008484e6); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008484f3, 0x70008484ee);
        }
        110 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008484f3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008484f5); }
            blk = 96; continue;
        }
        111 => {
            c.w32(11, c.r32(11));
            let r184 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r184);
            c.w64(9, 0xfffffffffffc0000u64);
            let r185 = c.r64(9) & c.r64(11);
            c.w64(9, r185);
            c.logic(r185, 64);
            let Some(t186) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700084850b); };
            c.logic(t186 & 0x2u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084851a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084851b); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000848528, 0x7000848523);
        }
        113 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000848528); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700084852a); }
            blk = 83; continue;
        }
        114 => {
            return RegfileExit::jump(0x7000848534);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000848554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000848554); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000848558); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848559); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084855a); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700084855f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000848563); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700084856d); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 87; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084857b); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848582); };
            c.cmp(t5, 0x82fu64, 16);
            if c.cc_ne() { blk = 87; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084858f); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000848596); };
            c.w64(2, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700084859a); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008485a7); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008485ae); };
            c.w32(11, t11);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 49; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008485be); };
            c.cmp(t12, c.r32(9), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(9, c.r16(11));
            c.cmp(c.r32(9), 0xbau64, 32);
            if c.cc_ge() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(10, c.r64(4));
            let r13 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            let r14 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r14);
            c.logic(r14, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008485ee); }
            c.w64(1, c.r64(13));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x70008485f6); };
            c.w64(0, t15);
            c.w64(10, 0x7000848604u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000848604); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000848608); }
            return c.call(c.r64(0), 0x700084860e, 0x700084860c);
        }
        9 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084860e); }
            let Some(t16) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000848616); };
            c.w64(4, t16);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848624); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848628); };
            c.w64(6, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084862c); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000848630); };
            c.w64(2, t20);
            blk = 18; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000848636); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848643); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            let Some(t25) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084864a); };
            c.w32(11, t25);
            c.cmp(c.r16(11), 0xb0u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(11), 0x7bu64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 18; continue;
        }
        14 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000848663); };
            c.w32(11, t26);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 83; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000848677); };
            c.w32(9, t27);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 18; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000848683); };
            c.w32(9, t28);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r29 = c.r64(9) & 0xfffffu64;
            c.w64(9, r29);
            c.logic(r29, 64);
            blk = 18; continue;
        }
        17 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 18; continue;
        }
        18 => {
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008486a0); };
            c.w32(11, t30);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            let r31 = c.r64(12) & c.r64(9);
            c.w64(12, r31);
            c.logic(r31, 64);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008486b1); };
            c.w32(9, t32);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 20; continue;
        }
        19 => {
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008486be); };
            c.w32(9, t33);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 80; continue; }
            blk = 21; continue;
        }
        21 => {
            let r34 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r34);
            let r35 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r35);
            c.w64(12, c.r64(9).wrapping_mul(4));
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008486e2); };
            c.cmp(t36, c.r32(2), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 19; continue;
        }
        23 => {
            c.w64(10, c.r64(4));
            let r37 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r37);
            let r38 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r38);
            c.logic(r38, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008486fa); }
            c.w64(1, c.r64(13));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x7000848702); };
            c.w64(0, t39);
            c.w64(10, 0x7000848710u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000848710); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000848714); }
            return c.call(c.r64(0), 0x700084871a, 0x7000848718);
        }
        24 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084871a); }
            let Some(t40) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000848722); };
            c.w64(4, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000848727); };
            c.w64(7, t41);
            let Some(t42) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084872b); };
            c.w32(8, t42);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r43 = c.r64(11) & c.r64(9);
            c.w64(11, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848742); };
            c.w32(9, t44);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700084874d); }
            blk = 26; continue;
        }
        25 => {
            c.w64(9, c.r64(11));
            let Some(t45) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000848756); };
            c.w32(9, t45);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 80; continue; }
            blk = 27; continue;
        }
        27 => {
            let r46 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r46);
            let r47 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r47);
            c.w64(11, c.r64(9).wrapping_mul(4));
            let Some(t48) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084877a); };
            c.w32(12, t48);
            let r49 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000848782); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000848786); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t50) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848790); };
            c.w32(15, t50);
            let r51 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r51);
            let Some(t52) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848798); };
            c.cmp(t52, 0x41u64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008487a0); };
            if !c.push64(t53) { return RegfileExit::fault(0x70008487a0); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008487a3); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008487a5); };
            c.w64(3, t54);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008487b8, 0x70008487b3);
        }
        30 => {
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008487b8); };
            c.cmp(t55, c.r32(0), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008487c1); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008487c5); };
            c.w64(11, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008487c9); };
            c.w64(8, t58);
            blk = 25; continue;
        }
        32 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008487cf); };
            c.w64(9, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008487d3); };
            c.w64(8, t60);
            blk = 70; continue;
        }
        33 => {
            c.w64(10, c.r64(4));
            let r61 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r61);
            let r62 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r62);
            c.logic(r62, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008487e7); }
            c.w64(1, c.r64(13));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x70008487ef); };
            c.w64(0, t63);
            c.w64(10, 0x70008487fdu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008487fd); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000848801); }
            return c.call(c.r64(0), 0x7000848807, 0x7000848805);
        }
        34 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000848807); }
            let Some(t64) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084880f); };
            c.w64(4, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000848814); };
            c.w64(7, t65);
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000848818); };
            c.w32(8, t66);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t67) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700084881f); };
            c.w64(2, t67);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000848823); }
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r68 = c.r64(11) & c.r64(9);
            c.w64(11, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848838); };
            c.w32(9, t69);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 36; continue;
        }
        35 => {
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000848845); };
            c.w32(9, t70);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 80; continue; }
            blk = 37; continue;
        }
        37 => {
            let r71 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r71);
            let r72 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r72);
            c.w64(11, c.r64(9).wrapping_mul(4));
            let Some(t73) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848869); };
            c.w32(12, t73);
            let r74 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r74);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t75) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848877); };
            c.w32(15, t75);
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084887c); };
            c.cmp(t76, c.r32(15), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000848885); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            if c.cc_e() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if c.cc_e() { blk = 35; continue; }
            blk = 43; continue;
        }
        43 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 48; continue; }
            blk = 44; continue;
        }
        44 => {
            if c.cc_e() { blk = 35; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 48; continue;
        }
        46 => {
            let r77 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r77);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 35; continue; }
            blk = 47; continue;
        }
        47 => {
            if c.cc_ne() { blk = 35; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(8, c.r64(7));
            blk = 70; continue;
        }
        49 => {
            let Some(t78) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008488c2); };
            c.w32(9, t78);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let r79 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r79);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 53; continue;
        }
        51 => {
            c.w64(10, c.r64(4));
            let r80 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r80);
            let r81 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r81);
            c.logic(r81, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008488e0); }
            c.w64(1, c.r64(13));
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x70008488e8); };
            c.w64(0, t82);
            c.w64(10, 0x70008488f6u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008488f6); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008488fa); }
            return c.call(c.r64(0), 0x7000848900, 0x70008488fe);
        }
        52 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000848900); }
            let Some(t83) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000848908); };
            c.w64(4, t83);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            let Some(t84) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848916); };
            c.w64(7, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084891a); };
            c.w64(6, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084891e); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000848922); };
            c.w64(2, t87);
            blk = 53; continue;
        }
        53 => {
            let Some(t88) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000848926); };
            c.w32(11, t88);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            let r89 = c.r64(12) & c.r64(9);
            c.w64(12, r89);
            c.logic(r89, 64);
            let Some(t90) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848937); };
            c.w32(9, t90);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000848942); }
            blk = 55; continue;
        }
        54 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            let Some(t91) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700084894e); };
            c.w32(8, t91);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 80; continue; }
            blk = 56; continue;
        }
        56 => {
            let r92 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r92);
            let r93 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r93);
            c.w64(12, c.r64(9).wrapping_mul(4));
            let Some(t94) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848978); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r95);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000848980); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000848984); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t96) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084898c); };
            c.w32(15, t96);
            let r97 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r97);
            let Some(t98) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848993); };
            c.cmp(t98, 0x40u64, 16);
            if c.cc_ae() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x70008489a2, 0x700084899d);
        }
        59 => {
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008489a2); };
            c.cmp(t99, c.r32(0), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008489ab); };
            c.w64(8, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008489af); };
            c.w64(2, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008489b3); };
            c.w64(12, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008489b7); };
            c.w64(11, t103);
            blk = 54; continue;
        }
        61 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008489bd); };
            c.w64(9, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008489c1); };
            c.w64(8, t105);
            blk = 70; continue;
        }
        62 => {
            c.w64(9, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t106) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008489d0); };
            c.w32(11, t106);
            c.w32(12, c.r32(9));
            let r107 = c.shl(c.r32(9), 0xfu64, 32).unwrap_or(c.r32(9));
            c.w32(9, r107);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r108 = !c.r32(12);
            c.w32(12, r108);
            let r109 = c.add(c.r32(9), c.r32(12), 32);
            c.w32(9, r109);
            c.w64(12, c.r64(9));
            let r110 = c.shr(c.r32(12), 0xcu64, 32).unwrap_or(c.r32(12));
            c.w32(12, r110);
            let r111 = c.r32(12) ^ c.r32(9);
            c.w32(12, r111);
            c.logic(r111, 32);
            c.w32(9, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(4)));
            c.w64(12, c.r64(9));
            let r112 = c.shr(c.r32(12), 0x4u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r112);
            let r113 = c.r32(12) ^ c.r32(9);
            c.w32(12, r113);
            c.logic(r113, 32);
            let r114 = c.imul2(c.r32(12), 0x809u64, 32);
            c.w32(9, r114);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            c.w64(15, c.r64(9));
            let r115 = c.shr(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r115);
            let r116 = c.r32(15) ^ c.r32(9);
            c.w32(15, r116);
            c.logic(r116, 32);
            let r117 = c.r32(15) & 0x3fffffffu64;
            c.w32(15, r117);
            c.logic(r117, 32);
            c.w64(9, (c.r32(15) as u32 as i32 as i64 as u64));
            let r118 = c.r64(12) & c.r64(9);
            c.w64(12, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848a21); };
            c.w32(9, t119);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(2));
            blk = 64; continue;
        }
        63 => {
            let Some(t120) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000848a31); };
            c.w32(9, t120);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 80; continue; }
            blk = 65; continue;
        }
        65 => {
            let r121 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r121);
            let r122 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r122);
            c.w64(15, c.r64(9).wrapping_mul(4));
            let Some(t123) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848a55); };
            c.w32(0, t123);
            let r124 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r124);
            c.cmp(c.r32(12), c.r32(0), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 63; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t125) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848a66); };
            c.w32(3, t125);
            let Some(t126) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000848a69); };
            c.cmp(t126, c.r32(3), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(3, c.r64(12));
            let r127 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r127);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000848a77); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 63; continue; }
            blk = 69; continue;
        }
        69 => {
            if c.cc_ne() { blk = 63; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t128) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000848a8c); };
            c.w32(7, t128);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(11, c.r64(9).wrapping_add(0x3));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 84; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(7, c.r64(9).wrapping_mul(4));
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000848aab); };
            c.w64(11, t129);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x7000848ab2); }
            let Some(t130) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000848ab7); };
            c.w32(12, t130);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r131 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r131);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 85; continue; }
            blk = 72; continue;
        }
        72 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x7000848ace); }
            let Some(t132) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848ad3); };
            c.w32(7, t132);
            let r133 = c.sub(c.r32(7), 0x2u64, 32);
            c.w32(7, r133);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000848add); }
            let Some(t134) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000848ae1); };
            c.w32(9, t134);
            let r135 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r135);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000848aec); }
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000848af0); };
            c.w32(9, t136);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000848b00); };
            c.w32(8, t137);
            let r138 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r138);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_l() { blk = 78; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000848b0b); };
            c.w64(0, t139);
            blk = 75; continue;
        }
        75 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000848b12); };
            c.w64(1, t140);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000848b19); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return c.ret(0x10, 0x7000848b20);
        }
        77 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000848b23); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000848b2a); }
            return c.ret(0x0, 0x7000848b2c);
        }
        78 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848b2d); };
            if !c.push64(t141) { return RegfileExit::fault(0x7000848b2d); }
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x1dd8)) else { return RegfileExit::fault(0x7000848b30); };
            c.w64(3, t142);
            c.w32(0, 0x1u64);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848b3c); };
            c.w64(6, t143);
            return c.call(0x700086ed74, 0x7000848b45, 0x7000848b40);
        }
        79 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000848b45); };
            c.w64(0, t144);
            blk = 75; continue;
        }
        80 => {
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000848b4e); };
            c.w64(0, t145);
            blk = 75; continue;
        }
        81 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000848b57); };
            c.w64(3, t146);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000848b65, 0x7000848b60);
        }
        82 => {
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848b65); };
            c.w64(6, t147);
            blk = 1; continue;
        }
        83 => {
            return RegfileExit::fault(0x7000848b6e);
        }
        84 => {
            return RegfileExit::fault(0x7000848b6f);
        }
        85 => {
            return RegfileExit::fault(0x7000848b70);
        }
        86 => {
            return RegfileExit::fault(0x7000848b71);
        }
        87 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000848b78); }
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000848b7a); };
            c.w64(8, t148);
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000848b81); };
            c.w32(8, t149);
            let r150 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r150);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000848b88); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848b8a); }
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000848b8b); };
            c.w64(3, t151);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000848b9c, 0x7000848b97);
        }
        88 => {
            return RegfileExit::fault(0x7000848b9c);
        }
        89 => {
            return RegfileExit::jump(0x7000848ba0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000848bb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000848bb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000848bb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848bb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000848bba); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000848bbf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848bc9); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848bd6); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848bdc); };
            c.cmp(t5, 0x82fu64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000848be8); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000848bee); }
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000848bf2); };
            c.w64(3, t8);
            return c.call(0x70008466f4, 0x7000848bfb, 0x7000848bf6);
        }
        4 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000848c00); };
            c.w64(0, t9);
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000848c07); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000848c0e); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x7000848c15);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000848c18); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000848c1f); }
            return c.ret(0x0, 0x7000848c21);
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000848c22); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000848c26); };
            c.w32(1, t12);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(7));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r13 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r13);
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000848c41); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            blk = 6; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000848c4a); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000848c4e); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000848c5c, 0x7000848c57);
        }
        12 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848c5c); };
            c.w64(6, t17);
            blk = 1; continue;
        }
        13 => {
            return RegfileExit::fault(0x7000848c65);
        }
        14 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000848c6b); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000848c6c); };
            c.w64(1, t18);
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x37)) else { return RegfileExit::fault(0x7000848c73); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000848c79); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000848c7a); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000848c7b); };
            c.w64(3, t21);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000848c8c, 0x7000848c87);
        }
        15 => {
            return RegfileExit::fault(0x7000848c8c);
        }
        16 => {
            return RegfileExit::jump(0x7000848c90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000848c94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000848c94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000848c98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848c99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000848c9a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000848c9f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848ca5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848cae); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848cb4); };
            c.cmp(t5, 0x82fu64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000848cbc); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000848cc2); };
            c.w64(3, t8);
            return c.call(0x70008466f4, 0x7000848ccb, 0x7000848cc6);
        }
        4 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000848cd0); };
            c.w64(0, t9);
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000848cd7); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000848cde); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x7000848ce5);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000848ce8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000848cef); }
            return c.ret(0x0, 0x7000848cf1);
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000848cf2); };
            c.w64(0, t11);
            blk = 6; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000848cfb); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000848cff); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000848d0d, 0x7000848d08);
        }
        11 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848d0d); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        12 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000848d18); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000848d19); };
            c.w64(1, t14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x3b)) else { return RegfileExit::fault(0x7000848d20); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000848d26); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000848d27); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000848d28); };
            c.w64(3, t17);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000848d39, 0x7000848d34);
        }
        13 => {
            return RegfileExit::fault(0x7000848d39);
        }
        14 => {
            return RegfileExit::jump(0x7000848d3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000848d54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a10)) else { return RegfileExit::fault(0x7000848d54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000848d74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000848d74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000848d78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848d79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000848d7a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000848d7f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848d89); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848d96); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848d9c); };
            c.cmp(t5, 0x82fu64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848da8); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000848dae); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000848db4); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000848dbb); };
            c.w32(1, t11);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x84u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x7000848dca);
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x217)) else { return RegfileExit::fault(0x7000848dcb); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000848dd1); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000848dd8); };
            c.w64(9, t14);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000848ddf); };
            c.w64(12, t15);
            let Some(t16) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000848de6); };
            c.cmp(t16, c.r64(11), 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000848dec); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000848df3); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000848df7); };
            c.w64(7, t17);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000848dfe); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000848e02); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000848e06); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x7000848e0c); }
            c.w64(0, c.r64(8));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000848e13); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000848e1a); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x7000848e21);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000848e24); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000848e2b); }
            return c.ret(0x0, 0x7000848e2d);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000848e2e); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000848e32); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000848e40, 0x7000848e3b);
        }
        11 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848e40); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        12 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000848e4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000848e52); }
            return c.call(0x70007febf4, 0x7000848e5b, 0x7000848e56);
        }
        13 => {
            c.w64(8, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848e5e); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000848e62); };
            c.w64(7, t22);
            blk = 7; continue;
        }
        14 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848e6d); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000848e6e); };
            c.w64(7, t23);
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x3f)) else { return RegfileExit::fault(0x7000848e75); };
            c.w32(7, t24);
            let r25 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r25);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848e7b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000848e7c); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000848e7d); };
            c.w64(3, t26);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000848e8e, 0x7000848e89);
        }
        15 => {
            return RegfileExit::fault(0x7000848e8e);
        }
        16 => {
            return RegfileExit::jump(0x7000848e90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000848e94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000848e94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000848e98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848e99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000848e9a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000848e9f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000848ea5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848eae); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848eb4); };
            c.cmp(t5, 0x82fu64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000848ebc); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848ec2); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000848ec8); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000848ecf); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x8, 0x7000848ed6);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000848ed9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000848ee0); }
            return c.ret(0x0, 0x7000848ee2);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000848ee3); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000848ee7); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000848ef5, 0x7000848ef0);
        }
        7 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848ef5); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        8 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000848f00); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000848f01); };
            c.w64(1, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000848f08); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000848f0e); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000848f0f); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000848f10); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000848f21, 0x7000848f1c);
        }
        9 => {
            return RegfileExit::fault(0x7000848f21);
        }
        10 => {
            return RegfileExit::jump(0x7000848f24);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000848f34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000848f34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000848f38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000848f39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000848f3a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000848f3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000848f43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 22; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000848f5b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000848f63); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000848f67); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000848f71); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000848f77); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000848f7e); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000848f88); };
            c.w64(1, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000848f8e); };
            c.w64(1, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000848f95); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848fa3); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000848faa); };
            c.cmp(t9, 0x82fu64, 16);
            if c.cc_ne() { blk = 30; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000848fc0); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000848fc7); };
            c.logic(t12 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000848fd2); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            c.w64(15, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(11));
            blk = 14; continue;
        }
        12 => {
            let Some(t15) = c.ld32(c.r64(15).wrapping_add(c.r64(3)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000848fe4); };
            c.w32(2, t15);
            let r16 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000848fec); };
            if !c.push64(t17) { return RegfileExit::fault(0x7000848fec); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000848fef); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000848ff0); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000848ff1); };
            if !c.push64(t18) { return RegfileExit::fault(0x7000848ff1); }
            c.w32(0, 0x3u64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000848ff9); };
            c.w64(7, t19);
            return c.call(0x70007f2274, 0x7000849002, 0x7000848ffd);
        }
        13 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000849002); };
            c.w64(11, t20);
            let r21 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700084900a); };
            c.w64(15, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084900e); };
            c.w64(9, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000849012); };
            c.w64(6, t24);
            blk = 14; continue;
        }
        14 => {
            let Some(t25) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849016); };
            c.w32(0, t25);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t26) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849022); };
            c.w32(0, t26);
            let Some(t27) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000849026); };
            c.w32(3, t27);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000849030); };
            c.w32(7, t28);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r29 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700084903d); }
            blk = 17; continue;
        }
        16 => {
            let r30 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r30);
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ge() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            let r31 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(3, r31);
            c.w64(2, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r32 = c.add(c.r64(2), c.r64(3), 64);
            c.w64(2, r32);
            c.w64(3, c.r64(2).wrapping_mul(4));
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(c.r64(3)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000849061); };
            c.w32(1, t33);
            let r34 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r34);
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000849069); };
            c.cmp(t35, c.r32(1), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t36) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000849072); };
            c.w32(0, t36);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r37 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r37);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000849080); }
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 12; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 27; continue;
        }
        21 => {
            c.w64(2, c.r64(9));
            let r38 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000849096); };
            c.w64(0, t39);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008490a0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008490a1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008490a7); }
            return c.ret(0x0, 0x70008490a9);
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008490aa); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008490ae); };
            c.w64(3, t40);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008490bf, 0x70008490ba);
        }
        23 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008490bf); };
            c.w64(6, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008490c3); };
            c.w64(0, t42);
            blk = 1; continue;
        }
        24 => {
            let Some(t43) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008490cc); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008490d3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(0, c.r64(15));
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fc694, 0x70008490ed, 0x70008490e8);
        }
        26 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w64(11, c.r64(2));
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008490f6); };
            c.w64(15, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008490fa); };
            c.w64(6, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008490fe); };
            c.w64(9, t47);
            blk = 24; continue;
        }
        27 => {
            return RegfileExit::fault(0x7000849104);
        }
        28 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000849105); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x21c8)) else { return RegfileExit::fault(0x7000849106); };
            c.w64(3, t48);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000849117, 0x7000849112);
        }
        29 => {
            return RegfileExit::fault(0x7000849117);
        }
        30 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084911d); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700084911e); };
            c.w64(2, t49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_add(0x47)) else { return RegfileExit::fault(0x7000849125); };
            c.w32(2, t50);
            let r51 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r51);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084912b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084912c); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700084912e); };
            c.w64(3, t52);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084913f, 0x700084913a);
        }
        31 => {
            return RegfileExit::fault(0x700084913f);
        }
        32 => {
            return RegfileExit::jump(0x7000849144);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000849154(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000849154); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000849158); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000849159); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084915a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700084915f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000849169); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849176); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084917c); };
            c.cmp(t5, 0x82fu64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849188); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084918e); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000849194); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084919b); };
            c.w32(1, t11);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x83u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008491aa);
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x213)) else { return RegfileExit::fault(0x70008491ab); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008491b1); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008491b8); };
            c.w64(9, t14);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008491bf); };
            c.w64(12, t15);
            let Some(t16) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008491c6); };
            c.cmp(t16, c.r64(11), 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008491cc); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008491d3); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008491d7); };
            c.w64(7, t17);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008491de); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008491e2); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008491e6); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008491ec); }
            c.w64(0, c.r64(8));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008491f3); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008491fa); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x7000849201);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000849204); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700084920b); }
            return c.ret(0x0, 0x700084920d);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700084920e); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000849212); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000849220, 0x700084921b);
        }
        11 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000849220); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        12 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700084922e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000849232); }
            return c.call(0x70007febf4, 0x700084923b, 0x7000849236);
        }
        13 => {
            c.w64(8, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084923e); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000849242); };
            c.w64(7, t22);
            blk = 7; continue;
        }
        14 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084924d); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700084924e); };
            c.w64(7, t23);
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x4b)) else { return RegfileExit::fault(0x7000849255); };
            c.w32(7, t24);
            let r25 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r25);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084925b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084925c); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700084925d); };
            c.w64(3, t26);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084926e, 0x7000849269);
        }
        15 => {
            return RegfileExit::fault(0x700084926e);
        }
        16 => {
            return RegfileExit::jump(0x7000849270);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000849274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000849274); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000849278); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000849279); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084927a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700084927f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000849289); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849296); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084929c); };
            c.cmp(t5, 0x82fu64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008492a8); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008492ae); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008492b4); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008492bb); };
            c.w32(1, t11);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x85u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008492ca);
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x21b)) else { return RegfileExit::fault(0x70008492cb); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008492d1); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008492d8); };
            c.w64(9, t14);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008492df); };
            c.w64(12, t15);
            let Some(t16) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008492e6); };
            c.cmp(t16, c.r64(11), 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008492ec); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008492f3); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008492f7); };
            c.w64(7, t17);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008492fe); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000849302); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000849306); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x700084930c); }
            c.w64(0, c.r64(8));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000849313); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084931a); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x7000849321);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000849324); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700084932b); }
            return c.ret(0x0, 0x700084932d);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700084932e); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000849332); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000849340, 0x700084933b);
        }
        11 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000849340); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        12 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700084934e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000849352); }
            return c.call(0x70007febf4, 0x700084935b, 0x7000849356);
        }
        13 => {
            c.w64(8, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084935e); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000849362); };
            c.w64(7, t22);
            blk = 7; continue;
        }
        14 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084936d); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700084936e); };
            c.w64(7, t23);
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x4f)) else { return RegfileExit::fault(0x7000849375); };
            c.w32(7, t24);
            let r25 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r25);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084937b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084937c); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700084937d); };
            c.w64(3, t26);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084938e, 0x7000849389);
        }
        15 => {
            return RegfileExit::fault(0x700084938e);
        }
        16 => {
            return RegfileExit::jump(0x7000849390);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000849394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000849394); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000849398); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000849399); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084939a); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700084939f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008493a9); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008493b6); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008493bc); };
            c.w32(7, t5);
            c.cmp(c.r16(7), 0x826u64, 16);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r16(7), 0x825u64, 16);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(7), 0x827u64, 16);
            if c.cc_ne() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008493d9); };
            c.w32(8, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008493dd); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008493e7); };
            c.w32(11, t9);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008493f8); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008493fc); };
            c.w32(12, t11);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000849406); };
            c.w32(15, t12);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r13 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r13);
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ge() { blk = 44; continue; }
            blk = 8; continue;
        }
        8 => {
            let r14 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(12, r14);
            c.w64(0, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r15 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r15);
            let r16 = c.add(c.r64(12), c.r64(0), 64);
            c.w64(12, r16);
            c.w64(0, c.r64(12).wrapping_mul(4));
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(c.r64(0)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000849435); };
            c.w32(3, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700084943a); };
            c.cmp(t18, c.r32(3), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 9; continue;
        }
        9 => {
            let r19 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r19);
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x7000849446); }
            c.cmp(c.r16(7), 0x825u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000849455); };
            c.w32(1, t20);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(8, c.r64(12).wrapping_add(0x4));
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 45; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(c.r64(0)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700084946d); };
            c.w32(1, t21);
            c.cmp(c.r16(7), 0x827u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084947d); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000849483); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700084948a); };
            c.w64(9, t25);
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084948d); };
            c.w32(7, t26);
            let r27 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r27);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000849497); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084949e); };
            c.cmp(t29, c.r64(11), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008494a8); }
            let r30 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r30);
            blk = 14; continue;
        }
        14 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008494af); };
            c.w64(11, t31);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008494b6); }
            c.w32(11, 0x4u64);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008494c0); }
            let Some(t32) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008494c4); };
            c.w32(15, t32);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008494d8); }
            let Some(t33) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008494dc); };
            c.w32(15, t33);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008494f0); }
            let Some(t34) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008494f4); };
            c.w32(1, t34);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x6bu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x7000849503);
        }
        18 => {
            let Some(t35) = c.ld32(c.r64(7).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000849504); };
            c.w32(1, t35);
            let Some(t36) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700084950a); };
            c.w64(15, t36);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            let Some(t37) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000849511); };
            c.cmp(t37, c.r64(0), 64);
            if c.cc_be() { blk = 50; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700084951b); }
            let r38 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r38);
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000849522); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000849526); };
            c.w64(0, t39);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700084952d); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000849531); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000849535); }
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000849539); };
            c.w32(1, t40);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x68u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000849548);
        }
        22 => {
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x7000849549); };
            c.w32(1, t41);
            let Some(t42) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700084954f); };
            c.w64(7, t42);
            c.w64(9, c.r64(7).wrapping_add(0x14));
            let Some(t43) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000849556); };
            c.cmp(t43, c.r64(9), 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(8), c.r64(9)) { return RegfileExit::fault(0x7000849560); }
            let r44 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r44);
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000849567); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700084956a); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700084956d); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x7000849570); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000849574); };
            c.w64(1, t45);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x700084957b); }
            c.w64(0, c.r64(7));
            blk = 25; continue;
        }
        25 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000849581); };
            c.w64(1, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000849588); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            return c.ret(0x8, 0x700084958f);
        }
        27 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000849592); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000849599); }
            return c.ret(0x0, 0x700084959b);
        }
        28 => {
            c.w64(3, c.r64(1));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700084959f); };
            c.w64(1, t47);
            c.w64(7, c.r64(1));
            blk = 30; continue;
        }
        29 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008495ab); };
            c.w64(1, t48);
            blk = 30; continue;
        }
        30 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008495b2); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008495b8); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008495be); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x68u64, 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x70008495ce);
        }
        32 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008495cf); };
            c.w32(7, t54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008495d5); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008495dc); };
            c.w64(9, t56);
            c.w64(12, c.r64(9).wrapping_add(0x14));
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008495e3); };
            c.w64(11, t57);
            let Some(t58) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008495ea); };
            c.cmp(t58, c.r64(12), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008495f3); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008495fa); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008495fe); };
            c.w64(9, t59);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000849605); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000849609); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x700084960d); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x7000849611); }
            c.w64(0, c.r64(8));
            blk = 25; continue;
        }
        35 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700084961d); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000849621); };
            c.w64(3, t60);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700084962f, 0x700084962a);
        }
        36 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084962f); };
            c.w64(6, t61);
            blk = 1; continue;
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000849638); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700084963c); }
            blk = 38; continue;
        }
        38 => {
            c.w32(3, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849644); };
            c.w32(12, t62);
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700084964e); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(9));
            c.w64(6, c.r64(11));
            return c.call(0x70007fc694, 0x7000849663, 0x700084965e);
        }
        40 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(8, c.r64(1));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084966c); };
            c.w64(9, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000849670); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000849674); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000849678); };
            c.w64(7, t67);
            blk = 38; continue;
        }
        41 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700084967e); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r68 = c.r64(11) & c.r64(1);
            c.w64(11, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000849692); };
            c.logic(t69 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x700084969d); }
            blk = 6; continue;
        }
        44 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x400)) else { return RegfileExit::fault(0x70008496a5); };
            c.w64(7, t70);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008496ac); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008496af); };
            c.w64(3, t71);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008496b6); };
            c.w64(1, t72);
            blk = 30; continue;
        }
        45 => {
            return RegfileExit::fault(0x70008496c2);
        }
        46 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008496c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008496cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008496d0); }
            return c.call(0x70007febf4, 0x70008496d9, 0x70008496d4);
        }
        47 => {
            c.w64(9, c.r64(0));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008496dc); };
            c.w64(8, t73);
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008496e3); };
            c.w64(12, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008496ea); };
            c.w64(3, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008496ee); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008496f2); };
            c.w64(7, t77);
            blk = 14; continue;
        }
        48 => {
            return RegfileExit::fault(0x70008496fb);
        }
        49 => {
            return RegfileExit::fault(0x70008496fc);
        }
        50 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008496fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000849701); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000849705); }
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x7000849713, 0x700084970e);
        }
        51 => {
            c.w64(15, c.r64(0));
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000849716); };
            c.w64(8, t78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084971d); };
            c.w64(12, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000849724); };
            c.w64(7, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000849728); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084972c); };
            c.w64(1, t82);
            c.w32(11, 0x4u64);
            blk = 20; continue;
        }
        52 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x7000849740); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000849744); }
            return c.call(0x70007febf4, 0x700084974d, 0x7000849748);
        }
        53 => {
            c.w64(7, c.r64(0));
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000849750); };
            c.w64(0, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000849757); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084975b); };
            c.w64(1, t85);
            blk = 24; continue;
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000849764); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000849768); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084976c); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700084977a, 0x7000849775);
        }
        55 => {
            c.w64(8, c.r64(0));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084977d); };
            c.w64(1, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000849781); };
            c.w64(3, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000849785); };
            c.w64(7, t88);
            blk = 34; continue;
        }
        56 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000849793); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000849794); };
            c.w64(7, t89);
            let Some(t90) = c.ld32(c.r64(7).wrapping_add(0x53)) else { return RegfileExit::fault(0x700084979b); };
            c.w32(7, t90);
            let r91 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r91);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008497a1); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008497a2); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008497a3); };
            c.w64(3, t92);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008497b4, 0x70008497af);
        }
        57 => {
            return RegfileExit::fault(0x70008497b4);
        }
        58 => {
            c.w32(9, c.r32(9));
            let r93 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r93);
            c.w64(11, 0xfffffffffffc0000u64);
            let r94 = c.r64(11) & c.r64(9);
            c.w64(11, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008497c5); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(12, c.r64(1).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008497d4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008497d5); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008497e2, 0x70008497dd);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008497e2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008497e4); }
            blk = 43; continue;
        }
        61 => {
            return RegfileExit::jump(0x70008497ec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008497f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008497f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008497f8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008497fe); };
            c.w32(7, t1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000849801); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084980b); };
            c.w32(11, t4);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000849812); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849820); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000849827); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084982e); };
            c.w32(12, t9);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849835); };
            c.w32(15, t10);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700084983c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000849840); }
            c.cmp(c.r64(12), 0x6bu64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x700084984a);
        }
        3 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x700084984e); };
            c.w32(11, t11);
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700084985b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700084985f); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(2, c.r64(12).wrapping_mul(2).wrapping_add(0x18));
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(15), 0x2000000u64, 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 50; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008498a0); };
            c.w64(3, t12);
            let Some(t13) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008498a7); };
            c.w64(2, t13);
            c.w64(1, c.r64(8).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008498b2); };
            c.w64(7, t14);
            let Some(t15) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008498b9); };
            c.cmp(t15, c.r64(6), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 46; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008498cf); }
            let r16 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r16);
            blk = 10; continue;
        }
        10 => {
            let r17 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r17);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008498d9); };
            c.w64(1, t18);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008498e0); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008498e3); }
            c.w64(1, c.r64(2).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r19 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r19);
            let r20 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r20);
            c.logic(r20, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008498f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008498fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008498ff); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(2));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x7000849908); };
            c.w64(0, t21);
            c.w64(10, 0x7000849916u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000849916); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084991a); }
            return c.call(c.r64(0), 0x7000849920, 0x700084991e);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000849920); }
            let Some(t22) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000849928); };
            c.w64(4, t22);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084992d); };
            c.w64(7, t23);
            let Some(t24) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000849934); };
            c.w64(9, t24);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084993b); };
            c.w64(12, t25);
            let Some(t26) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000849942); };
            c.cmp(t26, c.r64(11), 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700084994c); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 13; continue;
        }
        13 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000849953); };
            c.w64(9, t27);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000849957); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084995b); };
            c.w64(9, t28);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700084995f); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000849963); };
            c.w64(9, t29);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700084996a); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084996e); };
            c.w64(0, t30);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000849972); }
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000849975); };
            c.w64(12, t31);
            c.w64(11, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(7));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000849982); };
            c.w64(0, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000849986); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084998a); };
            c.w64(7, t34);
            blk = 20; continue;
        }
        14 => {
            let r35 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000849996); };
            c.w64(3, t36);
            let Some(t37) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700084999d); };
            c.w64(1, t37);
            c.w64(6, c.r64(2).wrapping_add(c.r64(1)));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008499a4); };
            c.w64(8, t38);
            let Some(t39) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008499ab); };
            c.cmp(t39, c.r64(6), 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008499b4); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 16; continue;
        }
        16 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008499bb); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008499bf); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008499c3); };
            c.w64(11, t40);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008499ca); }
            c.w64(11, c.r64(3).wrapping_add(0x10));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008499d2); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008499d6); };
            c.w64(2, t41);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008499dd); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008499e1); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 20; continue;
        }
        17 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008499ea); };
            c.w64(3, t42);
            let Some(t43) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008499f1); };
            c.w64(2, t43);
            c.w64(1, c.r64(2).wrapping_add(0x10));
            let r44 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r44);
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008499fb); };
            c.w64(6, t45);
            let Some(t46) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000849a02); };
            c.cmp(t46, c.r64(1), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x7000849a0b); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000849a12); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x7000849a16); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000849a1a); };
            c.w64(11, t47);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000849a21); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000849a25); }
            c.w32(15, 0x1u64);
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000849a43); };
            c.w64(15, t48);
            blk = 23; continue;
        }
        23 => {
            let r49 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r49);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(15)) { return RegfileExit::fault(0x7000849a4e); }
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_ne() { blk = 23; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t50) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849a58); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            c.w32(12, 0x7u64);
            c.w64(15, c.r64(0));
            blk = 26; continue;
        }
        25 => {
            let r52 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r52);
            c.w64(7, c.r64(6));
            blk = 26; continue;
        }
        26 => {
            let Some(t53) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849a71); };
            c.w32(0, t53);
            let Some(t54) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000849a75); };
            c.w32(2, t54);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000849a7f); };
            c.w32(1, t55);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r56 = c.add(c.r64(0), c.r64(2), 64);
            c.w64(0, r56);
            c.w64(2, c.r64(7));
            blk = 28; continue;
        }
        27 => {
            c.w64(2, c.r64(6));
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ge() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(6, c.r64(2).wrapping_add(0x1));
            let r57 = c.imul2(c.r64(2), 0x3u64, 64);
            c.w64(2, r57);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r58 = c.add(c.r64(2), c.r64(8), 64);
            c.w64(2, r58);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000849ab2); };
            c.w32(8, t59);
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000849ab7); };
            c.cmp(t60, c.r32(8), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t61) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849ac0); };
            c.w32(7, t61);
            let r62 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r62);
            let Some(t63) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849ac7); };
            c.cmp(t63, 0x825u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000849acf); };
            c.w32(7, t64);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x7000849ad4); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r65 = c.r64(0) & c.r64(11);
            c.w64(0, r65);
            c.logic(r65, 64);
            let Some(t66) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000849ae8); };
            c.logic(t66 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 25; continue;
        }
        34 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(8)) { return RegfileExit::fault(0x7000849af7); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r67 = c.r64(7) & c.r64(11);
            c.w64(7, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000849b0b); };
            c.logic(t68 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 25; continue;
        }
        37 => {
            let r69 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r69);
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x400)) else { return RegfileExit::fault(0x7000849b1c); };
            c.w64(9, t70);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000849b23); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000849b27); }
            c.w64(0, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000849b31); }
            return c.ret(0x0, 0x7000849b32);
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000849b33); }
            blk = 39; continue;
        }
        39 => {
            c.w32(3, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t71) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849b3a); };
            c.w32(11, t71);
            let r72 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r72);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x7000849b41); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(9));
            c.w64(7, c.r64(6));
            return c.call(0x70007fc694, 0x7000849b58, 0x7000849b53);
        }
        41 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000849b5e); };
            c.w64(9, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000849b62); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000849b66); };
            c.w64(0, t75);
            blk = 39; continue;
        }
        42 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000849b6c); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r76 = c.r64(11) & c.r64(0);
            c.w64(11, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000849b80); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x7000849b8b); }
            blk = 1; continue;
        }
        45 => {
            return RegfileExit::fault(0x7000849b93);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x7000849b94); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000849b98); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000849ba4, 0x7000849b9f);
        }
        47 => {
            c.w64(2, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000849ba7); };
            c.w64(15, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000849bab); };
            c.w64(8, t79);
            blk = 10; continue;
        }
        48 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x7000849bbe, 0x7000849bb9);
        }
        49 => {
            c.w64(7, c.r64(0));
            blk = 13; continue;
        }
        50 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000849bc6); };
            c.w64(3, t80);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000849bd6, 0x7000849bd1);
        }
        51 => {
            return RegfileExit::fault(0x7000849bd6);
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x7000849bd7); }
            return c.call(0x70007febf4, 0x7000849be0, 0x7000849bdb);
        }
        53 => {
            c.w64(3, c.r64(0));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000849be3); };
            c.w64(0, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000849be7); };
            c.w64(9, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000849beb); };
            c.w64(7, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000849bef); };
            c.w64(12, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000849bf3); };
            c.w64(11, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000849bf7); };
            c.w64(15, t86);
            blk = 16; continue;
        }
        54 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x7000849c05); }
            return c.call(0x70007febf4, 0x7000849c0e, 0x7000849c09);
        }
        55 => {
            c.w64(3, c.r64(0));
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000849c11); };
            c.w64(0, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000849c15); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000849c19); };
            c.w64(7, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000849c1d); };
            c.w64(12, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000849c21); };
            c.w64(11, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000849c25); };
            c.w64(15, t92);
            blk = 19; continue;
        }
        56 => {
            c.w32(9, c.r32(9));
            let r93 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r93);
            c.w64(11, 0xfffffffffffc0000u64);
            let r94 = c.r64(11) & c.r64(9);
            c.w64(11, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000849c3e); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(12, c.r64(0).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000849c4d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000849c4e); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000849c5b, 0x7000849c56);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000849c5b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000849c5d); }
            blk = 44; continue;
        }
        59 => {
            c.w32(8, c.r32(8));
            let r96 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r96);
            c.w64(7, 0xfffffffffffc0000u64);
            let r97 = c.r64(7) & c.r64(8);
            c.w64(7, r97);
            c.logic(r97, 64);
            let Some(t98) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000849c73); };
            c.logic(t98 & 0x2u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, c.r64(11).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000849c81); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000849c82); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000849c8f, 0x7000849c8a);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000849c8f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000849c91); }
            blk = 36; continue;
        }
        62 => {
            c.w32(7, c.r32(7));
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            c.w64(0, 0xfffffffffffc0000u64);
            let r100 = c.r64(0) & c.r64(7);
            c.w64(0, r100);
            c.logic(r100, 64);
            let Some(t101) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000849ca6); };
            c.logic(t101 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000849cb4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000849cb5); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000849cc2, 0x7000849cbd);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000849cc2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000849cc4); }
            blk = 33; continue;
        }
        65 => {
            return RegfileExit::jump(0x7000849ccc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000849cd4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a18)) else { return RegfileExit::fault(0x7000849cd4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000849cf4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a20)) else { return RegfileExit::fault(0x7000849cf4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000849d14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a28)) else { return RegfileExit::fault(0x7000849d14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000849d34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1a30)) else { return RegfileExit::fault(0x7000849d34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000849d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000849d54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000849d58); }
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849d7c); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849d82); };
            c.cmp(t2, c.r32(1), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849d8b); };
            c.w32(1, t3);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 29; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849d9f); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849da5); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_ne() { blk = 29; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000849db0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000849db1); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x7000849db2); };
            c.w64(3, t7);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000849dc5, 0x7000849dc0);
        }
        8 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000849dc5); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 29; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849dd7); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000849ddd); };
            c.cmp(t11, 0x40u64, 16);
            if c.cc_ae() { blk = 29; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x7000849def, 0x7000849dea);
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000849def); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 29; continue;
        }
        14 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849e01); };
            c.w32(1, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849e04); };
            c.cmp(t14, c.r32(1), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849e11); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849e16); }
            blk = 21; continue;
        }
        16 => {
            let Some(t15) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849e1d); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849e20); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 17; continue;
        }
        17 => {
            let r17 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r17);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849e2f); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 21; continue;
        }
        18 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849e43); };
            c.w32(1, t18);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849e46); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 20; continue;
        }
        20 => {
            let r20 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r20);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849e51); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 21; continue;
        }
        21 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            if c.cc_e() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            if c.cc_e() { blk = 29; continue; }
            blk = 25; continue;
        }
        25 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 31; continue; }
            blk = 26; continue;
        }
        26 => {
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 31; continue;
        }
        28 => {
            c.movq_rx(1, 1);
            if let Some(sr) = c.shr(c.r64(1), 0x20u64, 64) { c.w64(1, sr); }
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000849e8e); };
            c.w64(0, t21);
            blk = 30; continue;
        }
        30 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000849e98); }
            return c.ret(0x0, 0x7000849e99);
        }
        31 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000849e9a); };
            c.w64(0, t22);
            blk = 30; continue;
        }
        32 => {
            return RegfileExit::jump(0x7000849ea4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000849eb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849ece); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849ed1); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849ede); };
            c.w32(3, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849ee1); };
            c.cmp(t3, c.r32(3), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849eee); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849ef3); }
            blk = 11; continue;
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849efa); };
            c.w32(3, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849efd); };
            c.cmp(t5, c.r32(3), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            let r6 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r6);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849f0c); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 11; continue;
        }
        8 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849f20); };
            c.w32(3, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849f23); };
            c.cmp(t8, c.r32(3), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849f2e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 11; continue;
        }
        11 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            if c.cc_e() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 20; continue;
        }
        18 => {
            c.movq_rx(3, 1);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.movq_rx(2, 0);
            if let Some(sr) = c.shr(c.r64(2), 0x20u64, 64) { c.w64(2, sr); }
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000849f6b); };
            c.w64(0, t10);
            return c.ret(0x0, 0x7000849f72);
        }
        20 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000849f73); };
            c.w64(0, t11);
            return c.ret(0x0, 0x7000849f7a);
        }
        21 => {
            return RegfileExit::jump(0x7000849f7c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000849f94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.w32(7, c.r32(0));
            c.w32(8, c.r32(2));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000849fb5); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000849fb8); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000849fc3); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 16; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000849fd5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000849fd9); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 34; continue;
        }
        5 => {
            let r4 = c.add(c.r32(7), c.r32(8), 32);
            c.w32(7, r4);
            if c.cc_o() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000849fe9); };
            c.w64(1, t5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000849fed); };
            c.w32(8, t6);
            c.w32(9, c.r32(8));
            let r7 = c.r32(9) | 0x2u64;
            c.w32(9, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084a001); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a006); }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x700084a010);
        }
        9 => {
            let r8 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r8);
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 16; continue;
        }
        10 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a027); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a02d); };
            c.cmp(t12, c.r32(7), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a040); };
            c.w32(7, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a043); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a04c); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a051); }
            blk = 16; continue;
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a058); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a05c); }
            let r15 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r15);
            blk = 34; continue;
        }
        15 => {
            let r16 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r16);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a069); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 16; continue;
        }
        16 => {
            let Some(t17) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a076); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a07a); };
            c.w32(7, t18);
            c.w32(8, c.r32(7));
            let r19 = c.r32(8) | 0xeu64;
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084a08d); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a091); }
            blk = 18; continue;
        }
        18 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084a098); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084a09f); };
            c.w64(7, t21);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084a0a6); };
            c.w64(9, t22);
            let Some(t23) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084a0ad); };
            c.cmp(t23, c.r64(8), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084a0b6); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 20; continue;
        }
        20 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a0c1); };
            c.w64(7, t24);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084a0c8); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084a0cb); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084a0d3);
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a0d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a0d8); }
            let r25 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r25);
            let Some(t26) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a0de); };
            c.w32(7, t26);
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a106); };
            c.w32(7, t27);
            let r28 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r28);
            let Some(t29) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a10c); };
            c.cmp(t29, 0x41u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 26; continue;
        }
        26 => {
            blk = 35; continue;
        }
        27 => {
            let Some(t30) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a119); };
            c.w32(7, t30);
            let r31 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r31);
            let Some(t32) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a11f); };
            c.cmp(t32, 0x40u64, 16);
            if c.cc_ae() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t33) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a126); };
            c.w64(7, t33);
            let Some(t34) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a12a); };
            c.w32(8, t34);
            c.w32(9, c.r32(8));
            let r35 = c.r32(9) | 0x20u64;
            c.w32(9, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084a13e); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a143); }
            blk = 30; continue;
        }
        30 => {
            let Some(t36) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a14a); };
            c.w64(1, t36);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(1));
            return c.call(0x700086f9d4, 0x700084a15c, 0x700084a157);
        }
        31 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a15f); }
            return c.ret(0x0, 0x700084a160);
        }
        32 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a167); };
            c.w32(7, t37);
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a16a); };
            c.cmp(t38, c.r32(7), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a173); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a179); };
            c.cmp(t41, 0x43u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(7, 0xfeu64);
            blk = 37; continue;
        }
        36 => {
            c.w32(7, 0x1eu64);
            blk = 37; continue;
        }
        37 => {
            let Some(t42) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a18c); };
            c.w64(8, t42);
            let Some(t43) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a190); };
            c.w32(9, t43);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r44 = c.r32(7) | c.r32(11);
            c.w32(7, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084a1a5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a1aa); }
            blk = 39; continue;
        }
        39 => {
            let Some(t45) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a1b2); };
            c.w64(1, t45);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(1));
            return c.call(0x70008ad794, 0x700084a1c4, 0x700084a1bf);
        }
        40 => {
            blk = 31; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a1c6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a1ca); }
            let r46 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r46);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700084a1d5); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700084a1da); }
            return c.call(0x70007febf4, 0x700084a1e4, 0x700084a1df);
        }
        42 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084a1e7); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084a1ec); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a1f4); }
            blk = 20; continue;
        }
        43 => {
            let Some(t47) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a1fa); };
            c.w64(7, t47);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084a1fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700084a202); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x700089b334, 0x700084a214, 0x700084a20f);
        }
        44 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084a218); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a21c); };
            c.w64(1, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084a220); };
            c.w64(3, t50);
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a224); };
            c.w32(7, t51);
            c.w32(8, c.r32(7));
            let r52 = c.r32(8) | 0x40u64;
            c.w32(8, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084a237); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a23b); }
            blk = 47; continue;
        }
        47 => {
            blk = 31; continue;
        }
        48 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084a247); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a24b); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084a24f); };
            c.w64(8, t55);
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a253); };
            c.w32(9, t56);
            c.w32(11, c.r32(9));
            let r57 = c.r32(11) | 0xfeu64;
            c.w32(11, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700084a26a); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a26f); }
            blk = 50; continue;
        }
        50 => {
            let Some(t58) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a276); };
            c.w64(1, t58);
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084a27f); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700084a280); };
            c.w64(3, t59);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700084a294, 0x700084a28f);
        }
        51 => {
            return RegfileExit::fault(0x700084a294);
        }
        52 => {
            return RegfileExit::jump(0x700084a298);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084a2b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.w32(7, c.r32(2));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a2c9); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a2cc); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a2d7); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 19; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a2e9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a2ed); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 29; continue;
        }
        5 => {
            let r4 = c.sub(c.r32(7), c.r32(8), 32);
            c.w32(7, r4);
            if c.cc_o() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(8, 0x2u64);
            blk = 10; continue;
        }
        7 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084a308); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084a30f); };
            c.w64(8, t6);
            let r7 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r7);
            let r8 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084a31a); };
            c.w64(11, t9);
            let Some(t10) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084a321); };
            c.cmp(t10, c.r64(9), 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700084a32a); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 9; continue;
        }
        9 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a345); };
            c.w64(8, t11);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700084a34c); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084a350); }
            c.w32(8, 0xeu64);
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a35b); };
            c.w64(1, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a35f); };
            c.w32(9, t13);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r14 = c.r32(8) | c.r32(11);
            c.w32(8, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084a375); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a37a); }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700084a384);
        }
        13 => {
            let Some(t15) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a385); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a38b); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a39c); };
            c.w32(7, t18);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a39f); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a3a8); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a3ad); }
            blk = 19; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a3b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a3b8); }
            let r20 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r20);
            blk = 29; continue;
        }
        18 => {
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a3c5); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 19; continue;
        }
        19 => {
            let Some(t22) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a3d2); };
            c.w64(1, t22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a3d6); };
            c.w32(7, t23);
            c.w32(8, c.r32(7));
            let r24 = c.r32(8) | 0xeu64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084a3e9); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a3ed); }
            blk = 21; continue;
        }
        21 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084a3f4); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084a3fb); };
            c.w64(7, t26);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084a402); };
            c.w64(9, t27);
            let Some(t28) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084a409); };
            c.cmp(t28, c.r64(8), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084a412); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 23; continue;
        }
        23 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a41d); };
            c.w64(7, t29);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084a424); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084a427); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084a42f);
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a430); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a434); }
            let r30 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r30);
            let Some(t31) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a43a); };
            c.w32(7, t31);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a44e); };
            c.w32(7, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a451); };
            c.cmp(t33, c.r32(7), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(7, 0x1eu64);
            blk = 35; continue;
        }
        29 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a461); };
            c.w32(7, t34);
            let r35 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r35);
            let Some(t36) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a467); };
            c.cmp(t36, 0x43u64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(7, 0x1eu64);
            blk = 35; continue;
        }
        31 => {
            c.w32(7, 0x1eu64);
            blk = 35; continue;
        }
        32 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a480); };
            c.w32(7, t37);
            let r38 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r38);
            let Some(t39) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a486); };
            c.cmp(t39, 0x41u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(7, 0xfeu64);
            blk = 35; continue;
        }
        35 => {
            let Some(t40) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a496); };
            c.w64(8, t40);
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a49a); };
            c.w32(9, t41);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r42 = c.r32(7) | c.r32(11);
            c.w32(7, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084a4af); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a4b4); }
            blk = 37; continue;
        }
        37 => {
            let Some(t43) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a4bc); };
            c.w64(1, t43);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(1));
            return c.call(0x70008adb14, 0x700084a4ce, 0x700084a4c9);
        }
        38 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a4d1); }
            return c.ret(0x0, 0x700084a4d2);
        }
        39 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a4d3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a4d7); }
            let r44 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700084a4dd); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084a4e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700084a4ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084a4ee); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700084a4fa, 0x700084a4f5);
        }
        40 => {
            c.w64(7, c.r64(0));
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084a4fd); };
            c.w64(1, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084a501); };
            c.w64(3, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084a505); };
            c.w64(2, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a509); };
            c.w64(0, t48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a510); }
            blk = 9; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a516); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a51a); }
            let r49 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r49);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700084a525); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700084a52a); }
            return c.call(0x70007febf4, 0x700084a534, 0x700084a52f);
        }
        42 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084a537); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084a53c); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a544); }
            blk = 23; continue;
        }
        43 => {
            let Some(t50) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a54a); };
            c.w64(7, t50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a54e); };
            c.w32(8, t51);
            c.w32(9, c.r32(8));
            let r52 = c.r32(9) | 0x40u64;
            c.w32(9, r52);
            c.logic(r52, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084a55a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700084a55e); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084a56a); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a56f); }
            blk = 45; continue;
        }
        45 => {
            let Some(t53) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a576); };
            c.w64(7, t53);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x700089bcd4, 0x700084a588, 0x700084a583);
        }
        46 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 38; continue;
        }
        48 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084a591); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a595); };
            c.w64(7, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084a599); };
            c.w64(3, t56);
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a59d); };
            c.w32(8, t57);
            c.w32(9, c.r32(8));
            let r58 = c.r32(9) | 0xfeu64;
            c.w32(9, r58);
            c.logic(r58, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084a5b4); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a5b9); }
            blk = 50; continue;
        }
        50 => {
            let Some(t59) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a5c0); };
            c.w64(1, t59);
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084a5c9); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700084a5ca); };
            c.w64(3, t60);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700084a5de, 0x700084a5d9);
        }
        51 => {
            return RegfileExit::fault(0x700084a5de);
        }
        52 => {
            return RegfileExit::jump(0x700084a5e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084a5f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a604); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a607); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a612); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 24; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a624); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a628); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 34; continue;
        }
        5 => {
            c.w64(7, c.r64(0));
            let r4 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r4);
            c.w64(8, c.r64(2));
            let r5 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r5);
            c.w64(9, c.r64(8));
            let r6 = c.imul2(c.r32(9), c.r32(7), 32);
            c.w32(9, r6);
            if c.cc_o() { blk = 49; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(7, c.r64(9));
            let r7 = c.add(c.r32(7), c.r32(9), 32);
            c.w32(7, r7);
            if c.cc_o() { blk = 44; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        9 => {
            let r8 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r8);
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            let r10 = c.r32(0) | c.r32(2);
            c.w32(0, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700084a671); };
            c.w64(7, t11);
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(8, 0xeu64);
            blk = 15; continue;
        }
        14 => {
            c.w32(8, 0x2u64);
            blk = 15; continue;
        }
        15 => {
            let Some(t12) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a68c); };
            c.w64(1, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a690); };
            c.w32(9, t13);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r14 = c.r32(8) | c.r32(11);
            c.w32(8, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084a6a6); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a6ab); }
            blk = 17; continue;
        }
        17 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700084a6b5);
        }
        18 => {
            let Some(t15) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a6b6); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a6bc); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a6cd); };
            c.w32(7, t18);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a6d0); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a6d9); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a6de); }
            blk = 24; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a6e5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a6e9); }
            let r20 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r20);
            blk = 34; continue;
        }
        23 => {
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a6f6); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            let Some(t22) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a703); };
            c.w64(1, t22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a707); };
            c.w32(7, t23);
            c.w32(8, c.r32(7));
            let r24 = c.r32(8) | 0xeu64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084a71a); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a71e); }
            blk = 26; continue;
        }
        26 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084a725); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084a72c); };
            c.w64(7, t26);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084a733); };
            c.w64(9, t27);
            let Some(t28) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084a73a); };
            c.cmp(t28, c.r64(8), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084a743); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a74e); };
            c.w64(7, t29);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084a755); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084a758); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084a760);
        }
        29 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a761); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a765); }
            let r30 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r30);
            let Some(t31) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a76b); };
            c.w32(7, t31);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 39; continue; }
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a77f); };
            c.w32(7, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a782); };
            c.cmp(t33, c.r32(7), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        34 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a792); };
            c.w32(7, t34);
            let r35 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r35);
            let Some(t36) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a798); };
            c.cmp(t36, 0x43u64, 16);
            if c.cc_ne() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        36 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        37 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a7b1); };
            c.w32(7, t37);
            let r38 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r38);
            let Some(t39) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084a7b7); };
            c.cmp(t39, 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(7, 0xfeu64);
            blk = 40; continue;
        }
        40 => {
            let Some(t40) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a7c7); };
            c.w64(8, t40);
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a7cb); };
            c.w32(9, t41);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r42 = c.r32(7) | c.r32(11);
            c.w32(7, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084a7e0); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a7e5); }
            blk = 42; continue;
        }
        42 => {
            let Some(t43) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a7ed); };
            c.w64(1, t43);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(1));
            return c.call(0x70008adcd4, 0x700084a7ff, 0x700084a7fa);
        }
        43 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a802); }
            return c.ret(0x0, 0x700084a803);
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a804); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a808); }
            let r44 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r44);
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084a80e); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084a815); };
            c.w64(8, t46);
            c.w64(11, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084a81c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700084a820); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084a824); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084a828); };
            c.w64(12, t47);
            let Some(t48) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084a82f); };
            c.cmp(t48, c.r64(11), 64);
            if c.cc_a() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084a83f, 0x700084a83a);
        }
        46 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084a83f); };
            c.w64(9, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084a843); };
            c.w64(1, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084a847); };
            c.w64(3, t51);
            blk = 48; continue;
        }
        47 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700084a84d); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 48; continue;
        }
        48 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a860); };
            c.w64(7, t52);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084a867); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084a86a); }
            c.w64(7, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a875); }
            blk = 12; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a87b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a87f); }
            let r53 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r53);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084a885); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084a88c); };
            c.w64(11, t55);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084a893); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700084a897); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084a89b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084a89f); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084a8a3); };
            c.w64(15, t56);
            let Some(t57) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700084a8aa); };
            c.cmp(t57, c.r64(12), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084a8b9, 0x700084a8b4);
        }
        51 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a8b9); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084a8bd); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084a8c1); };
            c.w64(3, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084a8c5); };
            c.w64(8, t61);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700084a8cb); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 53; continue;
        }
        53 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a8ea); };
            c.w64(7, t62);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084a8f1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084a8f4); }
            c.w64(7, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a8ff); }
            blk = 12; continue;
        }
        54 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a905); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a909); }
            let r63 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r63);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700084a914); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700084a919); }
            return c.call(0x70007febf4, 0x700084a923, 0x700084a91e);
        }
        55 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084a926); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084a92b); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084a933); }
            blk = 28; continue;
        }
        56 => {
            let Some(t64) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084a939); };
            c.w64(7, t64);
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084a93d); };
            c.w32(8, t65);
            c.w32(9, c.r32(8));
            let r66 = c.r32(9) | 0x40u64;
            c.w32(9, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084a951); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084a956); }
            blk = 58; continue;
        }
        58 => {
            let Some(t67) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084a95d); };
            c.w64(1, t67);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084a961); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084a962); }
            c.w32(7, 0x4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084a968); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084a969); };
            c.w64(3, t68);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700084a97d, 0x700084a978);
        }
        59 => {
            blk = 43; continue;
        }
        60 => {
            return RegfileExit::jump(0x700084a984);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084a994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084a9a4); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084a9a7); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084a9b2); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 24; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084a9c4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084a9c8); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 34; continue;
        }
        5 => {
            c.w64(7, c.r64(0));
            let r4 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r4);
            c.w64(8, c.r64(2));
            let r5 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r5);
            c.w32(9, 0);
            c.logic(0, 32);
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(7), 0xffffffffffffffffu64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(8), 0xc0000000u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(8));
            c.cdq();
            c.idiv(c.r32(7), 32);
            c.w64(9, c.r64(0));
            let r6 = c.imul2(c.r32(9), c.r32(7), 32);
            c.w32(9, r6);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(7, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(8, 0x2u64);
            blk = 15; continue;
        }
        12 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084aa1b); };
            c.w64(9, t7);
            let Some(t8) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084aa22); };
            c.w64(11, t8);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084aa29); };
            c.w64(15, t9);
            let Some(t10) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700084aa30); };
            c.cmp(t10, c.r64(12), 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700084aa39); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084aa58); };
            c.w64(7, t11);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084aa5f); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084aa63); }
            c.w32(8, 0x6u64);
            c.w64(7, c.r64(9));
            blk = 15; continue;
        }
        15 => {
            let Some(t12) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084aa72); };
            c.w64(1, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084aa76); };
            c.w32(9, t13);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r14 = c.r32(8) | c.r32(11);
            c.w32(8, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084aa8c); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084aa91); }
            blk = 17; continue;
        }
        17 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700084aa9b);
        }
        18 => {
            let Some(t15) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084aa9c); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084aaa2); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084aab3); };
            c.w32(7, t18);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084aab6); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084aabf); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084aac4); }
            blk = 24; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084aacb); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084aacf); }
            let r20 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r20);
            blk = 34; continue;
        }
        23 => {
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084aadc); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            let Some(t22) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084aae9); };
            c.w64(1, t22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084aaed); };
            c.w32(7, t23);
            c.w32(8, c.r32(7));
            let r24 = c.r32(8) | 0xeu64;
            c.w32(8, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084ab00); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084ab04); }
            blk = 26; continue;
        }
        26 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084ab0b); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084ab12); };
            c.w64(7, t26);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084ab19); };
            c.w64(9, t27);
            let Some(t28) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084ab20); };
            c.cmp(t28, c.r64(8), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084ab29); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ab37); };
            c.w64(7, t29);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084ab3e); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084ab41); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084ab49);
        }
        29 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084ab4a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084ab4e); }
            let r30 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r30);
            let Some(t31) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ab54); };
            c.w32(7, t31);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 39; continue; }
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ab68); };
            c.w32(7, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ab6b); };
            c.cmp(t33, c.r32(7), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        34 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ab7b); };
            c.w32(7, t34);
            let r35 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r35);
            let Some(t36) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ab81); };
            c.cmp(t36, 0x43u64, 16);
            if c.cc_ne() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        36 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        37 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ab9a); };
            c.w32(7, t37);
            let r38 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r38);
            let Some(t39) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084aba0); };
            c.cmp(t39, 0x41u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(7, 0xfeu64);
            blk = 40; continue;
        }
        40 => {
            let Some(t40) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084abb0); };
            c.w64(8, t40);
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084abb4); };
            c.w32(9, t41);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r42 = c.r32(7) | c.r32(11);
            c.w32(7, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084abc9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084abce); }
            blk = 42; continue;
        }
        42 => {
            let Some(t43) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084abd6); };
            c.w64(1, t43);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(1));
            return c.call(0x70008adf94, 0x700084abe8, 0x700084abe3);
        }
        43 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084abeb); }
            return c.ret(0x0, 0x700084abec);
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084abed); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084abf1); }
            let r44 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r44);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084abfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700084ac00); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084ac04); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084ac08); }
            return c.call(0x70007febf4, 0x700084ac11, 0x700084ac0c);
        }
        45 => {
            c.w64(9, c.r64(0));
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ac14); };
            c.w64(1, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ac18); };
            c.w64(3, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084ac1c); };
            c.w64(8, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ac20); };
            c.w64(7, t48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084ac27); }
            blk = 14; continue;
        }
        46 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084ac2d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084ac31); }
            let r49 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r49);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700084ac3c); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700084ac41); }
            return c.call(0x70007febf4, 0x700084ac4b, 0x700084ac46);
        }
        47 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084ac4e); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084ac53); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084ac5b); }
            blk = 28; continue;
        }
        48 => {
            let Some(t50) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ac61); };
            c.w64(7, t50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084ac65); };
            c.w32(8, t51);
            c.w32(9, c.r32(8));
            let r52 = c.r32(9) | 0x40u64;
            c.w32(9, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084ac79); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084ac7e); }
            blk = 50; continue;
        }
        50 => {
            let Some(t53) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084ac85); };
            c.w64(1, t53);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084ac89); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084ac8a); }
            c.w32(7, 0x6u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084ac90); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084ac91); };
            c.w64(3, t54);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700084aca5, 0x700084aca0);
        }
        51 => {
            blk = 43; continue;
        }
        52 => {
            return RegfileExit::jump(0x700084acac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084acb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084acc4); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084acc7); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084acd2); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 20; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084ace4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084ace8); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 32; continue;
        }
        5 => {
            c.w64(7, c.r64(0));
            let r4 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r4);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 6; continue;
        }
        6 => {
            let r5 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r5);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 42; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, c.r64(2));
            c.cdq();
            c.idiv(c.r32(7), 32);
            c.w32(7, c.r64(2).wrapping_add(c.r64(2)));
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(8, 0xeu64);
            blk = 11; continue;
        }
        10 => {
            c.w32(8, 0x2u64);
            blk = 11; continue;
        }
        11 => {
            let Some(t6) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ad29); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084ad2d); };
            c.w32(9, t7);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r8 = c.r32(8) | c.r32(11);
            c.w32(8, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084ad43); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084ad48); }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700084ad52);
        }
        14 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ad53); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ad59); };
            c.cmp(t11, c.r32(7), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ad6a); };
            c.w32(7, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ad6d); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084ad76); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084ad7b); }
            blk = 20; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084ad82); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084ad86); }
            let r14 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r14);
            blk = 32; continue;
        }
        19 => {
            let r15 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r15);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084ad93); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 20; continue;
        }
        20 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ada0); };
            c.w64(1, t16);
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084ada4); };
            c.w32(7, t17);
            c.w32(8, c.r32(7));
            let r18 = c.r32(8) | 0xeu64;
            c.w32(8, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084adb7); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084adbb); }
            blk = 22; continue;
        }
        22 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084adc2); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084adc9); };
            c.w64(7, t20);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084add0); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084add7); };
            c.cmp(t22, c.r64(8), 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084ade0); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 24; continue;
        }
        24 => {
            let r23 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r23);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084adeb); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700084adf0); }
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x700084adf3); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700084adf8); }
            blk = 25; continue;
        }
        25 => {
            c.fprem();
            c.fnstsw_ax();
            let r24 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r24);
            let r25 = c.r32(0) & 0xffu64;
            c.w32(0, r25);
            c.logic(r25, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084ae07); }
            if !c.popfq() { return RegfileExit::fault(0x700084ae08); }
            if c.cc_p() { blk = 25; continue; }
            blk = 26; continue;
        }
        26 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x700084ae0d); }
            if !c.movsd_rm(1, c.r64(4)) { return RegfileExit::fault(0x700084ae10); }
            let r26 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r26);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ae19); };
            c.w64(7, t27);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084ae20); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084ae23); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084ae2b);
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084ae2c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084ae30); }
            let r28 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r28);
            let Some(t29) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ae36); };
            c.w32(7, t29);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t30) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ae4a); };
            c.w32(7, t30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ae4d); };
            c.cmp(t31, c.r32(7), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(7, 0x1eu64);
            blk = 38; continue;
        }
        32 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ae5d); };
            c.w32(7, t32);
            let r33 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r33);
            let Some(t34) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ae63); };
            c.cmp(t34, 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1eu64);
            blk = 38; continue;
        }
        34 => {
            c.w32(7, 0x1eu64);
            blk = 38; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ae7c); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            let Some(t37) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ae82); };
            c.cmp(t37, 0x41u64, 16);
            if c.cc_e() { blk = 55; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(7, 0xfeu64);
            blk = 38; continue;
        }
        38 => {
            let Some(t38) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ae92); };
            c.w64(8, t38);
            let Some(t39) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084ae96); };
            c.w32(9, t39);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r40 = c.r32(7) | c.r32(11);
            c.w32(7, r40);
            c.logic(r40, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084aeab); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084aeb0); }
            blk = 40; continue;
        }
        40 => {
            let Some(t41) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084aeb8); };
            c.w64(1, t41);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(1));
            return c.call(0x70008ae1b4, 0x700084aeca, 0x700084aec5);
        }
        41 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084aecd); }
            return c.ret(0x0, 0x700084aece);
        }
        42 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084aecf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084aed3); }
            let r42 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r42);
            c.w64(0, c.r64(2));
            c.cdq();
            c.idiv(c.r32(7), 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084aedf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700084aee3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700084aee7); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(7, c.r64(2));
            let r43 = c.add(c.r32(7), c.r32(2), 32);
            c.w32(7, r43);
            if c.cc_o() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 50; continue;
        }
        45 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084aeff); };
            c.w64(7, t44);
            let Some(t45) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084af06); };
            c.w64(8, t45);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084af0d); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084af14); };
            c.cmp(t47, c.r64(9), 64);
            if c.cc_a() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700084af26, 0x700084af21);
        }
        47 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084af26); };
            c.w64(2, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084af2a); };
            c.w64(1, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084af2e); };
            c.w64(3, t50);
            blk = 49; continue;
        }
        48 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700084af34); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 49; continue;
        }
        49 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084af46); };
            c.w64(7, t51);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084af4d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084af50); }
            blk = 50; continue;
        }
        50 => {
            c.w64(7, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084af5b); }
            blk = 8; continue;
        }
        51 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700084af61); };
            c.w64(7, t52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084af6b); }
            blk = 8; continue;
        }
        52 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700084af71); };
            c.w64(7, t53);
            blk = 8; continue;
        }
        53 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084af7d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084af81); }
            let r54 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r54);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700084af8c); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700084af91); }
            return c.call(0x70007febf4, 0x700084af9b, 0x700084af96);
        }
        54 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084af9e); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084afa3); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084afab); }
            blk = 24; continue;
        }
        55 => {
            let Some(t55) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084afb1); };
            c.w64(7, t55);
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084afb5); };
            c.w32(8, t56);
            c.w32(9, c.r32(8));
            let r57 = c.r32(9) | 0x40u64;
            c.w32(9, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084afc9); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084afce); }
            blk = 57; continue;
        }
        57 => {
            let Some(t58) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084afd5); };
            c.w64(1, t58);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084afd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084afda); }
            c.w32(7, 0x8u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084afe0); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084afe1); };
            c.w64(3, t59);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700084aff5, 0x700084aff0);
        }
        58 => {
            blk = 41; continue;
        }
        59 => {
            return RegfileExit::jump(0x700084affc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084b014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084b014); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084b018); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084b01e); };
            c.w64(1, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700084b022); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084b026); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b037); };
            c.w32(7, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b03a); };
            c.cmp(t3, c.r32(7), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 3; continue;
        }
        3 => {
            let r4 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r4);
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084b049); }
            c.xorpd_rr(3, 3);
            c.cvtsi2sd(3, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084b05b); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084b062); };
            c.w64(8, t6);
            let r7 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r7);
            let r8 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084b06d); };
            c.w64(11, t9);
            let Some(t10) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084b074); };
            c.cmp(t10, c.r64(9), 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700084b07d); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b094); };
            c.w64(8, t11);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700084b09b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084b09f); }
            c.w64(10, c.r64(4));
            let r12 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r12);
            let r13 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r13);
            c.logic(r13, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700084b0ae); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x700084b0b3); };
            c.w64(0, t14);
            c.w64(10, 0x700084b0c1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700084b0c1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084b0c5); }
            return c.call(c.r64(0), 0x700084b0cb, 0x700084b0c9);
        }
        7 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084b0cb); }
            let Some(t15) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084b0d3); };
            c.w64(4, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b0d8); };
            c.w64(0, t16);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084b0dc); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084b0e1); };
            c.w64(1, t17);
            let Some(t18) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b0e5); };
            c.w64(1, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b0e9); };
            c.w64(7, t19);
            let Some(t20) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084b0ed); };
            c.w32(8, t20);
            c.w32(9, c.r32(8));
            let r21 = c.r32(9) | 0xeu64;
            c.w32(9, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084b101); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084b106); }
            blk = 9; continue;
        }
        9 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084b110); }
            return c.ret(0x0, 0x700084b111);
        }
        10 => {
            let Some(t22) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b112); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b118); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b129); };
            c.w32(7, t25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b12c); };
            c.cmp(t26, c.r32(7), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(3, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084b139); }
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084b13e); }
            blk = 15; continue;
        }
        14 => {
            let r27 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            if !c.movsd_rm(3, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084b147); }
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            let Some(t28) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b154); };
            c.w64(1, t28);
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084b158); };
            c.w32(7, t29);
            c.w32(8, c.r32(7));
            let r30 = c.r32(8) | 0xeu64;
            c.w32(8, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084b16b); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084b16f); }
            blk = 17; continue;
        }
        17 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084b176); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084b17d); };
            c.w64(7, t32);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084b184); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084b18b); };
            c.cmp(t34, c.r64(8), 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084b194); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b19b); };
            c.w64(7, t35);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084b1a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084b1a5); }
            c.mov128_rr(0, 3);
            c.mov128_rr(1, 2);
            c.w64(10, c.r64(4));
            let r36 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r36);
            let r37 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r37);
            c.logic(r37, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700084b1ba); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x700084b1bf); };
            c.w64(0, t38);
            c.w64(10, 0x700084b1cdu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700084b1cd); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084b1d1); }
            return c.call(c.r64(0), 0x700084b1d7, 0x700084b1d5);
        }
        20 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084b1d7); }
            let Some(t39) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084b1df); };
            c.w64(4, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084b1e4); };
            c.w64(0, t40);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084b1e8); }
            blk = 9; continue;
        }
        21 => {
            let Some(t41) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b1f2); };
            c.w32(7, t41);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b206); };
            c.w32(7, t42);
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b209); };
            c.cmp(t43, c.r32(7), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(7, 0x1eu64);
            blk = 32; continue;
        }
        26 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b219); };
            c.w32(7, t44);
            let r45 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r45);
            let Some(t46) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b21f); };
            c.cmp(t46, 0x43u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(7, 0x1eu64);
            blk = 32; continue;
        }
        28 => {
            c.w32(7, 0x1eu64);
            blk = 32; continue;
        }
        29 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b238); };
            c.w32(7, t47);
            let r48 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r48);
            let Some(t49) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b23e); };
            c.cmp(t49, 0x41u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(7, 0xfeu64);
            blk = 32; continue;
        }
        32 => {
            let Some(t50) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b24e); };
            c.w64(8, t50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084b252); };
            c.w32(9, t51);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r52 = c.r32(7) | c.r32(11);
            c.w32(7, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084b267); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084b26c); }
            blk = 34; continue;
        }
        34 => {
            let Some(t53) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b274); };
            c.w64(1, t53);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(1));
            return c.call(0x70008ae454, 0x700084b286, 0x700084b281);
        }
        35 => {
            blk = 9; continue;
        }
        36 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084b28b); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700084b294); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700084b2a0, 0x700084b29b);
        }
        37 => {
            c.w64(7, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b2a3); };
            c.w64(2, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084b2a7); };
            c.w64(0, t55);
            blk = 6; continue;
        }
        38 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 2) { return RegfileExit::fault(0x700084b2b5); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 3) { return RegfileExit::fault(0x700084b2ba); }
            return c.call(0x70007febf4, 0x700084b2c4, 0x700084b2bf);
        }
        39 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084b2c7); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084b2cc); }
            blk = 19; continue;
        }
        40 => {
            let Some(t56) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b2d6); };
            c.w64(7, t56);
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084b2da); };
            c.w32(8, t57);
            c.w32(9, c.r32(8));
            let r58 = c.r32(9) | 0x40u64;
            c.w32(9, r58);
            c.logic(r58, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084b2ee); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084b2f3); }
            blk = 42; continue;
        }
        42 => {
            let Some(t59) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b2fa); };
            c.w64(1, t59);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084b2fe); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084b2ff); }
            c.w32(7, 0xau64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084b305); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084b306); };
            c.w64(3, t60);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700084b31a, 0x700084b315);
        }
        43 => {
            blk = 9; continue;
        }
        44 => {
            return RegfileExit::jump(0x700084b320);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084b334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084b334); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084b338); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084b33e); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b342); };
            c.w64(7, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084b346); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700084b34a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084b34e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084b352); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084b359); }
            c.w64(9, c.r64(2));
            c.w64(11, c.r64(8));
            blk = 2; continue;
        }
        1 => {
            c.w64(9, c.r64(8));
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b36e); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b375); };
            c.cmp(t5, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b382); };
            c.w32(6, t6);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(0, c.r64(9));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084b39d, 0x700084b398);
        }
        7 => {
            c.w32(11, 0xfeu64);
            c.w64(8, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084b3a6); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b3aa); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084b3ae); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084b3b2); };
            c.w64(7, t10);
            blk = 1; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084b3b8); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            c.w32(11, 0x1eu64);
            c.w64(8, c.r64(9));
            blk = 1; continue;
        }
        9 => {
            c.w32(7, c.r32(11));
            let r13 = c.r32(7) | 0x2u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            let r14 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r14);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b3db); };
            c.w64(7, t15);
            c.w32(8, c.r32(11));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700084b3e2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b3ea); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b3f1); };
            c.cmp(t18, c.r32(11), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b3fa); };
            c.w32(11, t19);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084b40d); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084b419, 0x700084b414);
        }
        15 => {
            c.w32(12, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b41f); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084b423); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b427); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b42b); };
            c.w64(9, t23);
            blk = 20; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084b431); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            c.w32(12, 0x1eu64);
            blk = 20; continue;
        }
        17 => {
            c.w32(12, 0x40u64);
            blk = 20; continue;
        }
        18 => {
            c.w32(7, 0xeu64);
            c.w64(12, c.r64(7));
            blk = 20; continue;
        }
        19 => {
            c.w32(12, 0x2u64);
            blk = 20; continue;
        }
        20 => {
            let r26 = c.r32(8) | 0x40u64;
            c.w32(8, r26);
            c.logic(r26, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(9));
            blk = 33; continue;
        }
        21 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700084b466); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(9, c.r32(9));
            c.w32(7, c.r32(11));
            let r27 = c.r32(7) | 0xeu64;
            c.w32(7, r27);
            c.logic(r27, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b487); };
            c.w64(8, t28);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084b48b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700084b48f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084b493); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b49e); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b4a5); };
            c.cmp(t31, c.r32(12), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b4b2); };
            c.w32(12, t32);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084b4ce, 0x700084b4c9);
        }
        29 => {
            c.w32(11, 0xfeu64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084b4d4); };
            c.w64(8, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b4d8); };
            c.w64(1, t34);
            blk = 24; continue;
        }
        30 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084b4de); };
            c.w32(0, t35);
            let r36 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r36);
            c.w32(11, 0x1eu64);
            blk = 24; continue;
        }
        31 => {
            c.w32(7, c.r32(11));
            let r37 = c.r32(7) | 0x2u64;
            c.w32(7, r37);
            c.logic(r37, 32);
            let r38 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r38);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(7, c.r32(11));
            let r39 = c.r32(7) | 0x40u64;
            c.w32(7, r39);
            c.logic(r39, 32);
            c.w64(12, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b502); };
            c.w64(7, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b506); };
            c.w64(2, t41);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(12));
            c.w32(7, c.r32(7));
            let r42 = c.r32(8) | c.r32(7);
            c.w32(8, r42);
            c.logic(r42, 32);
            let Some(t43) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b512); };
            c.w64(7, t43);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084b519); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084b51a); }
            c.w32(2, 0xcu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084b520); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084b521); }
            c.w64(6, c.r64(7));
            c.w32(0, 0x3u64);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084b52d); };
            c.w64(3, t44);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700084b53c, 0x700084b537);
        }
        34 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b53c); };
            c.w64(8, t45);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084b542); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(0, c.r32(0));
            c.w32(7, c.r32(11));
            let r46 = c.r32(7) | 0xeu64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084b561); };
            let r48 = c.r32(0) & t47;
            c.w32(0, r48);
            c.logic(r48, 32);
            c.w64(8, c.r64(0));
            let r49 = c.add(c.r32(8), c.r32(0), 32);
            c.w32(8, r49);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(12, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(12, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(7, c.r32(7));
            let Some(t50) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b583); };
            c.w32(8, t50);
            c.w32(9, c.r32(12));
            let r51 = c.r32(7) | c.r32(8);
            c.w32(7, r51);
            c.logic(r51, 32);
            let r52 = c.r32(7) | c.r32(9);
            c.w32(7, r52);
            c.logic(r52, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b593); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b597); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084b59b); };
            c.w64(7, t55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084b59f); };
            c.w32(9, t56);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r57 = c.r32(8) | c.r32(11);
            c.w32(8, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084b5b5); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084b5ba); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084b5c4); }
            return c.ret(0x0, 0x700084b5c5);
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084b5c6); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700084b5cd); };
            c.w64(9, t59);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084b5d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700084b5d8); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084b5dc); };
            c.w64(12, t60);
            let Some(t61) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084b5e3); };
            c.cmp(t61, c.r64(11), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084b5f3, 0x700084b5ee);
        }
        48 => {
            c.w64(8, c.r64(0));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b5f6); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b5fa); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084b5fe); };
            c.w64(0, t64);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700084b604); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 50; continue;
        }
        50 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b60b); };
            c.w64(9, t65);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700084b612); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084b61e); }
            c.w64(0, c.r64(8));
            blk = 39; continue;
        }
        51 => {
            let r66 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r66);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084b630); }
            return c.call(0x70008708b4, 0x700084b63a, 0x700084b635);
        }
        52 => {
            let Some(t67) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084b63a); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r68);
            blk = 36; continue;
        }
        53 => {
            let r69 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r69);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084b64a); }
            return c.call(0x70008708b4, 0x700084b654, 0x700084b64f);
        }
        54 => {
            let Some(t70) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084b654); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r71);
            blk = 22; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084b664);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084b674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084b674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084b678); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084b67e); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b682); };
            c.w64(7, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084b686); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700084b68a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084b68e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084b692); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084b699); }
            c.w64(9, c.r64(2));
            c.w64(11, c.r64(8));
            blk = 2; continue;
        }
        1 => {
            c.w64(9, c.r64(8));
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b6ae); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b6b5); };
            c.cmp(t5, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b6c2); };
            c.w32(6, t6);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(0, c.r64(9));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084b6dd, 0x700084b6d8);
        }
        7 => {
            c.w32(11, 0xfeu64);
            c.w64(8, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084b6e6); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b6ea); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084b6ee); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084b6f2); };
            c.w64(7, t10);
            blk = 1; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084b6f8); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            c.w32(11, 0x1eu64);
            c.w64(8, c.r64(9));
            blk = 1; continue;
        }
        9 => {
            c.w32(7, c.r32(11));
            let r13 = c.r32(7) | 0x2u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            let r14 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r14);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b71b); };
            c.w64(7, t15);
            c.w32(8, c.r32(11));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700084b722); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b72a); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b731); };
            c.cmp(t18, c.r32(11), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b73a); };
            c.w32(11, t19);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084b74d); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084b759, 0x700084b754);
        }
        15 => {
            c.w32(7, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b75e); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084b762); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b766); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b76a); };
            c.w64(9, t23);
            blk = 20; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084b770); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            c.w32(7, 0x1eu64);
            blk = 20; continue;
        }
        17 => {
            c.w32(7, 0x40u64);
            c.w64(12, c.r64(7));
            blk = 20; continue;
        }
        18 => {
            c.w32(7, 0xeu64);
            blk = 20; continue;
        }
        19 => {
            c.w32(7, 0x2u64);
            blk = 20; continue;
        }
        20 => {
            let r26 = c.r32(8) | 0x40u64;
            c.w32(8, r26);
            c.logic(r26, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(2, c.r64(9));
            blk = 33; continue;
        }
        21 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700084b7a8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(9, c.r32(9));
            c.w32(7, c.r32(11));
            let r27 = c.r32(7) | 0xeu64;
            c.w32(7, r27);
            c.logic(r27, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b7c9); };
            c.w64(8, t28);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084b7cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700084b7d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084b7d5); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b7e0); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b7e7); };
            c.cmp(t31, c.r32(12), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084b7f4); };
            c.w32(12, t32);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084b810, 0x700084b80b);
        }
        29 => {
            c.w32(11, 0xfeu64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084b816); };
            c.w64(8, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b81a); };
            c.w64(1, t34);
            blk = 24; continue;
        }
        30 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084b820); };
            c.w32(0, t35);
            let r36 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r36);
            c.w32(11, 0x1eu64);
            blk = 24; continue;
        }
        31 => {
            c.w32(7, c.r32(11));
            let r37 = c.r32(7) | 0x2u64;
            c.w32(7, r37);
            c.logic(r37, 32);
            let r38 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r38);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(7, c.r32(11));
            let r39 = c.r32(7) | 0x40u64;
            c.w32(7, r39);
            c.logic(r39, 32);
            c.w64(12, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b844); };
            c.w64(7, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b848); };
            c.w64(2, t41);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(12));
            c.w32(7, c.r32(7));
            let r42 = c.r32(8) | c.r32(7);
            c.w32(8, r42);
            c.logic(r42, 32);
            let Some(t43) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b854); };
            c.w64(7, t43);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084b85b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084b85c); }
            c.w32(9, 0xeu64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700084b863); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084b865); }
            c.w64(6, c.r64(7));
            c.w32(0, 0x3u64);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084b871); };
            c.w64(3, t44);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700084b880, 0x700084b87b);
        }
        34 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b880); };
            c.w64(8, t45);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084b886); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(0, c.r32(0));
            c.w32(7, c.r32(11));
            let r46 = c.r32(7) | 0xeu64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084b8a5); };
            let r48 = c.r32(0) | t47;
            c.w32(0, r48);
            c.logic(r48, 32);
            c.w64(8, c.r64(0));
            let r49 = c.add(c.r32(8), c.r32(0), 32);
            c.w32(8, r49);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(8, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(8, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(7, c.r32(7));
            let Some(t50) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084b8c7); };
            c.w32(9, t50);
            c.w32(8, c.r32(8));
            let r51 = c.r32(7) | c.r32(9);
            c.w32(7, r51);
            c.logic(r51, 32);
            let r52 = c.r32(7) | c.r32(8);
            c.w32(7, r52);
            c.logic(r52, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b8d7); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b8db); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084b8df); };
            c.w64(7, t55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084b8e3); };
            c.w32(9, t56);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r57 = c.r32(8) | c.r32(11);
            c.w32(8, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084b8f9); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084b8fe); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084b908); }
            return c.ret(0x0, 0x700084b909);
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084b90a); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700084b911); };
            c.w64(9, t59);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084b918); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700084b91c); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084b920); };
            c.w64(12, t60);
            let Some(t61) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084b927); };
            c.cmp(t61, c.r64(11), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084b937, 0x700084b932);
        }
        48 => {
            c.w64(8, c.r64(0));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084b93a); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084b93e); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084b942); };
            c.w64(0, t64);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700084b948); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 50; continue;
        }
        50 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b94f); };
            c.w64(9, t65);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700084b956); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084b962); }
            c.w64(0, c.r64(8));
            blk = 39; continue;
        }
        51 => {
            let r66 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r66);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084b974); }
            return c.call(0x70008708b4, 0x700084b97e, 0x700084b979);
        }
        52 => {
            let Some(t67) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084b97e); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r68);
            blk = 36; continue;
        }
        53 => {
            let r69 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r69);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084b98e); }
            return c.call(0x70008708b4, 0x700084b998, 0x700084b993);
        }
        54 => {
            let Some(t70) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084b998); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r71);
            blk = 22; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084b9a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084b9b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084b9b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084b9b8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084b9be); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084b9c2); };
            c.w64(7, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084b9c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700084b9ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084b9ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084b9d2); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084b9d9); }
            c.w64(9, c.r64(2));
            c.w64(11, c.r64(8));
            blk = 2; continue;
        }
        1 => {
            c.w64(9, c.r64(8));
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084b9ee); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084b9f5); };
            c.cmp(t5, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ba02); };
            c.w32(6, t6);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(0, c.r64(9));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084ba1d, 0x700084ba18);
        }
        7 => {
            c.w32(11, 0xfeu64);
            c.w64(8, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ba26); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ba2a); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084ba2e); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084ba32); };
            c.w64(7, t10);
            blk = 1; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084ba38); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            c.w32(11, 0x1eu64);
            c.w64(8, c.r64(9));
            blk = 1; continue;
        }
        9 => {
            c.w32(7, c.r32(11));
            let r13 = c.r32(7) | 0x2u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            let r14 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r14);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084ba5b); };
            c.w64(7, t15);
            c.w32(8, c.r32(11));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700084ba62); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ba6a); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ba71); };
            c.cmp(t18, c.r32(11), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ba7a); };
            c.w32(11, t19);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084ba8d); }
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084ba99, 0x700084ba94);
        }
        15 => {
            c.w32(12, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ba9f); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084baa3); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084baa7); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084baab); };
            c.w64(9, t23);
            blk = 20; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084bab1); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            c.w32(12, 0x1eu64);
            blk = 20; continue;
        }
        17 => {
            c.w32(12, 0x40u64);
            blk = 20; continue;
        }
        18 => {
            c.w32(7, 0xeu64);
            c.w64(12, c.r64(7));
            blk = 20; continue;
        }
        19 => {
            c.w32(12, 0x2u64);
            blk = 20; continue;
        }
        20 => {
            let r26 = c.r32(8) | 0x40u64;
            c.w32(8, r26);
            c.logic(r26, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(9));
            blk = 33; continue;
        }
        21 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700084bae6); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(9, c.r32(9));
            c.w32(7, c.r32(11));
            let r27 = c.r32(7) | 0xeu64;
            c.w32(7, r27);
            c.logic(r27, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084bb07); };
            c.w64(8, t28);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084bb0b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700084bb0f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084bb13); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084bb1e); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084bb25); };
            c.cmp(t31, c.r32(12), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084bb32); };
            c.w32(12, t32);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084bb4e, 0x700084bb49);
        }
        29 => {
            c.w32(11, 0xfeu64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084bb54); };
            c.w64(8, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084bb58); };
            c.w64(1, t34);
            blk = 24; continue;
        }
        30 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084bb5e); };
            c.w32(0, t35);
            let r36 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r36);
            c.w32(11, 0x1eu64);
            blk = 24; continue;
        }
        31 => {
            c.w32(7, c.r32(11));
            let r37 = c.r32(7) | 0x2u64;
            c.w32(7, r37);
            c.logic(r37, 32);
            let r38 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r38);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(7, c.r32(11));
            let r39 = c.r32(7) | 0x40u64;
            c.w32(7, r39);
            c.logic(r39, 32);
            c.w64(12, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084bb82); };
            c.w64(7, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084bb86); };
            c.w64(2, t41);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(12));
            c.w32(7, c.r32(7));
            let r42 = c.r32(8) | c.r32(7);
            c.w32(8, r42);
            c.logic(r42, 32);
            let Some(t43) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084bb92); };
            c.w64(7, t43);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084bb99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084bb9a); }
            c.w32(9, 0x10u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700084bba1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084bba3); }
            c.w64(6, c.r64(7));
            c.w32(0, 0x3u64);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084bbaf); };
            c.w64(3, t44);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700084bbbe, 0x700084bbb9);
        }
        34 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084bbbe); };
            c.w64(8, t45);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084bbc4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(0, c.r32(0));
            c.w32(7, c.r32(11));
            let r46 = c.r32(7) | 0xeu64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084bbe3); };
            let r48 = c.r32(0) ^ t47;
            c.w32(0, r48);
            c.logic(r48, 32);
            c.w64(8, c.r64(0));
            let r49 = c.add(c.r32(8), c.r32(0), 32);
            c.w32(8, r49);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(12, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(12, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(7, c.r32(7));
            let Some(t50) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084bc05); };
            c.w32(8, t50);
            c.w32(9, c.r32(12));
            let r51 = c.r32(7) | c.r32(8);
            c.w32(7, r51);
            c.logic(r51, 32);
            let r52 = c.r32(7) | c.r32(9);
            c.w32(7, r52);
            c.logic(r52, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084bc15); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084bc19); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084bc1d); };
            c.w64(7, t55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084bc21); };
            c.w32(9, t56);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r57 = c.r32(8) | c.r32(11);
            c.w32(8, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084bc37); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084bc3c); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084bc46); }
            return c.ret(0x0, 0x700084bc47);
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084bc48); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700084bc4f); };
            c.w64(9, t59);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084bc56); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700084bc5a); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084bc5e); };
            c.w64(12, t60);
            let Some(t61) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084bc65); };
            c.cmp(t61, c.r64(11), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084bc75, 0x700084bc70);
        }
        48 => {
            c.w64(8, c.r64(0));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084bc78); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084bc7c); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084bc80); };
            c.w64(0, t64);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700084bc86); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 50; continue;
        }
        50 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084bc8d); };
            c.w64(9, t65);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700084bc94); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084bca0); }
            c.w64(0, c.r64(8));
            blk = 39; continue;
        }
        51 => {
            let r66 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r66);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084bcb2); }
            return c.call(0x70008708b4, 0x700084bcbc, 0x700084bcb7);
        }
        52 => {
            let Some(t67) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084bcbc); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r68);
            blk = 36; continue;
        }
        53 => {
            let r69 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r69);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084bccc); }
            return c.call(0x70008708b4, 0x700084bcd6, 0x700084bcd1);
        }
        54 => {
            let Some(t70) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084bcd6); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r71);
            blk = 22; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084bce4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084bcf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084bcf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084bcf8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084bcfe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084bd02); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084bd06); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700084bd0a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084bd0e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700084bd12); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084bd19); }
            c.w64(11, c.r64(2));
            c.w64(12, c.r64(9));
            blk = 2; continue;
        }
        1 => {
            c.w64(11, c.r64(9));
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084bd2e); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084bd35); };
            c.cmp(t5, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084bd42); };
            c.w32(6, t6);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084bd5d, 0x700084bd58);
        }
        7 => {
            c.w32(12, 0xfeu64);
            c.w64(9, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084bd66); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084bd6a); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084bd6e); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084bd72); };
            c.w64(8, t10);
            blk = 1; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084bd78); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            c.w32(12, 0x1eu64);
            c.w64(9, c.r64(11));
            blk = 1; continue;
        }
        9 => {
            c.w32(8, c.r32(12));
            let r13 = c.r32(8) | 0x2u64;
            c.w32(8, r13);
            c.logic(r13, 32);
            let r14 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r14);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084bd9c); };
            c.w64(8, t15);
            c.w32(9, c.r32(12));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700084bda3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084bdaf); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084bdb6); };
            c.cmp(t18, c.r32(12), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084bdbf); };
            c.w32(12, t19);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700084bdd3); }
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084bddf, 0x700084bdda);
        }
        15 => {
            c.w32(15, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084bde5); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084bde9); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084bded); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084bdf1); };
            c.w64(11, t23);
            blk = 20; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084bdf7); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            c.w32(15, 0x1eu64);
            blk = 20; continue;
        }
        17 => {
            c.w32(15, 0x40u64);
            blk = 20; continue;
        }
        18 => {
            c.w32(8, 0xeu64);
            c.w64(15, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            c.w32(15, 0x2u64);
            blk = 20; continue;
        }
        20 => {
            let r26 = c.r32(9) | 0x40u64;
            c.w32(9, r26);
            c.logic(r26, 32);
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(11));
            blk = 33; continue;
        }
        21 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700084be2d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(11, c.r32(11));
            c.w32(8, c.r32(12));
            let r27 = c.r32(8) | 0xeu64;
            c.w32(8, r27);
            c.logic(r27, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084be4f); };
            c.w64(9, t28);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084be53); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700084be57); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700084be5b); }
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084be66); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084be6d); };
            c.cmp(t31, c.r32(15), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084be7a); };
            c.w32(15, t32);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(6, c.r64(9));
            return c.call(0x70008a10d4, 0x700084be95, 0x700084be90);
        }
        29 => {
            c.w32(12, 0xfeu64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084be9b); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084be9f); };
            c.w64(7, t34);
            blk = 24; continue;
        }
        30 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084bea5); };
            c.w32(0, t35);
            let r36 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r36);
            c.w32(12, 0x1eu64);
            blk = 24; continue;
        }
        31 => {
            c.w32(8, c.r32(12));
            let r37 = c.r32(8) | 0x2u64;
            c.w32(8, r37);
            c.logic(r37, 32);
            let r38 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r38);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(8, c.r32(12));
            let r39 = c.r32(8) | 0x40u64;
            c.w32(8, r39);
            c.logic(r39, 32);
            c.w64(15, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084becb); };
            c.w64(8, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084becf); };
            c.w64(2, t41);
            blk = 33; continue;
        }
        33 => {
            c.w32(9, c.r32(15));
            c.w32(8, c.r32(8));
            let r42 = c.r32(9) | c.r32(8);
            c.w32(9, r42);
            c.logic(r42, 32);
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084bedc); };
            c.w64(8, t43);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084bee3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084bee4); }
            c.w32(11, 0x12u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700084beeb); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084beed); }
            c.w64(6, c.r64(8));
            c.w32(0, 0x3u64);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084bef9); };
            c.w64(3, t44);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700084bf08, 0x700084bf03);
        }
        34 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084bf08); };
            c.w64(9, t45);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084bf0e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(12));
            let r46 = c.r32(8) | 0xeu64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            c.w64(1, c.r64(0));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084bf31); };
            c.w64(9, t47);
            let r48 = c.shl(c.r32(9), c.r8(1), 32).unwrap_or(c.r32(9));
            c.w32(9, r48);
            c.w64(11, c.r64(9));
            let r49 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r49);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(15, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(15, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(8, c.r32(8));
            let Some(t50) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084bf58); };
            c.w32(9, t50);
            c.w32(11, c.r32(15));
            let r51 = c.r32(8) | c.r32(9);
            c.w32(8, r51);
            c.logic(r51, 32);
            let r52 = c.r32(8) | c.r32(11);
            c.w32(8, r52);
            c.logic(r52, 32);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084bf68); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084bf6c); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084bf70); };
            c.w64(8, t55);
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084bf74); };
            c.w32(11, t56);
            c.w32(9, c.r32(9));
            c.w32(12, c.r32(11));
            let r57 = c.r32(9) | c.r32(12);
            c.w32(9, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700084bf8a); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084bf8f); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084bf99); }
            return c.ret(0x0, 0x700084bf9a);
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084bf9b); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084bfa2); };
            c.w64(12, t59);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700084bfaa); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700084bfae); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084bfb2); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700084bfb9); };
            c.cmp(t61, c.r64(15), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084bfc8, 0x700084bfc3);
        }
        48 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084bfc8); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084bfcc); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084bfd0); };
            c.w64(8, t64);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700084bfd6); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 50; continue;
        }
        50 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084bfe1); };
            c.w64(11, t65);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700084bfe8); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084bff5); }
            blk = 39; continue;
        }
        51 => {
            let r66 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r66);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084c003); }
            return c.call(0x70008708b4, 0x700084c00d, 0x700084c008);
        }
        52 => {
            let Some(t67) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084c00d); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r68);
            blk = 36; continue;
        }
        53 => {
            let r69 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r69);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084c01d); }
            return c.call(0x70008708b4, 0x700084c027, 0x700084c022);
        }
        54 => {
            let Some(t70) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084c027); };
            c.w32(11, t70);
            let r71 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r71);
            blk = 22; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084c038);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084c054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084c054); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084c058); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084c05e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084c062); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084c066); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700084c06a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084c06e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700084c072); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084c079); }
            c.w64(11, c.r64(2));
            c.w64(12, c.r64(9));
            blk = 2; continue;
        }
        1 => {
            c.w64(11, c.r64(9));
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c08e); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c095); };
            c.cmp(t5, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c0a2); };
            c.w32(6, t6);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084c0bd, 0x700084c0b8);
        }
        7 => {
            c.w32(12, 0xfeu64);
            c.w64(9, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084c0c6); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c0ca); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084c0ce); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084c0d2); };
            c.w64(8, t10);
            blk = 1; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084c0d8); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            c.w32(12, 0x1eu64);
            c.w64(9, c.r64(11));
            blk = 1; continue;
        }
        9 => {
            c.w32(8, c.r32(12));
            let r13 = c.r32(8) | 0x2u64;
            c.w32(8, r13);
            c.logic(r13, 32);
            let r14 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r14);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084c0fc); };
            c.w64(8, t15);
            c.w32(9, c.r32(12));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700084c103); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c10f); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c116); };
            c.cmp(t18, c.r32(12), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c11f); };
            c.w32(12, t19);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700084c133); }
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084c13f, 0x700084c13a);
        }
        15 => {
            c.w32(15, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084c145); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084c149); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c14d); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c151); };
            c.w64(11, t23);
            blk = 20; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084c157); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            c.w32(15, 0x1eu64);
            blk = 20; continue;
        }
        17 => {
            c.w32(15, 0x40u64);
            blk = 20; continue;
        }
        18 => {
            c.w32(8, 0xeu64);
            c.w64(15, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            c.w32(15, 0x2u64);
            blk = 20; continue;
        }
        20 => {
            let r26 = c.r32(9) | 0x40u64;
            c.w32(9, r26);
            c.logic(r26, 32);
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(11));
            blk = 33; continue;
        }
        21 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c18d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(11, c.r32(11));
            c.w32(8, c.r32(12));
            let r27 = c.r32(8) | 0xeu64;
            c.w32(8, r27);
            c.logic(r27, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084c1af); };
            c.w64(9, t28);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084c1b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700084c1b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700084c1bb); }
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c1c6); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c1cd); };
            c.cmp(t31, c.r32(15), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c1da); };
            c.w32(15, t32);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(6, c.r64(9));
            return c.call(0x70008a10d4, 0x700084c1f5, 0x700084c1f0);
        }
        29 => {
            c.w32(12, 0xfeu64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084c1fb); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c1ff); };
            c.w64(7, t34);
            blk = 24; continue;
        }
        30 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084c205); };
            c.w32(0, t35);
            let r36 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r36);
            c.w32(12, 0x1eu64);
            blk = 24; continue;
        }
        31 => {
            c.w32(8, c.r32(12));
            let r37 = c.r32(8) | 0x2u64;
            c.w32(8, r37);
            c.logic(r37, 32);
            let r38 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r38);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(8, c.r32(12));
            let r39 = c.r32(8) | 0x40u64;
            c.w32(8, r39);
            c.logic(r39, 32);
            c.w64(15, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c22b); };
            c.w64(8, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084c22f); };
            c.w64(2, t41);
            blk = 33; continue;
        }
        33 => {
            c.w32(9, c.r32(15));
            c.w32(8, c.r32(8));
            let r42 = c.r32(9) | c.r32(8);
            c.w32(9, r42);
            c.logic(r42, 32);
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084c23c); };
            c.w64(8, t43);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084c243); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084c244); }
            c.w32(11, 0x14u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700084c24b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084c24d); }
            c.w64(6, c.r64(8));
            c.w32(0, 0x3u64);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084c259); };
            c.w64(3, t44);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700084c268, 0x700084c263);
        }
        34 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c268); };
            c.w64(9, t45);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c26e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(12));
            let r46 = c.r32(8) | 0xeu64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            c.w64(1, c.r64(0));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084c291); };
            c.w64(9, t47);
            let r48 = c.sar(c.r32(9), c.r8(1), 32).unwrap_or(c.r32(9));
            c.w32(9, r48);
            c.w64(11, c.r64(9));
            let r49 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r49);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(15, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(15, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(8, c.r32(8));
            let Some(t50) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c2b8); };
            c.w32(9, t50);
            c.w32(11, c.r32(15));
            let r51 = c.r32(8) | c.r32(9);
            c.w32(8, r51);
            c.logic(r51, 32);
            let r52 = c.r32(8) | c.r32(11);
            c.w32(8, r52);
            c.logic(r52, 32);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c2c8); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084c2cc); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084c2d0); };
            c.w64(8, t55);
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084c2d4); };
            c.w32(11, t56);
            c.w32(9, c.r32(9));
            c.w32(12, c.r32(11));
            let r57 = c.r32(9) | c.r32(12);
            c.w32(9, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700084c2ea); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084c2ef); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084c2f9); }
            return c.ret(0x0, 0x700084c2fa);
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084c2fb); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084c302); };
            c.w64(12, t59);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700084c30a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700084c30e); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084c312); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700084c319); };
            c.cmp(t61, c.r64(15), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084c328, 0x700084c323);
        }
        48 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084c328); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c32c); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084c330); };
            c.w64(8, t64);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700084c336); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 50; continue;
        }
        50 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c341); };
            c.w64(11, t65);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700084c348); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084c355); }
            blk = 39; continue;
        }
        51 => {
            let r66 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r66);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084c363); }
            return c.call(0x70008708b4, 0x700084c36d, 0x700084c368);
        }
        52 => {
            let Some(t67) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084c36d); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r68);
            blk = 36; continue;
        }
        53 => {
            let r69 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r69);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084c37d); }
            return c.call(0x70008708b4, 0x700084c387, 0x700084c382);
        }
        54 => {
            let Some(t70) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084c387); };
            c.w32(11, t70);
            let r71 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r71);
            blk = 22; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084c398);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084c3b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084c3b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084c3b8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084c3be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084c3c2); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084c3c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700084c3ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084c3ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700084c3d2); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084c3d9); }
            c.w64(11, c.r64(2));
            c.w64(12, c.r64(9));
            blk = 2; continue;
        }
        1 => {
            c.w64(11, c.r64(9));
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c3ee); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c3f5); };
            c.cmp(t5, c.r32(6), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c402); };
            c.w32(6, t6);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084c41d, 0x700084c418);
        }
        7 => {
            c.w32(12, 0xfeu64);
            c.w64(9, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084c426); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c42a); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084c42e); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084c432); };
            c.w64(8, t10);
            blk = 1; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084c438); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            c.w32(12, 0x1eu64);
            c.w64(9, c.r64(11));
            blk = 1; continue;
        }
        9 => {
            c.w32(8, c.r32(12));
            let r13 = c.r32(8) | 0x2u64;
            c.w32(8, r13);
            c.logic(r13, 32);
            let r14 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r14);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084c45c); };
            c.w64(8, t15);
            c.w32(9, c.r32(12));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700084c463); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c46f); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c476); };
            c.cmp(t18, c.r32(12), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c47f); };
            c.w32(12, t19);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700084c493); }
            c.w64(6, c.r64(8));
            return c.call(0x70008a10d4, 0x700084c49f, 0x700084c49a);
        }
        15 => {
            c.w32(15, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084c4a5); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084c4a9); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c4ad); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c4b1); };
            c.w64(11, t23);
            blk = 20; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084c4b7); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            c.w32(15, 0x1eu64);
            blk = 20; continue;
        }
        17 => {
            c.w32(15, 0x40u64);
            blk = 20; continue;
        }
        18 => {
            c.w32(8, 0xeu64);
            c.w64(15, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            c.w32(15, 0x2u64);
            blk = 20; continue;
        }
        20 => {
            let r26 = c.r32(9) | 0x40u64;
            c.w32(9, r26);
            c.logic(r26, 32);
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(11));
            blk = 33; continue;
        }
        21 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c4ed); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(11, c.r32(11));
            c.w32(8, c.r32(12));
            let r27 = c.r32(8) | 0xeu64;
            c.w32(8, r27);
            c.logic(r27, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084c50f); };
            c.w64(9, t28);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084c513); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700084c517); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700084c51b); }
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c526); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c52d); };
            c.cmp(t31, c.r32(15), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c53a); };
            c.w32(15, t32);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(6, c.r64(9));
            return c.call(0x70008a10d4, 0x700084c555, 0x700084c550);
        }
        29 => {
            c.w32(12, 0xfeu64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084c55b); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c55f); };
            c.w64(7, t34);
            blk = 24; continue;
        }
        30 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084c565); };
            c.w32(0, t35);
            let r36 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r36);
            c.w32(12, 0x1eu64);
            blk = 24; continue;
        }
        31 => {
            c.w32(8, c.r32(12));
            let r37 = c.r32(8) | 0x2u64;
            c.w32(8, r37);
            c.logic(r37, 32);
            let r38 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r38);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(8, c.r32(12));
            let r39 = c.r32(8) | 0x40u64;
            c.w32(8, r39);
            c.logic(r39, 32);
            c.w64(15, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c58b); };
            c.w64(8, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084c58f); };
            c.w64(2, t41);
            blk = 33; continue;
        }
        33 => {
            c.w32(9, c.r32(15));
            c.w32(8, c.r32(8));
            let r42 = c.r32(9) | c.r32(8);
            c.w32(9, r42);
            c.logic(r42, 32);
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084c59c); };
            c.w64(8, t43);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084c5a3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084c5a4); }
            c.w32(11, 0x16u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700084c5ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084c5ad); }
            c.w64(6, c.r64(8));
            c.w32(0, 0x3u64);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084c5b9); };
            c.w64(3, t44);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700084c5c8, 0x700084c5c3);
        }
        34 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c5c8); };
            c.w64(9, t45);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c5ce); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(12));
            let r46 = c.r32(8) | 0xeu64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            c.w64(1, c.r64(0));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084c5f1); };
            c.w64(9, t47);
            let r48 = c.shr(c.r32(9), c.r8(1), 32).unwrap_or(c.r32(9));
            c.w32(9, r48);
            c.cmp(c.r32(9), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(15, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(15, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(8, c.r32(8));
            let Some(t49) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c61a); };
            c.w32(9, t49);
            c.w32(11, c.r32(15));
            let r50 = c.r32(8) | c.r32(9);
            c.w32(8, r50);
            c.logic(r50, 32);
            let r51 = c.r32(8) | c.r32(11);
            c.w32(8, r51);
            c.logic(r51, 32);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c62a); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084c62e); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084c632); };
            c.w64(8, t54);
            let Some(t55) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084c636); };
            c.w32(11, t55);
            c.w32(9, c.r32(9));
            c.w32(12, c.r32(11));
            let r56 = c.r32(9) | c.r32(12);
            c.w32(9, r56);
            c.logic(r56, 32);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700084c64c); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084c651); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084c65b); }
            return c.ret(0x0, 0x700084c65c);
        }
        46 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084c65d); };
            c.w64(11, t57);
            let Some(t58) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084c664); };
            c.w64(12, t58);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700084c66c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700084c670); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084c674); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700084c67b); };
            c.cmp(t60, c.r64(15), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084c68a, 0x700084c685);
        }
        48 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084c68a); };
            c.w64(9, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c68e); };
            c.w64(7, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084c692); };
            c.w64(8, t63);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700084c698); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 50; continue;
        }
        50 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c6a3); };
            c.w64(11, t64);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700084c6aa); }
            c.w32(10, c.r32(9));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084c6ba); }
            blk = 39; continue;
        }
        51 => {
            let r65 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084c6c8); }
            return c.call(0x70008708b4, 0x700084c6d2, 0x700084c6cd);
        }
        52 => {
            let Some(t66) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084c6d2); };
            c.w32(0, t66);
            let r67 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r67);
            blk = 36; continue;
        }
        53 => {
            let r68 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r68);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084c6e2); }
            return c.call(0x70008708b4, 0x700084c6ec, 0x700084c6e7);
        }
        54 => {
            let Some(t69) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084c6ec); };
            c.w32(11, t69);
            let r70 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r70);
            blk = 22; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084c6fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084c714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(7, c.r32(3));
            c.w32(8, c.r32(0));
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c732); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c735); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r2);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c740); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 16; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084c752); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084c756); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 34; continue;
        }
        5 => {
            let r4 = c.add(c.r32(7), c.r32(8), 32);
            c.w32(7, r4);
            if c.cc_o() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084c766); };
            c.w32(8, t5);
            c.w32(9, c.r32(8));
            let r6 = c.r32(9) | 0x2u64;
            c.w32(9, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084c77a); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084c77f); }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x700084c789);
        }
        9 => {
            let r7 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r7);
            let r8 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r8);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 16; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c7a0); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c7a6); };
            c.cmp(t11, c.r32(7), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c7b9); };
            c.w32(7, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c7bc); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c7c5); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c7ca); }
            blk = 16; continue;
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084c7d1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084c7d5); }
            let r14 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r14);
            blk = 34; continue;
        }
        15 => {
            let r15 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r15);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c7e2); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 16; continue;
        }
        16 => {
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084c7ef); };
            c.w32(7, t16);
            c.w32(8, c.r32(7));
            let r17 = c.r32(8) | 0xeu64;
            c.w32(8, r17);
            c.logic(r17, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084c802); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084c806); }
            blk = 18; continue;
        }
        18 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084c80d); };
            c.w64(1, t18);
            let Some(t19) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084c814); };
            c.w64(7, t19);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084c81b); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084c822); };
            c.cmp(t21, c.r64(8), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084c82b); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 20; continue;
        }
        20 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c836); };
            c.w64(7, t22);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084c83d); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084c840); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084c848);
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084c849); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084c84d); }
            let r23 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r23);
            let Some(t24) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c853); };
            c.w32(7, t24);
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c873); };
            c.w32(7, t25);
            let r26 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r26);
            let Some(t27) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c879); };
            c.cmp(t27, 0x41u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 26; continue;
        }
        26 => {
            blk = 35; continue;
        }
        27 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c886); };
            c.w32(7, t28);
            let r29 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r29);
            let Some(t30) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c88c); };
            c.cmp(t30, 0x40u64, 16);
            if c.cc_ae() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084c893); };
            c.w32(7, t31);
            c.w32(8, c.r32(7));
            let r32 = c.r32(8) | 0x20u64;
            c.w32(8, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084c8a6); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084c8aa); }
            blk = 30; continue;
        }
        30 => {
            return c.call(0x700086f9d4, 0x700084c8b6, 0x700084c8b1);
        }
        31 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084c8b9); }
            return c.ret(0x0, 0x700084c8ba);
        }
        32 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t33) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c8c1); };
            c.w32(7, t33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c8c4); };
            c.cmp(t34, c.r32(7), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c8cd); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            let Some(t37) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084c8d3); };
            c.cmp(t37, 0x43u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(7, 0xfeu64);
            blk = 37; continue;
        }
        36 => {
            c.w32(7, 0x1eu64);
            blk = 37; continue;
        }
        37 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084c8e6); };
            c.w32(8, t38);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(8));
            let r39 = c.r32(7) | c.r32(9);
            c.w32(7, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084c8fb); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084c8ff); }
            blk = 39; continue;
        }
        39 => {
            return c.call(0x70008ad794, 0x700084c90b, 0x700084c906);
        }
        40 => {
            blk = 31; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084c90d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084c911); }
            let r40 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r40);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x700084c91c); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700084c921); }
            return c.call(0x70007febf4, 0x700084c92b, 0x700084c926);
        }
        42 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700084c92e); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084c933); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084c93b); }
            blk = 20; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700084c941); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x700084c945); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700084c949); }
            return c.call(0x700089b334, 0x700084c952, 0x700084c94d);
        }
        44 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c956); };
            c.w64(2, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c95a); };
            c.w64(1, t42);
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084c95e); };
            c.w32(7, t43);
            c.w32(8, c.r32(7));
            let r44 = c.r32(8) | 0x40u64;
            c.w32(8, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084c971); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084c975); }
            blk = 47; continue;
        }
        47 => {
            blk = 31; continue;
        }
        48 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084c981); };
            c.w64(1, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084c985); };
            c.w64(7, t46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084c989); };
            c.w32(8, t47);
            c.w32(9, c.r32(8));
            let r48 = c.r32(9) | 0xfeu64;
            c.w32(9, r48);
            c.logic(r48, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084c9a0); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084c9a5); }
            blk = 50; continue;
        }
        50 => {
            c.w32(1, 0x180u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084c9b1); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700084c9b2); };
            c.w64(3, t49);
            c.w32(0, 0x1u64);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084c9be); };
            c.w64(6, t50);
            return c.call(0x700086ed74, 0x700084c9c7, 0x700084c9c2);
        }
        51 => {
            return RegfileExit::fault(0x700084c9c7);
        }
        52 => {
            return RegfileExit::jump(0x700084c9cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084c9d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(7, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(8, c.r32(3));
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084c9e6); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084c9e9); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r2);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084c9f4); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 19; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084ca06); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084ca0a); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 29; continue;
        }
        5 => {
            let r4 = c.sub(c.r32(7), c.r32(8), 32);
            c.w32(7, r4);
            if c.cc_o() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(8, 0x2u64);
            blk = 10; continue;
        }
        7 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084ca25); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084ca2c); };
            c.w64(8, t6);
            let r7 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r7);
            let r8 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r8);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084ca37); };
            c.w64(11, t9);
            let Some(t10) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084ca3e); };
            c.cmp(t10, c.r64(9), 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700084ca47); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 9; continue;
        }
        9 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ca62); };
            c.w64(8, t11);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700084ca69); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084ca6d); }
            c.w32(8, 0xeu64);
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084ca78); };
            c.w32(9, t12);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r13 = c.r32(8) | c.r32(11);
            c.w32(8, r13);
            c.logic(r13, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084ca8e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084ca93); }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700084ca9d);
        }
        13 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ca9e); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084caa4); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t17) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084cab6); };
            c.w32(7, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084cab9); };
            c.cmp(t18, c.r32(7), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084cac2); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084cac7); }
            blk = 19; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cace); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cad2); }
            let r19 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r19);
            blk = 29; continue;
        }
        18 => {
            let r20 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r20);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084cadf); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 19; continue;
        }
        19 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084caec); };
            c.w32(7, t21);
            c.w32(8, c.r32(7));
            let r22 = c.r32(8) | 0xeu64;
            c.w32(8, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084caff); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084cb03); }
            blk = 21; continue;
        }
        21 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084cb0a); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084cb11); };
            c.w64(7, t24);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084cb18); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084cb1f); };
            c.cmp(t26, c.r64(8), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084cb28); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 23; continue;
        }
        23 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084cb33); };
            c.w64(7, t27);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084cb3a); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084cb3d); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084cb45);
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cb46); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cb4a); }
            let r28 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r28);
            let Some(t29) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084cb50); };
            c.w32(7, t29);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t30) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084cb65); };
            c.w32(7, t30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084cb68); };
            c.cmp(t31, c.r32(7), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(7, 0x1eu64);
            blk = 35; continue;
        }
        29 => {
            let Some(t32) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084cb78); };
            c.w32(7, t32);
            let r33 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r33);
            let Some(t34) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084cb7e); };
            c.cmp(t34, 0x43u64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(7, 0x1eu64);
            blk = 35; continue;
        }
        31 => {
            c.w32(7, 0x1eu64);
            blk = 35; continue;
        }
        32 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084cb98); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            let Some(t37) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084cb9e); };
            c.cmp(t37, 0x41u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(7, 0xfeu64);
            blk = 35; continue;
        }
        35 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084cbae); };
            c.w32(8, t38);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(8));
            let r39 = c.r32(7) | c.r32(9);
            c.w32(7, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084cbc3); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084cbc7); }
            blk = 37; continue;
        }
        37 => {
            return c.call(0x70008adb14, 0x700084cbd3, 0x700084cbce);
        }
        38 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084cbd6); }
            return c.ret(0x0, 0x700084cbd7);
        }
        39 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cbd8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cbdc); }
            let r40 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r40);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x700084cbe2); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700084cbeb); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700084cbef); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700084cbf3); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700084cbff, 0x700084cbfa);
        }
        40 => {
            c.w64(7, c.r64(0));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084cc02); };
            c.w64(2, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084cc06); };
            c.w64(1, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084cc0a); };
            c.w64(0, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084cc0e); };
            c.w64(3, t44);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084cc15); }
            blk = 9; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cc1b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cc1f); }
            let r45 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r45);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x700084cc2a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700084cc2f); }
            return c.call(0x70007febf4, 0x700084cc39, 0x700084cc34);
        }
        42 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700084cc3c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084cc41); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084cc49); }
            blk = 23; continue;
        }
        43 => {
            let Some(t46) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084cc4f); };
            c.w32(7, t46);
            c.w32(8, c.r32(7));
            let r47 = c.r32(8) | 0x40u64;
            c.w32(8, r47);
            c.logic(r47, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700084cc5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x700084cc5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700084cc62); }
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084cc6e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084cc72); }
            blk = 45; continue;
        }
        45 => {
            return c.call(0x700089bcd4, 0x700084cc7e, 0x700084cc79);
        }
        46 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 38; continue;
        }
        48 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084cc87); };
            c.w64(2, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084cc8b); };
            c.w64(1, t49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084cc8f); };
            c.w32(7, t50);
            c.w32(8, c.r32(7));
            let r51 = c.r32(8) | 0xfeu64;
            c.w32(8, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084cca5); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084cca9); }
            blk = 50; continue;
        }
        50 => {
            c.w32(1, 0x180u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084ccb5); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700084ccb6); };
            c.w64(3, t52);
            c.w32(0, 0x1u64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ccc2); };
            c.w64(6, t53);
            return c.call(0x700086ed74, 0x700084cccb, 0x700084ccc6);
        }
        51 => {
            return RegfileExit::fault(0x700084cccb);
        }
        52 => {
            return RegfileExit::jump(0x700084ccd0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084ccd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084cce1); };
            c.w32(7, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084cce4); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r2);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084ccef); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 24; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cd01); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cd05); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 34; continue;
        }
        5 => {
            c.w64(7, c.r64(3));
            let r4 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r4);
            c.w64(8, c.r64(0));
            let r5 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r5);
            c.w64(9, c.r64(8));
            let r6 = c.imul2(c.r32(9), c.r32(7), 32);
            c.w32(9, r6);
            if c.cc_o() { blk = 49; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(7, c.r64(9));
            let r7 = c.add(c.r32(7), c.r32(9), 32);
            c.w32(7, r7);
            if c.cc_o() { blk = 44; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 12; continue;
        }
        9 => {
            let r8 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r8);
            let r9 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r9);
            let r10 = c.r32(3) | c.r32(0);
            c.w32(3, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700084cd4e); };
            c.w64(7, t11);
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(8, 0xeu64);
            blk = 15; continue;
        }
        14 => {
            c.w32(8, 0x2u64);
            blk = 15; continue;
        }
        15 => {
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084cd69); };
            c.w32(9, t12);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r13 = c.r32(8) | c.r32(11);
            c.w32(8, r13);
            c.logic(r13, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084cd7f); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084cd84); }
            blk = 17; continue;
        }
        17 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x700084cd8e);
        }
        18 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084cd8f); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084cd95); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084cda7); };
            c.w32(7, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084cdaa); };
            c.cmp(t18, c.r32(7), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084cdb3); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084cdb8); }
            blk = 24; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cdbf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cdc3); }
            let r19 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r19);
            blk = 34; continue;
        }
        23 => {
            let r20 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r20);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084cdd0); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084cddd); };
            c.w32(7, t21);
            c.w32(8, c.r32(7));
            let r22 = c.r32(8) | 0xeu64;
            c.w32(8, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084cdf0); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084cdf4); }
            blk = 26; continue;
        }
        26 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084cdfb); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084ce02); };
            c.w64(7, t24);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084ce09); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084ce10); };
            c.cmp(t26, c.r64(8), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084ce19); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ce24); };
            c.w64(7, t27);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084ce2b); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084ce2e); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084ce36);
        }
        29 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084ce37); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084ce3b); }
            let r28 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r28);
            let Some(t29) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ce41); };
            c.w32(7, t29);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 39; continue; }
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t30) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ce56); };
            c.w32(7, t30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ce59); };
            c.cmp(t31, c.r32(7), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        34 => {
            let Some(t32) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ce69); };
            c.w32(7, t32);
            let r33 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r33);
            let Some(t34) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ce6f); };
            c.cmp(t34, 0x43u64, 16);
            if c.cc_ne() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        36 => {
            c.w32(7, 0x1eu64);
            blk = 40; continue;
        }
        37 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ce89); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            let Some(t37) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ce8f); };
            c.cmp(t37, 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(7, 0xfeu64);
            blk = 40; continue;
        }
        40 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084ce9f); };
            c.w32(8, t38);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(8));
            let r39 = c.r32(7) | c.r32(9);
            c.w32(7, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084ceb4); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084ceb8); }
            blk = 42; continue;
        }
        42 => {
            return c.call(0x70008adcd4, 0x700084cec4, 0x700084cebf);
        }
        43 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084cec7); }
            return c.ret(0x0, 0x700084cec8);
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cec9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cecd); }
            let r40 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084ced3); };
            c.w64(7, t41);
            let Some(t42) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084ceda); };
            c.w64(8, t42);
            c.w64(11, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084cee1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084cee5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084cee9); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084ceed); };
            c.w64(12, t43);
            let Some(t44) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084cef4); };
            c.cmp(t44, c.r64(11), 64);
            if c.cc_a() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700084cf07, 0x700084cf02);
        }
        46 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084cf07); };
            c.w64(9, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084cf0b); };
            c.w64(2, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084cf0f); };
            c.w64(1, t47);
            blk = 48; continue;
        }
        47 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700084cf15); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 48; continue;
        }
        48 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084cf28); };
            c.w64(7, t48);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084cf2f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084cf32); }
            c.w64(7, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084cf3d); }
            blk = 12; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cf43); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cf47); }
            let r49 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r49);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084cf4d); };
            c.w64(9, t50);
            let Some(t51) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084cf54); };
            c.w64(11, t51);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084cf5b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084cf5f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084cf63); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084cf67); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084cf6b); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700084cf72); };
            c.cmp(t53, c.r64(12), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700084cf84, 0x700084cf7f);
        }
        51 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084cf84); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084cf88); };
            c.w64(2, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084cf8c); };
            c.w64(1, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084cf90); };
            c.w64(8, t57);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700084cf96); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 53; continue;
        }
        53 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084cfb5); };
            c.w64(7, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084cfbc); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084cfbf); }
            c.w64(7, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084cfca); }
            blk = 12; continue;
        }
        54 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084cfd0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084cfd4); }
            let r59 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r59);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700084cfdf); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700084cfe4); }
            return c.call(0x70007febf4, 0x700084cfee, 0x700084cfe9);
        }
        55 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084cff1); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084cff6); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084cffe); }
            blk = 28; continue;
        }
        56 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d004); };
            c.w32(7, t60);
            c.w32(8, c.r32(7));
            let r61 = c.r32(8) | 0x40u64;
            c.w32(8, r61);
            c.logic(r61, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084d017); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d01b); }
            blk = 58; continue;
        }
        58 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084d022); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084d023); }
            c.w32(1, 0x4u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084d029); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084d02a); };
            c.w64(3, t62);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084d03b, 0x700084d036);
        }
        59 => {
            blk = 43; continue;
        }
        60 => {
            return RegfileExit::jump(0x700084d044);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084d054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(2));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d064); };
            c.w32(8, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d068); };
            c.cmp(t1, c.r32(8), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r2);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d073); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 24; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d085); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d089); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 34; continue;
        }
        5 => {
            c.w64(8, c.r64(3));
            let r4 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r4);
            c.w64(9, c.r64(0));
            let r5 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r5);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(3), c.r32(11), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(8), 0xffffffffffffffffu64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(9), 0xc0000000u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(9));
            c.cdq();
            c.idiv(c.r32(8), 32);
            c.w64(11, c.r64(0));
            let r6 = c.imul2(c.r32(11), c.r32(8), 32);
            c.w32(11, r6);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(8, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(9, 0x2u64);
            blk = 15; continue;
        }
        12 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084d0e0); };
            c.w64(11, t7);
            let Some(t8) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084d0e7); };
            c.w64(12, t8);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084d0ef); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700084d0f6); };
            c.cmp(t10, c.r64(15), 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700084d0ff); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d120); };
            c.w64(8, t11);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700084d127); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084d12b); }
            c.w32(9, 0x6u64);
            c.w64(8, c.r64(11));
            blk = 15; continue;
        }
        15 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d13a); };
            c.w32(11, t12);
            c.w32(9, c.r32(9));
            c.w32(12, c.r32(11));
            let r13 = c.r32(9) | c.r32(12);
            c.w32(9, r13);
            c.logic(r13, 32);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700084d150); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d155); }
            blk = 17; continue;
        }
        17 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x700084d15f);
        }
        18 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d160); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d167); };
            c.cmp(t16, c.r32(8), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d179); };
            c.w32(8, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d17d); };
            c.cmp(t18, c.r32(8), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d186); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d18b); }
            blk = 24; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d192); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d196); }
            let r19 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r19);
            blk = 34; continue;
        }
        23 => {
            let r20 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r20);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d1a3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d1b0); };
            c.w32(8, t21);
            c.w32(9, c.r32(8));
            let r22 = c.r32(9) | 0xeu64;
            c.w32(9, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084d1c4); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d1c9); }
            blk = 26; continue;
        }
        26 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084d1d0); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084d1d7); };
            c.w64(7, t24);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084d1de); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084d1e5); };
            c.cmp(t26, c.r64(8), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084d1ee); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d1fc); };
            c.w64(7, t27);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084d203); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084d206); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084d20e);
        }
        29 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d20f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d213); }
            let r28 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r28);
            let Some(t29) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d219); };
            c.w32(8, t29);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 39; continue; }
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t30) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d231); };
            c.w32(8, t30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d235); };
            c.cmp(t31, c.r32(8), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(8, 0x1eu64);
            blk = 40; continue;
        }
        34 => {
            let Some(t32) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d246); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d24d); };
            c.cmp(t34, 0x43u64, 16);
            if c.cc_ne() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(8, 0x1eu64);
            blk = 40; continue;
        }
        36 => {
            c.w32(8, 0x1eu64);
            blk = 40; continue;
        }
        37 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d26a); };
            c.w32(8, t35);
            let r36 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r36);
            let Some(t37) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d271); };
            c.cmp(t37, 0x41u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(8, 0xfeu64);
            blk = 40; continue;
        }
        40 => {
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d283); };
            c.w32(9, t38);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r39 = c.r32(8) | c.r32(11);
            c.w32(8, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084d299); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d29e); }
            blk = 42; continue;
        }
        42 => {
            return c.call(0x70008adf94, 0x700084d2aa, 0x700084d2a5);
        }
        43 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084d2ad); }
            return c.ret(0x0, 0x700084d2ae);
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d2af); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d2b3); }
            let r40 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r40);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700084d2be); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084d2c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700084d2c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700084d2ca); }
            return c.call(0x70007febf4, 0x700084d2d3, 0x700084d2ce);
        }
        45 => {
            c.w64(11, c.r64(0));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084d2d6); };
            c.w64(7, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084d2da); };
            c.w64(1, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084d2de); };
            c.w64(9, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084d2e2); };
            c.w64(8, t44);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084d2e9); }
            blk = 14; continue;
        }
        46 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d2ef); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d2f3); }
            let r45 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r45);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700084d2fe); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700084d303); }
            return c.call(0x70007febf4, 0x700084d30d, 0x700084d308);
        }
        47 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084d310); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084d315); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084d31d); }
            blk = 28; continue;
        }
        48 => {
            let Some(t46) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d323); };
            c.w32(8, t46);
            c.w32(9, c.r32(8));
            let r47 = c.r32(9) | 0x40u64;
            c.w32(9, r47);
            c.logic(r47, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084d337); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d33c); }
            blk = 50; continue;
        }
        50 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084d343); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084d344); }
            c.w32(1, 0x6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084d34a); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084d34b); };
            c.w64(3, t48);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084d35c, 0x700084d357);
        }
        51 => {
            blk = 43; continue;
        }
        52 => {
            return RegfileExit::jump(0x700084d364);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084d374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(2));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d384); };
            c.w32(8, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d388); };
            c.cmp(t1, c.r32(8), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r2 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r2);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d393); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 20; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d3a5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d3a9); }
            let r3 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r3);
            blk = 32; continue;
        }
        5 => {
            let r4 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r4);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 6; continue;
        }
        6 => {
            let r5 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r5);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 42; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cdq();
            c.idiv(c.r32(3), 32);
            c.w32(8, c.r64(2).wrapping_add(c.r64(2)));
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(9, 0xeu64);
            blk = 11; continue;
        }
        10 => {
            c.w32(9, 0x2u64);
            blk = 11; continue;
        }
        11 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d3e5); };
            c.w32(11, t6);
            c.w32(9, c.r32(9));
            c.w32(12, c.r32(11));
            let r7 = c.r32(9) | c.r32(12);
            c.w32(9, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x700084d3fb); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d400); }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x700084d40a);
        }
        14 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d40b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d412); };
            c.cmp(t10, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d424); };
            c.w32(8, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d428); };
            c.cmp(t12, c.r32(8), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d431); }
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d436); }
            blk = 20; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d43d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d441); }
            let r13 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r13);
            blk = 32; continue;
        }
        19 => {
            let r14 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r14);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d44e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 20; continue;
        }
        20 => {
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d45b); };
            c.w32(8, t15);
            c.w32(9, c.r32(8));
            let r16 = c.r32(9) | 0xeu64;
            c.w32(9, r16);
            c.logic(r16, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084d46f); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d474); }
            blk = 22; continue;
        }
        22 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084d47b); };
            c.w64(1, t17);
            let Some(t18) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084d482); };
            c.w64(7, t18);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084d489); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084d490); };
            c.cmp(t20, c.r64(8), 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084d499); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 24; continue;
        }
        24 => {
            let r21 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r21);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084d4a4); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700084d4a9); }
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x700084d4ac); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700084d4b1); }
            blk = 25; continue;
        }
        25 => {
            c.fprem();
            c.fnstsw_ax();
            let r22 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r22);
            let r23 = c.r32(0) & 0xffu64;
            c.w32(0, r23);
            c.logic(r23, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084d4c0); }
            if !c.popfq() { return RegfileExit::fault(0x700084d4c1); }
            if c.cc_p() { blk = 25; continue; }
            blk = 26; continue;
        }
        26 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x700084d4c6); }
            if !c.movsd_rm(1, c.r64(4)) { return RegfileExit::fault(0x700084d4c9); }
            let r24 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r24);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d4d2); };
            c.w64(7, t25);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084d4d9); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700084d4dc); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700084d4e4);
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d4e5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d4e9); }
            let r26 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r26);
            let Some(t27) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d4ef); };
            c.w32(8, t27);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d507); };
            c.w32(8, t28);
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d50b); };
            c.cmp(t29, c.r32(8), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(8, 0x1eu64);
            blk = 38; continue;
        }
        32 => {
            let Some(t30) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d51c); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d523); };
            c.cmp(t32, 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(8, 0x1eu64);
            blk = 38; continue;
        }
        34 => {
            c.w32(8, 0x1eu64);
            blk = 38; continue;
        }
        35 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t33) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d540); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            let Some(t35) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d547); };
            c.cmp(t35, 0x41u64, 16);
            if c.cc_e() { blk = 55; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(8, 0xfeu64);
            blk = 38; continue;
        }
        38 => {
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d559); };
            c.w32(9, t36);
            c.w32(8, c.r32(8));
            c.w32(11, c.r32(9));
            let r37 = c.r32(8) | c.r32(11);
            c.w32(8, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084d56f); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d574); }
            blk = 40; continue;
        }
        40 => {
            return c.call(0x70008ae1b4, 0x700084d580, 0x700084d57b);
        }
        41 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084d583); }
            return c.ret(0x0, 0x700084d584);
        }
        42 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d585); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d589); }
            let r38 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r38);
            c.cdq();
            c.idiv(c.r32(3), 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700084d592); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084d596); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700084d59a); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(8, c.r64(2));
            let r39 = c.add(c.r32(8), c.r32(2), 32);
            c.w32(8, r39);
            if c.cc_o() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 50; continue;
        }
        45 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084d5b7); };
            c.w64(8, t40);
            let Some(t41) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700084d5be); };
            c.w64(9, t41);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084d5c5); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084d5cc); };
            c.cmp(t43, c.r64(11), 64);
            if c.cc_a() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700084d5df, 0x700084d5da);
        }
        47 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084d5df); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084d5e3); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084d5e7); };
            c.w64(1, t46);
            blk = 49; continue;
        }
        48 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700084d5ed); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 49; continue;
        }
        49 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d5ff); };
            c.w64(8, t47);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700084d606); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084d60a); }
            blk = 50; continue;
        }
        50 => {
            c.w64(8, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084d615); }
            blk = 8; continue;
        }
        51 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700084d61b); };
            c.w64(8, t48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084d625); }
            blk = 8; continue;
        }
        52 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700084d62b); };
            c.w64(8, t49);
            blk = 8; continue;
        }
        53 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d637); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d63b); }
            let r50 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r50);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700084d646); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x700084d64b); }
            return c.call(0x70007febf4, 0x700084d655, 0x700084d650);
        }
        54 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084d658); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084d65d); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084d665); }
            blk = 24; continue;
        }
        55 => {
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d66b); };
            c.w32(8, t51);
            c.w32(9, c.r32(8));
            let r52 = c.r32(9) | 0x40u64;
            c.w32(9, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084d67f); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d684); }
            blk = 57; continue;
        }
        57 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084d68b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084d68c); }
            c.w32(1, 0x8u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084d692); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084d693); };
            c.w64(3, t53);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084d6a4, 0x700084d69f);
        }
        58 => {
            blk = 41; continue;
        }
        59 => {
            return RegfileExit::jump(0x700084d6ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084d6b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d6b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d6b8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084d6be); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084d6c2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d6d3); };
            c.w32(7, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d6d6); };
            c.cmp(t2, c.r32(7), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 3; continue;
        }
        3 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            if !c.movsd_rm(2, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d6e5); }
            c.xorpd_rr(3, 3);
            c.cvtsi2sd(3, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084d6f7); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084d6fe); };
            c.w64(8, t5);
            let r6 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r6);
            let r7 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r7);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084d709); };
            c.w64(11, t8);
            let Some(t9) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084d710); };
            c.cmp(t9, c.r64(9), 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700084d719); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d730); };
            c.w64(8, t10);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700084d737); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084d73b); }
            c.w64(10, c.r64(4));
            let r11 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            let r12 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r12);
            c.logic(r12, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700084d74a); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x700084d74f); };
            c.w64(0, t13);
            c.w64(10, 0x700084d75du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700084d75d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084d761); }
            return c.call(c.r64(0), 0x700084d767, 0x700084d765);
        }
        7 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084d767); }
            let Some(t14) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084d76f); };
            c.w64(4, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084d774); };
            c.w64(0, t15);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084d778); }
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084d77d); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084d781); };
            c.w64(7, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d785); };
            c.w32(8, t18);
            c.w32(9, c.r32(8));
            let r19 = c.r32(9) | 0xeu64;
            c.w32(9, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084d799); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d79e); }
            blk = 9; continue;
        }
        9 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084d7a8); }
            return c.ret(0x0, 0x700084d7a9);
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d7aa); };
            c.w32(7, t20);
            let r21 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r21);
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d7b0); };
            c.cmp(t22, c.r32(7), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d7c2); };
            c.w32(7, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d7c5); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(3, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d7d2); }
            if !c.movsd_rm(2, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d7d7); }
            blk = 15; continue;
        }
        14 => {
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            if !c.movsd_rm(3, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084d7e0); }
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(3) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d7ed); };
            c.w32(7, t26);
            c.w32(8, c.r32(7));
            let r27 = c.r32(8) | 0xeu64;
            c.w32(8, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084d800); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d804); }
            blk = 17; continue;
        }
        17 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084d80b); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084d812); };
            c.w64(7, t29);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084d819); };
            c.w64(9, t30);
            let Some(t31) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084d820); };
            c.cmp(t31, c.r64(8), 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700084d829); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d830); };
            c.w64(7, t32);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084d837); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x700084d83a); }
            c.mov128_rr(0, 3);
            c.mov128_rr(1, 2);
            c.w64(10, c.r64(4));
            let r33 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r33);
            let r34 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r34);
            c.logic(r34, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700084d84f); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x700084d854); };
            c.w64(0, t35);
            c.w64(10, 0x700084d862u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700084d862); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084d866); }
            return c.call(c.r64(0), 0x700084d86c, 0x700084d86a);
        }
        20 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084d86c); }
            let Some(t36) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084d874); };
            c.w64(4, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084d879); };
            c.w64(0, t37);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084d87d); }
            blk = 9; continue;
        }
        21 => {
            let Some(t38) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d887); };
            c.w32(7, t38);
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t39) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d89c); };
            c.w32(7, t39);
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d89f); };
            c.cmp(t40, c.r32(7), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(7, 0x1eu64);
            blk = 32; continue;
        }
        26 => {
            let Some(t41) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d8af); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            let Some(t43) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d8b5); };
            c.cmp(t43, 0x43u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(7, 0x1eu64);
            blk = 32; continue;
        }
        28 => {
            c.w32(7, 0x1eu64);
            blk = 32; continue;
        }
        29 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t44) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d8cf); };
            c.w32(7, t44);
            let r45 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r45);
            let Some(t46) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d8d5); };
            c.cmp(t46, 0x41u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(7, 0xfeu64);
            blk = 32; continue;
        }
        32 => {
            let Some(t47) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d8e1); };
            c.w32(8, t47);
            c.w32(7, c.r32(7));
            c.w32(9, c.r32(8));
            let r48 = c.r32(7) | c.r32(9);
            c.w32(7, r48);
            c.logic(r48, 32);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084d8f6); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d8fa); }
            blk = 34; continue;
        }
        34 => {
            return c.call(0x70008ae454, 0x700084d906, 0x700084d901);
        }
        35 => {
            blk = 9; continue;
        }
        36 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084d910); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700084d914); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700084d920, 0x700084d91b);
        }
        37 => {
            c.w64(7, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084d923); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084d927); };
            c.w64(3, t50);
            blk = 6; continue;
        }
        38 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 2) { return RegfileExit::fault(0x700084d935); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 3) { return RegfileExit::fault(0x700084d93a); }
            return c.call(0x70007febf4, 0x700084d944, 0x700084d93f);
        }
        39 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700084d947); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700084d94c); }
            blk = 19; continue;
        }
        40 => {
            let Some(t51) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084d956); };
            c.w32(7, t51);
            c.w32(8, c.r32(7));
            let r52 = c.r32(8) | 0x40u64;
            c.w32(8, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084d969); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084d96d); }
            blk = 42; continue;
        }
        42 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084d974); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084d975); }
            c.w32(1, 0xau64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084d97b); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084d97c); };
            c.w64(3, t53);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084d98d, 0x700084d988);
        }
        43 => {
            blk = 9; continue;
        }
        44 => {
            return RegfileExit::jump(0x700084d994);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084d9b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084d9b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084d9b8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700084d9be); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700084d9c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700084d9c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084d9ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084d9ce); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084d9e0); };
            c.w32(15, t1);
            let r2 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084d9e7); };
            c.cmp(t3, c.r32(15), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084d9f4); };
            c.w32(15, t4);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a10d4, 0x700084da0f, 0x700084da0a);
        }
        6 => {
            c.w32(9, 0xfeu64);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084da15); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084da19); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084da1d); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084da21); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084da27); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(8));
            blk = 1; continue;
        }
        8 => {
            c.w32(7, c.r32(9));
            let r11 = c.r32(7) | 0x2u64;
            c.w32(7, r11);
            c.logic(r11, 32);
            let r12 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        9 => {
            c.w32(7, c.r32(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084da4d); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084da5d); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084da64); };
            c.cmp(t15, c.r32(9), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084da6d); };
            c.w32(9, t16);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084da80); }
            return c.call(0x70008a10d4, 0x700084da89, 0x700084da84);
        }
        14 => {
            c.w32(12, 0xfeu64);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084da8f); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084da93); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084da97); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084da9b); };
            c.w64(6, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084da9f); };
            c.w64(8, t21);
            blk = 19; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084daa5); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            c.w32(12, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        16 => {
            c.w32(11, 0x40u64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        17 => {
            c.w32(11, 0xeu64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        18 => {
            c.w32(12, 0x2u64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        19 => {
            let r24 = c.r32(7) | 0x40u64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(11, c.r64(12));
            blk = 33; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700084dae9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(8, c.r32(8));
            c.w32(7, c.r32(9));
            let r25 = c.r32(7) | 0xeu64;
            c.w32(7, r25);
            c.logic(r25, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084db0a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700084db0e); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            c.w64(3, c.r64(0));
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084db1f); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084db26); };
            c.cmp(t28, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084db33); };
            c.w32(11, t29);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x700084db4e, 0x700084db49);
        }
        29 => {
            c.w32(9, 0xfeu64);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084db54); };
            c.w64(6, t30);
            blk = 23; continue;
        }
        30 => {
            let Some(t31) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084db5a); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 23; continue;
        }
        31 => {
            c.w32(1, c.r32(9));
            let r33 = c.r32(1) | 0x2u64;
            c.w32(1, r33);
            c.logic(r33, 32);
            let r34 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r34);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(1, c.r32(9));
            let r35 = c.r32(1) | 0x40u64;
            c.w32(1, r35);
            c.logic(r35, 32);
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084db81); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084db85); };
            c.w64(0, t37);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(11));
            c.w32(7, c.r32(7));
            let r38 = c.r32(8) | c.r32(7);
            c.w32(8, r38);
            c.logic(r38, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084db94); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084db95); }
            c.w32(8, 0xcu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084db9c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084db9e); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084dba2); };
            c.w64(3, t39);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084dbb3, 0x700084dbae);
        }
        34 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084dbb3); };
            c.w64(7, t40);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084dbb9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(3, c.r32(3));
            c.w32(1, c.r32(9));
            let r41 = c.r32(1) | 0xeu64;
            c.w32(1, r41);
            c.logic(r41, 32);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            let Some(t42) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084dbd8); };
            let r43 = c.r32(3) & t42;
            c.w32(3, r43);
            c.logic(r43, 32);
            c.w64(7, c.r64(3));
            let r44 = c.add(c.r32(7), c.r32(3), 32);
            c.w32(7, r44);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(12, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(12, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(1, c.r32(1));
            let Some(t45) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084dbf9); };
            c.w32(7, t45);
            c.w32(8, c.r32(12));
            let r46 = c.r32(1) | c.r32(7);
            c.w32(1, r46);
            c.logic(r46, 32);
            let r47 = c.r32(1) | c.r32(8);
            c.w32(1, r47);
            c.logic(r47, 32);
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084dc07); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084dc0b); };
            c.w64(8, t49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084dc0f); };
            c.w32(9, t50);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r51 = c.r32(7) | c.r32(11);
            c.w32(7, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084dc24); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084dc29); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084dc33); }
            return c.ret(0x0, 0x700084dc34);
        }
        46 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084dc35); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084dc3c); };
            c.w64(8, t53);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700084dc43); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700084dc47); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084dc4b); };
            c.w64(11, t54);
            let Some(t55) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084dc52); };
            c.cmp(t55, c.r64(9), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084dc61, 0x700084dc5c);
        }
        48 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084dc61); };
            c.w64(3, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084dc65); };
            c.w64(1, t57);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700084dc6b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 50; continue;
        }
        50 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084dc75); };
            c.w64(7, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084dc7c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084dc87); }
            blk = 39; continue;
        }
        51 => {
            let r59 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r59);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084dc95); }
            return c.call(0x70008708b4, 0x700084dc9f, 0x700084dc9a);
        }
        52 => {
            let Some(t60) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084dc9f); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            blk = 36; continue;
        }
        53 => {
            let r62 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084dcaf); }
            return c.call(0x70008708b4, 0x700084dcb9, 0x700084dcb4);
        }
        54 => {
            let Some(t63) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084dcb9); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r64);
            blk = 21; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084dcc8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084dcd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084dcd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084dcd8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700084dcde); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700084dce2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700084dce6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084dcea); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084dcee); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084dd00); };
            c.w32(15, t1);
            let r2 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084dd07); };
            c.cmp(t3, c.r32(15), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084dd14); };
            c.w32(15, t4);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a10d4, 0x700084dd2f, 0x700084dd2a);
        }
        6 => {
            c.w32(9, 0xfeu64);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084dd35); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084dd39); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084dd3d); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084dd41); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084dd47); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(8));
            blk = 1; continue;
        }
        8 => {
            c.w32(7, c.r32(9));
            let r11 = c.r32(7) | 0x2u64;
            c.w32(7, r11);
            c.logic(r11, 32);
            let r12 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        9 => {
            c.w32(7, c.r32(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084dd6d); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084dd7d); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084dd84); };
            c.cmp(t15, c.r32(9), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084dd8d); };
            c.w32(9, t16);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084dda0); }
            return c.call(0x70008a10d4, 0x700084dda9, 0x700084dda4);
        }
        14 => {
            c.w32(11, 0xfeu64);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084ddaf); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ddb3); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ddb7); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ddbb); };
            c.w64(6, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084ddbf); };
            c.w64(8, t21);
            blk = 19; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084ddc5); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            c.w32(11, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        16 => {
            c.w32(11, 0x40u64);
            blk = 19; continue;
        }
        17 => {
            c.w32(11, 0xeu64);
            blk = 19; continue;
        }
        18 => {
            c.w32(11, 0x2u64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        19 => {
            let r24 = c.r32(7) | 0x40u64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(8));
            blk = 33; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700084de00); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(8, c.r32(8));
            c.w32(7, c.r32(9));
            let r25 = c.r32(7) | 0xeu64;
            c.w32(7, r25);
            c.logic(r25, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084de21); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700084de25); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            c.w64(3, c.r64(0));
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084de36); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084de3d); };
            c.cmp(t28, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084de4a); };
            c.w32(11, t29);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x700084de65, 0x700084de60);
        }
        29 => {
            c.w32(9, 0xfeu64);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084de6b); };
            c.w64(6, t30);
            blk = 23; continue;
        }
        30 => {
            let Some(t31) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084de71); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 23; continue;
        }
        31 => {
            c.w32(1, c.r32(9));
            let r33 = c.r32(1) | 0x2u64;
            c.w32(1, r33);
            c.logic(r33, 32);
            let r34 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r34);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(1, c.r32(9));
            let r35 = c.r32(1) | 0x40u64;
            c.w32(1, r35);
            c.logic(r35, 32);
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084de98); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084de9c); };
            c.w64(0, t37);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(11));
            c.w32(7, c.r32(7));
            let r38 = c.r32(8) | c.r32(7);
            c.w32(8, r38);
            c.logic(r38, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084deab); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084deac); }
            c.w32(8, 0xeu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084deb3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084deb5); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084deb9); };
            c.w64(3, t39);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084deca, 0x700084dec5);
        }
        34 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084deca); };
            c.w64(7, t40);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084ded0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(3, c.r32(3));
            c.w32(1, c.r32(9));
            let r41 = c.r32(1) | 0xeu64;
            c.w32(1, r41);
            c.logic(r41, 32);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            let Some(t42) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084deef); };
            let r43 = c.r32(3) | t42;
            c.w32(3, r43);
            c.logic(r43, 32);
            c.w64(7, c.r64(3));
            let r44 = c.add(c.r32(7), c.r32(3), 32);
            c.w32(7, r44);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(11, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(11, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(1, c.r32(1));
            let Some(t45) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084df10); };
            c.w32(7, t45);
            c.w32(8, c.r32(11));
            let r46 = c.r32(1) | c.r32(7);
            c.w32(1, r46);
            c.logic(r46, 32);
            let r47 = c.r32(1) | c.r32(8);
            c.w32(1, r47);
            c.logic(r47, 32);
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084df1e); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084df22); };
            c.w64(8, t49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084df26); };
            c.w32(9, t50);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r51 = c.r32(7) | c.r32(11);
            c.w32(7, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084df3b); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084df40); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084df4a); }
            return c.ret(0x0, 0x700084df4b);
        }
        46 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084df4c); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084df53); };
            c.w64(8, t53);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700084df5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700084df5e); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084df62); };
            c.w64(11, t54);
            let Some(t55) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084df69); };
            c.cmp(t55, c.r64(9), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084df78, 0x700084df73);
        }
        48 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084df78); };
            c.w64(3, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084df7c); };
            c.w64(1, t57);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700084df82); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 50; continue;
        }
        50 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084df8c); };
            c.w64(7, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084df93); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084df9e); }
            blk = 39; continue;
        }
        51 => {
            let r59 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r59);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084dfac); }
            return c.call(0x70008708b4, 0x700084dfb6, 0x700084dfb1);
        }
        52 => {
            let Some(t60) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084dfb6); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            blk = 36; continue;
        }
        53 => {
            let r62 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084dfc6); }
            return c.call(0x70008708b4, 0x700084dfd0, 0x700084dfcb);
        }
        54 => {
            let Some(t63) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084dfd0); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r64);
            blk = 21; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084dfe0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084dff4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084dff4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084dff8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700084dffe); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700084e002); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700084e006); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084e00a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084e00e); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e020); };
            c.w32(15, t1);
            let r2 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e027); };
            c.cmp(t3, c.r32(15), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e034); };
            c.w32(15, t4);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a10d4, 0x700084e04f, 0x700084e04a);
        }
        6 => {
            c.w32(9, 0xfeu64);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084e055); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e059); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e05d); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e061); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e067); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(8));
            blk = 1; continue;
        }
        8 => {
            c.w32(7, c.r32(9));
            let r11 = c.r32(7) | 0x2u64;
            c.w32(7, r11);
            c.logic(r11, 32);
            let r12 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        9 => {
            c.w32(7, c.r32(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084e08d); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e09d); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e0a4); };
            c.cmp(t15, c.r32(9), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e0ad); };
            c.w32(9, t16);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084e0c0); }
            return c.call(0x70008a10d4, 0x700084e0c9, 0x700084e0c4);
        }
        14 => {
            c.w32(12, 0xfeu64);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e0cf); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e0d3); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e0d7); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e0db); };
            c.w64(6, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e0df); };
            c.w64(8, t21);
            blk = 19; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e0e5); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            c.w32(12, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        16 => {
            c.w32(11, 0x40u64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        17 => {
            c.w32(11, 0xeu64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        18 => {
            c.w32(12, 0x2u64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        19 => {
            let r24 = c.r32(7) | 0x40u64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(11, c.r64(12));
            blk = 33; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700084e129); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(8, c.r32(8));
            c.w32(7, c.r32(9));
            let r25 = c.r32(7) | 0xeu64;
            c.w32(7, r25);
            c.logic(r25, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084e14a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700084e14e); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            c.w64(3, c.r64(0));
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e15f); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e166); };
            c.cmp(t28, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e173); };
            c.w32(11, t29);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x700084e18e, 0x700084e189);
        }
        29 => {
            c.w32(9, 0xfeu64);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e194); };
            c.w64(6, t30);
            blk = 23; continue;
        }
        30 => {
            let Some(t31) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e19a); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 23; continue;
        }
        31 => {
            c.w32(1, c.r32(9));
            let r33 = c.r32(1) | 0x2u64;
            c.w32(1, r33);
            c.logic(r33, 32);
            let r34 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r34);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(1, c.r32(9));
            let r35 = c.r32(1) | 0x40u64;
            c.w32(1, r35);
            c.logic(r35, 32);
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e1c1); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e1c5); };
            c.w64(0, t37);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(11));
            c.w32(7, c.r32(7));
            let r38 = c.r32(8) | c.r32(7);
            c.w32(8, r38);
            c.logic(r38, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084e1d4); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084e1d5); }
            c.w32(8, 0x10u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084e1dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084e1de); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084e1e2); };
            c.w64(3, t39);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084e1f3, 0x700084e1ee);
        }
        34 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e1f3); };
            c.w64(7, t40);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084e1f9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(3, c.r32(3));
            c.w32(1, c.r32(9));
            let r41 = c.r32(1) | 0xeu64;
            c.w32(1, r41);
            c.logic(r41, 32);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            let Some(t42) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084e218); };
            let r43 = c.r32(3) ^ t42;
            c.w32(3, r43);
            c.logic(r43, 32);
            c.w64(7, c.r64(3));
            let r44 = c.add(c.r32(7), c.r32(3), 32);
            c.w32(7, r44);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(12, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(12, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(1, c.r32(1));
            let Some(t45) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e239); };
            c.w32(7, t45);
            c.w32(8, c.r32(12));
            let r46 = c.r32(1) | c.r32(7);
            c.w32(1, r46);
            c.logic(r46, 32);
            let r47 = c.r32(1) | c.r32(8);
            c.w32(1, r47);
            c.logic(r47, 32);
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e247); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e24b); };
            c.w64(8, t49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084e24f); };
            c.w32(9, t50);
            c.w32(7, c.r32(7));
            c.w32(11, c.r32(9));
            let r51 = c.r32(7) | c.r32(11);
            c.w32(7, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084e264); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084e269); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084e273); }
            return c.ret(0x0, 0x700084e274);
        }
        46 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084e275); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084e27c); };
            c.w64(8, t53);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700084e283); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700084e287); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084e28b); };
            c.w64(11, t54);
            let Some(t55) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700084e292); };
            c.cmp(t55, c.r64(9), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084e2a1, 0x700084e29c);
        }
        48 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e2a1); };
            c.w64(3, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e2a5); };
            c.w64(1, t57);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700084e2ab); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 50; continue;
        }
        50 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e2b5); };
            c.w64(7, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700084e2bc); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084e2c7); }
            blk = 39; continue;
        }
        51 => {
            let r59 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r59);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084e2d5); }
            return c.call(0x70008708b4, 0x700084e2df, 0x700084e2da);
        }
        52 => {
            let Some(t60) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084e2df); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            blk = 36; continue;
        }
        53 => {
            let r62 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084e2ef); }
            return c.call(0x70008708b4, 0x700084e2f9, 0x700084e2f4);
        }
        54 => {
            let Some(t63) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084e2f9); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r64);
            blk = 21; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084e308);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084e314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084e314); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084e318); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700084e31e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700084e322); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700084e326); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084e32a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084e32e); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e340); };
            c.w32(15, t1);
            let r2 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e347); };
            c.cmp(t3, c.r32(15), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e354); };
            c.w32(15, t4);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a10d4, 0x700084e36f, 0x700084e36a);
        }
        6 => {
            c.w32(9, 0xfeu64);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084e375); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e379); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e37d); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e381); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e387); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(8));
            blk = 1; continue;
        }
        8 => {
            c.w32(7, c.r32(9));
            let r11 = c.r32(7) | 0x2u64;
            c.w32(7, r11);
            c.logic(r11, 32);
            let r12 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        9 => {
            c.w32(7, c.r32(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084e3ad); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e3bd); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e3c4); };
            c.cmp(t15, c.r32(9), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e3cd); };
            c.w32(9, t16);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084e3e0); }
            return c.call(0x70008a10d4, 0x700084e3e9, 0x700084e3e4);
        }
        14 => {
            c.w32(12, 0xfeu64);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e3ef); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e3f3); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e3f7); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e3fb); };
            c.w64(6, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e3ff); };
            c.w64(8, t21);
            blk = 19; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e405); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            c.w32(12, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        16 => {
            c.w32(11, 0x40u64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        17 => {
            c.w32(11, 0xeu64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        18 => {
            c.w32(12, 0x2u64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        19 => {
            let r24 = c.r32(7) | 0x40u64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(11, c.r64(12));
            blk = 33; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700084e449); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(8, c.r32(8));
            c.w32(7, c.r32(9));
            let r25 = c.r32(7) | 0xeu64;
            c.w32(7, r25);
            c.logic(r25, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084e46a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084e46e); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            c.w64(3, c.r64(0));
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e47f); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e486); };
            c.cmp(t28, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e493); };
            c.w32(11, t29);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x700084e4ae, 0x700084e4a9);
        }
        29 => {
            c.w32(9, 0xfeu64);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e4b4); };
            c.w64(6, t30);
            blk = 23; continue;
        }
        30 => {
            let Some(t31) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e4ba); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 23; continue;
        }
        31 => {
            c.w32(7, c.r32(9));
            let r33 = c.r32(7) | 0x2u64;
            c.w32(7, r33);
            c.logic(r33, 32);
            let r34 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r34);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(7, c.r32(9));
            let r35 = c.r32(7) | 0x40u64;
            c.w32(7, r35);
            c.logic(r35, 32);
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e4e1); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084e4e5); };
            c.w64(0, t37);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(11));
            c.w32(7, c.r32(7));
            let r38 = c.r32(8) | c.r32(7);
            c.w32(8, r38);
            c.logic(r38, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084e4f4); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084e4f5); }
            c.w32(8, 0x12u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084e4fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084e4fe); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084e502); };
            c.w64(3, t39);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084e513, 0x700084e50e);
        }
        34 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e513); };
            c.w64(7, t40);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084e519); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(3, c.r32(3));
            c.w32(7, c.r32(9));
            let r41 = c.r32(7) | 0xeu64;
            c.w32(7, r41);
            c.logic(r41, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            c.w64(1, c.r64(3));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e53b); };
            c.w64(8, t42);
            let r43 = c.shl(c.r32(8), c.r8(1), 32).unwrap_or(c.r32(8));
            c.w32(8, r43);
            c.w64(9, c.r64(8));
            let r44 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r44);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(12, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(12, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(7, c.r32(7));
            let Some(t45) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e561); };
            c.w32(8, t45);
            c.w32(9, c.r32(12));
            let r46 = c.r32(7) | c.r32(8);
            c.w32(7, r46);
            c.logic(r46, 32);
            let r47 = c.r32(7) | c.r32(9);
            c.w32(7, r47);
            c.logic(r47, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e571); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e575); };
            c.w64(9, t49);
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084e579); };
            c.w32(11, t50);
            c.w32(7, c.r32(7));
            c.w32(12, c.r32(11));
            let r51 = c.r32(7) | c.r32(12);
            c.w32(7, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084e58e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084e593); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084e59e); }
            return c.ret(0x0, 0x700084e59f);
        }
        46 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084e5a0); };
            c.w64(9, t52);
            let Some(t53) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084e5a7); };
            c.w64(11, t53);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084e5ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084e5b2); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084e5b6); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700084e5bd); };
            c.cmp(t55, c.r64(12), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084e5cc, 0x700084e5c7);
        }
        48 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e5cc); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e5d0); };
            c.w64(7, t57);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700084e5d6); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 50; continue;
        }
        50 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e5e0); };
            c.w64(9, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700084e5e7); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084e5f4); }
            blk = 39; continue;
        }
        51 => {
            let r59 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r59);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084e602); }
            return c.call(0x70008708b4, 0x700084e60c, 0x700084e607);
        }
        52 => {
            let Some(t60) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084e60c); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            blk = 36; continue;
        }
        53 => {
            let r62 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084e61c); }
            return c.call(0x70008708b4, 0x700084e626, 0x700084e621);
        }
        54 => {
            let Some(t63) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084e626); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r64);
            blk = 21; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084e634);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084e654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084e654); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084e658); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700084e65e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700084e662); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700084e666); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084e66a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084e66e); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e680); };
            c.w32(15, t1);
            let r2 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e687); };
            c.cmp(t3, c.r32(15), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e694); };
            c.w32(15, t4);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a10d4, 0x700084e6af, 0x700084e6aa);
        }
        6 => {
            c.w32(9, 0xfeu64);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084e6b5); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e6b9); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e6bd); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e6c1); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e6c7); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(8));
            blk = 1; continue;
        }
        8 => {
            c.w32(7, c.r32(9));
            let r11 = c.r32(7) | 0x2u64;
            c.w32(7, r11);
            c.logic(r11, 32);
            let r12 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        9 => {
            c.w32(7, c.r32(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084e6ed); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e6fd); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e704); };
            c.cmp(t15, c.r32(9), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e70d); };
            c.w32(9, t16);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084e720); }
            return c.call(0x70008a10d4, 0x700084e729, 0x700084e724);
        }
        14 => {
            c.w32(12, 0xfeu64);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e72f); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e733); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e737); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e73b); };
            c.w64(6, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e73f); };
            c.w64(8, t21);
            blk = 19; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e745); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            c.w32(12, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        16 => {
            c.w32(11, 0x40u64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        17 => {
            c.w32(11, 0xeu64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        18 => {
            c.w32(12, 0x2u64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        19 => {
            let r24 = c.r32(7) | 0x40u64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(11, c.r64(12));
            blk = 33; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700084e789); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(8, c.r32(8));
            c.w32(7, c.r32(9));
            let r25 = c.r32(7) | 0xeu64;
            c.w32(7, r25);
            c.logic(r25, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084e7aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084e7ae); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            c.w64(3, c.r64(0));
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e7bf); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e7c6); };
            c.cmp(t28, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e7d3); };
            c.w32(11, t29);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x700084e7ee, 0x700084e7e9);
        }
        29 => {
            c.w32(9, 0xfeu64);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e7f4); };
            c.w64(6, t30);
            blk = 23; continue;
        }
        30 => {
            let Some(t31) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084e7fa); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 23; continue;
        }
        31 => {
            c.w32(7, c.r32(9));
            let r33 = c.r32(7) | 0x2u64;
            c.w32(7, r33);
            c.logic(r33, 32);
            let r34 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r34);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(7, c.r32(9));
            let r35 = c.r32(7) | 0x40u64;
            c.w32(7, r35);
            c.logic(r35, 32);
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e821); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084e825); };
            c.w64(0, t37);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(11));
            c.w32(7, c.r32(7));
            let r38 = c.r32(8) | c.r32(7);
            c.w32(8, r38);
            c.logic(r38, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084e834); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084e835); }
            c.w32(8, 0x14u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084e83c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084e83e); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084e842); };
            c.w64(3, t39);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084e853, 0x700084e84e);
        }
        34 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e853); };
            c.w64(7, t40);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084e859); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(3, c.r32(3));
            c.w32(7, c.r32(9));
            let r41 = c.r32(7) | 0xeu64;
            c.w32(7, r41);
            c.logic(r41, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            c.w64(1, c.r64(3));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e87b); };
            c.w64(8, t42);
            let r43 = c.sar(c.r32(8), c.r8(1), 32).unwrap_or(c.r32(8));
            c.w32(8, r43);
            c.w64(9, c.r64(8));
            let r44 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r44);
            if c.cc_o() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(12, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(12, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(7, c.r32(7));
            let Some(t45) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084e8a1); };
            c.w32(8, t45);
            c.w32(9, c.r32(12));
            let r46 = c.r32(7) | c.r32(8);
            c.w32(7, r46);
            c.logic(r46, 32);
            let r47 = c.r32(7) | c.r32(9);
            c.w32(7, r47);
            c.logic(r47, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e8b1); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e8b5); };
            c.w64(9, t49);
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084e8b9); };
            c.w32(11, t50);
            c.w32(7, c.r32(7));
            c.w32(12, c.r32(11));
            let r51 = c.r32(7) | c.r32(12);
            c.w32(7, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084e8ce); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084e8d3); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084e8de); }
            return c.ret(0x0, 0x700084e8df);
        }
        46 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084e8e0); };
            c.w64(9, t52);
            let Some(t53) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084e8e7); };
            c.w64(11, t53);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084e8ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084e8f2); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084e8f6); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700084e8fd); };
            c.cmp(t55, c.r64(12), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084e90c, 0x700084e907);
        }
        48 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084e90c); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084e910); };
            c.w64(7, t57);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700084e916); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 50; continue;
        }
        50 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e920); };
            c.w64(9, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700084e927); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084e934); }
            blk = 39; continue;
        }
        51 => {
            let r59 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r59);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084e942); }
            return c.call(0x70008708b4, 0x700084e94c, 0x700084e947);
        }
        52 => {
            let Some(t60) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084e94c); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            blk = 36; continue;
        }
        53 => {
            let r62 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084e95c); }
            return c.call(0x70008708b4, 0x700084e966, 0x700084e961);
        }
        54 => {
            let Some(t63) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084e966); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r64);
            blk = 21; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084e974);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084e994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084e994); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084e998); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700084e99e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700084e9a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700084e9a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084e9aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084e9ae); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084e9c0); };
            c.w32(15, t1);
            let r2 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084e9c7); };
            c.cmp(t3, c.r32(15), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084e9d4); };
            c.w32(15, t4);
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a10d4, 0x700084e9ef, 0x700084e9ea);
        }
        6 => {
            c.w32(9, 0xfeu64);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084e9f5); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084e9f9); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084e9fd); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ea01); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084ea07); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(8));
            blk = 1; continue;
        }
        8 => {
            c.w32(7, c.r32(9));
            let r11 = c.r32(7) | 0x2u64;
            c.w32(7, r11);
            c.logic(r11, 32);
            let r12 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r12);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        9 => {
            c.w32(7, c.r32(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084ea2d); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(3));
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ea3d); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ea44); };
            c.cmp(t15, c.r32(9), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ea4d); };
            c.w32(9, t16);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700084ea60); }
            return c.call(0x70008a10d4, 0x700084ea69, 0x700084ea64);
        }
        14 => {
            c.w32(12, 0xfeu64);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084ea6f); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ea73); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ea77); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ea7b); };
            c.w64(6, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084ea7f); };
            c.w64(8, t21);
            blk = 19; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084ea85); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            c.w32(12, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        16 => {
            c.w32(11, 0x40u64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        17 => {
            c.w32(11, 0xeu64);
            c.w64(12, c.r64(11));
            blk = 19; continue;
        }
        18 => {
            c.w32(12, 0x2u64);
            c.w64(0, c.r64(3));
            blk = 19; continue;
        }
        19 => {
            let r24 = c.r32(7) | 0x40u64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(11, c.r64(12));
            blk = 33; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700084eac9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_o() { blk = 53; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(8, c.r32(8));
            c.w32(7, c.r32(9));
            let r25 = c.r32(7) | 0xeu64;
            c.w32(7, r25);
            c.logic(r25, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084eaea); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084eaee); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            c.w64(3, c.r64(0));
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084eaff); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084eb06); };
            c.cmp(t28, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084eb13); };
            c.w32(11, t29);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a10d4, 0x700084eb2e, 0x700084eb29);
        }
        29 => {
            c.w32(9, 0xfeu64);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084eb34); };
            c.w64(6, t30);
            blk = 23; continue;
        }
        30 => {
            let Some(t31) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084eb3a); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            c.w32(9, 0x1eu64);
            c.w64(0, c.r64(3));
            blk = 23; continue;
        }
        31 => {
            c.w32(7, c.r32(9));
            let r33 = c.r32(7) | 0x2u64;
            c.w32(7, r33);
            c.logic(r33, 32);
            let r34 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r34);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        32 => {
            c.w32(7, c.r32(9));
            let r35 = c.r32(7) | 0x40u64;
            c.w32(7, r35);
            c.logic(r35, 32);
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084eb61); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084eb65); };
            c.w64(0, t37);
            blk = 33; continue;
        }
        33 => {
            c.w32(8, c.r32(11));
            c.w32(7, c.r32(7));
            let r38 = c.r32(8) | c.r32(7);
            c.w32(8, r38);
            c.logic(r38, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084eb74); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700084eb75); }
            c.w32(8, 0x16u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084eb7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700084eb7e); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700084eb82); };
            c.w64(3, t39);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084eb93, 0x700084eb8e);
        }
        34 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084eb93); };
            c.w64(7, t40);
            blk = 43; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700084eb99); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_o() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(3, c.r32(3));
            c.w32(7, c.r32(9));
            let r41 = c.r32(7) | 0xeu64;
            c.w32(7, r41);
            c.logic(r41, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 37; continue;
        }
        37 => {
            c.w64(1, c.r64(3));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084ebbb); };
            c.w64(8, t42);
            let r43 = c.shr(c.r32(8), c.r8(1), 32).unwrap_or(c.r32(8));
            c.w32(8, r43);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(12, 0xeu64);
            blk = 42; continue;
        }
        41 => {
            c.w32(12, 0x2u64);
            blk = 42; continue;
        }
        42 => {
            c.w32(7, c.r32(7));
            let Some(t44) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084ebe3); };
            c.w32(8, t44);
            c.w32(9, c.r32(12));
            let r45 = c.r32(7) | c.r32(8);
            c.w32(7, r45);
            c.logic(r45, 32);
            let r46 = c.r32(7) | c.r32(9);
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 43; continue;
        }
        43 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ebf3); };
            c.w64(8, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ebf7); };
            c.w64(9, t48);
            let Some(t49) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084ebfb); };
            c.w32(11, t49);
            c.w32(7, c.r32(7));
            c.w32(12, c.r32(11));
            let r50 = c.r32(7) | c.r32(12);
            c.w32(7, r50);
            c.logic(r50, 32);
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084ec10); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084ec15); }
            blk = 45; continue;
        }
        45 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084ec20); }
            return c.ret(0x0, 0x700084ec21);
        }
        46 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084ec22); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084ec29); };
            c.w64(11, t52);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700084ec30); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084ec34); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084ec38); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700084ec3f); };
            c.cmp(t54, c.r64(12), 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084ec4e, 0x700084ec49);
        }
        48 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084ec4e); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084ec52); };
            c.w64(7, t56);
            blk = 50; continue;
        }
        49 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700084ec58); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 50; continue;
        }
        50 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ec62); };
            c.w64(9, t57);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700084ec69); }
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084ec79); }
            blk = 39; continue;
        }
        51 => {
            let r58 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r58);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084ec87); }
            return c.call(0x70008708b4, 0x700084ec91, 0x700084ec8c);
        }
        52 => {
            let Some(t59) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084ec91); };
            c.w32(3, t59);
            let r60 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r60);
            blk = 36; continue;
        }
        53 => {
            let r61 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700084eca1); }
            return c.call(0x70008708b4, 0x700084ecab, 0x700084eca6);
        }
        54 => {
            let Some(t62) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700084ecab); };
            c.w32(8, t62);
            let r63 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            blk = 21; continue;
        }
        55 => {
            return RegfileExit::jump(0x700084ecbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084ecd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084ecd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084ecd8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084ecde); };
            c.w64(1, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084ece2); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(8, 0x60u64);
            c.w32(9, 0x20u64);
            c.w64(7, c.r64(6));
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ed28); };
            c.w32(11, t2);
            let r3 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ed2f); };
            c.w32(12, t4);
            let r5 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r5);
            let Some(t6) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ed36); };
            c.w32(15, t6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ed3b); };
            c.w32(3, t7);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_b() { blk = 95; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(15), 0x42u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(15), 0x41u64, 16);
            if c.cc_e() { blk = 87; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(3), 0xbau64, 16);
            if c.cc_ae() { blk = 58; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700084ed7b); };
            c.logic(t8 & 0x10u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 85; continue;
        }
        12 => {
            c.w32(7, c.r32(7));
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700084ed8e); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 13; continue;
        }
        13 => {
            let r10 = c.r32(7) | 0x8u64;
            c.w32(7, r10);
            c.logic(r10, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084edaa); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            blk = 1; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r13 = c.r32(7) | 0x6u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            c.cmp(c.r16(15), c.r16(3), 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_b() { blk = 94; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(3), 0x43u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r16(3), 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r16(3), 0xbau64, 16);
            if c.cc_b() { blk = 69; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 26; continue;
        }
        21 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 26; continue;
        }
        22 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700084edef); };
            c.cmp(t14, c.r32(12), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 23; continue;
        }
        23 => {
            let r15 = c.r32(7) | 0x8u64;
            c.w32(7, r15);
            c.logic(r15, 32);
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084edff); };
            c.w32(0, t16);
            let r17 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        24 => {
            c.cmp(c.r16(3), 0xbau64, 16);
            if c.cc_b() { blk = 71; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(7, 0x3feu64);
            blk = 26; continue;
        }
        26 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(2, c.r64(10));
            blk = 1; continue;
        }
        27 => {
            c.w32(7, c.r32(7));
            let r18 = c.r32(7) | 0x2u64;
            c.w32(7, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084ee3a); };
            c.w32(11, t19);
            let r20 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084ee41); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t22) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084ee4e); };
            c.w32(12, t22);
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 99; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 119; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 97; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 74; continue;
        }
        34 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700084ee81); };
            c.cmp(t23, c.r32(11), 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 35; continue;
        }
        35 => {
            let r24 = c.r32(7) | 0x8u64;
            c.w32(7, r24);
            c.logic(r24, 32);
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084ee91); };
            c.w32(0, t25);
            let r26 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r26);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        36 => {
            c.w32(7, c.r32(7));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t27) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084eeab); };
            c.w32(11, t27);
            let r28 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r28);
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084eeb2); };
            c.cmp(t29, c.r32(11), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t30) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084eebf); };
            c.w32(12, t30);
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 48; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 47; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let r31 = c.r32(7) | 0x80u64;
            c.w32(7, r31);
            c.logic(r31, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        43 => {
            let r32 = c.r32(7) | 0x100u64;
            c.w32(7, r32);
            c.logic(r32, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        44 => {
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700084eefd); };
            c.cmp(t33, c.r32(11), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let r34 = c.r32(7) | 0x210u64;
            c.w32(7, r34);
            c.logic(r34, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        46 => {
            let r35 = c.r32(7) | 0x8u64;
            c.w32(7, r35);
            c.logic(r35, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        47 => {
            let r36 = c.r32(7) | 0x200u64;
            c.w32(7, r36);
            c.logic(r36, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        48 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(9, c.r64(8));
            blk = 50; continue;
        }
        50 => {
            c.w32(8, c.r32(9));
            let r37 = c.r32(8) | c.r32(7);
            c.w32(8, r37);
            c.logic(r37, 32);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        51 => {
            let r38 = c.r32(7) | 0x6u64;
            c.w32(7, r38);
            c.logic(r38, 32);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084ef47); }
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 81; continue; }
            blk = 52; continue;
        }
        52 => {
            if c.cc_e() { blk = 79; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 81; continue;
        }
        54 => {
            let r39 = c.r32(7) | 0x2u64;
            c.w32(7, r39);
            c.logic(r39, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        55 => {
            let Some(t40) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700084ef6f); };
            c.logic(t40 & 0x10u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(7, 0x210u64);
            blk = 81; continue;
        }
        57 => {
            c.w32(7, 0x210u64);
            blk = 79; continue;
        }
        58 => {
            c.w32(7, c.r32(7));
            let r41 = c.r32(7) | 0x200u64;
            c.w32(7, r41);
            c.logic(r41, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        59 => {
            let r42 = c.r32(7) | 0x10u64;
            c.w32(7, r42);
            c.logic(r42, 32);
            let Some(t43) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700084ef9d); };
            c.logic(t43 & 0x10u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r16(3), 0x42u64, 16);
            if c.cc_e() { blk = 66; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r16(3), 0x43u64, 16);
            if c.cc_e() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let r44 = c.r32(7) | 0x210u64;
            c.w32(7, r44);
            c.logic(r44, 32);
            c.cmp(c.r16(3), 0xbau64, 16);
            if c.cc_ae() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let r45 = c.r32(7) | 0x3feu64;
            c.w32(7, r45);
            c.logic(r45, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        64 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        65 => {
            let r46 = c.r32(7) | 0x18u64;
            c.w32(7, r46);
            c.logic(r46, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        66 => {
            let r47 = c.r32(7) | 0x6u64;
            c.w32(7, r47);
            c.logic(r47, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        67 => {
            let r48 = c.r32(7) | 0x210u64;
            c.w32(7, r48);
            c.logic(r48, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        68 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084effb); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f000); }
            blk = 77; continue;
        }
        69 => {
            let r49 = c.r32(7) | 0x3feu64;
            c.w32(7, r49);
            c.logic(r49, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        70 => {
            let r50 = c.r32(7) | 0x18u64;
            c.w32(7, r50);
            c.logic(r50, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        71 => {
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w32(7, 0x3feu64);
            blk = 81; continue;
        }
        73 => {
            c.w32(7, c.r32(7));
            let r51 = c.r32(7) | 0x80u64;
            c.w32(7, r51);
            c.logic(r51, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        74 => {
            let r52 = c.r32(7) | 0x3feu64;
            c.w32(7, r52);
            c.logic(r52, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        75 => {
            let r53 = c.r32(7) | 0x18u64;
            c.w32(7, r53);
            c.logic(r53, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        76 => {
            let r54 = c.r32(7) | 0x6u64;
            c.w32(7, r54);
            c.logic(r54, 32);
            let r55 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r55);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f04f); }
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 77; continue;
        }
        77 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 81; continue; }
            blk = 78; continue;
        }
        78 => {
            if c.cc_ne() { blk = 81; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084f067); };
            c.w64(0, t56);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f06e); }
            blk = 82; continue;
        }
        80 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        81 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700084f077); };
            c.w64(0, t57);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f07e); }
            blk = 82; continue;
        }
        82 => {
            let Some(t58) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084f082); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084f086); };
            c.w64(7, t59);
            let Some(t60) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084f08a); };
            c.w32(8, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084f08f); };
            c.w64(9, t61);
            c.w32(9, c.r32(9));
            c.w32(11, c.r32(8));
            let r62 = c.r32(9) | c.r32(11);
            c.w32(9, r62);
            c.logic(r62, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700084f0a4); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084f0a9); }
            blk = 84; continue;
        }
        84 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084f0b3); }
            return c.ret(0x0, 0x700084f0b4);
        }
        85 => {
            c.w32(7, c.r32(7));
            let Some(t63) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f0b7); };
            c.w64(11, t63);
            let r64 = c.r32(7) | 0x3feu64;
            c.w32(7, r64);
            c.logic(r64, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f0c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700084f0c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f0cc); }
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(11));
            return c.call(0x70008a2374, 0x700084f0db, 0x700084f0d6);
        }
        86 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084f0db); };
            c.w64(7, t65);
            c.w64(2, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084f0e2); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f0e6); };
            c.w64(1, t67);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(9, 0x20u64);
            c.w32(8, 0x60u64);
            blk = 1; continue;
        }
        87 => {
            c.w32(7, c.r32(7));
            let r68 = c.r32(7) | 0x100u64;
            c.w32(7, r68);
            c.logic(r68, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f105); }
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f109); };
            c.cmp(t69, c.r32(12), 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r16(3), 0x41u64, 16);
            if c.cc_e() { blk = 107; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_b() { blk = 105; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700084f12a); };
            c.cmp(t70, c.r32(12), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r16(3), 0xbau64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 81; continue;
        }
        93 => {
            let r71 = c.r32(7) | 0x8u64;
            c.w32(7, r71);
            c.logic(r71, 32);
            let Some(t72) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084f149); };
            c.w32(0, t72);
            let r73 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r73);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        94 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 100; continue;
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f15c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f160); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700084f164); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700084f168); }
            c.cmp(c.r16(3), 0x40u64, 16);
            if c.cc_b() { blk = 111; continue; }
            blk = 96; continue;
        }
        96 => {
            blk = 26; continue;
        }
        97 => {
            let r74 = c.r32(7) | 0x200u64;
            c.w32(7, r74);
            c.logic(r74, 32);
            let Some(t75) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f181); };
            c.w64(11, t75);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f188); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084f18c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f190); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a2374, 0x700084f19c, 0x700084f197);
        }
        98 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084f19c); };
            c.w64(7, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084f1a0); };
            c.w64(2, t77);
            c.w32(8, 0x60u64);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f1aa); };
            c.w64(1, t78);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(9, 0x20u64);
            blk = 1; continue;
        }
        99 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 100; continue;
        }
        100 => {
            let Some(t79) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f1be); };
            c.w32(11, t79);
            let r80 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r80);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f1c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084f1c9); }
            let Some(t81) = c.ld8(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f1cd); };
            c.logic(t81 & 0x20u64, 8);
            if c.cc_e() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(11, c.r64(8));
            blk = 103; continue;
        }
        102 => {
            c.w64(11, c.r64(9));
            blk = 103; continue;
        }
        103 => {
            c.w32(11, c.r32(11));
            c.w32(7, c.r32(7));
            let Some(t82) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f1e1); };
            c.w64(12, t82);
            let r83 = c.r32(11) | c.r32(7);
            c.w32(11, r83);
            c.logic(r83, 32);
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f1eb); }
            c.w64(6, c.r64(12));
            return c.call(0x70008a0fd4, 0x700084f1f7, 0x700084f1f2);
        }
        104 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084f1f7); };
            c.w64(7, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084f1fb); };
            c.w64(2, t85);
            c.w32(8, 0x60u64);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f205); };
            c.w64(1, t86);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(9, 0x20u64);
            blk = 1; continue;
        }
        105 => {
            let r87 = c.r32(7) | 0x60u64;
            c.w32(7, r87);
            c.logic(r87, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084f21c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084f21d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f21e); }
            c.w32(0, 0x2u64);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x1d50)) else { return RegfileExit::fault(0x700084f227); };
            c.w64(3, t88);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700084f236, 0x700084f231);
        }
        106 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f236); };
            c.w64(1, t89);
            blk = 82; continue;
        }
        107 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084f242); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084f243); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f244); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700084f248); };
            c.w64(3, t90);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700084f259, 0x700084f254);
        }
        108 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f259); };
            c.w64(1, t91);
            blk = 82; continue;
        }
        109 => {
            let r92 = c.r32(7) | 0x6u64;
            c.w32(7, r92);
            c.logic(r92, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084f268); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084f269); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f26a); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x700084f26e); };
            c.w64(3, t93);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700084f27f, 0x700084f27a);
        }
        110 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f27f); };
            c.w64(1, t94);
            blk = 82; continue;
        }
        111 => {
            let Some(t95) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f288); };
            c.w64(11, t95);
            c.w64(6, c.r64(11));
            return c.call(0x70007fb534, 0x700084f294, 0x700084f28f);
        }
        112 => {
            let Some(t96) = c.ld8(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084f294); };
            c.logic(t96 & 0x20u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w32(9, 0x60u64);
            blk = 115; continue;
        }
        114 => {
            c.w32(9, 0x20u64);
            blk = 115; continue;
        }
        115 => {
            let Some(t97) = c.ld8(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084f2a8); };
            c.logic(t97 & 0x20u64, 8);
            if c.cc_e() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(1, 0x60u64);
            blk = 118; continue;
        }
        117 => {
            c.w32(1, 0x20u64);
            blk = 118; continue;
        }
        118 => {
            c.w32(7, c.r32(9));
            c.w32(1, c.r32(1));
            let Some(t98) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084f2bf); };
            c.w32(8, t98);
            let r99 = c.r32(7) | c.r32(1);
            c.w32(7, r99);
            c.logic(r99, 32);
            let r100 = c.r32(7) | c.r32(8);
            c.w32(7, r100);
            c.logic(r100, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f2cb); }
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f2cf); };
            c.w64(1, t101);
            blk = 82; continue;
        }
        119 => {
            let r102 = c.r32(7) | 0x100u64;
            c.w32(7, r102);
            c.logic(r102, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084f2e1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084f2e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f2e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f2e7); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x700084f2eb); };
            c.w64(3, t103);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700084f2fc, 0x700084f2f7);
        }
        120 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f2fc); };
            c.w64(1, t104);
            blk = 82; continue;
        }
        121 => {
            return RegfileExit::jump(0x700084f308);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084f314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f338); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f33e); };
            c.cmp(t2, c.r32(1), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f354); };
            c.w32(7, t3);
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 70; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f36c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f373); };
            c.w32(9, t6);
            c.cmp(c.r16(7), 0xbau64, 16);
            if c.cc_ae() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700084f37f); };
            c.cmp(t7, c.r32(1), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(1, 0x80u64);
            blk = 61; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_b() { blk = 35; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f3bc); };
            c.cmp(t8, c.r32(8), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(1, 0x210u64);
            blk = 61; continue;
        }
        15 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700084f3d3); };
            c.cmp(t9, c.r32(8), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(1, 0x210u64);
            blk = 61; continue;
        }
        19 => {
            c.w32(1, 0x200u64);
            blk = 61; continue;
        }
        20 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f407); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f40d); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 35; continue;
        }
        22 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f422); };
            c.w32(1, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f425); };
            c.cmp(t14, c.r32(1), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f432); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f437); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x6u64);
            blk = 61; continue;
        }
        27 => {
            c.w32(1, 0x6u64);
            blk = 64; continue;
        }
        28 => {
            let r15 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r15);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f45a); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(1, 0x6u64);
            blk = 61; continue;
        }
        31 => {
            c.w32(1, 0x6u64);
            blk = 64; continue;
        }
        32 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f488); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f48e); };
            c.cmp(t18, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t19) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f497); };
            c.cmp(t19, 0x43u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(1, 0x3feu64);
            blk = 61; continue;
        }
        36 => {
            c.w32(1, 0x1eu64);
            blk = 61; continue;
        }
        37 => {
            let r20 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r20);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f4b4); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(1, 0x6u64);
            blk = 61; continue;
        }
        40 => {
            c.w32(1, 0x6u64);
            blk = 64; continue;
        }
        41 => {
            c.w32(1, 0x2u64);
            c.w64(7, c.r64(1));
            blk = 61; continue;
        }
        42 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f4f3); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f4f9); };
            c.cmp(t23, c.r32(1), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t24) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f506); };
            c.w32(7, t24);
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 54; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r16(7), 0xbau64, 16);
            if c.cc_ae() { blk = 53; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(1, 0x80u64);
            blk = 64; continue;
        }
        49 => {
            c.w32(1, 0x100u64);
            blk = 64; continue;
        }
        50 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700084f534); };
            c.cmp(t25, c.r32(1), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(1, 0x210u64);
            blk = 64; continue;
        }
        52 => {
            c.w32(1, 0x8u64);
            blk = 64; continue;
        }
        53 => {
            c.w32(1, 0x200u64);
            blk = 64; continue;
        }
        54 => {
            c.logic(c.r8(7) & 0x20u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w32(1, 0x60u64);
            blk = 57; continue;
        }
        56 => {
            c.w32(1, 0x20u64);
            blk = 57; continue;
        }
        57 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 64; continue;
        }
        58 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f569); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            if c.cc_e() { blk = 62; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(1, 0x6u64);
            blk = 61; continue;
        }
        61 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700084f57b); };
            c.w64(0, t26);
            blk = 65; continue;
        }
        62 => {
            c.w32(1, 0x6u64);
            blk = 64; continue;
        }
        63 => {
            c.w32(1, 0x2u64);
            blk = 64; continue;
        }
        64 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084f590); };
            c.w64(0, t27);
            blk = 65; continue;
        }
        65 => {
            c.w64(7, c.r64(5));
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084f59a); };
            c.w64(7, t28);
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084f59e); };
            c.w32(8, t29);
            c.w32(1, c.r32(1));
            c.w32(9, c.r32(8));
            let r30 = c.r32(1) | c.r32(9);
            c.w32(1, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700084f5b3); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084f5b7); }
            blk = 67; continue;
        }
        67 => {
            return c.ret(0x0, 0x700084f5be);
        }
        68 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084f5bf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084f5c3); }
            let r31 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r31);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084f5c9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084f5ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084f5cb); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700084f5cf); };
            c.w64(1, t32);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x700084f5e5, 0x700084f5e0);
        }
        69 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084f5e5); };
            c.w64(3, t33);
            c.w32(1, 0x100u64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084f5f1); }
            blk = 65; continue;
        }
        70 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084f5f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084f5f8); }
            let r34 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r34);
            let Some(t35) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f5fe); };
            c.w32(1, t35);
            let r36 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r36);
            let Some(t37) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f604); };
            c.w32(1, t37);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084f608); }
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084f615); }
            blk = 35; continue;
        }
        72 => {
            c.logic(c.r8(7) & 0x20u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(7, 0x60u64);
            blk = 75; continue;
        }
        74 => {
            c.w32(7, 0x20u64);
            blk = 75; continue;
        }
        75 => {
            c.logic(c.r8(1) & 0x20u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(1, 0x60u64);
            blk = 78; continue;
        }
        77 => {
            c.w32(1, 0x20u64);
            blk = 78; continue;
        }
        78 => {
            c.w32(1, c.r32(1));
            c.w32(7, c.r32(7));
            let r38 = c.r32(1) | c.r32(7);
            c.w32(1, r38);
            c.logic(r38, 32);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f649); }
            return c.call(0x70007fb534, 0x700084f652, 0x700084f64d);
        }
        79 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f652); };
            c.w64(1, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084f656); };
            c.w64(3, t40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084f65d); }
            blk = 65; continue;
        }
        80 => {
            return RegfileExit::jump(0x700084f664);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084f674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(8));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f67e); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084f682); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084f686); };
            c.w64(3, t2);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084f68d); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(8, 0x3feu64);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f6a5); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f6ac); };
            c.cmp(t5, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f6c1); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 52; continue;
        }
        9 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084f6ed); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f6f4); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 55; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f702); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f707); }
            blk = 15; continue;
        }
        13 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f710); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r12 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r12);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084f721); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r13 = c.r32(7) | 0x6u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(7));
            let r14 = c.r32(7) | 0x2u64;
            c.w32(7, r14);
            c.logic(r14, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700084f74a); };
            c.w64(0, t15);
            blk = 20; continue;
        }
        19 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084f753); };
            c.w64(0, t16);
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084f75a); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084f75e); };
            c.w32(7, t18);
            c.w32(8, c.r32(8));
            c.w32(9, c.r32(7));
            let r19 = c.r32(8) | c.r32(9);
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084f773); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084f777); }
            blk = 22; continue;
        }
        22 => {
            return c.ret(0x0, 0x700084f77e);
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084f77f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084f783); }
            let r20 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f789); };
            c.w32(11, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084f78e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f792); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700084f796); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084f79a); }
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f7bb); };
            c.cmp(t22, c.r32(9), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f7c4); };
            c.cmp(t23, 0x43u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(7, c.r64(8));
            blk = 30; continue;
        }
        29 => {
            c.w32(7, c.r32(7));
            let r24 = c.r32(7) | 0x1eu64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f7d9); }
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t25) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f7e5); };
            c.w64(9, t25);
            c.w64(6, c.r64(9));
            return c.call(0x70008a11d4, 0x700084f7f1, 0x700084f7ec);
        }
        32 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f7f1); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f7f5); };
            c.w64(7, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700084f7f9); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084f7fd); };
            c.w64(0, t28);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084f809, 0x700084f804);
        }
        33 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084f809); };
            c.w64(8, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084f80d); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084f811); };
            c.w64(0, t30);
            blk = 1; continue;
        }
        34 => {
            let Some(t31) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f81a); };
            c.w64(9, t31);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(9));
            return c.call(0x70008a2474, 0x700084f829, 0x700084f824);
        }
        35 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084f829); };
            c.w64(8, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084f82d); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084f831); };
            c.w64(0, t33);
            blk = 1; continue;
        }
        36 => {
            let Some(t34) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f83a); };
            c.w32(9, t34);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f85d); };
            c.w64(7, t35);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084f86c, 0x700084f867);
        }
        40 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f86c); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f870); };
            c.w64(7, t37);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084f874); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084f878); };
            c.w64(0, t38);
            c.w64(6, c.r64(7));
            return c.call(0x70008a11d4, 0x700084f884, 0x700084f87f);
        }
        41 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        42 => {
            let Some(t39) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f88f); };
            c.w64(7, t39);
            c.w64(6, c.r64(7));
            return c.call(0x70008a2474, 0x700084f89b, 0x700084f896);
        }
        43 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        44 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084f8a6); };
            c.cmp(t40, c.r32(9), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t41) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f8b3); };
            c.w32(9, t41);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t42) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f8ce); };
            c.w64(7, t42);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084f8da, 0x700084f8d5);
        }
        48 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084f8e5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084f8e9); }
            let r43 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084f8ef); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f8f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084f8f7); }
            let Some(t44) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f8fb); };
            c.cmp(t44, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t45) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f907); };
            c.w64(7, t45);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084f913, 0x700084f90e);
        }
        51 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084f91e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084f922); }
            let r46 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r46);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084f928); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f92c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700084f930); }
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f934); };
            c.cmp(t47, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t48) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f940); };
            c.w64(7, t48);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084f94f, 0x700084f94a);
        }
        54 => {
            c.w32(8, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084f955); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084f959); };
            c.w64(0, t49);
            blk = 1; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084f962); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084f966); }
            let r50 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084f96c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084f970); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084f974); }
            let Some(t51) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084f978); };
            c.cmp(t51, 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t52) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f984); };
            c.w64(7, t52);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084f990, 0x700084f98b);
        }
        57 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        58 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084f9a0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084f9a1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084f9a2); }
            c.w32(0, 0x3u64);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700084f9a8); };
            c.w64(3, t53);
            c.w64(9, c.r64(0));
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700084f9ba, 0x700084f9b5);
        }
        59 => {
            c.w32(8, 0x3feu64);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f9c0); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084f9c4); };
            c.w64(3, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084f9cb); }
            blk = 20; continue;
        }
        60 => {
            c.w32(7, c.r32(7));
            let Some(t56) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084f9d3); };
            c.w64(8, t56);
            let r57 = c.r32(7) | 0x60u64;
            c.w32(7, r57);
            c.logic(r57, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084f9dd); }
            c.w64(6, c.r64(8));
            return c.call(0x70007fb9f4, 0x700084f9e9, 0x700084f9e4);
        }
        61 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084f9e9); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084f9ed); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084f9f1); };
            c.w64(3, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084f9f8); }
            blk = 20; continue;
        }
        62 => {
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084fa03); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084fa04); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084fa05); }
            c.w32(0, 0x3u64);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700084fa0b); };
            c.w64(3, t61);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700084fa1a, 0x700084fa15);
        }
        63 => {
            c.w32(8, 0x3feu64);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fa20); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084fa24); };
            c.w64(3, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084fa2b); }
            blk = 20; continue;
        }
        64 => {
            c.w32(7, c.r32(7));
            let r64 = c.r32(7) | 0x100u64;
            c.w32(7, r64);
            c.logic(r64, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084fa41); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084fa42); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084fa43); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084fa44); }
            c.w32(0, 0x3u64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x700084fa4d); };
            c.w64(3, t65);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700084fa5c, 0x700084fa57);
        }
        65 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084fa5c); };
            c.w64(8, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fa60); };
            c.w64(1, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084fa64); };
            c.w64(3, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084fa6b); }
            blk = 20; continue;
        }
        66 => {
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084fa76); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084fa77); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084fa78); }
            c.w32(0, 0x3u64);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700084fa7e); };
            c.w64(3, t69);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700084fa8d, 0x700084fa88);
        }
        67 => {
            c.w32(8, 0x3feu64);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fa93); };
            c.w64(1, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084fa97); };
            c.w64(3, t71);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084fa9e); }
            blk = 20; continue;
        }
        68 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084faa9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084faaa); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084faab); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700084faac); };
            c.w64(3, t72);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084fabd, 0x700084fab8);
        }
        69 => {
            c.w32(8, 0x3feu64);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fac3); };
            c.w64(1, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084fac7); };
            c.w64(3, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084face); }
            blk = 20; continue;
        }
        70 => {
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084fad9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084fada); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084fadb); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700084fadc); };
            c.w64(3, t75);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084faed, 0x700084fae8);
        }
        71 => {
            c.w32(8, 0x3feu64);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084faf3); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084faf7); };
            c.w64(3, t77);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084fafe); }
            blk = 20; continue;
        }
        72 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084fb09); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084fb0a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084fb0b); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700084fb0c); };
            c.w64(3, t78);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084fb1d, 0x700084fb18);
        }
        73 => {
            c.w32(8, 0x3feu64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fb23); };
            c.w64(1, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084fb27); };
            c.w64(3, t80);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084fb2e); }
            blk = 20; continue;
        }
        74 => {
            return RegfileExit::jump(0x700084fb38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700084fb54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(8));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fb5e); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084fb62); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084fb66); };
            c.w64(3, t2);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084fb6d); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(8, 0x3feu64);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084fb85); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084fb8c); };
            c.cmp(t5, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084fba1); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 52; continue;
        }
        9 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084fbcd); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084fbd4); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 55; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084fbe2); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084fbe7); }
            blk = 15; continue;
        }
        13 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700084fbf0); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r12 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r12);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084fc01); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r13 = c.r32(7) | 0x6u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(7));
            let r14 = c.r32(7) | 0x2u64;
            c.w32(7, r14);
            c.logic(r14, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700084fc2a); };
            c.w64(0, t15);
            blk = 20; continue;
        }
        19 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084fc33); };
            c.w64(0, t16);
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084fc3a); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084fc3e); };
            c.w32(7, t18);
            c.w32(8, c.r32(8));
            c.w32(9, c.r32(7));
            let r19 = c.r32(8) | c.r32(9);
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700084fc53); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700084fc57); }
            blk = 22; continue;
        }
        22 => {
            return c.ret(0x0, 0x700084fc5e);
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084fc5f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084fc63); }
            let r20 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084fc69); };
            c.w32(11, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084fc6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084fc72); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700084fc76); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084fc7a); }
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084fc9b); };
            c.cmp(t22, c.r32(9), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084fca4); };
            c.cmp(t23, 0x43u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(7, c.r64(8));
            blk = 30; continue;
        }
        29 => {
            c.w32(7, c.r32(7));
            let r24 = c.r32(7) | 0x1eu64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084fcb9); }
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t25) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fcc5); };
            c.w64(9, t25);
            c.w64(6, c.r64(9));
            return c.call(0x70008a11d4, 0x700084fcd1, 0x700084fccc);
        }
        32 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fcd1); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fcd5); };
            c.w64(7, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700084fcd9); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084fcdd); };
            c.w64(0, t28);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084fce9, 0x700084fce4);
        }
        33 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084fce9); };
            c.w64(8, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084fced); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084fcf1); };
            c.w64(0, t30);
            blk = 1; continue;
        }
        34 => {
            let Some(t31) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fcfa); };
            c.w64(9, t31);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(9));
            return c.call(0x70008a2474, 0x700084fd09, 0x700084fd04);
        }
        35 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084fd09); };
            c.w64(8, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084fd0d); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084fd11); };
            c.w64(0, t33);
            blk = 1; continue;
        }
        36 => {
            let Some(t34) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084fd1a); };
            c.w32(9, t34);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fd3d); };
            c.w64(7, t35);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084fd4c, 0x700084fd47);
        }
        40 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fd4c); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fd50); };
            c.w64(7, t37);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084fd54); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084fd58); };
            c.w64(0, t38);
            c.w64(6, c.r64(7));
            return c.call(0x70008a11d4, 0x700084fd64, 0x700084fd5f);
        }
        41 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        42 => {
            let Some(t39) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fd6f); };
            c.w64(7, t39);
            c.w64(6, c.r64(7));
            return c.call(0x70008a2474, 0x700084fd7b, 0x700084fd76);
        }
        43 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        44 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084fd86); };
            c.cmp(t40, c.r32(9), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t41) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084fd93); };
            c.w32(9, t41);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t42) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fdae); };
            c.w64(7, t42);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084fdba, 0x700084fdb5);
        }
        48 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084fdc5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084fdc9); }
            let r43 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084fdcf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084fdd3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084fdd7); }
            let Some(t44) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084fddb); };
            c.cmp(t44, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t45) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fde7); };
            c.w64(7, t45);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084fdf3, 0x700084fdee);
        }
        51 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084fdfe); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084fe02); }
            let r46 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r46);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084fe08); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084fe0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700084fe10); }
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084fe14); };
            c.cmp(t47, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t48) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fe20); };
            c.w64(7, t48);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084fe2f, 0x700084fe2a);
        }
        54 => {
            c.w32(8, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700084fe35); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084fe39); };
            c.w64(0, t49);
            blk = 1; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084fe42); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084fe46); }
            let r50 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700084fe4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700084fe50); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084fe54); }
            let Some(t51) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084fe58); };
            c.cmp(t51, 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t52) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084fe64); };
            c.w64(7, t52);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700084fe70, 0x700084fe6b);
        }
        57 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        58 => {
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084fe80); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084fe81); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084fe82); }
            c.w32(0, 0x3u64);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700084fe88); };
            c.w64(3, t53);
            c.w64(9, c.r64(0));
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700084fe9a, 0x700084fe95);
        }
        59 => {
            c.w32(8, 0x3feu64);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fea0); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084fea4); };
            c.w64(3, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084feab); }
            blk = 20; continue;
        }
        60 => {
            c.w32(7, c.r32(7));
            let Some(t56) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084feb3); };
            c.w64(8, t56);
            let r57 = c.r32(7) | 0x60u64;
            c.w32(7, r57);
            c.logic(r57, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700084febd); }
            c.w64(6, c.r64(8));
            return c.call(0x70007fb774, 0x700084fec9, 0x700084fec4);
        }
        61 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084fec9); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084fecd); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084fed1); };
            c.w64(3, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084fed8); }
            blk = 20; continue;
        }
        62 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084fee3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084fee4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084fee5); }
            c.w32(0, 0x3u64);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x700084feeb); };
            c.w64(3, t61);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700084fefa, 0x700084fef5);
        }
        63 => {
            c.w32(8, 0x3feu64);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ff00); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ff04); };
            c.w64(3, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084ff0b); }
            blk = 20; continue;
        }
        64 => {
            c.w32(7, c.r32(7));
            let r64 = c.r32(7) | 0x100u64;
            c.w32(7, r64);
            c.logic(r64, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084ff21); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084ff22); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084ff23); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700084ff24); }
            c.w32(0, 0x3u64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x700084ff2d); };
            c.w64(3, t65);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700084ff3c, 0x700084ff37);
        }
        65 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084ff3c); };
            c.w64(8, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ff40); };
            c.w64(1, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ff44); };
            c.w64(3, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084ff4b); }
            blk = 20; continue;
        }
        66 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084ff56); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084ff57); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084ff58); }
            c.w32(0, 0x3u64);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700084ff5e); };
            c.w64(3, t69);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700084ff6d, 0x700084ff68);
        }
        67 => {
            c.w32(8, 0x3feu64);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ff73); };
            c.w64(1, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ff77); };
            c.w64(3, t71);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084ff7e); }
            blk = 20; continue;
        }
        68 => {
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084ff89); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084ff8a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084ff8b); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700084ff8c); };
            c.w64(3, t72);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084ff9d, 0x700084ff98);
        }
        69 => {
            c.w32(8, 0x3feu64);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ffa3); };
            c.w64(1, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ffa7); };
            c.w64(3, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084ffae); }
            blk = 20; continue;
        }
        70 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084ffb9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084ffba); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084ffbb); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700084ffbc); };
            c.w64(3, t75);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084ffcd, 0x700084ffc8);
        }
        71 => {
            c.w32(8, 0x3feu64);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084ffd3); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084ffd7); };
            c.w64(3, t77);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700084ffde); }
            blk = 20; continue;
        }
        72 => {
            c.w32(7, 0x24u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084ffe9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084ffea); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084ffeb); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700084ffec); };
            c.w64(3, t78);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700084fffd, 0x700084fff8);
        }
        73 => {
            c.w32(8, 0x3feu64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850003); };
            c.w64(1, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850007); };
            c.w64(3, t80);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085000e); }
            blk = 20; continue;
        }
        74 => {
            return RegfileExit::jump(0x7000850018);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000850034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(8));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085003e); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000850042); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850046); };
            c.w64(3, t2);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085004d); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(8, 0x3feu64);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850065); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085006c); };
            c.cmp(t5, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850081); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 52; continue;
        }
        9 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008500ad); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008500b4); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 55; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008500c2); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008500c7); }
            blk = 15; continue;
        }
        13 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008500d0); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r12 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r12);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008500e1); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r13 = c.r32(7) | 0x6u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(7));
            let r14 = c.r32(7) | 0x2u64;
            c.w32(7, r14);
            c.logic(r14, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085010a); };
            c.w64(0, t15);
            blk = 20; continue;
        }
        19 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000850113); };
            c.w64(0, t16);
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085011a); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085011e); };
            c.w32(7, t18);
            c.w32(8, c.r32(8));
            c.w32(9, c.r32(7));
            let r19 = c.r32(8) | c.r32(9);
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000850133); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000850137); }
            blk = 22; continue;
        }
        22 => {
            return c.ret(0x0, 0x700085013e);
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085013f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000850143); }
            let r20 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850149); };
            c.w32(11, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700085014e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000850152); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000850156); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700085015a); }
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085017b); };
            c.cmp(t22, c.r32(9), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850184); };
            c.cmp(t23, 0x43u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(7, c.r64(8));
            blk = 30; continue;
        }
        29 => {
            c.w32(7, c.r32(7));
            let r24 = c.r32(7) | 0x1eu64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000850199); }
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t25) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008501a5); };
            c.w64(9, t25);
            c.w64(6, c.r64(9));
            return c.call(0x70008a11d4, 0x70008501b1, 0x70008501ac);
        }
        32 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008501b1); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008501b5); };
            c.w64(7, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008501b9); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008501bd); };
            c.w64(0, t28);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x70008501c9, 0x70008501c4);
        }
        33 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008501c9); };
            c.w64(8, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008501cd); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008501d1); };
            c.w64(0, t30);
            blk = 1; continue;
        }
        34 => {
            let Some(t31) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008501da); };
            c.w64(9, t31);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(9));
            return c.call(0x70008a2474, 0x70008501e9, 0x70008501e4);
        }
        35 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008501e9); };
            c.w64(8, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008501ed); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008501f1); };
            c.w64(0, t33);
            blk = 1; continue;
        }
        36 => {
            let Some(t34) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008501fa); };
            c.w32(9, t34);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700085021d); };
            c.w64(7, t35);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700085022c, 0x7000850227);
        }
        40 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085022c); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850230); };
            c.w64(7, t37);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000850234); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000850238); };
            c.w64(0, t38);
            c.w64(6, c.r64(7));
            return c.call(0x70008a11d4, 0x7000850244, 0x700085023f);
        }
        41 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        42 => {
            let Some(t39) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700085024f); };
            c.w64(7, t39);
            c.w64(6, c.r64(7));
            return c.call(0x70008a2474, 0x700085025b, 0x7000850256);
        }
        43 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        44 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000850266); };
            c.cmp(t40, c.r32(9), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t41) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850273); };
            c.w32(9, t41);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t42) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700085028e); };
            c.w64(7, t42);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700085029a, 0x7000850295);
        }
        48 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008502a5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008502a9); }
            let r43 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008502af); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008502b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008502b7); }
            let Some(t44) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008502bb); };
            c.cmp(t44, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t45) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008502c7); };
            c.w64(7, t45);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x70008502d3, 0x70008502ce);
        }
        51 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008502de); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008502e2); }
            let r46 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r46);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008502e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008502ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008502f0); }
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008502f4); };
            c.cmp(t47, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t48) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850300); };
            c.w64(7, t48);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700085030f, 0x700085030a);
        }
        54 => {
            c.w32(8, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000850315); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000850319); };
            c.w64(0, t49);
            blk = 1; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000850322); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000850326); }
            let r50 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700085032c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000850330); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000850334); }
            let Some(t51) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850338); };
            c.cmp(t51, 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t52) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850344); };
            c.w64(7, t52);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x7000850350, 0x700085034b);
        }
        57 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        58 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850360); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850361); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000850362); }
            c.w32(0, 0x3u64);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000850368); };
            c.w64(3, t53);
            c.w64(9, c.r64(0));
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700085037a, 0x7000850375);
        }
        59 => {
            c.w32(8, 0x3feu64);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850380); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850384); };
            c.w64(3, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085038b); }
            blk = 20; continue;
        }
        60 => {
            c.w32(7, c.r32(7));
            let Some(t56) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850393); };
            c.w64(8, t56);
            let r57 = c.r32(7) | 0x60u64;
            c.w32(7, r57);
            c.logic(r57, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700085039d); }
            c.w64(6, c.r64(8));
            return c.call(0x70007fbb34, 0x70008503a9, 0x70008503a4);
        }
        61 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008503a9); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008503ad); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008503b1); };
            c.w64(3, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008503b8); }
            blk = 20; continue;
        }
        62 => {
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008503c3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008503c4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008503c5); }
            c.w32(0, 0x3u64);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008503cb); };
            c.w64(3, t61);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008503da, 0x70008503d5);
        }
        63 => {
            c.w32(8, 0x3feu64);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008503e0); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008503e4); };
            c.w64(3, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008503eb); }
            blk = 20; continue;
        }
        64 => {
            c.w32(7, c.r32(7));
            let r64 = c.r32(7) | 0x100u64;
            c.w32(7, r64);
            c.logic(r64, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850401); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000850402); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850403); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850404); }
            c.w32(0, 0x3u64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x700085040d); };
            c.w64(3, t65);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700085041c, 0x7000850417);
        }
        65 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085041c); };
            c.w64(8, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850420); };
            c.w64(1, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850424); };
            c.w64(3, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085042b); }
            blk = 20; continue;
        }
        66 => {
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850436); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000850437); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850438); }
            c.w32(0, 0x3u64);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085043e); };
            c.w64(3, t69);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700085044d, 0x7000850448);
        }
        67 => {
            c.w32(8, 0x3feu64);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850453); };
            c.w64(1, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850457); };
            c.w64(3, t71);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085045e); }
            blk = 20; continue;
        }
        68 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850469); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085046a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085046b); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085046c); };
            c.w64(3, t72);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085047d, 0x7000850478);
        }
        69 => {
            c.w32(8, 0x3feu64);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850483); };
            c.w64(1, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850487); };
            c.w64(3, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085048e); }
            blk = 20; continue;
        }
        70 => {
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850499); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085049a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085049b); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085049c); };
            c.w64(3, t75);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008504ad, 0x70008504a8);
        }
        71 => {
            c.w32(8, 0x3feu64);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008504b3); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008504b7); };
            c.w64(3, t77);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008504be); }
            blk = 20; continue;
        }
        72 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008504c9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008504ca); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008504cb); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008504cc); };
            c.w64(3, t78);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008504dd, 0x70008504d8);
        }
        73 => {
            c.w32(8, 0x3feu64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008504e3); };
            c.w64(1, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008504e7); };
            c.w64(3, t80);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008504ee); }
            blk = 20; continue;
        }
        74 => {
            return RegfileExit::jump(0x70008504f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000850514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            c.w64(7, c.r64(8));
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085051e); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000850522); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850526); };
            c.w64(3, t2);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085052d); }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(8, 0x3feu64);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850545); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085054c); };
            c.cmp(t5, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850561); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 52; continue;
        }
        9 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085058d); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000850594); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 55; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008505a2); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008505a7); }
            blk = 15; continue;
        }
        13 => {
            let r11 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r11);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008505b0); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 15; continue;
        }
        14 => {
            let r12 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r12);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008505c1); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 15; continue;
        }
        15 => {
            c.w32(7, c.r32(7));
            let r13 = c.r32(7) | 0x6u64;
            c.w32(7, r13);
            c.logic(r13, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.w32(7, c.r32(7));
            let r14 = c.r32(7) | 0x2u64;
            c.w32(7, r14);
            c.logic(r14, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_le() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008505ea); };
            c.w64(0, t15);
            blk = 20; continue;
        }
        19 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008505f3); };
            c.w64(0, t16);
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008505fa); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008505fe); };
            c.w32(7, t18);
            c.w32(8, c.r32(8));
            c.w32(9, c.r32(7));
            let r19 = c.r32(8) | c.r32(9);
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000850613); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000850617); }
            blk = 22; continue;
        }
        22 => {
            return c.ret(0x0, 0x700085061e);
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085061f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000850623); }
            let r20 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850629); };
            c.w32(11, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700085062e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000850632); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000850636); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700085063a); }
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085065b); };
            c.cmp(t22, c.r32(9), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850664); };
            c.cmp(t23, 0x43u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(7, c.r64(8));
            blk = 30; continue;
        }
        29 => {
            c.w32(7, c.r32(7));
            let r24 = c.r32(7) | 0x1eu64;
            c.w32(7, r24);
            c.logic(r24, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000850679); }
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t25) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850685); };
            c.w64(9, t25);
            c.w64(6, c.r64(9));
            return c.call(0x70008a11d4, 0x7000850691, 0x700085068c);
        }
        32 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850691); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850695); };
            c.w64(7, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000850699); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085069d); };
            c.w64(0, t28);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x70008506a9, 0x70008506a4);
        }
        33 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008506a9); };
            c.w64(8, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008506ad); }
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008506b1); };
            c.w64(0, t30);
            blk = 1; continue;
        }
        34 => {
            let Some(t31) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008506ba); };
            c.w64(9, t31);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(9));
            return c.call(0x70008a2474, 0x70008506c9, 0x70008506c4);
        }
        35 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008506c9); };
            c.w64(8, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008506cd); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008506d1); };
            c.w64(0, t33);
            blk = 1; continue;
        }
        36 => {
            let Some(t34) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008506da); };
            c.w32(9, t34);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t35) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008506fd); };
            c.w64(7, t35);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700085070c, 0x7000850707);
        }
        40 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085070c); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850710); };
            c.w64(7, t37);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000850714); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000850718); };
            c.w64(0, t38);
            c.w64(6, c.r64(7));
            return c.call(0x70008a11d4, 0x7000850724, 0x700085071f);
        }
        41 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        42 => {
            let Some(t39) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700085072f); };
            c.w64(7, t39);
            c.w64(6, c.r64(7));
            return c.call(0x70008a2474, 0x700085073b, 0x7000850736);
        }
        43 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        44 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000850746); };
            c.cmp(t40, c.r32(9), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t41) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850753); };
            c.w32(9, t41);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t42) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700085076e); };
            c.w64(7, t42);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x700085077a, 0x7000850775);
        }
        48 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000850785); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000850789); }
            let r43 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700085078f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000850793); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000850797); }
            let Some(t44) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085079b); };
            c.cmp(t44, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t45) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008507a7); };
            c.w64(7, t45);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x70008507b3, 0x70008507ae);
        }
        51 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008507be); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008507c2); }
            let r46 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r46);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008507c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008507cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008507d0); }
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008507d4); };
            c.cmp(t47, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t48) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008507e0); };
            c.w64(7, t48);
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x70008507ef, 0x70008507ea);
        }
        54 => {
            c.w32(8, 0x3feu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008507f5); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008507f9); };
            c.w64(0, t49);
            blk = 1; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000850802); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000850806); }
            let r50 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700085080c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000850810); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000850814); }
            let Some(t51) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850818); };
            c.cmp(t51, 0x41u64, 16);
            if c.cc_e() { blk = 72; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t52) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850824); };
            c.w64(7, t52);
            c.w64(6, c.r64(7));
            return c.call(0x70008a10d4, 0x7000850830, 0x700085082b);
        }
        57 => {
            c.w32(8, 0x3feu64);
            blk = 1; continue;
        }
        58 => {
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850840); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850841); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000850842); }
            c.w32(0, 0x3u64);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x7000850848); };
            c.w64(3, t53);
            c.w64(9, c.r64(0));
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700085085a, 0x7000850855);
        }
        59 => {
            c.w32(8, 0x3feu64);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850860); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850864); };
            c.w64(3, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085086b); }
            blk = 20; continue;
        }
        60 => {
            c.w32(7, c.r32(7));
            let Some(t56) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000850873); };
            c.w64(8, t56);
            let r57 = c.r32(7) | 0x60u64;
            c.w32(7, r57);
            c.logic(r57, 32);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700085087d); }
            c.w64(6, c.r64(8));
            return c.call(0x70007fb8b4, 0x7000850889, 0x7000850884);
        }
        61 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000850889); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085088d); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850891); };
            c.w64(3, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000850898); }
            blk = 20; continue;
        }
        62 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008508a3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008508a4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008508a5); }
            c.w32(0, 0x3u64);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1d38)) else { return RegfileExit::fault(0x70008508ab); };
            c.w64(3, t61);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008508ba, 0x70008508b5);
        }
        63 => {
            c.w32(8, 0x3feu64);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008508c0); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008508c4); };
            c.w64(3, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008508cb); }
            blk = 20; continue;
        }
        64 => {
            c.w32(7, c.r32(7));
            let r64 = c.r32(7) | 0x100u64;
            c.w32(7, r64);
            c.logic(r64, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008508e1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008508e2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008508e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008508e4); }
            c.w32(0, 0x3u64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d28)) else { return RegfileExit::fault(0x70008508ed); };
            c.w64(3, t65);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008508fc, 0x70008508f7);
        }
        65 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008508fc); };
            c.w64(8, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850900); };
            c.w64(1, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850904); };
            c.w64(3, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085090b); }
            blk = 20; continue;
        }
        66 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850916); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000850917); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850918); }
            c.w32(0, 0x3u64);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085091e); };
            c.w64(3, t69);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700085092d, 0x7000850928);
        }
        67 => {
            c.w32(8, 0x3feu64);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850933); };
            c.w64(1, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850937); };
            c.w64(3, t71);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085093e); }
            blk = 20; continue;
        }
        68 => {
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850949); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085094a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085094b); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085094c); };
            c.w64(3, t72);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085095d, 0x7000850958);
        }
        69 => {
            c.w32(8, 0x3feu64);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850963); };
            c.w64(1, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850967); };
            c.w64(3, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085096e); }
            blk = 20; continue;
        }
        70 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000850979); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085097a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085097b); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x700085097c); };
            c.w64(3, t75);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085098d, 0x7000850988);
        }
        71 => {
            c.w32(8, 0x3feu64);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850993); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850997); };
            c.w64(3, t77);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085099e); }
            blk = 20; continue;
        }
        72 => {
            c.w32(7, 0x26u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008509a9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008509aa); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008509ab); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1d30)) else { return RegfileExit::fault(0x70008509ac); };
            c.w64(3, t78);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008509bd, 0x70008509b8);
        }
        73 => {
            c.w32(8, 0x3feu64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008509c3); };
            c.w64(1, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008509c7); };
            c.w64(3, t80);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008509ce); }
            blk = 20; continue;
        }
        74 => {
            return RegfileExit::jump(0x70008509d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008509f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008509f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008509f8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008509fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000850a02); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(9, 0x60u64);
            c.w32(11, 0x20u64);
            c.w64(8, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850a48); };
            c.w32(12, t1);
            let r2 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850a4f); };
            c.w32(15, t3);
            let r4 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850a56); };
            c.w32(1, t5);
            let Some(t6) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850a5c); };
            c.w32(2, t6);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 95; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x42u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 87; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(2), 0xbau64, 16);
            if c.cc_ae() { blk = 58; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t7) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000850a96); };
            c.logic(t7 & 0x10u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 85; continue;
        }
        12 => {
            c.w32(1, c.r32(8));
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000850aa9); };
            c.cmp(t8, c.r32(12), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 13; continue;
        }
        13 => {
            let r9 = c.r32(1) | 0x8u64;
            c.w32(1, r9);
            c.logic(r9, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000850ac5); };
            c.w32(0, t10);
            let r11 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r11);
            blk = 1; continue;
        }
        15 => {
            c.w32(8, c.r32(8));
            let r12 = c.r32(8) | 0x6u64;
            c.w32(8, r12);
            c.logic(r12, 32);
            c.cmp(c.r16(1), c.r16(2), 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_b() { blk = 94; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r16(2), 0xbau64, 16);
            if c.cc_b() { blk = 69; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 26; continue;
        }
        21 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 26; continue;
        }
        22 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000850b0b); };
            c.cmp(t13, c.r32(15), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 23; continue;
        }
        23 => {
            let r14 = c.r32(8) | 0x8u64;
            c.w32(8, r14);
            c.logic(r14, 32);
            let Some(t15) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000850b1c); };
            c.w32(3, t15);
            let r16 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r16);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        24 => {
            c.cmp(c.r16(2), 0xbau64, 16);
            if c.cc_b() { blk = 71; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(8, 0x3feu64);
            blk = 26; continue;
        }
        26 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 1; continue;
        }
        27 => {
            c.w32(1, c.r32(8));
            let r17 = c.r32(1) | 0x2u64;
            c.w32(1, r17);
            c.logic(r17, 32);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850b59); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000850b60); };
            c.cmp(t20, c.r32(8), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850b6d); };
            c.w32(12, t21);
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 99; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 119; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r16(12), 0xbau64, 16);
            if c.cc_ae() { blk = 97; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 74; continue;
        }
        34 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000850ba0); };
            c.cmp(t22, c.r32(8), 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 35; continue;
        }
        35 => {
            let r23 = c.r32(1) | 0x8u64;
            c.w32(1, r23);
            c.logic(r23, 32);
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000850bb0); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        36 => {
            c.w32(1, c.r32(8));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850bcb); };
            c.w32(7, t26);
            let r27 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000850bd1); };
            c.cmp(t28, c.r32(7), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t29) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850bde); };
            c.w32(8, t29);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 48; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_ae() { blk = 47; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let r30 = c.r32(1) | 0x80u64;
            c.w32(1, r30);
            c.logic(r30, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        43 => {
            let r31 = c.r32(1) | 0x100u64;
            c.w32(1, r31);
            c.logic(r31, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        44 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000850c1c); };
            c.cmp(t32, c.r32(7), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let r33 = c.r32(1) | 0x210u64;
            c.w32(1, r33);
            c.logic(r33, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        46 => {
            let r34 = c.r32(1) | 0x8u64;
            c.w32(1, r34);
            c.logic(r34, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        47 => {
            let r35 = c.r32(1) | 0x200u64;
            c.w32(1, r35);
            c.logic(r35, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        48 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(11, c.r64(9));
            blk = 50; continue;
        }
        50 => {
            c.w32(7, c.r32(11));
            let r36 = c.r32(7) | c.r32(1);
            c.w32(7, r36);
            c.logic(r36, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        51 => {
            let r37 = c.r32(1) | 0x6u64;
            c.w32(1, r37);
            c.logic(r37, 32);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000850c65); }
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 81; continue; }
            blk = 52; continue;
        }
        52 => {
            if c.cc_e() { blk = 79; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 81; continue;
        }
        54 => {
            let r38 = c.r32(1) | 0x2u64;
            c.w32(1, r38);
            c.logic(r38, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        55 => {
            let Some(t39) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000850c8d); };
            c.logic(t39 & 0x10u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(8, 0x210u64);
            blk = 81; continue;
        }
        57 => {
            c.w32(8, 0x210u64);
            blk = 79; continue;
        }
        58 => {
            c.w32(1, c.r32(8));
            let r40 = c.r32(1) | 0x200u64;
            c.w32(1, r40);
            c.logic(r40, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        59 => {
            let r41 = c.r32(1) | 0x10u64;
            c.w32(1, r41);
            c.logic(r41, 32);
            let Some(t42) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000850cbf); };
            c.logic(t42 & 0x10u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r16(2), 0x42u64, 16);
            if c.cc_e() { blk = 66; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_e() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let r43 = c.r32(1) | 0x210u64;
            c.w32(1, r43);
            c.logic(r43, 32);
            c.cmp(c.r16(2), 0xbau64, 16);
            if c.cc_ae() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let r44 = c.r32(1) | 0x3feu64;
            c.w32(1, r44);
            c.logic(r44, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        64 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        65 => {
            let r45 = c.r32(1) | 0x18u64;
            c.w32(1, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        66 => {
            let r46 = c.r32(1) | 0x6u64;
            c.w32(1, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        67 => {
            let r47 = c.r32(1) | 0x210u64;
            c.w32(1, r47);
            c.logic(r47, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        68 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000850d1c); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000850d21); }
            blk = 77; continue;
        }
        69 => {
            let r48 = c.r32(8) | 0x3feu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        70 => {
            let r49 = c.r32(8) | 0x18u64;
            c.w32(8, r49);
            c.logic(r49, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        71 => {
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w32(8, 0x3feu64);
            blk = 81; continue;
        }
        73 => {
            c.w32(1, c.r32(8));
            let r50 = c.r32(1) | 0x80u64;
            c.w32(1, r50);
            c.logic(r50, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        74 => {
            let r51 = c.r32(1) | 0x3feu64;
            c.w32(1, r51);
            c.logic(r51, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        75 => {
            let r52 = c.r32(1) | 0x18u64;
            c.w32(1, r52);
            c.logic(r52, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        76 => {
            let r53 = c.r32(1) | 0x6u64;
            c.w32(1, r53);
            c.logic(r53, 32);
            let r54 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r54);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000850d74); }
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 77; continue;
        }
        77 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 81; continue; }
            blk = 78; continue;
        }
        78 => {
            if c.cc_ne() { blk = 81; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000850d8c); };
            c.w64(0, t55);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850d93); }
            blk = 82; continue;
        }
        80 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 81; continue;
        }
        81 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000850d9c); };
            c.w64(0, t56);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850da3); }
            blk = 82; continue;
        }
        82 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000850da7); };
            c.w64(1, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000850dab); };
            c.w64(7, t58);
            let Some(t59) = c.ld32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000850daf); };
            c.w32(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000850db4); };
            c.w64(9, t60);
            c.w32(9, c.r32(9));
            c.w32(11, c.r32(8));
            let r61 = c.r32(9) | c.r32(11);
            c.w32(9, r61);
            c.logic(r61, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000850dc9); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000850dce); }
            blk = 84; continue;
        }
        84 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000850dd8); }
            return c.ret(0x0, 0x7000850dd9);
        }
        85 => {
            c.w32(1, c.r32(8));
            let r62 = c.r32(1) | 0x3feu64;
            c.w32(1, r62);
            c.logic(r62, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000850de6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000850dea); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850dee); }
            return c.call(0x70008a2374, 0x7000850df7, 0x7000850df2);
        }
        86 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000850df7); };
            c.w64(8, t63);
            c.w32(9, 0x60u64);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000850e01); };
            c.w64(3, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000850e05); };
            c.w64(6, t65);
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(11, 0x20u64);
            blk = 1; continue;
        }
        87 => {
            c.w32(1, c.r32(8));
            let r66 = c.r32(1) | 0x100u64;
            c.w32(1, r66);
            c.logic(r66, 32);
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000850e1f); };
            c.cmp(t67, c.r32(15), 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 107; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_b() { blk = 105; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t68) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x7000850e40); };
            c.cmp(t68, c.r32(15), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            c.cmp(c.r16(2), 0xbau64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 81; continue;
        }
        93 => {
            let r69 = c.r32(1) | 0x8u64;
            c.w32(1, r69);
            c.logic(r69, 32);
            let Some(t70) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000850e5f); };
            c.w32(3, t70);
            let r71 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r71);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        94 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 100; continue;
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850e72); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000850e76); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000850e7a); }
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_b() { blk = 111; continue; }
            blk = 96; continue;
        }
        96 => {
            blk = 26; continue;
        }
        97 => {
            let r72 = c.r32(1) | 0x200u64;
            c.w32(1, r72);
            c.logic(r72, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000850e96); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000850e9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850e9e); }
            c.w64(0, c.r64(3));
            return c.call(0x70008a2374, 0x7000850eaa, 0x7000850ea5);
        }
        98 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000850eaa); };
            c.w64(8, t73);
            c.w64(3, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000850eb1); };
            c.w64(0, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000850eb5); };
            c.w64(6, t75);
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(11, 0x20u64);
            c.w32(9, 0x60u64);
            blk = 1; continue;
        }
        99 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 100; continue;
        }
        100 => {
            let Some(t76) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000850ecf); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000850ed6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000850eda); }
            let Some(t78) = c.ld8(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000850ede); };
            c.logic(t78 & 0x20u64, 8);
            if c.cc_e() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(8, c.r64(9));
            blk = 103; continue;
        }
        102 => {
            c.w64(8, c.r64(11));
            blk = 103; continue;
        }
        103 => {
            c.w32(8, c.r32(8));
            c.w32(1, c.r32(1));
            let r79 = c.r32(8) | c.r32(1);
            c.w32(8, r79);
            c.logic(r79, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850ef8); }
            c.w64(0, c.r64(3));
            return c.call(0x70008a0fd4, 0x7000850f04, 0x7000850eff);
        }
        104 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000850f04); };
            c.w64(8, t80);
            c.w64(3, c.r64(0));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000850f0b); };
            c.w64(0, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000850f0f); };
            c.w64(6, t82);
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(11, 0x20u64);
            c.w32(9, 0x60u64);
            blk = 1; continue;
        }
        105 => {
            let r83 = c.r32(1) | 0x60u64;
            c.w32(1, r83);
            c.logic(r83, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850f2c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000850f2d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850f2e); }
            c.w64(6, c.r64(7));
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x1d50)) else { return RegfileExit::fault(0x7000850f35); };
            c.w64(3, t84);
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000850f49, 0x7000850f44);
        }
        106 => {
            blk = 82; continue;
        }
        107 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850f51); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000850f52); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850f53); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x7000850f57); };
            c.w64(3, t85);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000850f6b, 0x7000850f66);
        }
        108 => {
            blk = 82; continue;
        }
        109 => {
            let r86 = c.r32(1) | 0x6u64;
            c.w32(1, r86);
            c.logic(r86, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850f76); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000850f77); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850f78); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x7000850f7c); };
            c.w64(3, t87);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000850f90, 0x7000850f8b);
        }
        110 => {
            blk = 82; continue;
        }
        111 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb534, 0x7000850fa0, 0x7000850f9b);
        }
        112 => {
            let Some(t88) = c.ld8(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000850fa0); };
            c.logic(t88 & 0x20u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w32(11, 0x60u64);
            blk = 115; continue;
        }
        114 => {
            c.w32(11, 0x20u64);
            blk = 115; continue;
        }
        115 => {
            let Some(t89) = c.ld8(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000850fb4); };
            c.logic(t89 & 0x20u64, 8);
            if c.cc_e() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(1, 0x60u64);
            blk = 118; continue;
        }
        117 => {
            c.w32(1, 0x20u64);
            blk = 118; continue;
        }
        118 => {
            c.w32(7, c.r32(11));
            c.w32(1, c.r32(1));
            let Some(t90) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000850fcb); };
            c.w32(8, t90);
            let r91 = c.r32(7) | c.r32(1);
            c.w32(7, r91);
            c.logic(r91, 32);
            let r92 = c.r32(7) | c.r32(8);
            c.w32(7, r92);
            c.logic(r92, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850fd7); }
            blk = 82; continue;
        }
        119 => {
            let r93 = c.r32(1) | 0x100u64;
            c.w32(1, r93);
            c.logic(r93, 32);
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000850fe9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000850fea); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000850feb); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x7000850fef); };
            c.w64(3, t94);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000851003, 0x7000850ffe);
        }
        120 => {
            blk = 82; continue;
        }
        121 => {
            return RegfileExit::jump(0x700085100c);
        }
            _ => unreachable!(),
        }
    }
}
