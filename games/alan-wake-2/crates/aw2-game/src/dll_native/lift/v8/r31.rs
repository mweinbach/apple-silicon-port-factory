#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008f4774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f4777); };
            c.w64(11, t0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f477b); };
            c.w32(0, t1);
            c.w64(3, c.r64(11));
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4784); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4793); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4799); };
            c.w32(1, t5);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.logic(c.r64(1) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f47b1); };
            c.w32(1, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f47b4); };
            c.cmp(t7, c.r32(1), 32);
            if c.cc_ne() { blk = 268; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f47c7); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let r10 = c.add(c.r32(0), 0xffffffffffffffffu64, 32);
            c.w32(0, r10);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008f47d4);
        }
        9 => {
            return RegfileExit::fault(0x70008f47d5);
        }
        10 => {
            return RegfileExit::fault(0x70008f47d6);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f47d7); };
            c.w64(11, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f47db); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f47e2); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f47e9); };
            c.w32(0, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f47ed); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f47f6); };
            c.w64(11, t18);
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f47fd); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f480a); };
            c.w32(0, t20);
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(3, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f4817); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            let r23 = c.r64(2) & 0xfffffffffffffffdu64;
            c.w64(2, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f4831); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f4837); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r27);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f4848); };
            c.w32(8, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f484d); };
            c.w64(1, t29);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f4856); };
            c.w64(8, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f485a); };
            c.w32(8, t31);
            let r32 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f4861); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4868); };
            c.w32(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008f487b);
        }
        20 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x70008f487c); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4889); };
            c.w32(0, t38);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            let r39 = c.r64(3) & 0xfffu64;
            c.w64(3, r39);
            c.logic(r39, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(2, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ae() { blk = 265; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f48ad); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f48b5); };
            c.w32(0, t42);
            if let Some(sr) = c.shr(c.r64(11), 0xcu64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r43 = c.r64(11) & 0x3ffffu64;
            c.w64(11, r43);
            c.logic(r43, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70008f48cf);
        }
        23 => {
            let r44 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f48d4); };
            c.w32(0, t45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f48d9); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f48e1); };
            c.w64(1, t48);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f48ea); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f48ee); }
            let r49 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f48f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f48f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008f48fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008f4900); }
            blk = 263; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f4909); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f490d); }
            let r50 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008f4913); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f491b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f491f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f4923); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f4927); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008f492b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008f492f); }
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008f4933); };
            c.cmp(t53, c.r32(7), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t54) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f4940); };
            c.w64(3, t54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4944); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f494a); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4950); };
            c.w32(1, t59);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008f4959); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x70008f4963);
        }
        28 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f4964); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70008f496a); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008f4974);
        }
        30 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f4975); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4985); };
            c.w32(1, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70008f4988); };
            c.cmp(t65, c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4991); };
            c.w32(1, t66);
            let r67 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r67);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f499f); };
            c.cmp(t68, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f49a9); };
            c.w32(1, t69);
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f49ac); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f49b6); };
            c.cmp(t72, c.r32(0), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 37; continue;
        }
        37 => {
            let r73 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r73);
            c.logic(r73, 64);
            blk = 46; continue;
        }
        38 => {
            c.w64(0, c.r64(2));
            blk = 46; continue;
        }
        39 => {
            let r74 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r74);
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f49dd); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f49e2); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f49e6); };
            c.w64(5, t77);
            let r78 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r78);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(6, c.r64(9).wrapping_add(0x4));
            let Some(t79) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f49f8); };
            c.w32(9, t79);
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f49fe); };
            c.w32(12, t80);
            let r81 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r81);
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4a06); };
            c.w32(9, t82);
            let r83 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r83);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008f4a0e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008f4a12); }
            c.w64(1, c.r64(9));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(11));
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f4a1f); };
            c.logic(t84 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r85 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r85);
            blk = 43; continue;
        }
        42 => {
            let Some(t86) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f4a2e); };
            c.w32(7, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008f4a31); };
            c.w64(7, t87);
            blk = 43; continue;
        }
        43 => {
            return c.call(c.r64(7), 0x70008f4a3b, 0x70008f4a39);
        }
        44 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4a3b); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4a3f); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f4a43); };
            c.w64(9, t90);
            let Some(t91) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f4a47); };
            c.w32(11, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4a4c); };
            c.w64(15, t92);
            let Some(t93) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4a50); };
            c.w64(1, t93);
            let Some(t94) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4a54); };
            c.w64(5, t94);
            c.w64(12, c.r64(8));
            let r95 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r95);
            return RegfileExit::jump(c.r64(1));
        }
        45 => {
            c.w64(0, c.r64(2));
            c.w64(1, c.r64(7));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 46; continue;
        }
        46 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(3, c.r64(1));
            let r96 = c.r64(3) & 0xfu64;
            c.w64(3, r96);
            c.logic(r96, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f4a77); }
            c.cmp(c.r64(3), 0x4u64, 64);
            if c.cc_e() { blk = 193; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(3), 0x5u64, 64);
            if c.cc_e() { blk = 192; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(3), 0xdu64, 64);
            if c.cc_e() { blk = 190; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_e() { blk = 129; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(3), 0x6u64, 64);
            if c.cc_e() { blk = 127; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 122; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(3), 0x8u64, 64);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(3), 0x9u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(3), 0x3u64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_e() { blk = 125; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r64(3), 0xcu64, 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(3), 0xeu64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(3, c.r64(9).wrapping_add(0x4));
            let Some(t97) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f4b02); };
            c.w32(2, t97);
            let Some(t98) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4b08); };
            c.w32(1, t98);
            let r99 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r99);
            let Some(t100) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4b10); };
            c.w32(2, t100);
            let r101 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r101);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f4b17); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f4b1b); }
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f4b1c); };
            if !c.push64(t102) { return RegfileExit::fault(0x70008f4b1c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f4b1f); }
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f4b20); };
            if !c.push64(t103) { return RegfileExit::fault(0x70008f4b20); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f4b23); };
            if !c.push64(t104) { return RegfileExit::fault(0x70008f4b23); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008f4b26); }
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f4b2a); };
            c.w64(6, t105);
            c.w32(0, 0x5u64);
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x70008f4b33); };
            c.w64(3, t106);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008f4b42, 0x70008f4b3d);
        }
        59 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4b42); };
            c.w64(8, t107);
            let Some(t108) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4b46); };
            c.w64(8, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008f4b4a); };
            c.w64(9, t109);
            let Some(t110) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f4b4e); };
            c.w32(11, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4b53); };
            c.w64(15, t111);
            let Some(t112) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4b57); };
            c.w64(1, t112);
            let Some(t113) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4b5b); };
            c.w64(5, t113);
            c.w64(12, c.r64(8));
            let r114 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r114);
            return RegfileExit::jump(c.r64(1));
        }
        60 => {
            let Some(t115) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f4b68); };
            c.w32(11, t115);
            let r116 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r116);
            let Some(t117) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4b6f); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4b76); };
            c.w32(0, t119);
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r120 = c.r64(1) & 0x1ffffffu64;
            c.w64(1, r120);
            c.logic(r120, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_b() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x70008f4b90);
        }
        62 => {
            let Some(t121) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4b91); };
            c.w32(11, t121);
            let r122 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r122);
            let Some(t123) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4b99); };
            c.w32(0, t123);
            let r124 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r124);
            c.w64(11, c.r64(9).wrapping_add(0x4));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008f4ba4); }
            let Some(t125) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f4ba8); };
            c.cmp(t125, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t126) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f4bb1); };
            c.w32(8, t126);
            let Some(t127) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4bb6); };
            c.w64(1, t127);
            let Some(t128) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4bba); };
            c.w64(5, t128);
            c.w64(9, c.r64(11));
            let r129 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r129);
            return RegfileExit::jump(c.r64(1));
        }
        64 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t130) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f4bcb); };
            c.w32(3, t130);
            let Some(t131) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4bd1); };
            c.w32(2, t131);
            let r132 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r132);
            let Some(t133) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4bd9); };
            c.w32(3, t133);
            let r134 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r134);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f4be0); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f4be9); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f4bea); }
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008f4beb); };
            c.w64(3, t135);
            c.w32(0, 0x2u64);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f4bf7); };
            c.w64(6, t136);
            return c.call(0x700086ed74, 0x70008f4c00, 0x70008f4bfb);
        }
        65 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4c00); };
            c.w64(8, t137);
            let Some(t138) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4c04); };
            c.w64(8, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f4c08); };
            c.w64(9, t139);
            let Some(t140) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f4c0c); };
            c.w32(11, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4c11); };
            c.w64(15, t141);
            let Some(t142) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4c15); };
            c.w64(1, t142);
            let Some(t143) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4c19); };
            c.w64(5, t143);
            c.w64(12, c.r64(8));
            let r144 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r144);
            return RegfileExit::jump(c.r64(1));
        }
        66 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(2, c.r64(9).wrapping_add(0x4));
            let Some(t145) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f4c2e); };
            c.w32(1, t145);
            let Some(t146) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4c34); };
            c.w32(6, t146);
            let r147 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r147);
            let Some(t148) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4c3c); };
            c.w32(1, t148);
            let r149 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r149);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f4c43); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008f4c47); }
            c.w64(3, c.r64(1));
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f4c4e); };
            c.w64(1, t150);
            c.w64(2, c.r64(11));
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f4c55); };
            c.w64(6, t151);
            return c.call(0x70008bc174, 0x70008f4c5e, 0x70008f4c59);
        }
        67 => {
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4c5e); };
            c.w64(8, t152);
            let Some(t153) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4c62); };
            c.w64(8, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f4c66); };
            c.w64(9, t154);
            let Some(t155) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f4c6a); };
            c.w32(11, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4c6f); };
            c.w64(15, t156);
            let Some(t157) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4c73); };
            c.w64(1, t157);
            let Some(t158) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4c77); };
            c.w64(5, t158);
            c.w64(12, c.r64(8));
            let r159 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r159);
            return RegfileExit::jump(c.r64(1));
        }
        68 => {
            blk = 125; continue;
        }
        69 => {
            let Some(t160) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f4c89); };
            c.w32(3, t160);
            let r161 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r161);
            let Some(t162) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4c8f); };
            c.w32(2, t162);
            let Some(t163) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f4c92); };
            c.cmp(t163, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t164) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f4c9f); };
            c.w32(1, t164);
            let Some(t165) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4ca5); };
            c.w32(6, t165);
            let r166 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r166);
            let Some(t167) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4cad); };
            c.w32(1, t167);
            let r168 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r168);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t169) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4cbd); };
            c.w32(2, t169);
            let r170 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f4cc3); };
            c.cmp(t171, c.r32(2), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t172) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4cd0); };
            c.w32(2, t172);
            let r173 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r173);
            let Some(t174) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4cd6); };
            c.w32(6, t174);
            let r175 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r175);
            let Some(t176) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f4cdc); };
            c.logic(t176 & 0x2u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t177) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f4ce6); };
            c.cmp(t177, c.r32(6), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f4cef); };
            c.w64(3, t178);
            blk = 109; continue;
        }
        75 => {
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f4cfb); };
            c.w64(6, t179);
            let Some(t180) = c.ld8(c.r64(6)) else { return RegfileExit::fault(0x70008f4d02); };
            c.cmp(t180, 0x20u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t181) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f4d0b); };
            c.w32(1, t181);
            let Some(t182) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f4d0e); };
            c.cmp(t182, c.r32(1), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t183) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4d1b); };
            c.w32(1, t183);
            let r184 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r184);
            let Some(t185) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4d21); };
            c.w32(6, t185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f4d24); };
            c.cmp(t186, c.r32(6), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t187) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f4d2d); };
            c.w32(1, t187);
            let r188 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r188);
            let Some(t189) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4d33); };
            c.w32(6, t189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f4d36); };
            c.cmp(t190, c.r32(6), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t191) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4d48); };
            c.w32(6, t191);
            let r192 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r192);
            let Some(t193) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4d4e); };
            c.w32(7, t193);
            c.w32(12, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w32(1, c.r16(7));
            c.cmp(c.r32(1), 0x82au64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f4d6a); };
            c.cmp(t194, c.r32(6), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f4d73); };
            c.w64(12, t195);
            blk = 263; continue;
        }
        84 => {
            c.w64(0, c.r64(3));
            blk = 86; continue;
        }
        85 => {
            c.w64(0, c.r64(1));
            blk = 86; continue;
        }
        86 => {
            let Some(t196) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f4d84); };
            c.w32(0, t196);
            let r197 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r197);
            let Some(t198) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4d8a); };
            c.w32(0, t198);
            let r199 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r199);
            let Some(t200) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4d90); };
            c.w32(3, t200);
            let r201 = c.sub(c.r32(3), 0x89u64, 32);
            c.w32(3, r201);
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_a() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t202) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f4da3); };
            c.w32(6, t202);
            let r203 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r203);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f4da9); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f4dad); };
            if !c.push64(t204) { return RegfileExit::fault(0x70008f4dad); }
            c.w64(1, c.r64(11));
            return c.call(0x70007f6fb4, 0x70008f4db8, 0x70008f4db3);
        }
        88 => {
            c.w64(3, c.r64(0));
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4dbb); };
            c.w64(15, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f4dbf); };
            c.w64(9, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4dc3); };
            c.w64(8, t207);
            blk = 109; continue;
        }
        89 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f4dcc); };
            c.w64(12, t208);
            blk = 263; continue;
        }
        90 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f4dd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f4ddd); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f4dde); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f4ddf); };
            if !c.push64(t209) { return RegfileExit::fault(0x70008f4ddf); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f4de2); };
            c.w64(6, t210);
            c.w32(0, 0x3u64);
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f4deb); };
            c.w64(3, t211);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008f4dfa, 0x70008f4df5);
        }
        91 => {
            c.w64(3, c.r64(0));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4dfd); };
            c.w64(15, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f4e01); };
            c.w64(9, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4e05); };
            c.w64(8, t214);
            blk = 109; continue;
        }
        92 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f4e15); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f4e19); };
            if !c.push64(t215) { return RegfileExit::fault(0x70008f4e19); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f4e1c); };
            c.w64(6, t216);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x70008f4e2a, 0x70008f4e25);
        }
        93 => {
            c.w64(3, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4e2d); };
            c.w64(15, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f4e31); };
            c.w64(9, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4e35); };
            c.w64(8, t219);
            blk = 109; continue;
        }
        94 => {
            let Some(t220) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4e3e); };
            c.w32(11, t220);
            let r221 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r221);
            let Some(t222) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4e45); };
            c.w32(1, t222);
            c.cmp(c.r16(1), 0x834u64, 16);
            if c.cc_e() { blk = 108; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(6, c.r64(1).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 101; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r16(1), 0x411u64, 16);
            if c.cc_ne() { blk = 263; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t223) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4e6b); };
            c.w32(11, t223);
            let Some(t224) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f4e6f); };
            c.cmp(t224, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t225) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4e85); };
            c.cmp(t225, 0x40u64, 16);
            if c.cc_ae() { blk = 263; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t226) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4e90); };
            c.w32(11, t226);
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            blk = 109; continue;
        }
        101 => {
            let Some(t227) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4e9d); };
            c.w32(2, t227);
            let Some(t228) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f4ea0); };
            c.cmp(t228, c.r32(2), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t229) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f4ead); };
            c.w32(11, t229);
            c.w64(2, c.r64(11));
            let r230 = c.r32(2) & 0xc0u64;
            c.w32(2, r230);
            c.logic(r230, 32);
            c.cmp(c.r32(2), 0xc0u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t231) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f4ec3); };
            c.w32(3, t231);
            let r232 = c.r32(3) & 0x1fu64;
            c.w32(3, r232);
            c.logic(r232, 32);
            let r233 = c.sub(c.r32(3), 0x10u64, 32);
            c.w32(3, r233);
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_a() { blk = 263; continue; }
            blk = 104; continue;
        }
        104 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 263; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t234) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f4edf); };
            c.w32(3, t234);
            let r235 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r235);
            let Some(t236) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f4ee5); };
            c.cmp(t236, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t237) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4ef2); };
            c.w32(11, t237);
            let Some(t238) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f4ef6); };
            c.cmp(t238, c.r32(11), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t239) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f4eff); };
            c.w32(3, t239);
            let r240 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r240);
            blk = 109; continue;
        }
        108 => {
            let Some(t241) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4f07); };
            c.w32(11, t241);
            let Some(t242) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f4f0b); };
            c.cmp(t242, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 109; continue;
        }
        109 => {
            let r243 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r243);
            let Some(t244) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4f1c); };
            c.w64(8, t244);
            let Some(t245) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f4f20); };
            c.w32(11, t245);
            let Some(t246) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4f25); };
            c.w64(1, t246);
            let Some(t247) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4f29); };
            c.w64(5, t247);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(3));
            let r248 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r248);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 263; continue;
        }
        111 => {
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f4f3e); };
            c.w64(3, t249);
            let Some(t250) = c.ld8(c.r64(3)) else { return RegfileExit::fault(0x70008f4f45); };
            c.cmp(t250, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t251) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f4f53); };
            c.w32(3, t251);
            let r252 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r252);
            blk = 115; continue;
        }
        114 => {
            let Some(t253) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f4f5b); };
            c.w32(3, t253);
            let r254 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r254);
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return RegfileExit::fault(0x70008f4f66);
        }
        117 => {
            let r255 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r255);
            c.logic(r255, 64);
            let Some(t256) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4f6b); };
            c.w32(1, t256);
            let r257 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r257);
            let Some(t258) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f4f71); };
            c.w32(0, t258);
            let r259 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r259);
            let Some(t260) = c.ld64(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4f77); };
            c.w64(1, t260);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t261) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4f80); };
            c.w32(2, t261);
            let r262 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r262);
            let Some(t263) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f4f86); };
            c.w32(2, t263);
            let r264 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r264);
            blk = 119; continue;
        }
        119 => {
            c.w32(6, c.r64(9).wrapping_add(c.r64(9)));
            let r265 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r265);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f4f94); }
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f4f98); };
            if !c.push64(t266) { return RegfileExit::fault(0x70008f4f98); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f4f9b); }
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            return c.call(0x70007fe8d4, 0x70008f4fb3, 0x70008f4fae);
        }
        120 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4fb3); };
            c.w64(8, t267);
            let Some(t268) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4fb7); };
            c.w64(8, t268);
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f4fbb); };
            c.w64(9, t269);
            let Some(t270) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f4fbf); };
            c.w32(11, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4fc4); };
            c.w64(15, t271);
            let Some(t272) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4fc8); };
            c.w64(1, t272);
            let Some(t273) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4fcc); };
            c.w64(5, t273);
            c.w64(12, c.r64(8));
            let r274 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r274);
            return RegfileExit::jump(c.r64(1));
        }
        121 => {
            blk = 125; continue;
        }
        122 => {
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f4fdb); };
            c.w64(11, t275);
            let Some(t276) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f4fe2); };
            c.cmp(t276, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let r277 = c.r64(1) & 0x3ffu64;
            c.w64(1, r277);
            c.logic(r277, 64);
            let r278 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r278);
            let r279 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r279);
            let Some(t280) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4fff); };
            c.w32(2, t280);
            let r281 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r281);
            let Some(t282) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f5005); };
            c.w32(2, t282);
            let r283 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r283);
            let Some(t284) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f500b); };
            c.w32(3, t284);
            let r285 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r285);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f5012); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f5016); }
            c.w64(2, c.r64(6));
            c.w64(1, c.r64(0));
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f5020); };
            c.w64(6, t286);
            return c.call(0x70007fea14, 0x70008f5029, 0x70008f5024);
        }
        124 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5029); };
            c.w64(8, t287);
            let Some(t288) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f502d); };
            c.w64(8, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f5031); };
            c.w64(9, t289);
            let Some(t290) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f5035); };
            c.w32(11, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f503a); };
            c.w64(15, t291);
            let Some(t292) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f503e); };
            c.w64(1, t292);
            let Some(t293) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5042); };
            c.w64(5, t293);
            c.w64(12, c.r64(8));
            let r294 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r294);
            return RegfileExit::jump(c.r64(1));
        }
        125 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x4));
            let Some(t295) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f5057); };
            c.w32(3, t295);
            let Some(t296) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f505d); };
            c.w32(2, t296);
            let r297 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r297);
            let Some(t298) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5065); };
            c.w32(3, t298);
            let r299 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r299);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f506c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f5070); }
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f5071); };
            if !c.push64(t300) { return RegfileExit::fault(0x70008f5071); }
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f5074); };
            if !c.push64(t301) { return RegfileExit::fault(0x70008f5074); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f5077); }
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x70008f507b); };
            c.w64(3, t302);
            c.w32(0, 0x3u64);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f5087); };
            c.w64(6, t303);
            return c.call(0x700086ed74, 0x70008f5090, 0x70008f508b);
        }
        126 => {
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5090); };
            c.w64(8, t304);
            let Some(t305) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5094); };
            c.w64(8, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f5098); };
            c.w64(9, t306);
            let Some(t307) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f509c); };
            c.w32(11, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f50a1); };
            c.w64(15, t308);
            let Some(t309) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f50a5); };
            c.w64(1, t309);
            let Some(t310) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f50a9); };
            c.w64(5, t310);
            c.w64(12, c.r64(8));
            let r311 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r311);
            return RegfileExit::jump(c.r64(1));
        }
        127 => {
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let r312 = c.r64(1) & 0x3ffu64;
            c.w64(1, r312);
            c.logic(r312, 64);
            let r313 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r313);
            let r314 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r314);
            let Some(t315) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f50cd); };
            c.w32(0, t315);
            let r316 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r316);
            let Some(t317) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f50d3); };
            c.w32(0, t317);
            let r318 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r318);
            let Some(t319) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f50d9); };
            c.w32(0, t319);
            let r320 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r320);
            let Some(t321) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f50e0); };
            c.w32(7, t321);
            let r322 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r322);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f50e6); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008f50ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f50eb); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f50f1); };
            c.w64(6, t323);
            return c.call(0x70007f2274, 0x70008f50fa, 0x70008f50f5);
        }
        128 => {
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f50fa); };
            c.w64(8, t324);
            let Some(t325) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f50fe); };
            c.w64(8, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f5102); };
            c.w64(9, t326);
            let Some(t327) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f5106); };
            c.w32(11, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f510b); };
            c.w64(15, t328);
            let Some(t329) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f510f); };
            c.w64(1, t329);
            let Some(t330) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5113); };
            c.w64(5, t330);
            c.w64(12, c.r64(8));
            let r331 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r331);
            return RegfileExit::jump(c.r64(1));
        }
        129 => {
            let Some(t332) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5120); };
            c.w32(3, t332);
            let r333 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r333);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008f512b); };
            c.w64(3, t334);
            blk = 131; continue;
        }
        131 => {
            let Some(t335) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f5132); };
            c.w32(2, t335);
            let Some(t336) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f5135); };
            c.w32(1, t336);
            let Some(t337) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f513b); };
            c.w32(7, t337);
            let r338 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r338);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t339) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5146); };
            c.w32(1, t339);
            let r340 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r340);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r341 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r341);
            let Some(t342) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5154); };
            c.w32(6, t342);
            let r343 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r343);
            let r344 = c.r64(6) & c.r64(2);
            c.w64(6, r344);
            c.logic(r344, 64);
            blk = 132; continue;
        }
        132 => {
            c.w64(15, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(8, c.r64(15).wrapping_mul(4));
            let Some(t345) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f516a); };
            c.w32(0, t345);
            let Some(t346) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f516f); };
            c.cmp(t346, c.r32(0), 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r347 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r347);
            c.w64(8, c.r64(11).wrapping_add(c.r64(6)));
            c.w64(0, c.r64(2));
            let r348 = c.r64(0) & c.r64(8);
            c.w64(0, r348);
            c.logic(r348, 64);
            c.w64(6, c.r64(0));
            blk = 132; continue;
        }
        135 => {
            let Some(t349) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5193); };
            c.w32(11, t349);
            let Some(t350) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f5197); };
            c.w32(0, t350);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(2, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_b() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            return RegfileExit::fault(0x70008f51ab);
        }
        137 => {
            let Some(t351) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f51ac); };
            c.w32(11, t351);
            let Some(t352) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f51b2); };
            c.w32(8, t352);
            let r353 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r353);
            let Some(t354) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f51ba); };
            c.w32(11, t354);
            let r355 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r355);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t356) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f51ca); };
            c.w32(0, t356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f51ce); };
            c.cmp(t357, c.r32(0), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t358) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f51db); };
            c.w32(2, t358);
            let r359 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r359);
            let Some(t360) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f51e2); };
            c.w32(8, t360);
            let r361 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r361);
            let Some(t362) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f51e9); };
            c.logic(t362 & 0x2u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t363) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f51f4); };
            c.cmp(t363, c.r32(8), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f51fd); };
            c.w64(8, t364);
            blk = 188; continue;
        }
        142 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f5209); };
            c.w64(8, t365);
            let Some(t366) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x70008f5210); };
            c.cmp(t366, 0x20u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t367) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f521a); };
            c.w32(8, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f521e); };
            c.cmp(t368, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5227); };
            c.w64(8, t369);
            blk = 263; continue;
        }
        145 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f5230); };
            c.w64(0, t370);
            let Some(t371) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5234); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f523b); };
            c.w32(11, t373);
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f523f); };
            c.cmp(t374, c.r32(11), 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t375) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f5248); };
            c.w32(8, t375);
            let r376 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r376);
            let Some(t377) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f524f); };
            c.w32(11, t377);
            let Some(t378) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f5253); };
            c.cmp(t378, c.r32(11), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 147; continue;
        }
        147 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t379) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5266); };
            c.w32(11, t379);
            let r380 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r380);
            let Some(t381) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f526d); };
            c.w32(3, t381);
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 153; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w32(8, c.r16(3));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t382) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f528a); };
            c.cmp(t382, c.r32(11), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5293); };
            c.w64(8, t383);
            blk = 263; continue;
        }
        152 => {
            let Some(t384) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f529c); };
            c.w32(0, t384);
            let r385 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r385);
            blk = 154; continue;
        }
        153 => {
            c.w64(0, c.r64(8));
            blk = 154; continue;
        }
        154 => {
            let Some(t386) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f52a7); };
            c.w32(8, t386);
            let r387 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f52ae); };
            c.w32(8, t388);
            let r389 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r389);
            let Some(t390) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f52b5); };
            c.w32(11, t390);
            let r391 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r391);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t392) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f52cb); };
            c.w32(6, t392);
            let r393 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r393);
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f52d2); };
            c.w64(8, t394);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f52d6); }
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f52da); };
            if !c.push64(t395) { return RegfileExit::fault(0x70008f52da); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008f52e4, 0x70008f52df);
        }
        156 => {
            c.w64(8, c.r64(0));
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f52e7); };
            c.w64(9, t396);
            blk = 188; continue;
        }
        157 => {
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f52f0); };
            c.w64(8, t397);
            blk = 263; continue;
        }
        158 => {
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f52f9); };
            c.w64(8, t398);
            blk = 263; continue;
        }
        159 => {
            c.w64(8, c.r64(9));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5309); };
            c.w64(3, t399);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f530d); }
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f5311); };
            if !c.push64(t400) { return RegfileExit::fault(0x70008f5311); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f5314); }
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f5316); };
            if !c.push64(t401) { return RegfileExit::fault(0x70008f5316); }
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f5319); };
            c.w64(6, t402);
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f531d); };
            c.w64(3, t403);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008f5331, 0x70008f532c);
        }
        160 => {
            c.w64(8, c.r64(0));
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f5334); };
            c.w64(9, t404);
            blk = 188; continue;
        }
        161 => {
            c.w64(8, c.r64(9));
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5344); };
            c.w64(0, t405);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f5348); }
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f534c); };
            if !c.push64(t406) { return RegfileExit::fault(0x70008f534c); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f5354); };
            c.w64(6, t407);
            return c.call(0x70007f2274, 0x70008f535d, 0x70008f5358);
        }
        162 => {
            c.w64(8, c.r64(0));
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f5360); };
            c.w64(9, t408);
            blk = 188; continue;
        }
        163 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f5369); };
            c.w64(0, t409);
            let Some(t410) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f536d); };
            c.w32(11, t410);
            let r411 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r411);
            let Some(t412) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5374); };
            c.w32(3, t412);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 185; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w32(2, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 174; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5396); };
            c.w64(8, t413);
            blk = 263; continue;
        }
        167 => {
            let Some(t414) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f539f); };
            c.w32(8, t414);
            let Some(t415) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f53a3); };
            c.cmp(t415, c.r32(8), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f53ac); };
            c.w64(8, t416);
            blk = 263; continue;
        }
        169 => {
            let Some(t417) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f53b5); };
            c.w32(8, t417);
            let r418 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r418);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f53c2); };
            c.w64(8, t419);
            blk = 263; continue;
        }
        171 => {
            let Some(t420) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f53cb); };
            c.w32(11, t420);
            let r421 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r421);
            let Some(t422) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f53d2); };
            c.cmp(t422, 0x40u64, 16);
            if c.cc_b() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f53da); };
            c.w64(8, t423);
            blk = 263; continue;
        }
        173 => {
            let Some(t424) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f53e3); };
            c.w32(8, t424);
            let r425 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r425);
            blk = 188; continue;
        }
        174 => {
            let Some(t426) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f53ef); };
            c.w32(8, t426);
            let Some(t427) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f53f3); };
            c.cmp(t427, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f53fc); };
            c.w64(8, t428);
            blk = 263; continue;
        }
        176 => {
            let Some(t429) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f5405); };
            c.w32(8, t429);
            c.w64(11, c.r64(8));
            let r430 = c.r32(11) & 0xc0u64;
            c.w32(11, r430);
            c.logic(r430, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t431) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f541d); };
            c.w32(11, t431);
            let r432 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r432);
            let Some(t433) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f5424); };
            c.w32(11, t433);
            let r434 = c.r32(11) & 0x1fu64;
            c.w32(11, r434);
            c.logic(r434, 32);
            let r435 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r435);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5436); };
            c.w64(8, t436);
            blk = 263; continue;
        }
        179 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 184; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t437) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f5445); };
            c.w32(8, t437);
            let r438 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r438);
            let Some(t439) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f544c); };
            c.cmp(t439, c.r32(8), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t440) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5455); };
            c.w32(11, t440);
            let Some(t441) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f5459); };
            c.cmp(t441, c.r32(11), 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t442) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f5462); };
            c.w32(8, t442);
            let r443 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r443);
            blk = 188; continue;
        }
        183 => {
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f546b); };
            c.w64(8, t444);
            blk = 263; continue;
        }
        184 => {
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5474); };
            c.w64(8, t445);
            blk = 263; continue;
        }
        185 => {
            let Some(t446) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f547d); };
            c.w32(8, t446);
            let Some(t447) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f5481); };
            c.cmp(t447, c.r32(8), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f548a); };
            c.w64(8, t448);
            blk = 263; continue;
        }
        187 => {
            let Some(t449) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f5493); };
            c.w32(8, t449);
            let r450 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r450);
            blk = 188; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f54a1); };
            c.w64(0, t451);
            let Some(t452) = c.ld64(c.r64(0).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f54a5); };
            c.w64(11, t452);
            let Some(t453) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f54a9); };
            c.w32(0, t453);
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f54ae); };
            c.w64(15, t454);
            let Some(t455) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f54b2); };
            c.w64(1, t455);
            let Some(t456) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f54b6); };
            c.w64(5, t456);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            let r457 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r457);
            return RegfileExit::jump(c.r64(1));
        }
        189 => {
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f54c6); };
            c.w64(8, t458);
            blk = 263; continue;
        }
        190 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x4));
            let Some(t459) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f54d7); };
            c.w64(3, t459);
            let Some(t460) = c.ld8(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f54db); };
            c.w32(2, t460);
            let Some(t461) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f54e1); };
            c.w32(3, t461);
            let r462 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r462);
            let Some(t463) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f54e7); };
            c.w32(3, t463);
            let r464 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r464);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f54ee); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f54f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f54f3); }
            let Some(t465) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008f54f7); };
            c.w64(3, t465);
            let Some(t466) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f54fe); };
            c.w64(6, t466);
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008f550f, 0x70008f550a);
        }
        191 => {
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f550f); };
            c.w64(8, t467);
            let Some(t468) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5513); };
            c.w64(8, t468);
            let Some(t469) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f5517); };
            c.w64(9, t469);
            let Some(t470) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f551b); };
            c.w32(11, t470);
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5520); };
            c.w64(15, t471);
            let Some(t472) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5524); };
            c.w64(1, t472);
            let Some(t473) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5528); };
            c.w64(5, t473);
            c.w64(12, c.r64(8));
            let r474 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r474);
            return RegfileExit::jump(c.r64(1));
        }
        192 => {
            let r475 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r475);
            let Some(t476) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f5539); };
            c.w32(8, t476);
            let Some(t477) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f553e); };
            c.w64(1, t477);
            let Some(t478) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5542); };
            c.w64(5, t478);
            let r479 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r479);
            return RegfileExit::jump(c.r64(1));
        }
        193 => {
            c.logic(c.r8(1) & 0x40u64, 8);
            if c.cc_ne() { blk = 207; continue; }
            blk = 194; continue;
        }
        194 => {
            c.w64(11, c.r64(1));
            if let Some(sr) = c.shr(c.r64(11), 0x9u64, 64) { c.w64(11, sr); }
            c.logic(c.r16(1) & 0x100u64, 16);
            c.w8(3, (c.cc_ne() as u64));
            c.w32(3, c.r8(3));
            let r480 = c.r64(11) & 0x7ffu64;
            c.w64(11, r480);
            c.logic(r480, 64);
            c.logic(c.r8(1) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 202; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t481) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5573); };
            c.w32(0, t481);
            let r482 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r482);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t483) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008f557d); };
            c.w64(0, t483);
            blk = 197; continue;
        }
        197 => {
            let Some(t484) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5584); };
            c.w32(0, t484);
            let r485 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r485);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let r486 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r486);
            let Some(t487) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f5595); };
            c.w32(8, t487);
            let Some(t488) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f559a); };
            c.w64(1, t488);
            let Some(t489) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f559e); };
            c.w64(5, t489);
            let r490 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r490);
            return RegfileExit::jump(c.r64(1));
        }
        199 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t491) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f55b0); };
            c.w32(11, t491);
            let Some(t492) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f55b4); };
            c.cmp(t492, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 201; continue;
        }
        201 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f55c1); }
            blk = 255; continue;
        }
        202 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            let r493 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r493);
            let Some(t494) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f55d4); };
            c.w32(8, t494);
            let Some(t495) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f55d9); };
            c.w32(0, t495);
            let r496 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r496);
            let Some(t497) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f55e1); };
            c.w64(1, t497);
            let Some(t498) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f55e5); };
            c.w64(5, t498);
            let r499 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r499);
            return RegfileExit::jump(c.r64(1));
        }
        204 => {
            let Some(t500) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f55ef); };
            c.w32(11, t500);
            let r501 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r501);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t502) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5601); };
            c.w32(0, t502);
            let Some(t503) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f5605); };
            c.cmp(t503, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 206; continue;
        }
        206 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f5612); }
            blk = 255; continue;
        }
        207 => {
            c.w64(3, c.r64(1));
            let r504 = c.shr(c.r32(3), 0x7u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r504);
            let r505 = c.r32(3) & 0xfu64;
            c.w32(3, r505);
            c.logic(r505, 32);
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_l() { blk = 214; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_l() { blk = 212; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_e() { blk = 261; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(3), 0x6u64, 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 218; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 231; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 220; continue;
        }
        220 => {
            return RegfileExit::fault(0x70008f5683);
        }
        221 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r506 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r506);
            let r507 = c.r64(1) & 0xfffffu64;
            c.w64(1, r507);
            c.logic(r507, 64);
            let Some(t508) = c.ld8(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5693); };
            c.w32(8, (t508 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t509) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f569d); };
            c.w32(8, t509);
            let Some(t510) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f56a2); };
            c.w64(1, t510);
            let Some(t511) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f56a6); };
            c.w64(5, t511);
            let r512 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r512);
            return RegfileExit::jump(c.r64(1));
        }
        222 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r513 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r513);
            let r514 = c.r64(1) & 0xfffffu64;
            c.w64(1, r514);
            c.logic(r514, 64);
            let Some(t515) = c.ld16(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f56bf); };
            c.w32(8, (t515 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t516) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f56c9); };
            c.w32(8, t516);
            let Some(t517) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f56ce); };
            c.w64(1, t517);
            let Some(t518) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f56d2); };
            c.w64(5, t518);
            let r519 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r519);
            return RegfileExit::jump(c.r64(1));
        }
        223 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r520 = c.r64(1) & 0xfffffu64;
            c.w64(1, r520);
            c.logic(r520, 64);
            let Some(t521) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f56e7); };
            c.w32(8, t521);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f56ec); }
            c.w64(11, c.r64(8));
            let r522 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r522);
            if c.cc_o() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 230; continue;
        }
        225 => {
            let Some(t523) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f5700); };
            c.w64(11, t523);
            let Some(t524) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f5707); };
            c.w64(0, t524);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t525) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f570e); };
            c.w64(2, t525);
            let Some(t526) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f5715); };
            c.cmp(t526, c.r64(3), 64);
            if c.cc_a() { blk = 228; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f5724, 0x70008f571f);
        }
        227 => {
            let Some(t527) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5724); };
            c.w64(8, t527);
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5728); };
            c.w64(15, t528);
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f572c); };
            c.w64(9, t529);
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f5730); };
            c.w64(12, t530);
            blk = 229; continue;
        }
        228 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f5736); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 229; continue;
        }
        229 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t531) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f5749); };
            c.w64(8, t531);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f5750); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f5754); }
            blk = 230; continue;
        }
        230 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x4));
            let Some(t532) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f5760); };
            c.w32(8, t532);
            let Some(t533) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5765); };
            c.w64(1, t533);
            let Some(t534) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5769); };
            c.w64(5, t534);
            let r535 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r535);
            return RegfileExit::jump(c.r64(1));
        }
        231 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r536 = c.r64(1) & 0xfffffu64;
            c.w64(1, r536);
            c.logic(r536, 64);
            let Some(t537) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f577e); };
            c.w32(8, t537);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f5783); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let r538 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r538);
            blk = 238; continue;
        }
        233 => {
            let Some(t539) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f5795); };
            c.w64(11, t539);
            let Some(t540) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f579c); };
            c.w64(0, t540);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t541) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f57a3); };
            c.w64(2, t541);
            let Some(t542) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f57aa); };
            c.cmp(t542, c.r64(3), 64);
            if c.cc_a() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f57b9, 0x70008f57b4);
        }
        235 => {
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f57b9); };
            c.w64(8, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f57bd); };
            c.w64(15, t544);
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f57c1); };
            c.w64(9, t545);
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f57c5); };
            c.w64(12, t546);
            blk = 237; continue;
        }
        236 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f57cb); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 237; continue;
        }
        237 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t547) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f57e1); };
            c.w64(8, t547);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f57e8); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f57ec); }
            c.w64(8, c.r64(0));
            blk = 238; continue;
        }
        238 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t548) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f57fb); };
            c.w32(11, t548);
            let Some(t549) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5800); };
            c.w64(1, t549);
            let Some(t550) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5804); };
            c.w64(5, t550);
            c.w64(0, c.r64(8));
            let r551 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r551);
            return RegfileExit::jump(c.r64(1));
        }
        239 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r552 = c.r64(1) & 0xfffffu64;
            c.w64(1, r552);
            c.logic(r552, 64);
            let Some(t553) = c.ld64(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f581c); };
            c.w64(8, t553);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f5821); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 247; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t554) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f582b); };
            c.w64(11, t554);
            let Some(t555) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f5832); };
            c.w64(0, t555);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t556) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f5839); };
            c.w64(2, t556);
            let Some(t557) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f5840); };
            c.cmp(t557, c.r64(3), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x70008f584f, 0x70008f584a);
        }
        242 => {
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f584f); };
            c.w64(8, t558);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5853); };
            c.w64(15, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f5857); };
            c.w64(9, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f585b); };
            c.w64(12, t561);
            blk = 244; continue;
        }
        243 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f5861); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 244; continue;
        }
        244 => {
            let Some(t562) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f586b); };
            c.w64(3, t562);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008f5872); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008f587b); }
            let r563 = c.neg(c.r64(8), 64);
            c.w64(8, r563);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008f5885); }
            blk = 252; continue;
        }
        246 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008f588b); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008f5892); }
            blk = 252; continue;
        }
        247 => {
            let Some(t564) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f5898); };
            c.w64(8, t564);
            let Some(t565) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f589f); };
            c.w64(11, t565);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t566) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f58a6); };
            c.w64(0, t566);
            let Some(t567) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f58ad); };
            c.cmp(t567, c.r64(3), 64);
            if c.cc_a() { blk = 250; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008f58bc, 0x70008f58b7);
        }
        249 => {
            blk = 251; continue;
        }
        250 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008f58be); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 251; continue;
        }
        251 => {
            let Some(t568) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f58c8); };
            c.w64(8, t568);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f58cf); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008f58d3); }
            let Some(t569) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f58da); };
            c.w64(12, t569);
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f58de); };
            c.w64(15, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f58e2); };
            c.w64(9, t571);
            blk = 252; continue;
        }
        252 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x4));
            let Some(t572) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f58ed); };
            c.w32(8, t572);
            let Some(t573) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f58f2); };
            c.w64(1, t573);
            let Some(t574) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f58f6); };
            c.w64(5, t574);
            let r575 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r575);
            return RegfileExit::jump(c.r64(1));
        }
        253 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r576 = c.r64(1) & 0xfffffu64;
            c.w64(1, r576);
            c.logic(r576, 64);
            if !c.movss_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f590b); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 255; continue;
        }
        254 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r577 = c.r64(1) & 0xfffffu64;
            c.w64(1, r577);
            c.logic(r577, 64);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f5922); }
            blk = 255; continue;
        }
        255 => {
            let Some(t578) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f5928); };
            c.w64(8, t578);
            let Some(t579) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f592f); };
            c.w64(11, t579);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008f5936); }
            let Some(t580) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f593b); };
            c.w64(3, t580);
            let Some(t581) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f5942); };
            c.cmp(t581, c.r64(0), 64);
            if c.cc_a() { blk = 258; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f5951, 0x70008f594c);
        }
        257 => {
            blk = 259; continue;
        }
        258 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008f5953); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 259; continue;
        }
        259 => {
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f595d); };
            c.w64(3, t582);
            c.w64(9, c.r64(3).wrapping_add(0x4));
            let Some(t583) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f5965); };
            c.w64(8, t583);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f596c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008f5970); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f5975); }
            let Some(t584) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f597a); };
            c.w64(12, t584);
            let Some(t585) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f597e); };
            c.w32(8, t585);
            let Some(t586) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5983); };
            c.w64(15, t586);
            let Some(t587) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5987); };
            c.w64(1, t587);
            let Some(t588) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f598b); };
            c.w64(5, t588);
            let r589 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r589);
            return RegfileExit::jump(c.r64(1));
        }
        260 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r590 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r590);
            let r591 = c.r64(1) & 0xfffffu64;
            c.w64(1, r591);
            c.logic(r591, 64);
            let Some(t592) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f59a4); };
            c.w32(8, t592);
            let Some(t593) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f59a9); };
            c.w32(0, t593);
            let r594 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r594);
            let Some(t595) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f59b0); };
            c.w64(1, t595);
            let Some(t596) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f59b4); };
            c.w64(5, t596);
            let r597 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r597);
            return RegfileExit::jump(c.r64(1));
        }
        261 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f59c2); }
            let Some(t598) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008f59c6); };
            c.w64(8, t598);
            let Some(t599) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f59cd); };
            c.w32(8, t599);
            let r600 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r600);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f59d4); }
            let Some(t601) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x70008f59d6); };
            c.w64(3, t601);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008f59ea, 0x70008f59e5);
        }
        262 => {
            return RegfileExit::fault(0x70008f59ea);
        }
        263 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x4));
            let Some(t602) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f59f3); };
            c.w32(3, t602);
            let Some(t603) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f59f9); };
            c.w32(2, t603);
            let r604 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r604);
            let Some(t605) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f5a01); };
            c.w64(1, t605);
            let Some(t606) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5a05); };
            c.w32(3, t606);
            let r607 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r607);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f5a0c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f5a10); }
            let Some(t608) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f5a11); };
            if !c.push64(t608) { return RegfileExit::fault(0x70008f5a11); }
            let Some(t609) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f5a14); };
            if !c.push64(t609) { return RegfileExit::fault(0x70008f5a14); }
            c.w32(11, 0x2u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f5a1d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f5a1f); }
            let Some(t610) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x70008f5a23); };
            c.w64(3, t610);
            c.w64(6, c.r64(1));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008f5a3a, 0x70008f5a35);
        }
        264 => {
            let Some(t611) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5a3a); };
            c.w64(8, t611);
            let Some(t612) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5a3e); };
            c.w64(8, t612);
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f5a42); };
            c.w64(9, t613);
            let Some(t614) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f5a46); };
            c.w32(11, t614);
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5a4b); };
            c.w64(15, t615);
            let Some(t616) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5a4f); };
            c.w64(1, t616);
            let Some(t617) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5a53); };
            c.w64(5, t617);
            c.w64(12, c.r64(8));
            let r618 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r618);
            return RegfileExit::jump(c.r64(1));
        }
        265 => {
            return RegfileExit::fault(0x70008f5a60);
        }
        266 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f5a61); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f5a65); }
            let r619 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r619);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x4));
            let Some(t620) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f5a73); };
            c.w32(3, t620);
            let Some(t621) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5a79); };
            c.w32(2, t621);
            let r622 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r622);
            let Some(t623) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f5a81); };
            c.w64(1, t623);
            let Some(t624) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5a85); };
            c.w32(3, t624);
            let r625 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r625);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f5a8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f5a90); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f5a94); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f5a98); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            c.w32(0, 0xcu64);
            c.w64(11, c.r64(0));
            return c.call(0x7000804df4, 0x70008f5aaf, 0x70008f5aaa);
        }
        267 => {
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5aaf); };
            c.w64(8, t626);
            let Some(t627) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5ab3); };
            c.w64(8, t627);
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f5ab7); };
            c.w64(9, t628);
            let Some(t629) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f5abb); };
            c.w32(11, t629);
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5ac0); };
            c.w64(15, t630);
            let Some(t631) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5ac4); };
            c.w64(1, t631);
            let Some(t632) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5ac8); };
            c.w64(5, t632);
            c.w64(12, c.r64(8));
            let r633 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r633);
            return RegfileExit::jump(c.r64(1));
        }
        268 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f5ad5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f5ad9); }
            let r634 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r634);
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t635) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f5ae3); };
            c.w32(3, t635);
            let Some(t636) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5ae9); };
            c.w32(2, t636);
            let r637 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r637);
            let Some(t638) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5af1); };
            c.w32(3, t638);
            let r639 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r639);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f5af8); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f5afc); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f5afd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f5b01); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f5b05); }
            let Some(t640) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x70008f5b09); };
            c.w64(3, t640);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008f5b1d, 0x70008f5b18);
        }
        269 => {
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5b1d); };
            c.w64(8, t641);
            let Some(t642) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5b21); };
            c.w64(8, t642);
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f5b25); };
            c.w64(11, t643);
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t644) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f5b2d); };
            c.w32(11, t644);
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5b32); };
            c.w64(15, t645);
            let Some(t646) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5b36); };
            c.w64(1, t646);
            let Some(t647) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5b3a); };
            c.w64(5, t647);
            c.w64(12, c.r64(8));
            let r648 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r648);
            return RegfileExit::jump(c.r64(1));
        }
        270 => {
            return RegfileExit::jump(0x70008f5b48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f5b54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f5b54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f5b58); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f5b5e); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5b64); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5b6c); };
            c.w64(8, t4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5b70); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f5b77); };
            c.w64(7, t7);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f5b7f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f5b83); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f5b84); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f5b88); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f5b8c); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x70008f5b90); };
            c.w64(3, t8);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008f5ba4, 0x70008f5b9f);
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5ba4); };
            c.w64(2, t9);
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5ba8); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f5bac); };
            c.w64(7, t11);
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f5bb4); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5bb9); };
            c.w64(15, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5bbd); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5bc1); };
            c.w64(5, t15);
            c.w64(12, c.r64(2));
            let r16 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r16);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f5bd0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f5bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f5bd7); };
            c.w64(7, t0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f5bdb); };
            c.w32(8, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5be1); };
            c.w32(11, t2);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5bea); };
            c.w32(3, t3);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5bf9); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5bff); };
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
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f5c17); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f5c1a); };
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
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f5c29); };
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
            return RegfileExit::fault(0x70008f5c37);
        }
        9 => {
            return RegfileExit::fault(0x70008f5c38);
        }
        10 => {
            return RegfileExit::fault(0x70008f5c39);
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5c3a); };
            c.w32(2, t12);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x70008f5c48);
        }
        13 => {
            let r13 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f5c4d); };
            c.w32(2, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5c52); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5c5a); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f5c60); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f5c64); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            let Some(t19) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f5c6a); };
            c.w32(8, t19);
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5c70); };
            c.w32(11, t20);
            let r21 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r21);
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5c7c); };
            c.w32(8, t22);
            let r23 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r23);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f5c84); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f5c88); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f5c8a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f5c8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f5c92); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x70008f5c96); };
            c.w64(3, t24);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008f5caa, 0x70008f5ca5);
        }
        15 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5caa); };
            c.w64(2, t25);
            let Some(t26) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5cae); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f5cb2); };
            c.w64(9, t27);
            let r28 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r28);
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f5cba); };
            c.w32(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5cbf); };
            c.w64(15, t30);
            let Some(t31) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5cc3); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5cc7); };
            c.w64(5, t32);
            c.w64(12, c.r64(2));
            let r33 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r33);
            return RegfileExit::jump(c.r64(1));
        }
        16 => {
            return RegfileExit::jump(0x70008f5cd8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f5cf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f5cf7); };
            c.w64(11, t0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5cfb); };
            c.w32(0, t1);
            c.w64(3, c.r64(11));
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5d04); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5d13); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5d19); };
            c.w32(1, t5);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.logic(c.r64(1) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f5d31); };
            c.w32(1, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f5d34); };
            c.cmp(t7, c.r32(1), 32);
            if c.cc_ne() { blk = 268; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f5d47); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let r10 = c.add(c.r32(0), 0xffffffffffffffffu64, 32);
            c.w32(0, r10);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008f5d54);
        }
        9 => {
            return RegfileExit::fault(0x70008f5d55);
        }
        10 => {
            return RegfileExit::fault(0x70008f5d56);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5d57); };
            c.w64(11, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f5d5b); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5d62); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5d69); };
            c.w32(0, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f5d6d); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f5d76); };
            c.w64(11, t18);
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f5d7d); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f5d8a); };
            c.w32(0, t20);
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(3, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f5d97); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            let r23 = c.r64(2) & 0xfffffffffffffffdu64;
            c.w64(2, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f5db1); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f5db7); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r27);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f5dc8); };
            c.w32(8, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5dcd); };
            c.w64(1, t29);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f5dd6); };
            c.w64(8, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5dda); };
            c.w32(8, t31);
            let r32 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f5de1); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5de8); };
            c.w32(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008f5dfb);
        }
        20 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x70008f5dfc); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5e09); };
            c.w32(0, t38);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            let r39 = c.r64(3) & 0xfffu64;
            c.w64(3, r39);
            c.logic(r39, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(2, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ae() { blk = 265; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f5e2d); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5e35); };
            c.w32(0, t42);
            if let Some(sr) = c.shr(c.r64(11), 0xcu64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r43 = c.r64(11) & 0x3ffffu64;
            c.w64(11, r43);
            c.logic(r43, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70008f5e4f);
        }
        23 => {
            let r44 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f5e54); };
            c.w32(0, t45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5e59); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5e61); };
            c.w64(1, t48);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f5e6a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f5e6e); }
            let r49 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f5e74); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f5e78); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008f5e7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008f5e80); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f5e8c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f5e90); }
            let r50 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008f5e96); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f5e9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f5ea2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f5ea6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f5eaa); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008f5eae); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008f5eb2); }
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008f5eb6); };
            c.cmp(t53, c.r32(7), 32);
            if c.cc_e() { blk = 262; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t54) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f5ec3); };
            c.w64(3, t54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5ec7); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f5ecd); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5ed3); };
            c.w32(1, t59);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008f5edc); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x70008f5ee6);
        }
        28 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f5ee7); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008f5eed); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008f5ef7);
        }
        30 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f5ef8); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f5f08); };
            c.w32(1, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70008f5f0b); };
            c.cmp(t65, c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5f14); };
            c.w32(1, t66);
            let r67 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r67);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5f22); };
            c.cmp(t68, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f5f2c); };
            c.w32(1, t69);
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f5f2f); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f5f39); };
            c.cmp(t72, c.r32(0), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 37; continue;
        }
        37 => {
            let r73 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r73);
            c.logic(r73, 64);
            blk = 46; continue;
        }
        38 => {
            c.w64(0, c.r64(2));
            blk = 46; continue;
        }
        39 => {
            let r74 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r74);
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f5f60); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5f65); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5f69); };
            c.w64(5, t77);
            let r78 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r78);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(6, c.r64(9).wrapping_add(0x4));
            let Some(t79) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f5f7b); };
            c.w32(9, t79);
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5f81); };
            c.w32(12, t80);
            let r81 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r81);
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f5f89); };
            c.w32(9, t82);
            let r83 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r83);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008f5f91); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008f5f95); }
            c.w64(1, c.r64(9));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(11));
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f5fa2); };
            c.logic(t84 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r85 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r85);
            blk = 43; continue;
        }
        42 => {
            let Some(t86) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f5fb1); };
            c.w32(7, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008f5fb4); };
            c.w64(7, t87);
            blk = 43; continue;
        }
        43 => {
            return c.call(c.r64(7), 0x70008f5fbe, 0x70008f5fbc);
        }
        44 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5fbe); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f5fc2); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f5fc6); };
            c.w64(9, t90);
            let Some(t91) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f5fca); };
            c.w32(11, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f5fcf); };
            c.w64(15, t92);
            let Some(t93) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f5fd3); };
            c.w64(1, t93);
            let Some(t94) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f5fd7); };
            c.w64(5, t94);
            c.w64(12, c.r64(8));
            let r95 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r95);
            return RegfileExit::jump(c.r64(1));
        }
        45 => {
            c.w64(0, c.r64(2));
            c.w64(1, c.r64(7));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 46; continue;
        }
        46 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(3, c.r64(1));
            let r96 = c.r64(3) & 0xfu64;
            c.w64(3, r96);
            c.logic(r96, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f5ffa); }
            c.cmp(c.r64(3), 0x4u64, 64);
            if c.cc_e() { blk = 192; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(3), 0x5u64, 64);
            if c.cc_e() { blk = 191; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(3), 0xdu64, 64);
            if c.cc_e() { blk = 190; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_e() { blk = 129; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(3), 0x6u64, 64);
            if c.cc_e() { blk = 127; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 122; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(3), 0x8u64, 64);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(3), 0x9u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(3), 0x3u64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_e() { blk = 125; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r64(3), 0xcu64, 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(3), 0xeu64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(3, c.r64(9).wrapping_add(0x4));
            let Some(t97) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f6085); };
            c.w32(2, t97);
            let Some(t98) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f608b); };
            c.w32(1, t98);
            let r99 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r99);
            let Some(t100) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6093); };
            c.w32(2, t100);
            let r101 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r101);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f609a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f609e); }
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f609f); };
            if !c.push64(t102) { return RegfileExit::fault(0x70008f609f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f60a2); }
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f60a3); };
            if !c.push64(t103) { return RegfileExit::fault(0x70008f60a3); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f60a6); };
            if !c.push64(t104) { return RegfileExit::fault(0x70008f60a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008f60a9); }
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f60ad); };
            c.w64(6, t105);
            c.w32(0, 0x5u64);
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x70008f60b6); };
            c.w64(3, t106);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008f60c5, 0x70008f60c0);
        }
        59 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f60c5); };
            c.w64(8, t107);
            let Some(t108) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f60c9); };
            c.w64(8, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008f60cd); };
            c.w64(9, t109);
            let Some(t110) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f60d1); };
            c.w32(11, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f60d6); };
            c.w64(15, t111);
            let Some(t112) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f60da); };
            c.w64(1, t112);
            let Some(t113) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f60de); };
            c.w64(5, t113);
            c.w64(12, c.r64(8));
            let r114 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r114);
            return RegfileExit::jump(c.r64(1));
        }
        60 => {
            let Some(t115) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f60eb); };
            c.w32(11, t115);
            let r116 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r116);
            let Some(t117) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f60f2); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f60f9); };
            c.w32(0, t119);
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r120 = c.r64(1) & 0x1ffffffu64;
            c.w64(1, r120);
            c.logic(r120, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_b() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x70008f6113);
        }
        62 => {
            let Some(t121) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6114); };
            c.w32(11, t121);
            let r122 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r122);
            let Some(t123) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f611c); };
            c.w32(0, t123);
            let r124 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r124);
            c.w64(11, c.r64(9).wrapping_add(0x4));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008f6127); }
            let Some(t125) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f612b); };
            c.cmp(t125, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t126) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6134); };
            c.w32(8, t126);
            let Some(t127) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6139); };
            c.w64(1, t127);
            let Some(t128) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f613d); };
            c.w64(5, t128);
            c.w64(9, c.r64(11));
            let r129 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r129);
            return RegfileExit::jump(c.r64(1));
        }
        64 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t130) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f614e); };
            c.w32(3, t130);
            let Some(t131) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6154); };
            c.w32(2, t131);
            let r132 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r132);
            let Some(t133) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f615c); };
            c.w32(3, t133);
            let r134 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r134);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f6163); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f616c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f616d); }
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008f616e); };
            c.w64(3, t135);
            c.w32(0, 0x2u64);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f617a); };
            c.w64(6, t136);
            return c.call(0x700086ed74, 0x70008f6183, 0x70008f617e);
        }
        65 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6183); };
            c.w64(8, t137);
            let Some(t138) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6187); };
            c.w64(8, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f618b); };
            c.w64(9, t139);
            let Some(t140) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f618f); };
            c.w32(11, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6194); };
            c.w64(15, t141);
            let Some(t142) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6198); };
            c.w64(1, t142);
            let Some(t143) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f619c); };
            c.w64(5, t143);
            c.w64(12, c.r64(8));
            let r144 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r144);
            return RegfileExit::jump(c.r64(1));
        }
        66 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(3, c.r64(9).wrapping_add(0x4));
            let Some(t145) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f61b1); };
            c.w32(2, t145);
            let Some(t146) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f61b7); };
            c.w32(1, t146);
            let r147 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r147);
            let Some(t148) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f61bf); };
            c.w32(2, t148);
            let r149 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r149);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f61c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008f61ca); }
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f61ce); };
            c.w64(6, t150);
            c.w64(3, c.r64(2));
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f61d5); };
            c.w64(1, t151);
            c.w32(2, 0x2u64);
            c.w64(11, c.r64(2));
            return c.call(0x70008bc174, 0x70008f61e6, 0x70008f61e1);
        }
        67 => {
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f61e6); };
            c.w64(8, t152);
            let Some(t153) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f61ea); };
            c.w64(8, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f61ee); };
            c.w64(9, t154);
            let Some(t155) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f61f2); };
            c.w32(11, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f61f7); };
            c.w64(15, t156);
            let Some(t157) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f61fb); };
            c.w64(1, t157);
            let Some(t158) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f61ff); };
            c.w64(5, t158);
            c.w64(12, c.r64(8));
            let r159 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r159);
            return RegfileExit::jump(c.r64(1));
        }
        68 => {
            blk = 125; continue;
        }
        69 => {
            let Some(t160) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f6211); };
            c.w32(3, t160);
            let r161 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r161);
            let Some(t162) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6217); };
            c.w32(2, t162);
            let Some(t163) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f621a); };
            c.cmp(t163, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t164) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f6227); };
            c.w32(1, t164);
            let Some(t165) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f622d); };
            c.w32(6, t165);
            let r166 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r166);
            let Some(t167) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6235); };
            c.w32(1, t167);
            let r168 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r168);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t169) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6245); };
            c.w32(2, t169);
            let r170 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f624b); };
            c.cmp(t171, c.r32(2), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t172) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f6258); };
            c.w32(2, t172);
            let r173 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r173);
            let Some(t174) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f625e); };
            c.w32(6, t174);
            let r175 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r175);
            let Some(t176) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f6264); };
            c.logic(t176 & 0x2u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t177) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f626e); };
            c.cmp(t177, c.r32(6), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f6277); };
            c.w64(3, t178);
            blk = 109; continue;
        }
        75 => {
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f6283); };
            c.w64(6, t179);
            let Some(t180) = c.ld8(c.r64(6)) else { return RegfileExit::fault(0x70008f628a); };
            c.cmp(t180, 0x20u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t181) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f6293); };
            c.w32(1, t181);
            let Some(t182) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f6296); };
            c.cmp(t182, c.r32(1), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t183) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f62a3); };
            c.w32(1, t183);
            let r184 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r184);
            let Some(t185) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f62a9); };
            c.w32(6, t185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f62ac); };
            c.cmp(t186, c.r32(6), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t187) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f62b5); };
            c.w32(1, t187);
            let r188 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r188);
            let Some(t189) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f62bb); };
            c.w32(6, t189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f62be); };
            c.cmp(t190, c.r32(6), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t191) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f62d0); };
            c.w32(6, t191);
            let r192 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r192);
            let Some(t193) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f62d6); };
            c.w32(7, t193);
            c.w32(12, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w32(1, c.r16(7));
            c.cmp(c.r32(1), 0x82au64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f62f2); };
            c.cmp(t194, c.r32(6), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f62fb); };
            c.w64(12, t195);
            blk = 263; continue;
        }
        84 => {
            c.w64(0, c.r64(3));
            blk = 86; continue;
        }
        85 => {
            c.w64(0, c.r64(1));
            blk = 86; continue;
        }
        86 => {
            let Some(t196) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f630c); };
            c.w32(0, t196);
            let r197 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r197);
            let Some(t198) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6312); };
            c.w32(0, t198);
            let r199 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r199);
            let Some(t200) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6318); };
            c.w32(3, t200);
            let r201 = c.sub(c.r32(3), 0x89u64, 32);
            c.w32(3, r201);
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_a() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t202) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f632b); };
            c.w32(6, t202);
            let r203 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r203);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f6331); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f6335); };
            if !c.push64(t204) { return RegfileExit::fault(0x70008f6335); }
            c.w64(1, c.r64(11));
            return c.call(0x70007f6fb4, 0x70008f6340, 0x70008f633b);
        }
        88 => {
            c.w64(3, c.r64(0));
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6343); };
            c.w64(15, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f6347); };
            c.w64(9, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f634b); };
            c.w64(8, t207);
            blk = 109; continue;
        }
        89 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f6354); };
            c.w64(12, t208);
            blk = 263; continue;
        }
        90 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f6361); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f6365); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f6366); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f6367); };
            if !c.push64(t209) { return RegfileExit::fault(0x70008f6367); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f636a); };
            c.w64(6, t210);
            c.w32(0, 0x3u64);
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f6373); };
            c.w64(3, t211);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008f6382, 0x70008f637d);
        }
        91 => {
            c.w64(3, c.r64(0));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6385); };
            c.w64(15, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f6389); };
            c.w64(9, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f638d); };
            c.w64(8, t214);
            blk = 109; continue;
        }
        92 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f639d); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f63a1); };
            if !c.push64(t215) { return RegfileExit::fault(0x70008f63a1); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f63a4); };
            c.w64(6, t216);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x70008f63b2, 0x70008f63ad);
        }
        93 => {
            c.w64(3, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f63b5); };
            c.w64(15, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f63b9); };
            c.w64(9, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f63bd); };
            c.w64(8, t219);
            blk = 109; continue;
        }
        94 => {
            let Some(t220) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f63c6); };
            c.w32(1, t220);
            let r221 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r221);
            let Some(t222) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f63cc); };
            c.w32(6, t222);
            c.cmp(c.r16(6), 0x834u64, 16);
            if c.cc_e() { blk = 108; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(7, c.r64(6).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 101; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r16(6), 0x411u64, 16);
            if c.cc_ne() { blk = 263; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t223) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f63f1); };
            c.w32(0, t223);
            let Some(t224) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f63f4); };
            c.cmp(t224, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t225) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f640a); };
            c.cmp(t225, 0x40u64, 16);
            if c.cc_ae() { blk = 263; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t226) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6415); };
            c.w32(11, t226);
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            blk = 109; continue;
        }
        101 => {
            let Some(t227) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f6422); };
            c.w32(2, t227);
            let Some(t228) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f6425); };
            c.cmp(t228, c.r32(2), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t229) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f6432); };
            c.w32(2, t229);
            c.w64(1, c.r64(2));
            let r230 = c.r32(1) & 0xc0u64;
            c.w32(1, r230);
            c.logic(r230, 32);
            c.cmp(c.r32(1), 0xc0u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t231) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f6447); };
            c.w32(3, t231);
            let r232 = c.r32(3) & 0x1fu64;
            c.w32(3, r232);
            c.logic(r232, 32);
            let r233 = c.sub(c.r32(3), 0x10u64, 32);
            c.w32(3, r233);
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_a() { blk = 263; continue; }
            blk = 104; continue;
        }
        104 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 263; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t234) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f6462); };
            c.w32(3, t234);
            let r235 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r235);
            let Some(t236) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f6468); };
            c.cmp(t236, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t237) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6475); };
            c.w32(11, t237);
            let Some(t238) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f6479); };
            c.cmp(t238, c.r32(11), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t239) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f6482); };
            c.w32(3, t239);
            let r240 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r240);
            blk = 109; continue;
        }
        108 => {
            let Some(t241) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f648a); };
            c.w32(0, t241);
            let Some(t242) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f648d); };
            c.cmp(t242, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 109; continue;
        }
        109 => {
            let r243 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r243);
            let Some(t244) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f649e); };
            c.w64(8, t244);
            let Some(t245) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f64a2); };
            c.w32(11, t245);
            let Some(t246) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f64a7); };
            c.w64(1, t246);
            let Some(t247) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f64ab); };
            c.w64(5, t247);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(3));
            let r248 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r248);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 263; continue;
        }
        111 => {
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f64c0); };
            c.w64(3, t249);
            let Some(t250) = c.ld8(c.r64(3)) else { return RegfileExit::fault(0x70008f64c7); };
            c.cmp(t250, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t251) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f64d5); };
            c.w32(3, t251);
            let r252 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r252);
            blk = 115; continue;
        }
        114 => {
            let Some(t253) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f64dd); };
            c.w32(3, t253);
            let r254 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r254);
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return RegfileExit::fault(0x70008f64e8);
        }
        117 => {
            let r255 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r255);
            c.logic(r255, 64);
            let Some(t256) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f64ed); };
            c.w32(1, t256);
            let r257 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r257);
            let Some(t258) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f64f3); };
            c.w32(0, t258);
            let r259 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r259);
            let Some(t260) = c.ld64(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f64f9); };
            c.w64(1, t260);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t261) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6502); };
            c.w32(2, t261);
            let r262 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r262);
            let Some(t263) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f6508); };
            c.w32(2, t263);
            let r264 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r264);
            blk = 119; continue;
        }
        119 => {
            c.w32(6, c.r64(9).wrapping_add(c.r64(9)));
            let r265 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r265);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f6516); }
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f651a); };
            if !c.push64(t266) { return RegfileExit::fault(0x70008f651a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f651d); }
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            return c.call(0x70007fe8d4, 0x70008f6535, 0x70008f6530);
        }
        120 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6535); };
            c.w64(8, t267);
            let Some(t268) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6539); };
            c.w64(8, t268);
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f653d); };
            c.w64(9, t269);
            let Some(t270) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f6541); };
            c.w32(11, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6546); };
            c.w64(15, t271);
            let Some(t272) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f654a); };
            c.w64(1, t272);
            let Some(t273) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f654e); };
            c.w64(5, t273);
            c.w64(12, c.r64(8));
            let r274 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r274);
            return RegfileExit::jump(c.r64(1));
        }
        121 => {
            blk = 125; continue;
        }
        122 => {
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f655d); };
            c.w64(11, t275);
            let Some(t276) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f6564); };
            c.cmp(t276, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let r277 = c.r64(1) & 0x3ffu64;
            c.w64(1, r277);
            c.logic(r277, 64);
            let r278 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r278);
            let r279 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r279);
            let Some(t280) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6581); };
            c.w32(2, t280);
            let r281 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r281);
            let Some(t282) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f6587); };
            c.w32(2, t282);
            let r283 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r283);
            let Some(t284) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f658d); };
            c.w32(3, t284);
            let r285 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r285);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f6594); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f6598); }
            c.w64(2, c.r64(6));
            c.w64(1, c.r64(0));
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f65a2); };
            c.w64(6, t286);
            return c.call(0x70007fea14, 0x70008f65ab, 0x70008f65a6);
        }
        124 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f65ab); };
            c.w64(8, t287);
            let Some(t288) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f65af); };
            c.w64(8, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f65b3); };
            c.w64(9, t289);
            let Some(t290) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f65b7); };
            c.w32(11, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f65bc); };
            c.w64(15, t291);
            let Some(t292) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f65c0); };
            c.w64(1, t292);
            let Some(t293) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f65c4); };
            c.w64(5, t293);
            c.w64(12, c.r64(8));
            let r294 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r294);
            return RegfileExit::jump(c.r64(1));
        }
        125 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x4));
            let Some(t295) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f65d9); };
            c.w32(3, t295);
            let Some(t296) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f65df); };
            c.w32(2, t296);
            let r297 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r297);
            let Some(t298) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f65e7); };
            c.w32(3, t298);
            let r299 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r299);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f65ee); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f65f2); }
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f65f3); };
            if !c.push64(t300) { return RegfileExit::fault(0x70008f65f3); }
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f65f6); };
            if !c.push64(t301) { return RegfileExit::fault(0x70008f65f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f65f9); }
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x70008f65fd); };
            c.w64(3, t302);
            c.w32(0, 0x3u64);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f6609); };
            c.w64(6, t303);
            return c.call(0x700086ed74, 0x70008f6612, 0x70008f660d);
        }
        126 => {
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6612); };
            c.w64(8, t304);
            let Some(t305) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6616); };
            c.w64(8, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f661a); };
            c.w64(9, t306);
            let Some(t307) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f661e); };
            c.w32(11, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6623); };
            c.w64(15, t308);
            let Some(t309) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6627); };
            c.w64(1, t309);
            let Some(t310) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f662b); };
            c.w64(5, t310);
            c.w64(12, c.r64(8));
            let r311 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r311);
            return RegfileExit::jump(c.r64(1));
        }
        127 => {
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let r312 = c.r64(1) & 0x3ffu64;
            c.w64(1, r312);
            c.logic(r312, 64);
            let r313 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r313);
            let r314 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r314);
            let Some(t315) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f664f); };
            c.w32(0, t315);
            let r316 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r316);
            let Some(t317) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f6655); };
            c.w32(0, t317);
            let r318 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r318);
            let Some(t319) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f665b); };
            c.w32(0, t319);
            let r320 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r320);
            let Some(t321) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f6662); };
            c.w32(7, t321);
            let r322 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r322);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f6668); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008f666c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f666d); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f6673); };
            c.w64(6, t323);
            return c.call(0x70007f2274, 0x70008f667c, 0x70008f6677);
        }
        128 => {
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f667c); };
            c.w64(8, t324);
            let Some(t325) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6680); };
            c.w64(8, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f6684); };
            c.w64(9, t326);
            let Some(t327) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f6688); };
            c.w32(11, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f668d); };
            c.w64(15, t328);
            let Some(t329) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6691); };
            c.w64(1, t329);
            let Some(t330) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6695); };
            c.w64(5, t330);
            c.w64(12, c.r64(8));
            let r331 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r331);
            return RegfileExit::jump(c.r64(1));
        }
        129 => {
            let Some(t332) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f66a2); };
            c.w32(3, t332);
            let r333 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r333);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008f66ad); };
            c.w64(3, t334);
            blk = 131; continue;
        }
        131 => {
            let Some(t335) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f66b4); };
            c.w32(2, t335);
            let Some(t336) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f66b7); };
            c.w32(1, t336);
            let Some(t337) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f66bd); };
            c.w32(7, t337);
            let r338 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r338);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t339) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f66c8); };
            c.w32(1, t339);
            let r340 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r340);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r341 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r341);
            let Some(t342) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f66d6); };
            c.w32(6, t342);
            let r343 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r343);
            let r344 = c.r64(6) & c.r64(2);
            c.w64(6, r344);
            c.logic(r344, 64);
            blk = 132; continue;
        }
        132 => {
            c.w64(15, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(8, c.r64(15).wrapping_mul(4));
            let Some(t345) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f66ec); };
            c.w32(0, t345);
            let Some(t346) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f66f1); };
            c.cmp(t346, c.r32(0), 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r347 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r347);
            c.w64(8, c.r64(11).wrapping_add(c.r64(6)));
            c.w64(0, c.r64(2));
            let r348 = c.r64(0) & c.r64(8);
            c.w64(0, r348);
            c.logic(r348, 64);
            c.w64(6, c.r64(0));
            blk = 132; continue;
        }
        135 => {
            let Some(t349) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f6715); };
            c.w32(11, t349);
            let Some(t350) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f6719); };
            c.w32(0, t350);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(2, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_b() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            return RegfileExit::fault(0x70008f672d);
        }
        137 => {
            let Some(t351) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f672e); };
            c.w32(11, t351);
            let Some(t352) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f6734); };
            c.w32(8, t352);
            let r353 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r353);
            let Some(t354) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f673c); };
            c.w32(11, t354);
            let r355 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r355);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t356) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f674c); };
            c.w32(0, t356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f6750); };
            c.cmp(t357, c.r32(0), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t358) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f675d); };
            c.w32(2, t358);
            let r359 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r359);
            let Some(t360) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6764); };
            c.w32(8, t360);
            let r361 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r361);
            let Some(t362) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f676b); };
            c.logic(t362 & 0x2u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t363) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f6776); };
            c.cmp(t363, c.r32(8), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f677f); };
            c.w64(8, t364);
            blk = 188; continue;
        }
        142 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f678b); };
            c.w64(8, t365);
            let Some(t366) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x70008f6792); };
            c.cmp(t366, 0x20u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t367) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f679c); };
            c.w32(8, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f67a0); };
            c.cmp(t368, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f67a9); };
            c.w64(8, t369);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        145 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f67b5); };
            c.w64(0, t370);
            let Some(t371) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f67b9); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f67c0); };
            c.w32(11, t373);
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f67c4); };
            c.cmp(t374, c.r32(11), 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t375) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f67cd); };
            c.w32(8, t375);
            let r376 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r376);
            let Some(t377) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f67d4); };
            c.w32(11, t377);
            let Some(t378) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f67d8); };
            c.cmp(t378, c.r32(11), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 147; continue;
        }
        147 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t379) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f67eb); };
            c.w32(11, t379);
            let r380 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r380);
            let Some(t381) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f67f2); };
            c.w32(3, t381);
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 153; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w32(8, c.r16(3));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t382) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f680f); };
            c.cmp(t382, c.r32(11), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6818); };
            c.w64(8, t383);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        152 => {
            let Some(t384) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f6824); };
            c.w32(0, t384);
            let r385 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r385);
            blk = 154; continue;
        }
        153 => {
            c.w64(0, c.r64(8));
            blk = 154; continue;
        }
        154 => {
            let Some(t386) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f682f); };
            c.w32(8, t386);
            let r387 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6836); };
            c.w32(8, t388);
            let r389 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r389);
            let Some(t390) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f683d); };
            c.w32(11, t390);
            let r391 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r391);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t392) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f6853); };
            c.w32(6, t392);
            let r393 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r393);
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f685a); };
            c.w64(8, t394);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f685e); }
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f6862); };
            if !c.push64(t395) { return RegfileExit::fault(0x70008f6862); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008f686c, 0x70008f6867);
        }
        156 => {
            c.w64(8, c.r64(0));
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f686f); };
            c.w64(9, t396);
            blk = 188; continue;
        }
        157 => {
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6878); };
            c.w64(8, t397);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        158 => {
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6884); };
            c.w64(8, t398);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        159 => {
            c.w64(8, c.r64(9));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6897); };
            c.w64(3, t399);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f689b); }
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f689f); };
            if !c.push64(t400) { return RegfileExit::fault(0x70008f689f); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f68a2); }
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f68a4); };
            if !c.push64(t401) { return RegfileExit::fault(0x70008f68a4); }
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f68a7); };
            c.w64(6, t402);
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f68ab); };
            c.w64(3, t403);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008f68bf, 0x70008f68ba);
        }
        160 => {
            c.w64(8, c.r64(0));
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f68c2); };
            c.w64(9, t404);
            blk = 188; continue;
        }
        161 => {
            c.w64(8, c.r64(9));
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f68d2); };
            c.w64(0, t405);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f68d6); }
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f68da); };
            if !c.push64(t406) { return RegfileExit::fault(0x70008f68da); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f68e2); };
            c.w64(6, t407);
            return c.call(0x70007f2274, 0x70008f68eb, 0x70008f68e6);
        }
        162 => {
            c.w64(8, c.r64(0));
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f68ee); };
            c.w64(9, t408);
            blk = 188; continue;
        }
        163 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f68f7); };
            c.w64(0, t409);
            let Some(t410) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f68fb); };
            c.w32(11, t410);
            let r411 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r411);
            let Some(t412) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6902); };
            c.w32(3, t412);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 185; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w32(2, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 174; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6924); };
            c.w64(8, t413);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        167 => {
            let Some(t414) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f6930); };
            c.w32(8, t414);
            let Some(t415) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f6934); };
            c.cmp(t415, c.r32(8), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f693d); };
            c.w64(8, t416);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        169 => {
            let Some(t417) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f6949); };
            c.w32(8, t417);
            let r418 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r418);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6956); };
            c.w64(8, t419);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        171 => {
            let Some(t420) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6962); };
            c.w32(11, t420);
            let r421 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r421);
            let Some(t422) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6969); };
            c.cmp(t422, 0x40u64, 16);
            if c.cc_b() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6971); };
            c.w64(8, t423);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        173 => {
            let Some(t424) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f697d); };
            c.w32(8, t424);
            let r425 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r425);
            blk = 188; continue;
        }
        174 => {
            let Some(t426) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f6989); };
            c.w32(8, t426);
            let Some(t427) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f698d); };
            c.cmp(t427, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6996); };
            c.w64(8, t428);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        176 => {
            let Some(t429) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f69a2); };
            c.w32(8, t429);
            c.w64(11, c.r64(8));
            let r430 = c.r32(11) & 0xc0u64;
            c.w32(11, r430);
            c.logic(r430, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t431) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f69ba); };
            c.w32(11, t431);
            let r432 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r432);
            let Some(t433) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f69c1); };
            c.w32(11, t433);
            let r434 = c.r32(11) & 0x1fu64;
            c.w32(11, r434);
            c.logic(r434, 32);
            let r435 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r435);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f69d3); };
            c.w64(8, t436);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        179 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 184; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t437) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f69e5); };
            c.w32(8, t437);
            let r438 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r438);
            let Some(t439) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f69ec); };
            c.cmp(t439, c.r32(8), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t440) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f69f5); };
            c.w32(11, t440);
            let Some(t441) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f69f9); };
            c.cmp(t441, c.r32(11), 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t442) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f6a02); };
            c.w32(8, t442);
            let r443 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r443);
            blk = 188; continue;
        }
        183 => {
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6a0b); };
            c.w64(8, t444);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        184 => {
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6a17); };
            c.w64(8, t445);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        185 => {
            let Some(t446) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f6a23); };
            c.w32(8, t446);
            let Some(t447) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f6a27); };
            c.cmp(t447, c.r32(8), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6a30); };
            c.w64(8, t448);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        187 => {
            let Some(t449) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f6a3c); };
            c.w32(8, t449);
            let r450 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r450);
            blk = 188; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6a4a); };
            c.w64(3, t451);
            let Some(t452) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6a4e); };
            c.w64(11, t452);
            let Some(t453) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f6a52); };
            c.w32(0, t453);
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6a57); };
            c.w64(15, t454);
            let Some(t455) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6a5b); };
            c.w64(1, t455);
            let Some(t456) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6a5f); };
            c.w64(5, t456);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            let r457 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r457);
            return RegfileExit::jump(c.r64(1));
        }
        189 => {
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6a6f); };
            c.w64(8, t458);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        190 => {
            let r459 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r459);
            let Some(t460) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6a7f); };
            c.w64(8, t460);
            let Some(t461) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f6a83); };
            c.w32(11, t461);
            let Some(t462) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6a88); };
            c.w64(1, t462);
            let Some(t463) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6a8c); };
            c.w64(5, t463);
            let Some(t464) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f6a90); };
            c.w64(0, t464);
            c.w64(12, c.r64(8));
            let r465 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r465);
            return RegfileExit::jump(c.r64(1));
        }
        191 => {
            let r466 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r466);
            let Some(t467) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6aa4); };
            c.w32(8, t467);
            let Some(t468) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6aa9); };
            c.w64(1, t468);
            let Some(t469) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6aad); };
            c.w64(5, t469);
            let r470 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r470);
            return RegfileExit::jump(c.r64(1));
        }
        192 => {
            c.logic(c.r8(1) & 0x40u64, 8);
            if c.cc_ne() { blk = 206; continue; }
            blk = 193; continue;
        }
        193 => {
            c.w64(3, c.r64(1));
            if let Some(sr) = c.shr(c.r64(3), 0x9u64, 64) { c.w64(3, sr); }
            c.logic(c.r16(1) & 0x100u64, 16);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            let r471 = c.r64(3) & 0x7ffu64;
            c.w64(3, r471);
            c.logic(r471, 64);
            c.logic(c.r8(1) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 201; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t472) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f6ade); };
            c.w32(0, t472);
            let r473 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r473);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008f6ae8); };
            c.w64(0, t474);
            blk = 196; continue;
        }
        196 => {
            let Some(t475) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6aef); };
            c.w32(0, t475);
            let r476 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r476);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            let r477 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r477);
            let Some(t478) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6aff); };
            c.w32(8, t478);
            let Some(t479) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6b04); };
            c.w64(1, t479);
            let Some(t480) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6b08); };
            c.w64(5, t480);
            let r481 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r481);
            return RegfileExit::jump(c.r64(1));
        }
        198 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t482) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6b1a); };
            c.w32(3, t482);
            let Some(t483) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f6b1d); };
            c.cmp(t483, c.r32(3), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 200; continue;
        }
        200 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f6b2a); }
            blk = 254; continue;
        }
        201 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let r484 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r484);
            let Some(t485) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6b3d); };
            c.w32(8, t485);
            let Some(t486) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6b42); };
            c.w32(0, t486);
            let r487 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r487);
            let Some(t488) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6b49); };
            c.w64(1, t488);
            let Some(t489) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6b4d); };
            c.w64(5, t489);
            let r490 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r490);
            return RegfileExit::jump(c.r64(1));
        }
        203 => {
            let Some(t491) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6b57); };
            c.w32(0, t491);
            let r492 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r492);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t493) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6b66); };
            c.w32(3, t493);
            let Some(t494) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f6b69); };
            c.cmp(t494, c.r32(3), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 205; continue;
        }
        205 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f6b76); }
            blk = 254; continue;
        }
        206 => {
            c.w64(3, c.r64(1));
            let r495 = c.shr(c.r32(3), 0x7u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r495);
            let r496 = c.r32(3) & 0xfu64;
            c.w32(3, r496);
            c.logic(r496, 32);
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_l() { blk = 213; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_l() { blk = 211; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(3), 0x6u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 217; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            return RegfileExit::fault(0x70008f6be6);
        }
        220 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r497 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r497);
            let r498 = c.r64(1) & 0xfffffu64;
            c.w64(1, r498);
            c.logic(r498, 64);
            let Some(t499) = c.ld8(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6bf6); };
            c.w32(8, (t499 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t500) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6c00); };
            c.w32(8, t500);
            let Some(t501) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6c05); };
            c.w64(1, t501);
            let Some(t502) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6c09); };
            c.w64(5, t502);
            let r503 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r503);
            return RegfileExit::jump(c.r64(1));
        }
        221 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r504 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r504);
            let r505 = c.r64(1) & 0xfffffu64;
            c.w64(1, r505);
            c.logic(r505, 64);
            let Some(t506) = c.ld16(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6c22); };
            c.w32(8, (t506 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t507) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6c2c); };
            c.w32(8, t507);
            let Some(t508) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6c31); };
            c.w64(1, t508);
            let Some(t509) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6c35); };
            c.w64(5, t509);
            let r510 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r510);
            return RegfileExit::jump(c.r64(1));
        }
        222 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r511 = c.r64(1) & 0xfffffu64;
            c.w64(1, r511);
            c.logic(r511, 64);
            let Some(t512) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6c4a); };
            c.w32(8, t512);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f6c4f); }
            c.w64(11, c.r64(8));
            let r513 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r513);
            if c.cc_o() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 229; continue;
        }
        224 => {
            let Some(t514) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f6c63); };
            c.w64(11, t514);
            let Some(t515) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f6c6a); };
            c.w64(0, t515);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t516) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f6c71); };
            c.w64(2, t516);
            let Some(t517) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f6c78); };
            c.cmp(t517, c.r64(3), 64);
            if c.cc_a() { blk = 227; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f6c87, 0x70008f6c82);
        }
        226 => {
            let Some(t518) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6c87); };
            c.w64(8, t518);
            let Some(t519) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6c8b); };
            c.w64(15, t519);
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f6c8f); };
            c.w64(9, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f6c93); };
            c.w64(12, t521);
            blk = 228; continue;
        }
        227 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f6c99); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 228; continue;
        }
        228 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t522) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f6cac); };
            c.w64(8, t522);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f6cb3); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f6cb7); }
            blk = 229; continue;
        }
        229 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x4));
            let Some(t523) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6cc3); };
            c.w32(8, t523);
            let Some(t524) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6cc8); };
            c.w64(1, t524);
            let Some(t525) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6ccc); };
            c.w64(5, t525);
            let r526 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r526);
            return RegfileExit::jump(c.r64(1));
        }
        230 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r527 = c.r64(1) & 0xfffffu64;
            c.w64(1, r527);
            c.logic(r527, 64);
            let Some(t528) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6ce1); };
            c.w32(8, t528);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f6ce6); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            let r529 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r529);
            blk = 237; continue;
        }
        232 => {
            let Some(t530) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f6cf8); };
            c.w64(11, t530);
            let Some(t531) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f6cff); };
            c.w64(0, t531);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t532) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f6d06); };
            c.w64(2, t532);
            let Some(t533) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f6d0d); };
            c.cmp(t533, c.r64(3), 64);
            if c.cc_a() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f6d1c, 0x70008f6d17);
        }
        234 => {
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6d1c); };
            c.w64(8, t534);
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6d20); };
            c.w64(15, t535);
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f6d24); };
            c.w64(9, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f6d28); };
            c.w64(12, t537);
            blk = 236; continue;
        }
        235 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f6d2e); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 236; continue;
        }
        236 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t538) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f6d44); };
            c.w64(8, t538);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f6d4b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f6d4f); }
            c.w64(8, c.r64(0));
            blk = 237; continue;
        }
        237 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t539) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6d5e); };
            c.w32(11, t539);
            let Some(t540) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6d63); };
            c.w64(1, t540);
            let Some(t541) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6d67); };
            c.w64(5, t541);
            c.w64(0, c.r64(8));
            let r542 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r542);
            return RegfileExit::jump(c.r64(1));
        }
        238 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r543 = c.r64(1) & 0xfffffu64;
            c.w64(1, r543);
            c.logic(r543, 64);
            let Some(t544) = c.ld64(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6d7f); };
            c.w64(8, t544);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f6d84); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 246; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t545) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f6d8e); };
            c.w64(11, t545);
            let Some(t546) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f6d95); };
            c.w64(0, t546);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t547) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f6d9c); };
            c.w64(2, t547);
            let Some(t548) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f6da3); };
            c.cmp(t548, c.r64(3), 64);
            if c.cc_a() { blk = 242; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x70008f6db2, 0x70008f6dad);
        }
        241 => {
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6db2); };
            c.w64(8, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6db6); };
            c.w64(15, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f6dba); };
            c.w64(9, t551);
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f6dbe); };
            c.w64(12, t552);
            blk = 243; continue;
        }
        242 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f6dc4); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 243; continue;
        }
        243 => {
            let Some(t553) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f6dce); };
            c.w64(3, t553);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008f6dd5); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008f6dde); }
            let r554 = c.neg(c.r64(8), 64);
            c.w64(8, r554);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008f6de8); }
            blk = 251; continue;
        }
        245 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008f6dee); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008f6df5); }
            blk = 251; continue;
        }
        246 => {
            let Some(t555) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f6dfb); };
            c.w64(8, t555);
            let Some(t556) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f6e02); };
            c.w64(11, t556);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t557) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f6e09); };
            c.w64(0, t557);
            let Some(t558) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f6e10); };
            c.cmp(t558, c.r64(3), 64);
            if c.cc_a() { blk = 249; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008f6e1f, 0x70008f6e1a);
        }
        248 => {
            blk = 250; continue;
        }
        249 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008f6e21); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 250; continue;
        }
        250 => {
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f6e2b); };
            c.w64(8, t559);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f6e32); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008f6e36); }
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f6e3d); };
            c.w64(12, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6e41); };
            c.w64(15, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f6e45); };
            c.w64(9, t562);
            blk = 251; continue;
        }
        251 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x4));
            let Some(t563) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6e50); };
            c.w32(8, t563);
            let Some(t564) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6e55); };
            c.w64(1, t564);
            let Some(t565) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6e59); };
            c.w64(5, t565);
            let r566 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r566);
            return RegfileExit::jump(c.r64(1));
        }
        252 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r567 = c.r64(1) & 0xfffffu64;
            c.w64(1, r567);
            c.logic(r567, 64);
            if !c.movss_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f6e6e); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 254; continue;
        }
        253 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r568 = c.r64(1) & 0xfffffu64;
            c.w64(1, r568);
            c.logic(r568, 64);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f6e85); }
            blk = 254; continue;
        }
        254 => {
            let Some(t569) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f6e8b); };
            c.w64(8, t569);
            let Some(t570) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f6e92); };
            c.w64(11, t570);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008f6e99); }
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f6e9e); };
            c.w64(3, t571);
            let Some(t572) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f6ea5); };
            c.cmp(t572, c.r64(0), 64);
            if c.cc_a() { blk = 257; continue; }
            blk = 255; continue;
        }
        255 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f6eb4, 0x70008f6eaf);
        }
        256 => {
            blk = 258; continue;
        }
        257 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008f6eb6); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 258; continue;
        }
        258 => {
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f6ec0); };
            c.w64(3, t573);
            c.w64(9, c.r64(3).wrapping_add(0x4));
            let Some(t574) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f6ec8); };
            c.w64(8, t574);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f6ecf); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008f6ed3); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f6ed8); }
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f6edd); };
            c.w64(12, t575);
            let Some(t576) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6ee1); };
            c.w32(8, t576);
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6ee6); };
            c.w64(15, t577);
            let Some(t578) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6eea); };
            c.w64(1, t578);
            let Some(t579) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6eee); };
            c.w64(5, t579);
            let r580 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r580);
            return RegfileExit::jump(c.r64(1));
        }
        259 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r581 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r581);
            let r582 = c.r64(1) & 0xfffffu64;
            c.w64(1, r582);
            c.logic(r582, 64);
            let Some(t583) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f6f07); };
            c.w32(8, t583);
            let Some(t584) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f6f0c); };
            c.w32(0, t584);
            let r585 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r585);
            let Some(t586) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6f13); };
            c.w64(1, t586);
            let Some(t587) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6f17); };
            c.w64(5, t587);
            let r588 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r588);
            return RegfileExit::jump(c.r64(1));
        }
        260 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f6f25); }
            let Some(t589) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008f6f29); };
            c.w64(8, t589);
            let Some(t590) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6f30); };
            c.w32(8, t590);
            let r591 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r591);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f6f37); }
            let Some(t592) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x70008f6f39); };
            c.w64(3, t592);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008f6f4d, 0x70008f6f48);
        }
        261 => {
            return RegfileExit::fault(0x70008f6f4d);
        }
        262 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        263 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(3, c.r64(9).wrapping_add(0x4));
            let Some(t593) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f6f59); };
            c.w32(2, t593);
            let Some(t594) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6f5f); };
            c.w32(1, t594);
            let r595 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r595);
            let Some(t596) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f6f67); };
            c.w64(6, t596);
            let Some(t597) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6f6b); };
            c.w32(2, t597);
            let r598 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r598);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f6f72); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f6f76); }
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f6f77); };
            if !c.push64(t599) { return RegfileExit::fault(0x70008f6f77); }
            let Some(t600) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f6f7a); };
            if !c.push64(t600) { return RegfileExit::fault(0x70008f6f7a); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f6f7d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008f6f7f); }
            let Some(t601) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x70008f6f83); };
            c.w64(3, t601);
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008f6f97, 0x70008f6f92);
        }
        264 => {
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6f97); };
            c.w64(8, t602);
            let Some(t603) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f6f9b); };
            c.w64(8, t603);
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f6f9f); };
            c.w64(9, t604);
            let Some(t605) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f6fa3); };
            c.w32(11, t605);
            let Some(t606) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f6fa8); };
            c.w64(15, t606);
            let Some(t607) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f6fac); };
            c.w64(1, t607);
            let Some(t608) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f6fb0); };
            c.w64(5, t608);
            c.w64(12, c.r64(8));
            let r609 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r609);
            return RegfileExit::jump(c.r64(1));
        }
        265 => {
            return RegfileExit::fault(0x70008f6fbd);
        }
        266 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f6fbe); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f6fc2); }
            let r610 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r610);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x4));
            let Some(t611) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f6fd0); };
            c.w32(3, t611);
            let Some(t612) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6fd6); };
            c.w32(2, t612);
            let r613 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r613);
            let Some(t614) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f6fde); };
            c.w64(1, t614);
            let Some(t615) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f6fe2); };
            c.w32(3, t615);
            let r616 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r616);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f6fe9); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f6fed); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f6ff1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f6ff5); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            c.w32(0, 0xeu64);
            c.w64(11, c.r64(0));
            return c.call(0x7000804df4, 0x70008f700c, 0x70008f7007);
        }
        267 => {
            let Some(t617) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f700c); };
            c.w64(8, t617);
            let Some(t618) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7010); };
            c.w64(8, t618);
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f7014); };
            c.w64(9, t619);
            let Some(t620) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f7018); };
            c.w32(11, t620);
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f701d); };
            c.w64(15, t621);
            let Some(t622) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f7021); };
            c.w64(1, t622);
            let Some(t623) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f7025); };
            c.w64(5, t623);
            c.w64(12, c.r64(8));
            let r624 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r624);
            return RegfileExit::jump(c.r64(1));
        }
        268 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f7032); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f7036); }
            let r625 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r625);
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t626) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f7040); };
            c.w32(3, t626);
            let Some(t627) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7046); };
            c.w32(2, t627);
            let r628 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r628);
            let Some(t629) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f704e); };
            c.w32(3, t629);
            let r630 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r630);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f7055); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f7059); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f705a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f705e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f7062); }
            let Some(t631) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x70008f7066); };
            c.w64(3, t631);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008f707a, 0x70008f7075);
        }
        269 => {
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f707a); };
            c.w64(8, t632);
            let Some(t633) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f707e); };
            c.w64(8, t633);
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7082); };
            c.w64(11, t634);
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t635) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f708a); };
            c.w32(11, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f708f); };
            c.w64(15, t636);
            let Some(t637) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f7093); };
            c.w64(1, t637);
            let Some(t638) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f7097); };
            c.w64(5, t638);
            c.w64(12, c.r64(8));
            let r639 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r639);
            return RegfileExit::jump(c.r64(1));
        }
        270 => {
            return RegfileExit::jump(0x70008f70a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f70b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f70b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f70b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f70be); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f70c4); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f70cc); };
            c.w64(8, t4);
            let Some(t5) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f70d0); };
            c.w32(11, t5);
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f70d6); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f70dd); };
            c.w64(7, t8);
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f70e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f70e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f70ed); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f70fd); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f7101); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f7102); }
            c.w64(6, c.r64(7));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2728)) else { return RegfileExit::fault(0x70008f7106); };
            c.w64(3, t9);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008f711a, 0x70008f7115);
        }
        3 => {
            blk = 7; continue;
        }
        4 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f711c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f7120); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f7121); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2730)) else { return RegfileExit::fault(0x70008f7122); };
            c.w64(3, t10);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008f7136, 0x70008f7131);
        }
        5 => {
            blk = 7; continue;
        }
        6 => {
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f7138); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f713c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f713d); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2738)) else { return RegfileExit::fault(0x70008f713e); };
            c.w64(3, t11);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008f7152, 0x70008f714d);
        }
        7 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7152); };
            c.w64(2, t12);
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7156); };
            c.w64(2, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f715a); };
            c.w64(7, t14);
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f7162); };
            c.w32(7, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7167); };
            c.w64(15, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f716b); };
            c.w64(1, t17);
            let Some(t18) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f716f); };
            c.w64(5, t18);
            c.w64(12, c.r64(2));
            let r19 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r19);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            return RegfileExit::jump(0x70008f7180);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f7194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f7194); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f7198); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f719e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f71a2); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f71a6); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f71ad); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f71b4); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f71b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f71bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x70008f71c0); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f71c4); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f71d2); };
            c.w64(0, t9);
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f71d9); };
            c.w64(3, (t10 as u8 as i8 as i64 as u64));
            let Some(t11) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f71df); };
            c.w64(3, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f71e3); };
            c.w64(11, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f71ea); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 345; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f71fc); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r15);
            blk = 7; continue;
        }
        6 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f7204); };
            c.w64(2, t16);
            blk = 7; continue;
        }
        7 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f720b); };
            c.logic(t17 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t18) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7218); };
            c.w32(1, t18);
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(6, c.r32(2));
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f7226); };
            c.w32(7, t19);
            let r20 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r20);
            let r21 = c.r32(6) | 0x2u64;
            c.w32(6, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(1, c.r32(7));
            let r22 = c.r32(1) & 0x3u64;
            c.w32(1, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7242); };
            c.w32(1, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x70008f7245); };
            c.cmp(t24, c.r32(1), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t25) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7252); };
            c.w32(1, t25);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r26 = c.sub(c.r64(1), 0x2u64, 64);
            c.w64(1, r26);
            blk = 12; continue;
        }
        12 => {
            c.w64(6, c.r64(1).wrapping_mul(4));
            c.w32(15, c.r32(2));
            let Some(t27) = c.ld32(c.r64(7).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f726a); };
            c.w32(11, t27);
            let r28 = c.r32(15) | 0x2u64;
            c.w32(15, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let r29 = c.sub(c.r64(1), 0x2u64, 64);
            c.w64(1, r29);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_ge() { blk = 12; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 78; continue;
        }
        15 => {
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f7287); };
            c.w32(11, t30);
            let r31 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r31);
            c.w64(2, c.r64(11));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7292); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f7296); };
            c.w64(11, t33);
            blk = 17; continue;
        }
        16 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008f729f); };
            c.w32(2, t34);
            let r35 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r35);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 86; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(1, c.r64(3));
            c.w64(6, c.r64(2));
            blk = 19; continue;
        }
        19 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(7, c.r64(6));
            let r36 = c.r64(7) & 0xfu64;
            c.w64(7, r36);
            c.logic(r36, 64);
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(7), 0x5u64, 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r64(7), 0xdu64, 64);
            if c.cc_e() { blk = 80; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 173; continue;
        }
        23 => {
            c.w64(11, c.r64(1));
            blk = 80; continue;
        }
        24 => {
            c.logic(c.r8(6) & 0x40u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(6));
            if let Some(sr) = c.shr(c.r64(11), 0x9u64, 64) { c.w64(11, sr); }
            c.logic(c.r16(6) & 0x100u64, 16);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            let r37 = c.r64(11) & 0x7ffu64;
            c.w64(11, r37);
            c.logic(r37, 64);
            c.logic(c.r8(6) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 32; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f730e); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008f7319); };
            c.w64(1, t40);
            blk = 28; continue;
        }
        28 => {
            let Some(t41) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7320); };
            c.w32(11, t41);
            let r42 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r42);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 29; continue;
        }
        29 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f733b); };
            c.w32(2, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f733f); };
            c.cmp(t44, c.r32(2), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f734c); }
            blk = 74; continue;
        }
        32 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f735c); };
            c.w32(0, t45);
            let r46 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r46);
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        34 => {
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f736c); };
            c.w32(11, t47);
            let r48 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r48);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f737e); };
            c.w32(2, t49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f7382); };
            c.cmp(t50, c.r32(2), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f738f); }
            blk = 74; continue;
        }
        37 => {
            c.w64(11, c.r64(6));
            let r51 = c.shr(c.r32(11), 0x7u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r51);
            let r52 = c.r32(11) & 0xfu64;
            c.w32(11, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_l() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_l() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_e() { blk = 342; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 325; continue;
        }
        43 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 325; continue;
        }
        46 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 325; continue;
        }
        51 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 325; continue;
        }
        54 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r53 = c.r64(6) & 0xfffffu64;
            c.w64(6, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f742e); };
            c.w32(11, (t54 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        55 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r55 = c.r64(6) & 0xfffffu64;
            c.w64(6, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld16(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f744b); };
            c.w32(11, (t56 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        56 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r57 = c.r64(6) & 0xfffffu64;
            c.w64(6, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7468); };
            c.w32(11, t58);
            c.w64(0, c.r64(11));
            let r59 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r59);
            if c.cc_o() { blk = 326; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 58; continue;
        }
        58 => {
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        59 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r60 = c.r64(6) & 0xfffffu64;
            c.w64(6, r60);
            c.logic(r60, 64);
            let Some(t61) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f748f); };
            c.w32(11, t61);
            c.cmp(c.r32(11), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 331; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 61; continue;
        }
        61 => {
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        62 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r62 = c.r64(6) & 0xfffffu64;
            c.w64(6, r62);
            c.logic(r62, 64);
            let Some(t63) = c.ld64(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f74b8); };
            c.w64(11, t63);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f74c3); };
            c.w64(0, t64);
            let Some(t65) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f74ca); };
            c.w64(3, t65);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f74d1); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f74d8); };
            c.cmp(t67, c.r64(2), 64);
            if c.cc_be() { blk = 336; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008f74e1); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 65; continue;
        }
        65 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f74e8); };
            c.w64(3, t68);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008f74ef); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_g() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008f74f8); }
            let r69 = c.neg(c.r64(11), 64);
            c.w64(11, r69);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x70008f7502); }
            blk = 71; continue;
        }
        67 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008f7508); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x70008f750f); }
            blk = 71; continue;
        }
        68 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f7515); };
            c.w64(11, t70);
            let Some(t71) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f751c); };
            c.w64(0, t71);
            c.w64(2, c.r64(0).wrapping_add(0x8));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f7523); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f752a); };
            c.cmp(t73, c.r64(2), 64);
            if c.cc_be() { blk = 338; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008f7533); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f753a); };
            c.w64(0, t74);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008f7541); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008f7545); }
            c.w64(0, c.r64(11));
            blk = 71; continue;
        }
        71 => {
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        72 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r75 = c.r64(6) & 0xfffffu64;
            c.w64(6, r75);
            c.logic(r75, 64);
            if !c.movss_rm(0, c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f7563); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 74; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r76 = c.r64(6) & 0xfffffu64;
            c.w64(6, r76);
            c.logic(r76, 64);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f757a); }
            blk = 74; continue;
        }
        74 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f7580); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f7587); };
            c.w64(0, t78);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f758e); };
            c.w64(2, t79);
            let Some(t80) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f7595); };
            c.cmp(t80, c.r64(3), 64);
            if c.cc_be() { blk = 340; continue; }
            blk = 75; continue;
        }
        75 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f759e); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 76; continue;
        }
        76 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f75a5); };
            c.w64(0, t81);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008f75ac); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f75b0); }
            blk = 80; continue;
        }
        77 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r82 = c.r64(6) & 0xfffffu64;
            c.w64(6, r82);
            c.logic(r82, 64);
            let Some(t83) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f75c3); };
            c.w32(0, t83);
            let r84 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r84);
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        78 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t85) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f75d3); };
            c.w64(2, t85);
            let Some(t86) = c.ld8(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f75d7); };
            c.w32(1, t86);
            let Some(t87) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f75dd); };
            c.w32(6, t87);
            let r88 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r88);
            let Some(t89) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f75e3); };
            c.w64(7, t89);
            let Some(t90) = c.ld8(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f75e7); };
            c.w32(2, t90);
            let Some(t91) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f75ed); };
            c.w32(1, t91);
            let r92 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r92);
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f75f7); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f75fb); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f75fc); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f75fd); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f75fe); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2050)) else { return RegfileExit::fault(0x70008f75ff); };
            c.w64(3, t93);
            c.w32(0, 0x4u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008f7613, 0x70008f760e);
        }
        79 => {
            c.w64(11, c.r64(0));
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7616); };
            c.w64(15, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f761a); };
            c.w64(8, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f761e); };
            c.w64(9, t96);
            blk = 80; continue;
        }
        80 => {
            let Some(t97) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7622); };
            c.w64(0, t97);
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(3).wrapping_add(0x4));
            let Some(t98) = c.ld8(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008f762d); };
            c.w32(3, t98);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t99) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f7642); };
            c.w64(1, t99);
            let Some(t100) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f7646); };
            c.w64(5, t100);
            c.w64(12, c.r64(0));
            c.w64(0, c.r64(11));
            let r101 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r101);
            return RegfileExit::jump(c.r64(1));
        }
        82 => {
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f7656); }
            let r102 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r102);
            let Some(t103) = c.ld8(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008f7662); };
            c.w32(8, t103);
            let Some(t104) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f7667); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f766b); };
            c.w64(5, t105);
            c.w64(12, c.r64(0));
            c.w64(0, c.r64(11));
            let r106 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r106);
            return RegfileExit::jump(c.r64(1));
        }
        83 => {
            blk = 78; continue;
        }
        84 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t107) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7684); };
            c.w64(2, t107);
            let Some(t108) = c.ld8(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f7688); };
            c.w32(1, t108);
            let Some(t109) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f768e); };
            c.w32(6, t109);
            let r110 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r110);
            let Some(t111) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f7694); };
            c.w64(7, t111);
            let Some(t112) = c.ld8(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7698); };
            c.w32(2, t112);
            let Some(t113) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f769e); };
            c.w32(1, t113);
            let r114 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r114);
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f76a8); }
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(6, c.r64(7));
            return c.call(0x7000832fd4, 0x70008f76c6, 0x70008f76c1);
        }
        85 => {
            c.w64(11, c.r64(0));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f76c9); };
            c.w64(15, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f76cd); };
            c.w64(8, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f76d1); };
            c.w64(9, t117);
            blk = 80; continue;
        }
        86 => {
            let Some(t118) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f76da); };
            c.w32(1, t118);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008f76dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008f76e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008f76e5); }
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70008f76e9); };
            c.cmp(t119, c.r32(1), 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t120) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f76f6); };
            c.w32(1, t120);
            let r121 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r121);
            c.w32(6, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t122) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7702); };
            c.cmp(t122, c.r32(6), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t123) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f770b); };
            c.w32(1, t123);
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008f7714); }
            c.logic(c.r8(7) & 0x30u64, 8);
            if c.cc_e() { blk = 161; continue; }
            blk = 90; continue;
        }
        90 => {
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 153; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t124) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f772c); };
            c.w32(7, t124);
            let r125 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r125);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008f7738); };
            c.w64(7, t126);
            blk = 93; continue;
        }
        93 => {
            let Some(t127) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f773f); };
            c.w32(0, t127);
            let Some(t128) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f7742); };
            c.w32(1, t128);
            let Some(t129) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7748); };
            c.w32(2, t129);
            let r130 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r130);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t131) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7753); };
            c.w32(1, t131);
            let r132 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r132);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r133 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r133);
            let Some(t134) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7761); };
            c.w32(15, t134);
            let r135 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r135);
            let r136 = c.r64(15) & c.r64(0);
            c.w64(15, r136);
            c.logic(r136, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008f776c); }
            blk = 94; continue;
        }
        94 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(11, c.r64(3).wrapping_mul(4));
            let Some(t137) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f777d); };
            c.w32(2, t137);
            let Some(t138) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f7782); };
            c.cmp(t138, c.r32(2), 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            let r139 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r139);
            c.w64(11, c.r64(6).wrapping_add(c.r64(15)));
            c.w64(3, c.r64(0));
            let r140 = c.r64(3) & c.r64(11);
            c.w64(3, r140);
            c.logic(r140, 64);
            c.w64(15, c.r64(3));
            blk = 94; continue;
        }
        97 => {
            let Some(t141) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f77a6); };
            c.w32(0, t141);
            let Some(t142) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f77a9); };
            c.w32(2, t142);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r143 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r143);
            c.cmp(c.r64(3), c.r64(0), 64);
            if c.cc_b() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            return RegfileExit::fault(0x70008f77bd);
        }
        99 => {
            let Some(t144) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f77be); };
            c.w32(0, t144);
            let Some(t145) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f77c4); };
            c.w64(3, t145);
            let Some(t146) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f77c8); };
            c.w32(11, t146);
            let r147 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f77d0); };
            c.w64(1, t148);
            let Some(t149) = c.ld32(c.r64(1).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f77d4); };
            c.w32(0, t149);
            let r150 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r150);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t151) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f77e4); };
            c.w32(2, t151);
            let Some(t152) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f77e8); };
            c.cmp(t152, c.r32(2), 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t153) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f77f5); };
            c.w32(2, t153);
            let r154 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r154);
            let Some(t155) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f77fc); };
            c.w32(11, t155);
            let r156 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r156);
            let Some(t157) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f7803); };
            c.logic(t157 & 0x2u64, 8);
            if c.cc_ne() { blk = 123; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f780e); };
            c.cmp(t158, c.r32(11), 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f7817); };
            c.w64(11, t159);
            blk = 151; continue;
        }
        104 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f7823); };
            c.w64(11, t160);
            let Some(t161) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f782a); };
            c.cmp(t161, 0x20u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t162) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f7834); };
            c.w32(11, t162);
            let Some(t163) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f7838); };
            c.cmp(t163, c.r32(11), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f7841); };
            c.w64(3, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7845); };
            c.w64(0, t165);
            blk = 78; continue;
        }
        107 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f784e); };
            c.w64(1, t166);
            let Some(t167) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7852); };
            c.w32(11, t167);
            let r168 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r168);
            let Some(t169) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7859); };
            c.w32(0, t169);
            let Some(t170) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f785d); };
            c.cmp(t170, c.r32(0), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t171) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f7866); };
            c.w32(11, t171);
            let r172 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r172);
            let Some(t173) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f786d); };
            c.w32(0, t173);
            let Some(t174) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f7871); };
            c.cmp(t174, c.r32(0), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 109; continue;
        }
        109 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 120; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t175) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7884); };
            c.w32(0, t175);
            let r176 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r176);
            let Some(t177) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f788b); };
            c.w32(3, t177);
            c.w32(6, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 116; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w32(11, c.r16(3));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 115; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t178) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f78a7); };
            c.cmp(t178, c.r32(0), 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(11, c.r64(1));
            blk = 116; continue;
        }
        114 => {
            c.w64(3, c.r64(1));
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f78b8); };
            c.w64(0, t179);
            blk = 78; continue;
        }
        115 => {
            let Some(t180) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f78c1); };
            c.w32(11, t180);
            let r181 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r181);
            blk = 116; continue;
        }
        116 => {
            let Some(t182) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f78c8); };
            c.w32(11, t182);
            let r183 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r183);
            let Some(t184) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f78cf); };
            c.w32(11, t184);
            let r185 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r185);
            let Some(t186) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f78d6); };
            c.w32(0, t186);
            let r187 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r187);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 119; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t188) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f78e9); };
            c.w32(6, t188);
            let r189 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r189);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f78f0); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f78f4); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008f78fc, 0x70008f78f7);
        }
        118 => {
            c.w64(11, c.r64(0));
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f78ff); };
            c.w64(8, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7903); };
            c.w64(9, t191);
            blk = 151; continue;
        }
        119 => {
            c.w64(3, c.r64(1));
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f790f); };
            c.w64(0, t192);
            blk = 78; continue;
        }
        120 => {
            c.w64(3, c.r64(1));
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f791b); };
            c.w64(0, t193);
            blk = 78; continue;
        }
        121 => {
            c.w64(11, c.r64(9));
            c.w32(2, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f792b); }
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f792f); };
            if !c.push64(t194) { return RegfileExit::fault(0x70008f792f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f7932); }
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f7933); };
            if !c.push64(t195) { return RegfileExit::fault(0x70008f7933); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f793e); };
            c.w64(3, t196);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008f794d, 0x70008f7948);
        }
        122 => {
            c.w64(11, c.r64(0));
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7950); };
            c.w64(8, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7954); };
            c.w64(9, t198);
            blk = 151; continue;
        }
        123 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f7964); }
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f7968); };
            if !c.push64(t199) { return RegfileExit::fault(0x70008f7968); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70007f2274, 0x70008f7978, 0x70008f7973);
        }
        124 => {
            c.w64(11, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f797b); };
            c.w64(8, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f797f); };
            c.w64(9, t201);
            blk = 151; continue;
        }
        125 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f7988); };
            c.w64(2, t202);
            let Some(t203) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f798c); };
            c.w32(0, t203);
            let r204 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r204);
            let Some(t205) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7992); };
            c.w32(3, t205);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 148; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 136; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(3, c.r64(2));
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f79b6); };
            c.w64(0, t206);
            blk = 78; continue;
        }
        129 => {
            let Some(t207) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f79bf); };
            c.w32(11, t207);
            let Some(t208) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f79c3); };
            c.cmp(t208, c.r32(11), 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(3, c.r64(2));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f79cf); };
            c.w64(0, t209);
            blk = 78; continue;
        }
        131 => {
            let Some(t210) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f79d8); };
            c.w32(11, t210);
            let r211 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r211);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(3, c.r64(2));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f79e8); };
            c.w64(0, t212);
            blk = 78; continue;
        }
        133 => {
            let Some(t213) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f79f1); };
            c.w32(0, t213);
            let r214 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r214);
            let Some(t215) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f79f8); };
            c.cmp(t215, 0x40u64, 16);
            if c.cc_b() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            c.w64(3, c.r64(2));
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7a02); };
            c.w64(0, t216);
            blk = 78; continue;
        }
        135 => {
            let Some(t217) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7a0b); };
            c.w32(11, t217);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 151; continue;
        }
        136 => {
            let Some(t218) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7a1b); };
            c.w32(11, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f7a1f); };
            c.cmp(t219, c.r32(11), 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w64(3, c.r64(2));
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7a2b); };
            c.w64(0, t220);
            blk = 78; continue;
        }
        138 => {
            let Some(t221) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f7a34); };
            c.w32(11, t221);
            c.w64(0, c.r64(11));
            let r222 = c.r32(0) & 0xc0u64;
            c.w32(0, r222);
            c.logic(r222, 32);
            c.cmp(c.r32(0), 0xc0u64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t223) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f7a48); };
            c.w32(0, t223);
            let r224 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r224);
            let Some(t225) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f7a4e); };
            c.w32(0, t225);
            let r226 = c.r32(0) & 0x1fu64;
            c.w32(0, r226);
            c.logic(r226, 32);
            let r227 = c.sub(c.r32(0), 0x10u64, 32);
            c.w32(0, r227);
            c.cmp(c.r32(0), 0x4u64, 32);
            if c.cc_be() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(3, c.r64(2));
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7a5f); };
            c.w64(0, t228);
            blk = 78; continue;
        }
        141 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 147; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t229) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f7a6e); };
            c.w32(0, t229);
            let r230 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f7a74); };
            c.cmp(t231, c.r32(0), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t232) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7a7d); };
            c.w32(11, t232);
            let Some(t233) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f7a81); };
            c.cmp(t233, c.r32(11), 32);
            if c.cc_ne() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t234) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f7a8a); };
            c.w32(0, t234);
            let r235 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r235);
            blk = 145; continue;
        }
        145 => {
            c.w64(11, c.r64(0));
            blk = 151; continue;
        }
        146 => {
            c.w64(3, c.r64(2));
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7a98); };
            c.w64(0, t236);
            blk = 78; continue;
        }
        147 => {
            c.w64(3, c.r64(2));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7aa4); };
            c.w64(0, t237);
            blk = 78; continue;
        }
        148 => {
            let Some(t238) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7aad); };
            c.w32(11, t238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f7ab1); };
            c.cmp(t239, c.r32(11), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w64(3, c.r64(2));
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7abd); };
            c.w64(0, t240);
            blk = 78; continue;
        }
        150 => {
            let Some(t241) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f7ac6); };
            c.w32(0, t241);
            let r242 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r242);
            c.w64(11, c.r64(0));
            blk = 151; continue;
        }
        151 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7acf); };
            c.w64(15, t243);
            blk = 80; continue;
        }
        152 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7ad8); };
            c.w64(15, t244);
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f7adc); };
            c.w64(11, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f7ae3); };
            c.w64(3, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7ae7); };
            c.w64(0, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f7aeb); };
            c.w64(2, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f7aef); };
            c.w64(1, t249);
            blk = 161; continue;
        }
        153 => {
            let Some(t250) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f7af8); };
            c.w32(6, t250);
            let r251 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r251);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t252) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f7b07); };
            c.w64(7, t252);
            let r253 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r253);
            c.logic(r253, 64);
            let Some(t254) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7b0f); };
            c.w32(7, t254);
            let r255 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r255);
            let Some(t256) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f7b15); };
            c.w32(7, t256);
            let r257 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r257);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t258) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7b1f); };
            c.w32(12, t258);
            let r259 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r259);
            let Some(t260) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7b26); };
            c.cmp(t260, 0xbeu64, 16);
            if c.cc_ne() { blk = 78; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t261) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7b34); };
            c.w32(12, t261);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xa9u64, 64);
            if c.cc_a() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            return RegfileExit::fault(0x70008f7b47);
        }
        158 => {
            let Some(t262) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7b48); };
            c.w32(12, t262);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t263) = c.ld32(c.r64(6).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70008f7b4f); };
            c.w32(6, t263);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xa9u64, 64);
            if c.cc_a() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            return RegfileExit::fault(0x70008f7b61);
        }
        160 => {
            let Some(t264) = c.ld32(c.r64(7).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70008f7b62); };
            c.cmp(t264, c.r32(6), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t265) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f7b6e); };
            c.w32(6, t265);
            let r266 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r266);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 167; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t267) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f7b7a); };
            c.cmp(t267, c.r32(6), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 164; continue;
        }
        164 => {
            let r268 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r268);
            c.logic(r268, 64);
            blk = 166; continue;
        }
        165 => {
            c.w64(6, c.r64(3));
            blk = 166; continue;
        }
        166 => {
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(6));
            c.w64(6, c.r64(10));
            blk = 19; continue;
        }
        167 => {
            c.w64(11, c.r64(6));
            blk = 80; continue;
        }
        168 => {
            c.w64(11, c.r64(9));
            c.w32(1, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t269) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7bb2); };
            c.w64(6, t269);
            let Some(t270) = c.ld8(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f7bb6); };
            c.w32(7, t270);
            let Some(t271) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7bbc); };
            c.w32(9, t271);
            let r272 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r272);
            let Some(t273) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f7bc3); };
            c.w64(12, t273);
            let Some(t274) = c.ld8(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7bc7); };
            c.w32(6, t274);
            let Some(t275) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7bcd); };
            c.w32(7, t275);
            let r276 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r276);
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008f7bd8); }
            c.w64(8, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(1, c.r64(7));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(6));
            c.w64(6, c.r64(12));
            let Some(t277) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f7bee); };
            c.logic(t277 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            let r278 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r278);
            blk = 171; continue;
        }
        170 => {
            let Some(t279) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f7bfe); };
            c.w32(8, t279);
            let Some(t280) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008f7c02); };
            c.w64(8, t280);
            blk = 171; continue;
        }
        171 => {
            return c.call(c.r64(8), 0x70008f7c0d, 0x70008f7c0a);
        }
        172 => {
            c.w64(11, c.r64(0));
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7c10); };
            c.w64(15, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7c14); };
            c.w64(8, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7c18); };
            c.w64(9, t283);
            blk = 80; continue;
        }
        173 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008f7c21); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008f7c25); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008f7c29); }
            c.cmp(c.r64(7), 0x2u64, 64);
            if c.cc_e() { blk = 263; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r64(7), 0x6u64, 64);
            if c.cc_e() { blk = 261; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 256; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r64(7), 0x8u64, 64);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 245; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r64(7), 0x9u64, 64);
            if c.cc_e() { blk = 245; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r64(7), 0x3u64, 64);
            if c.cc_e() { blk = 192; continue; }
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 259; continue; }
            blk = 180; continue;
        }
        180 => {
            c.cmp(c.r64(7), 0xcu64, 64);
            if c.cc_e() { blk = 190; continue; }
            blk = 181; continue;
        }
        181 => {
            c.cmp(c.r64(7), 0xeu64, 64);
            if c.cc_e() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(11, c.r64(9));
            c.w32(2, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t284) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7c96); };
            c.w64(6, t284);
            let Some(t285) = c.ld8(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f7c9a); };
            c.w32(7, t285);
            let Some(t286) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7ca0); };
            c.w32(9, t286);
            let r287 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r287);
            let Some(t288) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f7ca7); };
            c.w64(12, t288);
            let Some(t289) = c.ld8(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7cab); };
            c.w32(6, t289);
            let Some(t290) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7cb1); };
            c.w32(7, t290);
            let r291 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r291);
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f7cbc); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008f7cc0); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f7cc1); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f7cc2); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008f7cc3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f7cc4); }
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x70008f7cc5); };
            c.w64(3, t292);
            c.w32(0, 0x5u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008f7cd9, 0x70008f7cd4);
        }
        183 => {
            c.w64(11, c.r64(0));
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7cdc); };
            c.w64(15, t293);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7ce0); };
            c.w64(8, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7ce4); };
            c.w64(9, t295);
            blk = 80; continue;
        }
        184 => {
            let Some(t296) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f7ced); };
            c.w32(11, t296);
            let r297 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r297);
            let Some(t298) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7cf4); };
            c.w32(11, t298);
            let r299 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r299);
            let Some(t300) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7cfb); };
            c.w32(0, t300);
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r301 = c.r64(6) & 0x1ffffffu64;
            c.w64(6, r301);
            c.logic(r301, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_b() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            return RegfileExit::fault(0x70008f7d15);
        }
        186 => {
            let Some(t302) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7d16); };
            c.w32(11, t302);
            let r303 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r303);
            let Some(t304) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7d1e); };
            c.w32(0, t304);
            let r305 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r305);
            let Some(t306) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f7d25); };
            c.cmp(t306, c.r32(0), 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t307) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7d3d); };
            c.w64(3, t307);
            let Some(t308) = c.ld8(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f7d41); };
            c.w32(2, t308);
            let Some(t309) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7d47); };
            c.w32(3, t309);
            let r310 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r310);
            let Some(t311) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f7d4d); };
            c.w64(1, t311);
            let Some(t312) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7d51); };
            c.w32(3, t312);
            let r313 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r313);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f7d58); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f7d61); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f7d62); }
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008f7d63); };
            c.w64(3, t314);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x70008f7d77, 0x70008f7d72);
        }
        189 => {
            c.w64(11, c.r64(0));
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7d7a); };
            c.w64(15, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7d7e); };
            c.w64(8, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7d82); };
            c.w64(9, t317);
            blk = 80; continue;
        }
        190 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t318) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7d92); };
            c.w64(2, t318);
            let Some(t319) = c.ld8(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f7d96); };
            c.w32(6, t319);
            let Some(t320) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7d9c); };
            c.w32(2, t320);
            let r321 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r321);
            let Some(t322) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f7da2); };
            c.w64(7, t322);
            let Some(t323) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7da6); };
            c.w32(2, t323);
            let r324 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r324);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f7dad); }
            c.w64(6, c.r64(7));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(2));
            c.w32(2, 0x2u64);
            return c.call(0x70008bc174, 0x70008f7dc7, 0x70008f7dc2);
        }
        191 => {
            c.w64(11, c.r64(0));
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7dca); };
            c.w64(15, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7dce); };
            c.w64(8, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7dd2); };
            c.w64(9, t327);
            blk = 80; continue;
        }
        192 => {
            let Some(t328) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f7ddb); };
            c.w32(2, t328);
            let r329 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r329);
            let Some(t330) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7de1); };
            c.w32(6, t330);
            let Some(t331) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f7de4); };
            c.cmp(t331, c.r32(2), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t332) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7df1); };
            c.w64(7, t332);
            let Some(t333) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f7df5); };
            c.w64(12, t333);
            let Some(t334) = c.ld8(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f7df9); };
            c.w32(0, t334);
            let Some(t335) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7dff); };
            c.w32(7, t335);
            let r336 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r336);
            let Some(t337) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7e05); };
            c.w32(0, t337);
            let r338 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r338);
            c.logic(c.r8(6) & 0x2u64, 8);
            if c.cc_e() { blk = 244; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t339) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7e16); };
            c.w32(6, t339);
            let r340 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r340);
            let Some(t341) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f7e1c); };
            c.cmp(t341, c.r32(6), 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t342) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7e29); };
            c.w32(6, t342);
            let r343 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r343);
            let Some(t344) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7e2f); };
            c.w32(7, t344);
            let r345 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r345);
            let Some(t346) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f7e35); };
            c.logic(t346 & 0x2u64, 8);
            if c.cc_ne() { blk = 217; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t347) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f7e3f); };
            c.cmp(t347, c.r32(7), 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            c.w64(2, c.r64(11));
            blk = 244; continue;
        }
        198 => {
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f7e50); };
            c.w64(11, t348);
            let Some(t349) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f7e57); };
            c.cmp(t349, 0x20u64, 8);
            if c.cc_e() { blk = 215; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t350) = c.ld32(c.r64(6).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f7e61); };
            c.w32(11, t350);
            let Some(t351) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f7e65); };
            c.cmp(t351, c.r32(11), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7e6e); };
            c.w64(0, t352);
            blk = 78; continue;
        }
        201 => {
            let Some(t353) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7e77); };
            c.w32(11, t353);
            let r354 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r354);
            let Some(t355) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7e7e); };
            c.w32(0, t355);
            let Some(t356) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f7e82); };
            c.cmp(t356, c.r32(0), 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t357) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f7e8b); };
            c.w32(11, t357);
            let r358 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r358);
            let Some(t359) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7e92); };
            c.w32(0, t359);
            let Some(t360) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f7e96); };
            c.cmp(t360, c.r32(0), 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 203; continue;
        }
        203 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 214; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t361) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7ea9); };
            c.w32(0, t361);
            let r362 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r362);
            let Some(t363) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7eb0); };
            c.w32(7, t363);
            c.w32(12, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 209; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w32(11, c.r16(7));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f7ece); };
            c.cmp(t364, c.r32(0), 32);
            if c.cc_ne() { blk = 210; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7ed7); };
            c.w64(0, t365);
            blk = 78; continue;
        }
        208 => {
            c.w64(1, c.r64(2));
            blk = 210; continue;
        }
        209 => {
            c.w64(1, c.r64(11));
            blk = 210; continue;
        }
        210 => {
            let Some(t366) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f7ee8); };
            c.w32(11, t366);
            let r367 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r367);
            let Some(t368) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7eef); };
            c.w32(11, t368);
            let r369 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r369);
            let Some(t370) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7ef6); };
            c.w32(0, t370);
            let r371 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r371);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 213; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t372) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f7f09); };
            c.w32(11, t372);
            let r373 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r373);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f7f10); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f7f14); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(11));
            return c.call(0x70007f6fb4, 0x70008f7f22, 0x70008f7f1d);
        }
        212 => {
            c.w64(2, c.r64(0));
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7f25); };
            c.w64(15, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7f29); };
            c.w64(8, t375);
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7f2d); };
            c.w64(9, t376);
            blk = 244; continue;
        }
        213 => {
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7f36); };
            c.w64(0, t377);
            blk = 78; continue;
        }
        214 => {
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7f3f); };
            c.w64(0, t378);
            blk = 78; continue;
        }
        215 => {
            c.w64(11, c.r64(9));
            c.w32(2, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f7f4f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f7f53); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f7f54); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f7f55); }
            let Some(t379) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f7f56); };
            c.w64(3, t379);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008f7f6a, 0x70008f7f65);
        }
        216 => {
            c.w64(2, c.r64(0));
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7f6d); };
            c.w64(15, t380);
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7f71); };
            c.w64(8, t381);
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7f75); };
            c.w64(9, t382);
            blk = 244; continue;
        }
        217 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f7f85); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f7f89); }
            c.w64(7, c.r64(6));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(12));
            return c.call(0x70007f2274, 0x70008f7f97, 0x70008f7f92);
        }
        218 => {
            c.w64(2, c.r64(0));
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f7f9a); };
            c.w64(15, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f7f9e); };
            c.w64(8, t384);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f7fa2); };
            c.w64(9, t385);
            blk = 244; continue;
        }
        219 => {
            let Some(t386) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f7fab); };
            c.w32(11, t386);
            let r387 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r387);
            let Some(t388) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7fb2); };
            c.w32(0, t388);
            c.cmp(c.r16(0), 0x834u64, 16);
            if c.cc_e() { blk = 242; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w32(7, c.r64(0).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 230; continue; }
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r16(0), 0x411u64, 16);
            if c.cc_e() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7fd2); };
            c.w64(0, t389);
            blk = 78; continue;
        }
        223 => {
            let Some(t390) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f7fdb); };
            c.w32(11, t390);
            let Some(t391) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f7fdf); };
            c.cmp(t391, c.r32(11), 32);
            if c.cc_e() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7fe8); };
            c.w64(0, t392);
            blk = 78; continue;
        }
        225 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f7ff6); };
            c.w64(0, t393);
            blk = 78; continue;
        }
        227 => {
            let Some(t394) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f7fff); };
            c.cmp(t394, 0x40u64, 16);
            if c.cc_b() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f8006); };
            c.w64(0, t395);
            blk = 78; continue;
        }
        229 => {
            let Some(t396) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f800f); };
            c.w32(11, t396);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(2, c.r64(0));
            blk = 244; continue;
        }
        230 => {
            let Some(t397) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f801f); };
            c.w32(0, t397);
            let Some(t398) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f8022); };
            c.cmp(t398, c.r32(0), 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f802b); };
            c.w64(0, t399);
            blk = 78; continue;
        }
        232 => {
            let Some(t400) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f8034); };
            c.w32(11, t400);
            c.w64(0, c.r64(11));
            let r401 = c.r32(0) & 0xc0u64;
            c.w32(0, r401);
            c.logic(r401, 32);
            c.cmp(c.r32(0), 0xc0u64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t402) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f8048); };
            c.w32(0, t402);
            let r403 = c.r32(0) & 0x1fu64;
            c.w32(0, r403);
            c.logic(r403, 32);
            let r404 = c.sub(c.r32(0), 0x10u64, 32);
            c.w32(0, r404);
            c.cmp(c.r32(0), 0x4u64, 32);
            if c.cc_be() { blk = 235; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f8056); };
            c.w64(0, t405);
            blk = 78; continue;
        }
        235 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t406) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f8065); };
            c.w32(0, t406);
            let r407 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r407);
            let Some(t408) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f806b); };
            c.cmp(t408, c.r32(0), 32);
            if c.cc_e() { blk = 240; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t409) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8074); };
            c.w32(11, t409);
            let Some(t410) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f8078); };
            c.cmp(t410, c.r32(11), 32);
            if c.cc_ne() { blk = 239; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t411) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f8081); };
            c.w32(0, t411);
            let r412 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r412);
            blk = 239; continue;
        }
        239 => {
            c.w64(2, c.r64(0));
            blk = 244; continue;
        }
        240 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f808c); };
            c.w64(0, t413);
            blk = 78; continue;
        }
        241 => {
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f8095); };
            c.w64(0, t414);
            blk = 78; continue;
        }
        242 => {
            let Some(t415) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f809e); };
            c.w32(11, t415);
            let Some(t416) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f80a2); };
            c.cmp(t416, c.r32(11), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f80ab); };
            c.w64(0, t417);
            blk = 78; continue;
        }
        244 => {
            c.w64(11, c.r64(2));
            blk = 80; continue;
        }
        245 => {
            let Some(t418) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f80bc); };
            c.w64(11, t418);
            let Some(t419) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f80c3); };
            c.cmp(t419, 0x20u64, 8);
            if c.cc_e() { blk = 259; continue; }
            blk = 246; continue;
        }
        246 => {
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t420) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f80d3); };
            c.w32(11, t420);
            let r421 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r421);
            blk = 249; continue;
        }
        248 => {
            let Some(t422) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f80dc); };
            c.w32(11, t422);
            let r423 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r423);
            blk = 249; continue;
        }
        249 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            return RegfileExit::fault(0x70008f80e9);
        }
        251 => {
            let r424 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r424);
            c.logic(r424, 64);
            let Some(t425) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f80ee); };
            c.w32(0, t425);
            let r426 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r426);
            let Some(t427) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f80f4); };
            c.w32(2, t427);
            let r428 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r428);
            let Some(t429) = c.ld64(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f80fa); };
            c.w64(0, t429);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t430) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8104); };
            c.w32(1, t430);
            let r431 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r431);
            let Some(t432) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f810a); };
            c.w32(1, t432);
            let r433 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r433);
            blk = 254; continue;
        }
        253 => {
            c.w64(1, c.r64(3));
            blk = 254; continue;
        }
        254 => {
            c.w64(6, c.r64(9));
            c.w32(7, c.r64(6).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008f811b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f811f); }
            c.w64(6, c.r64(11));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(7, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007fe8d4, 0x70008f8133, 0x70008f812e);
        }
        255 => {
            c.w64(11, c.r64(0));
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8136); };
            c.w64(15, t434);
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f813a); };
            c.w64(8, t435);
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f813e); };
            c.w64(9, t436);
            blk = 80; continue;
        }
        256 => {
            let Some(t437) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f8147); };
            c.w64(11, t437);
            let Some(t438) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f814e); };
            c.cmp(t438, 0x20u64, 8);
            if c.cc_e() { blk = 259; continue; }
            blk = 257; continue;
        }
        257 => {
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            let r439 = c.r64(6) & 0x3ffu64;
            c.w64(6, r439);
            c.logic(r439, 64);
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let r440 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r440);
            let Some(t441) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f816a); };
            c.w32(6, t441);
            let r442 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r442);
            let Some(t443) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f8170); };
            c.w32(6, t443);
            let r444 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r444);
            let Some(t445) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8176); };
            c.w64(7, t445);
            let Some(t446) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f817a); };
            c.w32(2, t446);
            let r447 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r447);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f8181); }
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(10));
            c.w64(6, c.r64(7));
            return c.call(0x70007fea14, 0x70008f8196, 0x70008f8191);
        }
        258 => {
            c.w64(11, c.r64(0));
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8199); };
            c.w64(15, t448);
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f819d); };
            c.w64(8, t449);
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f81a1); };
            c.w64(9, t450);
            blk = 80; continue;
        }
        259 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t451) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f81b1); };
            c.w64(2, t451);
            let Some(t452) = c.ld8(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f81b5); };
            c.w32(1, t452);
            let Some(t453) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f81bb); };
            c.w32(2, t453);
            let r454 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r454);
            let Some(t455) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f81c1); };
            c.w64(6, t455);
            let Some(t456) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f81c5); };
            c.w32(2, t456);
            let r457 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r457);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f81cc); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f81d0); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f81d1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f81d2); }
            let Some(t458) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f81d3); };
            c.w64(3, t458);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008f81e4, 0x70008f81df);
        }
        260 => {
            c.w64(11, c.r64(0));
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f81e7); };
            c.w64(15, t459);
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f81eb); };
            c.w64(8, t460);
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f81ef); };
            c.w64(9, t461);
            blk = 80; continue;
        }
        261 => {
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            let r462 = c.r64(6) & 0x3ffu64;
            c.w64(6, r462);
            c.logic(r462, 64);
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let r463 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r463);
            let Some(t464) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f820e); };
            c.w32(1, t464);
            let r465 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r465);
            let Some(t466) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f8214); };
            c.w32(1, t466);
            let r467 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r467);
            let Some(t468) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f821a); };
            c.w32(2, t468);
            let r469 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r469);
            let Some(t470) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8221); };
            c.w64(1, t470);
            let Some(t471) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8225); };
            c.w32(7, t471);
            let r472 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r472);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f822b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f822f); }
            c.w64(6, c.r64(1));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x70008f823a, 0x70008f8235);
        }
        262 => {
            c.w64(11, c.r64(0));
            let Some(t473) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f823d); };
            c.w64(15, t473);
            let Some(t474) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8241); };
            c.w64(8, t474);
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8245); };
            c.w64(9, t475);
            blk = 80; continue;
        }
        263 => {
            let Some(t476) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f824e); };
            c.w32(2, t476);
            let r477 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r477);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t478) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008f8259); };
            c.w64(2, t478);
            blk = 265; continue;
        }
        265 => {
            let Some(t479) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8260); };
            c.w64(6, t479);
            let Some(t480) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f8264); };
            c.w32(7, t480);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t481) = c.ld8(c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f826a); };
            c.w32(12, t481);
            let Some(t482) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8270); };
            c.w32(3, t482);
            let r483 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r483);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t484) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8279); };
            c.w32(12, t484);
            let r485 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r485);
            let r486 = c.sub(c.r64(7), 0x1u64, 64);
            c.w64(7, r486);
            let Some(t487) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8285); };
            c.w32(0, t487);
            let r488 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r488);
            let r489 = c.r64(0) & c.r64(7);
            c.w64(0, r489);
            c.logic(r489, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008f8290); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 266; continue;
        }
        266 => {
            c.w64(1, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(11, c.r64(1).wrapping_mul(4));
            let Some(t490) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f82a4); };
            c.w32(3, t490);
            let Some(t491) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f82a9); };
            c.cmp(t491, c.r32(3), 32);
            if c.cc_e() { blk = 324; continue; }
            blk = 267; continue;
        }
        267 => {
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            let r492 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r492);
            c.w64(11, c.r64(15).wrapping_add(c.r64(0)));
            c.w64(0, c.r64(7));
            let r493 = c.r64(0) & c.r64(11);
            c.w64(0, r493);
            c.logic(r493, 64);
            blk = 266; continue;
        }
        269 => {
            let Some(t494) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f82cb); };
            c.w32(0, t494);
            let Some(t495) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f82ce); };
            c.w32(3, t495);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r496 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r496);
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_b() { blk = 271; continue; }
            blk = 270; continue;
        }
        270 => {
            return RegfileExit::fault(0x70008f82e2);
        }
        271 => {
            let Some(t497) = c.ld8(c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f82e3); };
            c.w32(0, t497);
            let Some(t498) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f82e9); };
            c.w64(1, t498);
            let Some(t499) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f82ed); };
            c.w32(11, t499);
            let r500 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r500);
            let Some(t501) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f82f5); };
            c.w64(2, t501);
            let Some(t502) = c.ld32(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f82f9); };
            c.w32(0, t502);
            let r503 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r503);
            c.logic(c.r8(3) & 0x2u64, 8);
            if c.cc_e() { blk = 323; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t504) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8309); };
            c.w32(3, t504);
            let Some(t505) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f830d); };
            c.cmp(t505, c.r32(3), 32);
            if c.cc_ne() { blk = 297; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t506) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f831a); };
            c.w32(2, t506);
            let r507 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r507);
            let Some(t508) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8321); };
            c.w32(11, t508);
            let r509 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r509);
            let Some(t510) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f8328); };
            c.logic(t510 & 0x2u64, 8);
            if c.cc_ne() { blk = 295; continue; }
            blk = 274; continue;
        }
        274 => {
            let Some(t511) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f8333); };
            c.cmp(t511, c.r32(11), 32);
            if c.cc_e() { blk = 276; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t512) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f833c); };
            c.w64(11, t512);
            blk = 323; continue;
        }
        276 => {
            let Some(t513) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f8348); };
            c.w64(11, t513);
            let Some(t514) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f834f); };
            c.cmp(t514, 0x20u64, 8);
            if c.cc_e() { blk = 293; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t515) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f8359); };
            c.w32(11, t515);
            let Some(t516) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f835d); };
            c.cmp(t516, c.r32(11), 32);
            if c.cc_e() { blk = 279; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t517) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f8366); };
            c.w64(3, t517);
            let Some(t518) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f836a); };
            c.w64(0, t518);
            blk = 78; continue;
        }
        279 => {
            let Some(t519) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f8373); };
            c.w64(3, t519);
            let Some(t520) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8377); };
            c.w32(11, t520);
            let r521 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r521);
            let Some(t522) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f837e); };
            c.w32(0, t522);
            let Some(t523) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f8382); };
            c.cmp(t523, c.r32(0), 32);
            if c.cc_ne() { blk = 281; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t524) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f838b); };
            c.w32(11, t524);
            let r525 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r525);
            let Some(t526) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8392); };
            c.w32(0, t526);
            let Some(t527) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f8396); };
            c.cmp(t527, c.r32(0), 32);
            if c.cc_e() { blk = 280; continue; }
            blk = 281; continue;
        }
        281 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 292; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t528) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f83a9); };
            c.w32(0, t528);
            let r529 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r529);
            let Some(t530) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f83b0); };
            c.w32(1, t530);
            c.w32(6, c.r64(1).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 287; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w32(11, c.r16(1));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 286; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t531) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f83cc); };
            c.cmp(t531, c.r32(0), 32);
            if c.cc_ne() { blk = 288; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f83d5); };
            c.w64(3, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f83d9); };
            c.w64(0, t533);
            blk = 78; continue;
        }
        286 => {
            let Some(t534) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f83e2); };
            c.w32(0, t534);
            let r535 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r535);
            c.w64(3, c.r64(0));
            blk = 288; continue;
        }
        287 => {
            c.w64(3, c.r64(11));
            blk = 288; continue;
        }
        288 => {
            let Some(t536) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f83f0); };
            c.w32(11, t536);
            let r537 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r537);
            let Some(t538) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f83f7); };
            c.w32(11, t538);
            let r539 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r539);
            let Some(t540) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f83fe); };
            c.w32(0, t540);
            let r541 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r541);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 291; continue; }
            blk = 289; continue;
        }
        289 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t542) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f8411); };
            c.w32(6, t542);
            let r543 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r543);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f8418); }
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f841c); };
            if !c.push64(t544) { return RegfileExit::fault(0x70008f841c); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008f8426, 0x70008f8421);
        }
        290 => {
            c.w64(11, c.r64(0));
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8429); };
            c.w64(8, t545);
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f842d); };
            c.w64(9, t546);
            blk = 323; continue;
        }
        291 => {
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f8436); };
            c.w64(3, t547);
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f843a); };
            c.w64(0, t548);
            blk = 78; continue;
        }
        292 => {
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f8443); };
            c.w64(3, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f8447); };
            c.w64(0, t550);
            blk = 78; continue;
        }
        293 => {
            c.w64(11, c.r64(9));
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f8457); }
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f845b); };
            if !c.push64(t551) { return RegfileExit::fault(0x70008f845b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f845e); }
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f845f); };
            if !c.push64(t552) { return RegfileExit::fault(0x70008f845f); }
            let Some(t553) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f8462); };
            c.w64(3, t553);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x70008f8476, 0x70008f8471);
        }
        294 => {
            c.w64(11, c.r64(0));
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8479); };
            c.w64(8, t554);
            let Some(t555) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f847d); };
            c.w64(9, t555);
            blk = 323; continue;
        }
        295 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f848d); }
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f8491); };
            if !c.push64(t556) { return RegfileExit::fault(0x70008f8491); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x70007f2274, 0x70008f84a1, 0x70008f849c);
        }
        296 => {
            c.w64(11, c.r64(0));
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f84a4); };
            c.w64(8, t557);
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f84a8); };
            c.w64(9, t558);
            blk = 323; continue;
        }
        297 => {
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f84b1); };
            c.w64(0, t559);
            let Some(t560) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f84b5); };
            c.w32(3, t560);
            let r561 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r561);
            let Some(t562) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f84bb); };
            c.w32(2, t562);
            c.cmp(c.r16(2), 0x834u64, 16);
            if c.cc_e() { blk = 320; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w32(1, c.r64(2).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 308; continue; }
            blk = 299; continue;
        }
        299 => {
            c.cmp(c.r16(2), 0x411u64, 16);
            if c.cc_e() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f84e0); };
            c.w64(3, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f84e4); };
            c.w64(0, t564);
            blk = 78; continue;
        }
        301 => {
            let Some(t565) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f84ed); };
            c.w32(11, t565);
            let Some(t566) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f84f1); };
            c.cmp(t566, c.r32(11), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f84fa); };
            c.w64(3, t567);
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f84fe); };
            c.w64(0, t568);
            blk = 78; continue;
        }
        303 => {
            let Some(t569) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f8507); };
            c.w32(11, t569);
            let r570 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r570);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 305; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f8514); };
            c.w64(3, t571);
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f8518); };
            c.w64(0, t572);
            blk = 78; continue;
        }
        305 => {
            let Some(t573) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8521); };
            c.w32(0, t573);
            let r574 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r574);
            let Some(t575) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8528); };
            c.cmp(t575, 0x40u64, 16);
            if c.cc_b() { blk = 307; continue; }
            blk = 306; continue;
        }
        306 => {
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f852f); };
            c.w64(3, t576);
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f8533); };
            c.w64(0, t577);
            blk = 78; continue;
        }
        307 => {
            let Some(t578) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f853c); };
            c.w32(11, t578);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 323; continue;
        }
        308 => {
            let Some(t579) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f854c); };
            c.w32(11, t579);
            let Some(t580) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f8550); };
            c.cmp(t580, c.r32(11), 32);
            if c.cc_e() { blk = 310; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f8559); };
            c.w64(3, t581);
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f855d); };
            c.w64(0, t582);
            blk = 78; continue;
        }
        310 => {
            let Some(t583) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f8566); };
            c.w32(11, t583);
            c.w64(3, c.r64(11));
            let r584 = c.r32(3) & 0xc0u64;
            c.w32(3, r584);
            c.logic(r584, 32);
            c.cmp(c.r32(3), 0xc0u64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 311; continue;
        }
        311 => {
            let Some(t585) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f857c); };
            c.w32(3, t585);
            let r586 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r586);
            let Some(t587) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f8582); };
            c.w32(3, t587);
            let r588 = c.r32(3) & 0x1fu64;
            c.w32(3, r588);
            c.logic(r588, 32);
            let r589 = c.sub(c.r32(3), 0x10u64, 32);
            c.w32(3, r589);
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_be() { blk = 313; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f8590); };
            c.w64(3, t590);
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f8594); };
            c.w64(0, t591);
            blk = 78; continue;
        }
        313 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 319; continue; }
            blk = 314; continue;
        }
        314 => {
            let Some(t592) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f85a3); };
            c.w32(0, t592);
            let r593 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r593);
            let Some(t594) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f85a9); };
            c.cmp(t594, c.r32(0), 32);
            if c.cc_e() { blk = 318; continue; }
            blk = 315; continue;
        }
        315 => {
            let Some(t595) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f85b2); };
            c.w32(11, t595);
            let Some(t596) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f85b6); };
            c.cmp(t596, c.r32(11), 32);
            if c.cc_ne() { blk = 317; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t597) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f85bf); };
            c.w32(0, t597);
            let r598 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r598);
            blk = 317; continue;
        }
        317 => {
            c.w64(11, c.r64(0));
            blk = 323; continue;
        }
        318 => {
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f85ca); };
            c.w64(3, t599);
            let Some(t600) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f85ce); };
            c.w64(0, t600);
            blk = 78; continue;
        }
        319 => {
            let Some(t601) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f85d7); };
            c.w64(3, t601);
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f85db); };
            c.w64(0, t602);
            blk = 78; continue;
        }
        320 => {
            let Some(t603) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f85e4); };
            c.w32(11, t603);
            let Some(t604) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f85e8); };
            c.cmp(t604, c.r32(11), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f85f1); };
            c.w64(3, t605);
            let Some(t606) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f85f5); };
            c.w64(0, t606);
            blk = 78; continue;
        }
        322 => {
            let Some(t607) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f85fe); };
            c.w32(0, t607);
            let r608 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r608);
            c.w64(11, c.r64(0));
            blk = 323; continue;
        }
        323 => {
            let Some(t609) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8607); };
            c.w64(15, t609);
            blk = 80; continue;
        }
        324 => {
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f8610); };
            c.w64(3, t610);
            let Some(t611) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f8614); };
            c.w64(0, t611);
            blk = 78; continue;
        }
        325 => {
            return RegfileExit::fault(0x70008f861d);
        }
        326 => {
            let Some(t612) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f861e); };
            c.w64(0, t612);
            let Some(t613) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f8625); };
            c.w64(3, t613);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f862c); }
            let Some(t614) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f8630); };
            c.w64(1, t614);
            let Some(t615) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f8637); };
            c.cmp(t615, c.r64(2), 64);
            if c.cc_a() { blk = 329; continue; }
            blk = 327; continue;
        }
        327 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f8646, 0x70008f8641);
        }
        328 => {
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f8646); };
            c.w64(11, t616);
            let Some(t617) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f864a); };
            c.w64(15, t617);
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f864e); };
            c.w64(8, t618);
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8652); };
            c.w64(9, t619);
            blk = 330; continue;
        }
        329 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008f8658); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 330; continue;
        }
        330 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            let Some(t620) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f8668); };
            c.w64(11, t620);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f866f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f8673); }
            blk = 58; continue;
        }
        331 => {
            let Some(t621) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f867d); };
            c.w64(0, t621);
            let Some(t622) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f8684); };
            c.w64(3, t622);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f868b); }
            let Some(t623) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f868f); };
            c.w64(1, t623);
            let Some(t624) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f8696); };
            c.cmp(t624, c.r64(2), 64);
            if c.cc_a() { blk = 334; continue; }
            blk = 332; continue;
        }
        332 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f86a5, 0x70008f86a0);
        }
        333 => {
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f86a5); };
            c.w64(11, t625);
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f86a9); };
            c.w64(15, t626);
            let Some(t627) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f86ad); };
            c.w64(8, t627);
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f86b1); };
            c.w64(9, t628);
            blk = 335; continue;
        }
        334 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008f86b7); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 335; continue;
        }
        335 => {
            c.w32(10, c.r32(11));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t629) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f86ca); };
            c.w64(11, t629);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f86d1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f86d5); }
            blk = 61; continue;
        }
        336 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f86e4); }
            return c.call(0x70007febb4, 0x70008f86ed, 0x70008f86e8);
        }
        337 => {
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f86ed); };
            c.w64(11, t630);
            let Some(t631) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f86f1); };
            c.w64(15, t631);
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f86f5); };
            c.w64(8, t632);
            let Some(t633) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f86f9); };
            c.w64(9, t633);
            blk = 65; continue;
        }
        338 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008f870c, 0x70008f8707);
        }
        339 => {
            c.w64(11, c.r64(0));
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f870f); };
            c.w64(15, t634);
            let Some(t635) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8713); };
            c.w64(8, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8717); };
            c.w64(9, t636);
            blk = 70; continue;
        }
        340 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008f8725); }
            return c.call(0x70007febf4, 0x70008f872f, 0x70008f872a);
        }
        341 => {
            c.w64(11, c.r64(0));
            let Some(t637) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8732); };
            c.w64(15, t637);
            let Some(t638) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8736); };
            c.w64(8, t638);
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f873a); };
            c.w64(9, t639);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008f873e); }
            blk = 76; continue;
        }
        342 => {
            c.w64(11, c.r64(9));
            let r640 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r640);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f874e); }
            let Some(t641) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008f8752); };
            c.w64(8, t641);
            let Some(t642) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8759); };
            c.w32(8, t642);
            let r643 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r643);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f8760); }
            let Some(t644) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x70008f8762); };
            c.w64(3, t644);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008f8775, 0x70008f8770);
        }
        343 => {
            return RegfileExit::fault(0x70008f8775);
        }
        344 => {
            blk = 78; continue;
        }
        345 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t645) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8782); };
            c.w64(2, t645);
            let Some(t646) = c.ld8(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f8786); };
            c.w32(1, t646);
            let Some(t647) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f878c); };
            c.w32(2, t647);
            let r648 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r648);
            let Some(t649) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8792); };
            c.w64(6, t649);
            let Some(t650) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8796); };
            c.w32(1, t650);
            let r651 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r651);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f879d); }
            c.w32(0, 0xau64);
            c.w64(2, c.r64(3));
            return c.call(0x7000804ff4, 0x70008f87ae, 0x70008f87a9);
        }
        346 => {
            c.w64(11, c.r64(0));
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f87b1); };
            c.w64(15, t652);
            let Some(t653) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f87b5); };
            c.w64(8, t653);
            let Some(t654) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f87b9); };
            c.w64(9, t654);
            blk = 80; continue;
        }
        347 => {
            return RegfileExit::jump(0x70008f87c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f87d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f87d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f87d8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f87de); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f87e2); };
            c.w64(11, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f87e6); };
            c.w64(3, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f87ec); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f87f3); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f87f9); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f87ff); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8807); };
            c.w64(3, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f880b); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8812); };
            c.w32(6, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f8818); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f881e); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8825); };
            c.w32(0, t19);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f8829); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f882d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f8831); }
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f8835); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f883e); };
            c.w64(11, t21);
            blk = 2; continue;
        }
        2 => {
            let Some(t22) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8845); };
            c.w64(0, t22);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f8853); }
            c.w64(2, c.r64(3));
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(3, c.r64(11));
            return c.call(0x7000834434, 0x70008f886b, 0x70008f8866);
        }
        3 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f886b); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f886f); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8873); };
            c.w64(11, t25);
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t26) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f887b); };
            c.w32(11, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8880); };
            c.w64(15, t27);
            let Some(t28) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8884); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8888); };
            c.w64(5, t29);
            c.w64(12, c.r64(8));
            let r30 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r30);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f8898);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f88b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f88b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f88b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f88be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f88c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f88c6); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f88cc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f88d3); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f88d7); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f88de); };
            c.w32(3, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f88e4); };
            c.w32(2, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f88e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f88ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008f88f0); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f88f4); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f88fd); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8904); };
            c.w64(2, t13);
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008f8912); }
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(11));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(8));
            return c.call(0x7000836434, 0x70008f892a, 0x70008f8925);
        }
        3 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f892a); };
            c.w64(7, t14);
            let Some(t15) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f892e); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8932); };
            c.w64(11, t16);
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f893a); };
            c.w32(11, t17);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8950); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8954); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8958); };
            c.w64(5, t20);
            c.w64(12, c.r64(8));
            let r21 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r21);
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f8965); }
            let r22 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r22);
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f8971); };
            c.w32(7, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8976); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f897a); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f897e); };
            c.w64(5, t26);
            c.w64(12, c.r64(8));
            let r27 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        6 => {
            return RegfileExit::jump(0x70008f898c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f8994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8997); };
            c.w64(3, t0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f899b); };
            c.w32(2, (t1 as u8 as i8 as i64 as u64));
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f89a1); };
            c.w32(6, t2);
            c.cmp(c.r8(6), 0x0u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f89ad); };
            c.w32(7, t3);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f89bc); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let r6 = c.add(c.r32(6), 0xffffffffffffffffu64, 32);
            c.w32(6, r6);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008f89c9);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f89ca); };
            c.w32(6, t7);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008f89d9);
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f89da); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_g() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008f89e9); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f89ef); };
            c.w32(6, t12);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(7, 0xffffffffffffffffu64);
            let r13 = c.sub(c.r64(7), c.r64(2), 64);
            c.w64(7, r13);
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_ae() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8a07); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8a0e); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            blk = 12; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f8a16); };
            c.w32(3, t18);
            let r19 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r19);
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8a1c); };
            c.w32(6, t20);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r21 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r21);
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_ae() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8a2e); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8a35); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            blk = 12; continue;
        }
        12 => {
            let r26 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r26);
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f8a3f); };
            c.w32(2, t27);
            let Some(t28) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8a44); };
            c.w64(1, t28);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            return RegfileExit::fault(0x70008f8a4d);
        }
        14 => {
            return RegfileExit::fault(0x70008f8a4e);
        }
        15 => {
            return RegfileExit::jump(0x70008f8a50);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f8a54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f8a54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f8a58); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8a5e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8a62); };
            c.w64(7, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f8a66); };
            c.w32(8, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f8a6c); };
            c.w32(11, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008f8a72); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f8a76); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f8a7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f8a7e); }
            c.cmp(c.r8(11), 0x0u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8a88); };
            c.w32(3, t5);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f8a97); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let r8 = c.add(c.r32(11), 0xffffffffffffffffu64, 32);
            c.w32(11, r8);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008f8aa5);
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8aa6); };
            c.w32(11, t9);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008f8ab6);
        }
        7 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f8aba); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8ac6); };
            c.w64(7, t12);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f8ace); }
            c.w32(8, 0x6au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f8ad8); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x70008f8ada); };
            c.w64(3, t13);
            c.w64(6, c.r64(7));
            c.w32(0, 0x1u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008f8af1, 0x70008f8aec);
        }
        9 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8af1); };
            c.w64(0, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8af5); };
            c.w64(15, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8af9); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f8afd); };
            c.w64(2, t17);
            blk = 13; continue;
        }
        10 => {
            let r18 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r18);
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f8b07); };
            c.w32(7, t19);
            let r20 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r20);
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8b0d); };
            c.w32(11, t21);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8b1c); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008f8b24); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r24 = c.r64(8) & c.r64(7);
            c.w64(8, r24);
            c.logic(r24, 64);
            let Some(t25) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008f8b35); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t26) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8b3c); };
            c.w64(2, t26);
            c.w64(7, c.r64(9));
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f8b47); };
            c.w32(7, t27);
            let Some(t28) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8b4c); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8b50); };
            c.w64(5, t29);
            c.w64(12, c.r64(2));
            let r30 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r30);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            return RegfileExit::fault(0x70008f8b5d);
        }
        15 => {
            c.w32(0, c.r32(0));
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            c.w64(8, 0xfffffffffffc0000u64);
            let r32 = c.r64(8) & c.r64(0);
            c.w64(8, r32);
            c.logic(r32, 64);
            let Some(t33) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008f8b6d); };
            c.logic(t33 & 0x2u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(7).wrapping_add(0x3));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f8b78); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008f8b82, 0x70008f8b7d);
        }
        17 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008f8b82); }
            blk = 13; continue;
        }
        18 => {
            return RegfileExit::jump(0x70008f8b88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f8b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f8b94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f8b98); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8b9e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8ba2); };
            c.w64(11, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f8ba6); };
            c.w64(3, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f8bac); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f8bb3); };
            c.w32(2, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8bb9); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8bc1); };
            c.w64(3, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8bc5); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8bcc); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8bd2); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8bd9); };
            c.w32(2, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f8bdd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f8be1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f8be5); }
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f8be9); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f8bf2); };
            c.w64(11, t17);
            blk = 2; continue;
        }
        2 => {
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8bf9); };
            c.w64(2, t18);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008f8c07); }
            c.w64(7, c.r64(6));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x700083d494, 0x70008f8c1c, 0x70008f8c17);
        }
        3 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8c1c); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8c20); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8c24); };
            c.w64(11, t21);
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t22) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f8c2c); };
            c.w32(11, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8c31); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8c35); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8c39); };
            c.w64(5, t25);
            c.w64(12, c.r64(8));
            let r26 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r26);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f8c48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f8c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f8c54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f8c58); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8c5e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8c62); };
            c.w64(11, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f8c66); };
            c.w64(3, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f8c6c); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f8c73); };
            c.w32(2, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8c79); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8c81); };
            c.w64(3, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8c85); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8c8c); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f8c92); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8c99); };
            c.w32(2, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f8c9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f8ca1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f8ca5); }
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f8ca9); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f8cb2); };
            c.w64(11, t17);
            blk = 2; continue;
        }
        2 => {
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8cb9); };
            c.w64(2, t18);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008f8cc7); }
            c.w64(7, c.r64(6));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x700083d494, 0x70008f8cdc, 0x70008f8cd7);
        }
        3 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8cdc); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8ce0); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8ce4); };
            c.w64(11, t21);
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t22) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f8cec); };
            c.w32(11, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8cf1); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8cf5); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8cf9); };
            c.w64(5, t25);
            c.w64(12, c.r64(8));
            let r26 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r26);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f8d08);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f8d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f8d14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f8d18); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8d1e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8d22); };
            c.w64(11, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f8d26); };
            c.w64(3, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f8d2c); };
            c.w64(2, (t4 as u8 as i8 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f8d32); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8d39); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8d3d); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8d41); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8d48); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8d4e); };
            c.w32(6, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f8d52); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f8d56); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f8d5a); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f8d5e); };
            c.cmp(t13, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f8d67); };
            c.w64(11, t14);
            blk = 2; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8d6e); };
            c.w64(6, t15);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008f8d7c); }
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x700083eb74, 0x70008f8d91, 0x70008f8d8c);
        }
        3 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8d91); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8d95); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8d99); };
            c.w64(11, t18);
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f8da1); };
            c.w32(11, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8da6); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8daa); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8dae); };
            c.w64(5, t22);
            c.w64(12, c.r64(8));
            let r23 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r23);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f8dbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f8dd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f8dd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f8dd8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8dde); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8de2); };
            c.w64(11, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f8de6); };
            c.w64(3, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f8dec); };
            c.w64(2, (t4 as u8 as i8 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f8df2); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8df9); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8dfd); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8e01); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8e08); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8e0e); };
            c.w32(6, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f8e12); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f8e16); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f8e1a); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f8e1e); };
            c.cmp(t13, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f8e27); };
            c.w64(11, t14);
            blk = 2; continue;
        }
        2 => {
            let Some(t15) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8e2e); };
            c.w64(6, t15);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008f8e3c); }
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x7000840434, 0x70008f8e51, 0x70008f8e4c);
        }
        3 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8e51); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8e55); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8e59); };
            c.w64(11, t18);
            c.w64(9, c.r64(11).wrapping_add(0x4));
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f8e61); };
            c.w32(11, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8e66); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8e6a); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8e6e); };
            c.w64(5, t22);
            c.w64(12, c.r64(8));
            let r23 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r23);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f8e7c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f8e94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f8e94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f8e98); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8e9e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8ea2); };
            c.w64(7, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f8ea6); };
            c.w64(8, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f8eac); };
            c.w64(11, (t4 as u8 as i8 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f8eb2); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8eb8); };
            c.w64(8, t7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8ebc); };
            c.w64(11, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8ec0); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8ec6); };
            c.w32(3, t11);
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008f8ecc); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8ed2); };
            c.w32(6, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008f8ed5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f8ed9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f8edd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f8ee1); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f8ee5); };
            c.cmp(t14, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f8eee); };
            c.w64(7, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8ef5); };
            c.w64(6, t16);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let r17 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r17);
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f8f05); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f8f09); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f8f0b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f8f0d); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f8f0e); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008f8f0f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f8f10); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2360)) else { return RegfileExit::fault(0x70008f8f11); };
            c.w64(3, t18);
            c.w32(0, 0x6u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008f8f25, 0x70008f8f20);
        }
        3 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f8f25); };
            c.w64(2, t19);
            let Some(t20) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8f29); };
            c.w64(2, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8f2d); };
            c.w64(7, t21);
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t22) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f8f35); };
            c.w32(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8f3a); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8f3e); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8f42); };
            c.w64(5, t25);
            c.w64(12, c.r64(2));
            let Some(t26) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008f8f49); };
            c.w64(0, t26);
            let r27 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f8f58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f8f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f8f74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f8f78); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f8f7e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f8f82); };
            c.w64(7, t2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f8f86); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f8f8c); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f8f92); };
            c.w32(8, (t7 as u8 as i8 as i64 as u64));
            let Some(t8) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f8f98); };
            c.w32(11, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008f8f9c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f8fa0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f8fa4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f8fa8); }
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f8fac); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f8fb5); };
            c.w64(7, t10);
            blk = 2; continue;
        }
        2 => {
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f8fbc); };
            c.w64(11, t11);
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let r12 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r12);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f8fc7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f8fcb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f8fcd); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008f8fce); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2320)) else { return RegfileExit::fault(0x70008f8fcf); };
            c.w64(3, t13);
            c.w64(6, c.r64(11));
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008f8fe6, 0x70008f8fe1);
        }
        3 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f8fe6); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8fea); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f8fee); };
            c.w64(7, t16);
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f8ff6); };
            c.w32(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f8ffb); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f8fff); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f9003); };
            c.w64(5, t20);
            c.w64(12, c.r64(2));
            let Some(t21) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008f900a); };
            c.w64(0, t21);
            let r22 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f9018);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f9034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f9034); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f9038); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f903e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9042); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f9046); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f904c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f9053); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f9057); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f905b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f9062); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9068); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f906c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f9070); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008f9074); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f9078); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f907c); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f9085); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(0));
            c.w32(6, c.r32(11));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f909b); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f90b8); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f90bb); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f90cb); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 17; continue;
        }
        6 => {
            let r18 = c.add(c.r32(1), c.r32(6), 32);
            c.w32(1, r18);
            if c.cc_o() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f90e7); };
            c.w32(11, t19);
            c.w32(0, c.r32(11));
            let r20 = c.r32(0) | 0x2u64;
            c.w32(0, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f90fa); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f90ff); }
            blk = 10; continue;
        }
        10 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        11 => {
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            let r22 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 17; continue;
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9127); };
            c.w32(1, t23);
            let r24 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f912e); };
            c.cmp(t25, c.r32(1), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9141); };
            c.w32(1, t26);
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9144); };
            c.cmp(t27, c.r32(1), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f9151); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f9157); }
            blk = 17; continue;
        }
        16 => {
            let r28 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r28);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f9160); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9173); };
            c.w32(11, t29);
            c.w32(0, c.r32(11));
            let r30 = c.r32(0) | 0xeu64;
            c.w32(0, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f9186); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f918b); }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f9193); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f919a); };
            c.w64(11, t32);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f91a1); };
            c.w64(3, t33);
            let Some(t34) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f91a8); };
            c.cmp(t34, c.r64(0), 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008f91b1); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 22; continue;
        }
        22 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f91bc); };
            c.w64(11, t35);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f91c3); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f91c7); }
            blk = 45; continue;
        }
        23 => {
            c.w32(6, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t36) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f91d6); };
            c.w32(1, t36);
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 38; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f91fe); };
            c.w32(1, t37);
            let r38 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r38);
            let Some(t39) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9204); };
            c.cmp(t39, 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 38; continue;
        }
        29 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9214); };
            c.w32(1, t40);
            let r41 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r41);
            let Some(t42) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f921a); };
            c.cmp(t42, 0x40u64, 16);
            if c.cc_ae() { blk = 38; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t43) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9226); };
            c.w32(3, t43);
            c.w32(1, c.r32(3));
            let r44 = c.r32(1) | 0x20u64;
            c.w32(1, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x70008f9237); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f923c); }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f9244); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f924e); };
            c.w64(6, t45);
            return c.call(0x700086f9d4, 0x70008f9257, 0x70008f9252);
        }
        34 => {
            c.w64(8, c.r64(0));
            blk = 45; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9265); };
            c.w32(1, t46);
            let Some(t47) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9268); };
            c.cmp(t47, c.r32(1), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9271); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9277); };
            c.cmp(t50, 0x43u64, 16);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(1, 0xfeu64);
            blk = 40; continue;
        }
        39 => {
            c.w32(1, 0x1eu64);
            blk = 40; continue;
        }
        40 => {
            c.w64(6, c.r64(9));
            c.w32(9, c.r64(6).wrapping_add(c.r64(6)));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9296); };
            c.w32(3, t51);
            c.w32(1, c.r32(1));
            c.w32(12, c.r32(3));
            let r52 = c.r32(1) | c.r32(12);
            c.w32(1, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x70008f92aa); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f92af); }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008f92b7); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f92c1); };
            c.w64(6, t53);
            return c.call(0x70008ad794, 0x70008f92ca, 0x70008f92c5);
        }
        44 => {
            c.w64(8, c.r64(0));
            blk = 45; continue;
        }
        45 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f92cd); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f92d1); };
            c.w64(11, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f92d5); };
            c.w64(0, t56);
            c.w64(9, c.r64(0).wrapping_add(0x3));
            let Some(t57) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f92dd); };
            c.w32(0, t57);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f92f1); };
            c.w64(15, t58);
            let Some(t59) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f92f5); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f92f9); };
            c.w64(5, t60);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            let r61 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r61);
            return RegfileExit::jump(c.r64(1));
        }
        47 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(8)) { return RegfileExit::fault(0x70008f9309); }
            let r62 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r62);
            let Some(t63) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f9315); };
            c.w32(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f931a); };
            c.w64(15, t64);
            let Some(t65) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f931e); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f9322); };
            c.w64(5, t66);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            let r67 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r67);
            return RegfileExit::jump(c.r64(1));
        }
        48 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008f9337); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x70008f933c); }
            return c.call(0x70007febf4, 0x70008f9346, 0x70008f9341);
        }
        49 => {
            c.w64(8, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9349); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f934d); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9351); };
            c.w64(9, t70);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008f9355); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008f935a); }
            blk = 22; continue;
        }
        50 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f9364); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008f9368); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008f936c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008f9370); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008f9374); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f937e); };
            c.w64(6, t71);
            return c.call(0x700089b334, 0x70008f9387, 0x70008f9382);
        }
        51 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t72) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f938b); };
            c.cmp(t72, 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f9391); };
            c.w64(8, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f9395); };
            c.w64(2, t74);
            let Some(t75) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9399); };
            c.w32(7, t75);
            c.w32(11, c.r32(7));
            let r76 = c.r32(11) | 0x40u64;
            c.w32(11, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008f93ad); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f93b2); }
            blk = 55; continue;
        }
        55 => {
            c.w64(8, c.r64(0));
            blk = 45; continue;
        }
        56 => {
            let Some(t77) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f93c2); };
            c.cmp(t77, 0x0u64, 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f93c8); };
            c.w64(8, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f93cc); };
            c.w64(7, t79);
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f93d0); };
            c.w32(11, t80);
            c.w32(0, c.r32(11));
            let r81 = c.r32(0) | 0xfeu64;
            c.w32(0, r81);
            c.logic(r81, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f93e5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f93ea); }
            blk = 59; continue;
        }
        59 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f93f2); };
            c.w64(3, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f93f6); };
            c.w64(6, t83);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f93fa); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008f9403); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008f9404); };
            c.w64(3, t84);
            c.w32(0, 0x1u64);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f9410); };
            c.w64(6, t85);
            return c.call(0x700086ed74, 0x70008f9419, 0x70008f9414);
        }
        60 => {
            return RegfileExit::fault(0x70008f9419);
        }
        61 => {
            return RegfileExit::jump(0x70008f941c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f9434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f9434); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f9438); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f943e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9442); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f9446); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f944c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f9453); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f9457); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f945b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f9462); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9468); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f946c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f9470); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008f9474); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f9478); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f947c); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f9485); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(11));
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f948f); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(12, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f94af); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f94b2); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f94c2); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 19; continue;
        }
        6 => {
            let r18 = c.sub(c.r32(1), c.r32(12), 32);
            c.w32(1, r18);
            if c.cc_o() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(0, 0x2u64);
            blk = 11; continue;
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f94e4); };
            c.w64(3, t19);
            let Some(t20) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f94eb); };
            c.w64(1, t20);
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            let r22 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            c.w64(12, c.r64(1).wrapping_add(0xc));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f94f7); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008f94fe); };
            c.cmp(t24, c.r64(12), 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x70008f9507); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9523); };
            c.w64(11, t25);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f952a); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008f952e); }
            c.w32(0, 0xeu64);
            c.w64(11, c.r64(3));
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9544); };
            c.w32(3, t26);
            c.w32(0, c.r32(0));
            c.w32(1, c.r32(3));
            let r27 = c.r32(0) | c.r32(1);
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008f955a); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f955f); }
            blk = 41; continue;
        }
        14 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f956c); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9573); };
            c.cmp(t30, c.r32(1), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9584); };
            c.w32(1, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9587); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f9594); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f959a); }
            blk = 19; continue;
        }
        18 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f95a3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f95b6); };
            c.w32(11, t34);
            c.w32(0, c.r32(11));
            let r35 = c.r32(0) | 0xeu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f95c9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f95ce); }
            blk = 22; continue;
        }
        22 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f95d6); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f95dd); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f95e4); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f95eb); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008f95f4); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 24; continue;
        }
        24 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f95ff); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f9606); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008f960a); }
            c.w64(11, c.r64(8));
            blk = 41; continue;
        }
        25 => {
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t41) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f961c); };
            c.w32(1, t41);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9630); };
            c.w32(1, t42);
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9633); };
            c.cmp(t43, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(1, 0x1eu64);
            blk = 36; continue;
        }
        30 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9643); };
            c.w32(1, t44);
            let r45 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r45);
            let Some(t46) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9649); };
            c.cmp(t46, 0x43u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(1, 0x1eu64);
            blk = 36; continue;
        }
        32 => {
            c.w32(1, 0x1eu64);
            blk = 36; continue;
        }
        33 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9662); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9668); };
            c.cmp(t49, 0x41u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(1, 0xfeu64);
            blk = 36; continue;
        }
        36 => {
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9681); };
            c.w32(6, t50);
            c.w32(1, c.r32(1));
            c.w32(11, c.r32(6));
            let r51 = c.r32(1) | c.r32(11);
            c.w32(1, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f9695); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f969a); }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f96a2); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f96ac); };
            c.w64(0, t52);
            return c.call(0x70008adb14, 0x70008f96b5, 0x70008f96b0);
        }
        40 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f96b8); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f96bc); };
            c.w64(9, t54);
            blk = 41; continue;
        }
        41 => {
            let Some(t55) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f96c0); };
            c.w64(8, t55);
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(0).wrapping_add(0x3));
            let Some(t56) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f96cb); };
            c.w32(0, t56);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f96df); };
            c.w64(15, t57);
            let Some(t58) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f96e3); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f96e7); };
            c.w64(5, t59);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r60 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r60);
            return RegfileExit::jump(c.r64(1));
        }
        43 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f96f7); }
            let r61 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r61);
            let Some(t62) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f9703); };
            c.w32(7, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9708); };
            c.w64(15, t63);
            let Some(t64) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f970c); };
            c.w64(1, t64);
            let Some(t65) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f9710); };
            c.w64(5, t65);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r66 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r66);
            return RegfileExit::jump(c.r64(1));
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008f9720); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008f9729); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008f972d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f9731); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f9735); }
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70008f9741, 0x70008f973c);
        }
        45 => {
            c.w64(3, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f9744); };
            c.w64(7, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9748); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f974c); };
            c.w64(8, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f9750); };
            c.w64(2, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f9754); };
            c.w64(6, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f9758); };
            c.w64(11, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f975c); };
            c.w64(0, t73);
            blk = 10; continue;
        }
        46 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x70008f976a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70008f976f); }
            return c.call(0x70007febf4, 0x70008f9779, 0x70008f9774);
        }
        47 => {
            c.w64(8, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f977c); };
            c.w64(15, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f9780); };
            c.w64(7, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9784); };
            c.w64(9, t76);
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008f9788); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008f978d); }
            blk = 24; continue;
        }
        48 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008f9797); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008f979b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008f979f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008f97a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008f97a7); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t77) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f97b0); };
            c.w32(1, t77);
            c.w32(11, c.r32(1));
            let r78 = c.r32(11) | 0x40u64;
            c.w32(11, r78);
            c.logic(r78, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f97c4); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f97c9); }
            blk = 51; continue;
        }
        51 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f97d1); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f97db); };
            c.w64(0, t79);
            return c.call(0x700089bcd4, 0x70008f97e4, 0x70008f97df);
        }
        52 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f97eb); };
            c.w64(7, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f97ef); };
            c.w64(9, t81);
            blk = 41; continue;
        }
        54 => {
            let Some(t82) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f97f8); };
            c.cmp(t82, 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f97fe); };
            c.w64(8, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f9802); };
            c.w64(2, t84);
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9806); };
            c.w32(7, t85);
            c.w32(11, c.r32(7));
            let r86 = c.r32(11) | 0xfeu64;
            c.w32(11, r86);
            c.logic(r86, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008f981d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f9822); }
            blk = 57; continue;
        }
        57 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f982a); };
            c.w64(7, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f982e); };
            c.w64(12, t88);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f9832); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008f983b); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008f983c); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f9848); };
            c.w64(6, t90);
            return c.call(0x700086ed74, 0x70008f9851, 0x70008f984c);
        }
        58 => {
            return RegfileExit::fault(0x70008f9851);
        }
        59 => {
            return RegfileExit::jump(0x70008f9854);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f9874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f9874); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f9878); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f987e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9882); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f9886); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f988c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f9893); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f9897); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f989b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f98a2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f98a8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f98ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f98b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008f98b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f98b8); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f98bc); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f98c5); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f98cc); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f98e7); };
            c.w32(6, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f98ea); };
            c.cmp(t16, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f98fa); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 24; continue;
        }
        6 => {
            c.w64(3, c.r64(0));
            let r18 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r18);
            c.w64(6, c.r64(11));
            let r19 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r19);
            c.w64(12, c.r64(6));
            let r20 = c.imul2(c.r32(12), c.r32(3), 32);
            c.w32(12, r20);
            if c.cc_o() { blk = 54; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(11, c.r64(12));
            let r21 = c.add(c.r32(11), c.r32(12), 32);
            c.w32(11, r21);
            if c.cc_o() { blk = 49; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        10 => {
            let r22 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r22);
            let r23 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r23);
            let r24 = c.r32(0) | c.r32(11);
            c.w32(0, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        12 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008f994d); };
            c.w64(11, t25);
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(0, 0xeu64);
            blk = 16; continue;
        }
        15 => {
            c.w32(0, 0x2u64);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f996f); };
            c.w32(3, t26);
            c.w32(0, c.r32(0));
            c.w32(1, c.r32(3));
            let r27 = c.r32(0) | c.r32(1);
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008f9985); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f998a); }
            blk = 46; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9997); };
            c.w32(6, t28);
            let r29 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f999e); };
            c.cmp(t30, c.r32(6), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f99af); };
            c.w32(6, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f99b2); };
            c.cmp(t32, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f99bf); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f99c5); }
            blk = 24; continue;
        }
        23 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f99ce); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f99e1); };
            c.w32(11, t34);
            c.w32(0, c.r32(11));
            let r35 = c.r32(0) | 0xeu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f99f4); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f99f9); }
            blk = 27; continue;
        }
        27 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f9a01); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f9a08); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f9a0f); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f9a16); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008f9a1f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9a2a); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f9a31); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f9a35); }
            c.w64(11, c.r64(8));
            blk = 46; continue;
        }
        30 => {
            let Some(t41) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9a43); };
            c.w32(6, t41);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9a5b); };
            c.w32(6, t42);
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9a5e); };
            c.cmp(t43, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(6, 0x1eu64);
            blk = 41; continue;
        }
        35 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9a6e); };
            c.w32(6, t44);
            let r45 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r45);
            let Some(t46) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9a74); };
            c.cmp(t46, 0x43u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(6, 0x1eu64);
            blk = 41; continue;
        }
        37 => {
            c.w32(6, 0x1eu64);
            blk = 41; continue;
        }
        38 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9a8d); };
            c.w32(6, t47);
            let r48 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r48);
            let Some(t49) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9a93); };
            c.cmp(t49, 0x41u64, 16);
            if c.cc_e() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(6, 0xfeu64);
            blk = 41; continue;
        }
        41 => {
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9aac); };
            c.w32(1, t50);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r51 = c.r32(6) | c.r32(11);
            c.w32(6, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f9ac0); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f9ac5); }
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f9acd); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f9ad7); };
            c.w64(0, t52);
            return c.call(0x70008adcd4, 0x70008f9ae0, 0x70008f9adb);
        }
        45 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9ae3); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f9ae7); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9aeb); };
            c.w64(9, t55);
            blk = 46; continue;
        }
        46 => {
            let Some(t56) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9aef); };
            c.w64(8, t56);
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(0).wrapping_add(0x3));
            let Some(t57) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f9afa); };
            c.w32(0, t57);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t58) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f9b0e); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f9b12); };
            c.w64(5, t59);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r60 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r60);
            return RegfileExit::jump(c.r64(1));
        }
        48 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f9b22); }
            let r61 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r61);
            let Some(t62) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f9b2e); };
            c.w32(7, t62);
            let Some(t63) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f9b33); };
            c.w64(1, t63);
            let Some(t64) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f9b37); };
            c.w64(5, t64);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r65 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r65);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f9b47); };
            c.w64(11, t66);
            let Some(t67) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f9b4e); };
            c.w64(0, t67);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008f9b55); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008f9b59); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008f9b5d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008f9b61); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f9b65); };
            c.w64(6, t68);
            let Some(t69) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008f9b6c); };
            c.cmp(t69, c.r64(3), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008f9b7e, 0x70008f9b79);
        }
        51 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f9b7e); };
            c.w64(12, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9b82); };
            c.w64(15, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f9b86); };
            c.w64(7, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9b8a); };
            c.w64(9, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f9b8e); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f9b92); };
            c.w64(2, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f9b96); };
            c.w64(1, t76);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f9b9c); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 53; continue;
        }
        53 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9baf); };
            c.w64(11, t77);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f9bb6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f9bba); }
            c.w64(11, c.r64(0));
            blk = 13; continue;
        }
        54 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f9bc7); };
            c.w64(11, t78);
            let Some(t79) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f9bce); };
            c.w64(0, t79);
            c.w64(12, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008f9bd5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008f9bd9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008f9bdd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008f9be1); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008f9be5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f9be9); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f9bed); };
            c.w64(0, t80);
            let Some(t81) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f9bf4); };
            c.cmp(t81, c.r64(12), 64);
            if c.cc_a() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008f9c06, 0x70008f9c01);
        }
        56 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f9c06); };
            c.w64(3, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9c0a); };
            c.w64(15, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f9c0e); };
            c.w64(7, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9c12); };
            c.w64(9, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f9c16); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f9c1a); };
            c.w64(2, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f9c1e); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f9c22); };
            c.w64(6, t89);
            blk = 58; continue;
        }
        57 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x70008f9c28); }
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f9c2b); };
            c.w64(11, t90);
            let r91 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r91);
            c.w64(0, c.r64(11));
            blk = 58; continue;
        }
        58 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(6) as u32 as i32 as i64));
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9c4a); };
            c.w64(11, t92);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f9c51); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f9c55); }
            c.w64(11, c.r64(0));
            blk = 13; continue;
        }
        59 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008f9c67); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x70008f9c6c); }
            return c.call(0x70007febf4, 0x70008f9c76, 0x70008f9c71);
        }
        60 => {
            c.w64(8, c.r64(0));
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9c79); };
            c.w64(15, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f9c7d); };
            c.w64(7, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9c81); };
            c.w64(9, t95);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008f9c85); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008f9c8a); }
            blk = 29; continue;
        }
        61 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t96) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9c99); };
            c.w32(1, t96);
            c.w32(6, c.r32(1));
            let r97 = c.r32(6) | 0x40u64;
            c.w32(6, r97);
            c.logic(r97, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70008f9caa); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f9caf); }
            blk = 64; continue;
        }
        64 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f9cb7); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f9cbb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f9cbd); }
            c.w32(8, 0x4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f9cc4); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008f9cce); };
            c.w64(3, t98);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008f9cdd, 0x70008f9cd8);
        }
        65 => {
            c.w64(11, c.r64(0));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9ce0); };
            c.w64(15, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f9ce4); };
            c.w64(7, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9ce8); };
            c.w64(9, t101);
            blk = 46; continue;
        }
        66 => {
            return RegfileExit::jump(0x70008f9cf4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f9d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f9d14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f9d18); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f9d1e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9d22); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f9d26); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f9d2c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f9d33); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f9d37); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f9d3b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f9d42); };
            c.w32(1, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9d48); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f9d4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f9d50); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008f9d54); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f9d58); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f9d5c); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f9d65); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f9d6c); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9d89); };
            c.w32(2, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9d8c); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f9d9c); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 24; continue;
        }
        6 => {
            c.w64(3, c.r64(0));
            let r18 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r18);
            c.w64(12, c.r64(11));
            let r19 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r19);
            c.w32(2, 0);
            c.logic(0, 32);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(2), c.r32(11), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(3), 0xffffffffffffffffu64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(12), 0xc0000000u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(12));
            c.cdq();
            c.idiv(c.r32(3), 32);
            c.w64(11, c.r64(0));
            let r20 = c.imul2(c.r32(11), c.r32(3), 32);
            c.w32(11, r20);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(0, 0x2u64);
            blk = 16; continue;
        }
        13 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f9df5); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f9dfc); };
            c.w64(0, t22);
            c.w64(2, c.r64(0).wrapping_add(0xc));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f9e03); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008f9e0a); };
            c.cmp(t24, c.r64(2), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008f9e13); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9e32); };
            c.w64(0, t25);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008f9e39); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008f9e3d); }
            c.w32(0, 0x6u64);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9e51); };
            c.w32(3, t26);
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(3));
            let r27 = c.r32(0) | c.r32(2);
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008f9e67); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f9e6c); }
            blk = 46; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9e79); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9e80); };
            c.cmp(t30, c.r32(2), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9e91); };
            c.w32(2, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9e94); };
            c.cmp(t32, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f9ea1); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f9ea7); }
            blk = 24; continue;
        }
        23 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f9eb0); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9ec3); };
            c.w32(11, t34);
            c.w32(0, c.r32(11));
            let r35 = c.r32(0) | 0xeu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f9ed6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f9edb); }
            blk = 27; continue;
        }
        27 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f9ee3); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f9eea); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f9ef1); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f9ef8); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008f9f01); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9f0f); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f9f16); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008f9f1a); }
            c.w64(11, c.r64(8));
            blk = 46; continue;
        }
        30 => {
            let Some(t41) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9f28); };
            c.w32(2, t41);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9f40); };
            c.w32(2, t42);
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f9f43); };
            c.cmp(t43, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(2, 0x1eu64);
            blk = 41; continue;
        }
        35 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9f53); };
            c.w32(2, t44);
            let r45 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r45);
            let Some(t46) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9f59); };
            c.cmp(t46, 0x43u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0x1eu64);
            blk = 41; continue;
        }
        37 => {
            c.w32(2, 0x1eu64);
            blk = 41; continue;
        }
        38 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f9f72); };
            c.w32(2, t47);
            let r48 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r48);
            let Some(t49) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f9f78); };
            c.cmp(t49, 0x41u64, 16);
            if c.cc_e() { blk = 51; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(2, 0xfeu64);
            blk = 41; continue;
        }
        41 => {
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f9f91); };
            c.w32(6, t50);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r51 = c.r32(2) | c.r32(11);
            c.w32(2, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008f9fa5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008f9faa); }
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f9fb2); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f9fbc); };
            c.w64(0, t52);
            return c.call(0x70008adf94, 0x70008f9fc5, 0x70008f9fc0);
        }
        45 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f9fc8); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9fcc); };
            c.w64(7, t54);
            blk = 46; continue;
        }
        46 => {
            let Some(t55) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f9fd0); };
            c.w64(7, t55);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t56) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008f9fdb); };
            c.w32(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f9fe0); };
            c.w64(15, t57);
            let Some(t58) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f9fe4); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f9fe8); };
            c.w64(5, t59);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r60 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r60);
            return RegfileExit::jump(c.r64(1));
        }
        47 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008f9ffd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fa001); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008fa005); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008fa009); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008fa00d); }
            return c.call(0x70007febf4, 0x70008fa016, 0x70008fa011);
        }
        48 => {
            c.w64(11, c.r64(0));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa019); };
            c.w64(9, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa01d); };
            c.w64(7, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fa021); };
            c.w64(8, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fa025); };
            c.w64(1, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fa029); };
            c.w64(6, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fa02d); };
            c.w64(12, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fa031); };
            c.w64(3, t67);
            blk = 15; continue;
        }
        49 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x70008fa03f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008fa044); }
            return c.call(0x70007febf4, 0x70008fa04e, 0x70008fa049);
        }
        50 => {
            c.w64(8, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa051); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa055); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa059); };
            c.w64(7, t70);
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008fa05d); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008fa062); }
            blk = 29; continue;
        }
        51 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa071); };
            c.w32(2, t71);
            c.w32(6, c.r32(2));
            let r72 = c.r32(6) | 0x40u64;
            c.w32(6, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70008fa082); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa087); }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fa08f); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fa093); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fa095); }
            c.w32(8, 0x6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fa09c); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fa0a6); };
            c.w64(3, t73);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008fa0b5, 0x70008fa0b0);
        }
        55 => {
            c.w64(11, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa0b8); };
            c.w64(9, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa0bc); };
            c.w64(7, t75);
            blk = 46; continue;
        }
        56 => {
            return RegfileExit::jump(0x70008fa0c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fa0d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fa0d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fa0d8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fa0de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa0e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fa0e6); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fa0ec); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fa0f3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fa0f7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fa0fb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fa102); };
            c.w32(1, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa108); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fa10c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fa110); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fa114); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fa118); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fa11c); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fa125); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fa12c); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa149); };
            c.w32(2, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa14c); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fa15c); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 20; continue;
        }
        6 => {
            c.w64(3, c.r64(0));
            let r18 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r18);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 7; continue;
        }
        7 => {
            let r19 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r19);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(11));
            c.cdq();
            c.idiv(c.r32(3), 32);
            c.w32(11, c.r64(2).wrapping_add(c.r64(2)));
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(0, 0xeu64);
            blk = 12; continue;
        }
        11 => {
            c.w32(0, 0x2u64);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa1af); };
            c.w32(3, t20);
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(3));
            let r21 = c.r32(0) | c.r32(2);
            c.w32(0, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fa1c5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa1ca); }
            blk = 44; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa1d7); };
            c.w32(2, t22);
            let r23 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa1de); };
            c.cmp(t24, c.r32(2), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa1ef); };
            c.w32(2, t25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa1f2); };
            c.cmp(t26, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fa1ff); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fa205); }
            blk = 20; continue;
        }
        19 => {
            let r27 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fa20e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa221); };
            c.w32(11, t28);
            c.w32(0, c.r32(11));
            let r29 = c.r32(0) | 0xeu64;
            c.w32(0, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fa234); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa239); }
            blk = 23; continue;
        }
        23 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fa241); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008fa248); };
            c.w64(11, t31);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fa24f); };
            c.w64(3, t32);
            let Some(t33) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fa256); };
            c.cmp(t33, c.r64(0), 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008fa25f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            let r34 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r34);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fa26a); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x70008fa26f); }
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x70008fa272); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x70008fa277); }
            blk = 26; continue;
        }
        26 => {
            c.fprem();
            c.fnstsw_ax();
            let r35 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r35);
            let r36 = c.r32(0) & 0xffu64;
            c.w32(0, r36);
            c.logic(r36, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fa286); }
            if !c.popfq() { return RegfileExit::fault(0x70008fa287); }
            if c.cc_p() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x70008fa28c); }
            if !c.movsd_rm(1, c.r64(4)) { return RegfileExit::fault(0x70008fa28f); }
            let r37 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa298); };
            c.w64(11, t38);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008fa29f); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008fa2a3); }
            c.w64(11, c.r64(8));
            blk = 44; continue;
        }
        28 => {
            let Some(t39) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fa2b1); };
            c.w32(2, t39);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 38; continue; }
            blk = 30; continue;
        }
        30 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa2c9); };
            c.w32(2, t40);
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa2cc); };
            c.cmp(t41, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, 0x1eu64);
            blk = 39; continue;
        }
        33 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa2dc); };
            c.w32(2, t42);
            let r43 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r43);
            let Some(t44) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fa2e2); };
            c.cmp(t44, 0x43u64, 16);
            if c.cc_ne() { blk = 38; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(2, 0x1eu64);
            blk = 39; continue;
        }
        35 => {
            c.w32(2, 0x1eu64);
            blk = 39; continue;
        }
        36 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa2fb); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            let Some(t47) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fa301); };
            c.cmp(t47, 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xfeu64);
            blk = 39; continue;
        }
        39 => {
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t48) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa31a); };
            c.w32(6, t48);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r49 = c.r32(2) | c.r32(11);
            c.w32(2, r49);
            c.logic(r49, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fa32e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa333); }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fa33b); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fa345); };
            c.w64(0, t50);
            return c.call(0x70008ae1b4, 0x70008fa34e, 0x70008fa349);
        }
        43 => {
            c.w64(11, c.r64(0));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa351); };
            c.w64(15, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa355); };
            c.w64(9, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa359); };
            c.w64(7, t53);
            blk = 44; continue;
        }
        44 => {
            let Some(t54) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa35d); };
            c.w64(7, t54);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t55) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fa368); };
            c.w32(8, t55);
            let Some(t56) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fa36d); };
            c.w64(1, t56);
            let Some(t57) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fa371); };
            c.w64(5, t57);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r58 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r58);
            return RegfileExit::jump(c.r64(1));
        }
        45 => {
            c.w64(0, c.r64(11));
            c.cdq();
            c.idiv(c.r32(3), 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008fa387); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fa38b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008fa38f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008fa393); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(11, c.r64(2));
            let r59 = c.add(c.r32(11), c.r32(2), 32);
            c.w32(11, r59);
            if c.cc_o() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 53; continue;
        }
        48 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fa3b0); };
            c.w64(11, t60);
            let Some(t61) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fa3b7); };
            c.w64(0, t61);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fa3be); };
            c.w64(12, t62);
            let Some(t63) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fa3c5); };
            c.cmp(t63, c.r64(3), 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fa3d8, 0x70008fa3d3);
        }
        50 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fa3d8); };
            c.w64(2, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa3dc); };
            c.w64(15, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa3e0); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa3e4); };
            c.w64(7, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fa3e8); };
            c.w64(8, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fa3ec); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fa3f0); };
            c.w64(6, t70);
            blk = 52; continue;
        }
        51 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008fa3f6); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 52; continue;
        }
        52 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa408); };
            c.w64(11, t71);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008fa40f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fa413); }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(0));
            blk = 9; continue;
        }
        54 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008fa420); };
            c.w64(11, t72);
            blk = 9; continue;
        }
        55 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x70008fa42c); };
            c.w64(11, t73);
            blk = 9; continue;
        }
        56 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008fa43d); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x70008fa442); }
            return c.call(0x70007febf4, 0x70008fa44c, 0x70008fa447);
        }
        57 => {
            c.w64(8, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa44f); };
            c.w64(15, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa453); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa457); };
            c.w64(7, t76);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008fa45b); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008fa460); }
            blk = 25; continue;
        }
        58 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t77) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa46f); };
            c.w32(2, t77);
            c.w32(6, c.r32(2));
            let r78 = c.r32(6) | 0x40u64;
            c.w32(6, r78);
            c.logic(r78, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70008fa480); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa485); }
            blk = 61; continue;
        }
        61 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fa48d); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fa491); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fa493); }
            c.w32(8, 0x8u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fa49a); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fa4a4); };
            c.w64(3, t79);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008fa4b3, 0x70008fa4ae);
        }
        62 => {
            c.w64(11, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa4b6); };
            c.w64(15, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa4ba); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa4be); };
            c.w64(7, t82);
            blk = 44; continue;
        }
        63 => {
            return RegfileExit::jump(0x70008fa4c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fa4d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fa4d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fa4d8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fa4de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa4e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fa4e6); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fa4ec); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fa4f3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fa4f7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fa4fb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fa502); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa508); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fa50c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fa510); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fa514); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008fa518); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fa51c); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fa520); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fa529); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fa530); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008fa53d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008fa541); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa553); };
            c.w32(6, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa556); };
            c.cmp(t16, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fa566); }
            c.xorpd_rr(3, 3);
            c.cvtsi2sd(3, (c.r32(11) as u32 as i32 as i64));
            blk = 17; continue;
        }
        6 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fa579); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fa580); };
            c.w64(6, t19);
            let r20 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            let r21 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r21);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fa58c); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008fa593); };
            c.cmp(t23, c.r64(12), 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x70008fa59c); }
            c.w64(7, c.r64(6).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa5b4); };
            c.w64(6, t24);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008fa5bb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008fa5be); }
            c.w64(10, c.r64(4));
            let r25 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            let r26 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r26);
            c.logic(r26, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008fa5cd); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70008fa5d2); };
            c.w64(0, t27);
            c.w64(10, 0x70008fa5e0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008fa5e0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008fa5e4); }
            return c.call(c.r64(0), 0x70008fa5ea, 0x70008fa5e8);
        }
        9 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008fa5ea); }
            let Some(t28) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008fa5f2); };
            c.w64(4, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fa5f7); };
            c.w64(0, t29);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fa5fb); }
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fa600); };
            c.cmp(t30, 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fa60a); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fa60e); };
            c.w64(7, t32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa612); };
            c.w32(11, t33);
            c.w32(3, c.r32(11));
            let r34 = c.r32(3) | 0xeu64;
            c.w32(3, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fa629); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa62e); }
            blk = 39; continue;
        }
        12 => {
            let Some(t35) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa63b); };
            c.w32(6, t35);
            let r36 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa642); };
            c.cmp(t37, c.r32(6), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa653); };
            c.w32(6, t38);
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa656); };
            c.cmp(t39, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(3, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fa663); }
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fa669); }
            blk = 17; continue;
        }
        16 => {
            let r40 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r40);
            if !c.movsd_rm(3, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fa672); }
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa685); };
            c.w32(11, t41);
            c.w32(0, c.r32(11));
            let r42 = c.r32(0) | 0xeu64;
            c.w32(0, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fa698); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa69d); }
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fa6a5); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008fa6ac); };
            c.w64(11, t44);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fa6b3); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fa6ba); };
            c.cmp(t46, c.r64(0), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008fa6c3); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa6ca); };
            c.w64(11, t47);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008fa6d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fa6d5); }
            c.mov128_rr(0, 3);
            c.mov128_rr(1, 2);
            c.w64(10, c.r64(4));
            let r48 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r48);
            let r49 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r49);
            c.logic(r49, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008fa6ea); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70008fa6ef); };
            c.w64(0, t50);
            c.w64(10, 0x70008fa6fdu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008fa6fd); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008fa701); }
            return c.call(c.r64(0), 0x70008fa707, 0x70008fa705);
        }
        23 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008fa707); }
            let Some(t51) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008fa70f); };
            c.w64(4, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fa714); };
            c.w64(0, t52);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fa718); }
            blk = 39; continue;
        }
        24 => {
            let Some(t53) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fa722); };
            c.w32(6, t53);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa73a); };
            c.w32(6, t54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa73d); };
            c.cmp(t55, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(6, 0x1eu64);
            blk = 35; continue;
        }
        29 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa74d); };
            c.w32(6, t56);
            let r57 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r57);
            let Some(t58) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fa753); };
            c.cmp(t58, 0x43u64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(6, 0x1eu64);
            blk = 35; continue;
        }
        31 => {
            c.w32(6, 0x1eu64);
            blk = 35; continue;
        }
        32 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t59) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa76c); };
            c.w32(6, t59);
            let r60 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r60);
            let Some(t61) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fa772); };
            c.cmp(t61, 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(6, 0xfeu64);
            blk = 35; continue;
        }
        35 => {
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa78b); };
            c.w32(1, t62);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r63 = c.r32(6) | c.r32(11);
            c.w32(6, r63);
            c.logic(r63, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fa79f); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa7a4); }
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fa7ac); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fa7b6); };
            c.w64(0, t64);
            return c.call(0x70008ae454, 0x70008fa7bf, 0x70008fa7ba);
        }
        39 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa7bf); };
            c.w64(7, t65);
            let Some(t66) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa7c3); };
            c.w64(7, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa7c7); };
            c.w64(8, t67);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t68) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fa7cf); };
            c.w32(8, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa7d4); };
            c.w64(15, t69);
            let Some(t70) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fa7d8); };
            c.w64(1, t70);
            let Some(t71) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fa7dc); };
            c.w64(5, t71);
            c.w64(12, c.r64(7));
            let r72 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r72);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008fa7ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008fa7f2); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x70008fa7fe, 0x70008fa7f9);
        }
        41 => {
            c.w64(7, c.r64(0));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fa801); };
            c.w64(11, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fa805); };
            c.w64(0, t74);
            blk = 8; continue;
        }
        42 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 2) { return RegfileExit::fault(0x70008fa813); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 3) { return RegfileExit::fault(0x70008fa818); }
            return c.call(0x70007febf4, 0x70008fa822, 0x70008fa81d);
        }
        43 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008fa825); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x70008fa82a); }
            blk = 22; continue;
        }
        44 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t75) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fa839); };
            c.w32(1, t75);
            c.w32(6, c.r32(1));
            let r76 = c.r32(6) | 0x40u64;
            c.w32(6, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70008fa84a); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fa84f); }
            blk = 47; continue;
        }
        47 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fa857); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fa85b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fa85d); }
            c.w32(8, 0xau64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fa864); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fa86e); };
            c.w64(3, t77);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008fa87d, 0x70008fa878);
        }
        48 => {
            blk = 39; continue;
        }
        49 => {
            return RegfileExit::jump(0x70008fa884);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fa894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fa894); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fa898); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fa89e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa8a2); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fa8a6); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fa8ac); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fa8b3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fa8b7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fa8bb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fa8c2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa8c8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fa8cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fa8d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008fa8d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fa8d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fa8dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008fa8e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008fa8e4); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fa8e8); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fa8f1); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fa8fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008fa900); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70008fa906); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008fa90c); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa919); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa921); };
            c.cmp(t16, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fa92e); };
            c.w32(8, t17);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fa941); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fa950, 0x70008fa94b);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa959); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa95d); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa961); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fa965); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fa969); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008fa96d); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fa971); };
            c.w64(1, t24);
            blk = 3; continue;
        }
        10 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fa977); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fa98a); };
            c.w32(8, t27);
            let r28 = c.r32(8) | 0x2u64;
            c.w32(8, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fa99d); };
            c.w32(8, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fa9a1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fa9ad); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fa9b4); };
            c.cmp(t33, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fa9bd); };
            c.w32(11, t34);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fa9d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008fa9d7); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fa9e3, 0x70008fa9de);
        }
        17 => {
            c.w32(11, 0xfeu64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fa9e9); };
            c.w64(1, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fa9ed); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fa9f1); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fa9f5); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fa9f9); };
            c.w64(2, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fa9fd); };
            c.w64(3, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008faa01); };
            c.w64(12, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008faa05); };
            c.w64(8, t42);
            blk = 22; continue;
        }
        18 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008faa0b); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            c.w32(11, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(11, 0x40u64);
            blk = 22; continue;
        }
        20 => {
            c.w32(11, 0xeu64);
            blk = 22; continue;
        }
        21 => {
            c.w32(11, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r45 = c.r32(8) | 0x40u64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(11));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008faa41); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t46) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008faa5a); };
            c.w32(8, t46);
            let r47 = c.r32(8) | 0xeu64;
            c.w32(8, r47);
            c.logic(r47, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008faa65); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008faa69); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008faa76); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008faa7c); };
            c.cmp(t50, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008faa89); };
            c.w32(1, t51);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008faa99); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008faaa5, 0x70008faaa0);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008faaaa); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008faaae); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008faab2); };
            c.w64(3, t54);
            blk = 26; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008faab8); };
            c.w32(0, t55);
            let r56 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r56);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r57 = c.r32(8) | 0x2u64;
            c.w32(8, r57);
            c.logic(r57, 32);
            let r58 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r59 = c.r32(8) | 0x40u64;
            c.w32(8, r59);
            c.logic(r59, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008faae0); };
            c.w64(8, t60);
            c.w64(1, c.r64(11));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008faae7); };
            c.w64(11, t61);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r62 = c.r32(6) | c.r32(8);
            c.w32(6, r62);
            c.logic(r62, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008faaf6); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008faafa); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008faafc); }
            c.w32(11, 0xeu64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fab03); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fab05); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fab11); };
            c.w64(3, t63);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008fab20, 0x70008fab1b);
        }
        36 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fab20); };
            c.w64(8, t64);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fab26); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r65 = c.r32(8) | 0xeu64;
            c.w32(8, r65);
            c.logic(r65, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            let Some(t66) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fab46); };
            let r67 = c.r32(0) | t66;
            c.w32(0, r67);
            c.logic(r67, 32);
            c.w64(11, c.r64(0));
            let r68 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r68);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(11, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(11, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fab6d); };
            c.w32(3, t69);
            c.w32(11, c.r32(11));
            let r70 = c.r32(8) | c.r32(3);
            c.w32(8, r70);
            c.logic(r70, 32);
            let r71 = c.r32(8) | c.r32(11);
            c.w32(8, r71);
            c.logic(r71, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fab7c); };
            c.w64(7, t72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fab80); };
            c.cmp(t73, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fab89); };
            c.w64(11, t74);
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fab8d); };
            c.w32(3, t75);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r76 = c.r32(8) | c.r32(2);
            c.w32(8, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008faba2); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008faba7); }
            blk = 48; continue;
        }
        48 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fabae); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fabb2); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fabb6); };
            c.w64(8, t79);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t80) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fabbe); };
            c.w32(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fabc3); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fabc7); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fabcb); };
            c.w64(5, t83);
            c.w64(12, c.r64(7));
            let r84 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r84);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fabd8); };
            c.w64(11, t85);
            let Some(t86) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fabdf); };
            c.w64(3, t86);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008fabe6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008fabea); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fabee); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008fabf5); };
            c.cmp(t88, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fac04, 0x70008fabff);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fac07); };
            c.w64(7, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fac0b); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fac0f); };
            c.w64(0, t91);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008fac15); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fac1c); };
            c.w64(3, t92);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008fac23); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fac2f); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r93 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r93);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fac41); }
            return c.call(0x70008708b4, 0x70008fac4b, 0x70008fac46);
        }
        55 => {
            let Some(t94) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fac4b); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r95);
            blk = 38; continue;
        }
        56 => {
            let r96 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fac5b); }
            return c.call(0x70008708b4, 0x70008fac65, 0x70008fac60);
        }
        57 => {
            let Some(t97) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fac65); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r98);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x70008fac74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fac94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fac94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fac98); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fac9e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008faca2); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008faca6); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008facac); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008facb3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008facb7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008facbb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008facc2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008facc8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008faccc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008facd0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008facd4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008facd8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008facdc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008face0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008face4); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008face8); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008facf1); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008facfc); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008fad02); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70008fad06); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008fad0c); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fad19); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fad21); };
            c.cmp(t16, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fad2e); };
            c.w32(8, t17);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fad41); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fad50, 0x70008fad4b);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fad59); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fad5d); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fad61); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fad65); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fad69); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008fad6d); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fad71); };
            c.w64(1, t24);
            blk = 3; continue;
        }
        10 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fad77); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fad8a); };
            c.w32(8, t27);
            let r28 = c.r32(8) | 0x2u64;
            c.w32(8, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fad9d); };
            c.w32(8, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fada1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fadad); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fadb4); };
            c.cmp(t33, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fadbd); };
            c.w32(11, t34);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fadd3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008fadd7); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fade3, 0x70008fadde);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fade8); };
            c.w64(1, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fadec); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fadf0); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fadf4); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fadf8); };
            c.w64(2, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fadfc); };
            c.w64(3, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fae00); };
            c.w64(12, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fae04); };
            c.w64(8, t42);
            blk = 22; continue;
        }
        18 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fae0a); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r45 = c.r32(8) | 0x40u64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fae3f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t46) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fae58); };
            c.w32(8, t46);
            let r47 = c.r32(8) | 0xeu64;
            c.w32(8, r47);
            c.logic(r47, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fae63); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008fae67); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fae74); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fae7a); };
            c.cmp(t50, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fae87); };
            c.w32(1, t51);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fae97); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008faea3, 0x70008fae9e);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008faea8); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008faeac); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008faeb0); };
            c.w64(3, t54);
            blk = 26; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008faeb6); };
            c.w32(0, t55);
            let r56 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r56);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r57 = c.r32(8) | 0x2u64;
            c.w32(8, r57);
            c.logic(r57, 32);
            let r58 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r59 = c.r32(8) | 0x40u64;
            c.w32(8, r59);
            c.logic(r59, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008faede); };
            c.w64(8, t60);
            c.w64(1, c.r64(11));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008faee5); };
            c.w64(11, t61);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r62 = c.r32(6) | c.r32(8);
            c.w32(6, r62);
            c.logic(r62, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008faef4); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008faef8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008faefa); }
            c.w32(11, 0x10u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008faf01); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008faf03); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008faf0f); };
            c.w64(3, t63);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008faf1e, 0x70008faf19);
        }
        36 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008faf1e); };
            c.w64(8, t64);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008faf24); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r65 = c.r32(8) | 0xeu64;
            c.w32(8, r65);
            c.logic(r65, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            let Some(t66) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008faf44); };
            let r67 = c.r32(0) ^ t66;
            c.w32(0, r67);
            c.logic(r67, 32);
            c.w64(11, c.r64(0));
            let r68 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r68);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008faf69); };
            c.w32(11, t69);
            c.w32(3, c.r32(6));
            let r70 = c.r32(8) | c.r32(11);
            c.w32(8, r70);
            c.logic(r70, 32);
            let r71 = c.r32(8) | c.r32(3);
            c.w32(8, r71);
            c.logic(r71, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008faf78); };
            c.w64(7, t72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008faf7c); };
            c.cmp(t73, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008faf85); };
            c.w64(11, t74);
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008faf89); };
            c.w32(3, t75);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r76 = c.r32(8) | c.r32(2);
            c.w32(8, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008faf9e); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fafa3); }
            blk = 48; continue;
        }
        48 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fafaa); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fafae); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fafb2); };
            c.w64(8, t79);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t80) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fafba); };
            c.w32(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fafbf); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fafc3); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fafc7); };
            c.w64(5, t83);
            c.w64(12, c.r64(7));
            let r84 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r84);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fafd4); };
            c.w64(11, t85);
            let Some(t86) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fafdb); };
            c.w64(3, t86);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008fafe2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008fafe6); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fafea); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008faff1); };
            c.cmp(t88, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fb000, 0x70008faffb);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb003); };
            c.w64(7, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb007); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb00b); };
            c.w64(0, t91);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008fb011); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb018); };
            c.w64(3, t92);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008fb01f); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fb02b); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r93 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r93);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fb03d); }
            return c.call(0x70008708b4, 0x70008fb047, 0x70008fb042);
        }
        55 => {
            let Some(t94) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fb047); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r95);
            blk = 38; continue;
        }
        56 => {
            let r96 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fb057); }
            return c.call(0x70008708b4, 0x70008fb061, 0x70008fb05c);
        }
        57 => {
            let Some(t97) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fb061); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r98);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x70008fb070);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fb074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fb074); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fb078); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fb07e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb082); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fb086); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fb08c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fb093); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fb097); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fb09b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fb0a2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb0a8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fb0ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fb0b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008fb0b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fb0b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fb0bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008fb0c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008fb0c4); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fb0c8); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fb0d1); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fb0dc); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008fb0e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70008fb0e6); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008fb0ec); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb0f9); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb101); };
            c.cmp(t16, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fb10e); };
            c.w32(8, t17);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fb121); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fb130, 0x70008fb12b);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb139); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fb13d); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb141); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fb145); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb149); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008fb14d); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fb151); };
            c.w64(1, t24);
            blk = 3; continue;
        }
        10 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fb157); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fb16a); };
            c.w32(8, t27);
            let r28 = c.r32(8) | 0x2u64;
            c.w32(8, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fb17d); };
            c.w32(8, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fb181); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb18d); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb194); };
            c.cmp(t33, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fb19d); };
            c.w32(11, t34);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fb1b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008fb1b7); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fb1c3, 0x70008fb1be);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fb1c8); };
            c.w64(1, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb1cc); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fb1d0); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb1d4); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fb1d8); };
            c.w64(2, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb1dc); };
            c.w64(3, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb1e0); };
            c.w64(12, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb1e4); };
            c.w64(8, t42);
            blk = 22; continue;
        }
        18 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fb1ea); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r45 = c.r32(8) | 0x40u64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fb21f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t46) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fb238); };
            c.w32(8, t46);
            let r47 = c.r32(8) | 0xeu64;
            c.w32(8, r47);
            c.logic(r47, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fb243); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008fb247); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb254); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb25a); };
            c.cmp(t50, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fb267); };
            c.w32(1, t51);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fb277); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fb283, 0x70008fb27e);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fb288); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb28c); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb290); };
            c.w64(3, t54);
            blk = 26; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fb296); };
            c.w32(0, t55);
            let r56 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r56);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r57 = c.r32(8) | 0x2u64;
            c.w32(8, r57);
            c.logic(r57, 32);
            let r58 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r59 = c.r32(8) | 0x40u64;
            c.w32(8, r59);
            c.logic(r59, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb2be); };
            c.w64(8, t60);
            c.w64(1, c.r64(11));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb2c5); };
            c.w64(11, t61);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r62 = c.r32(6) | c.r32(8);
            c.w32(6, r62);
            c.logic(r62, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fb2d4); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fb2d8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fb2da); }
            c.w32(11, 0xcu64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fb2e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fb2e3); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fb2ef); };
            c.w64(3, t63);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008fb2fe, 0x70008fb2f9);
        }
        36 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb2fe); };
            c.w64(8, t64);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fb304); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r65 = c.r32(8) | 0xeu64;
            c.w32(8, r65);
            c.logic(r65, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            let Some(t66) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fb324); };
            let r67 = c.r32(0) & t66;
            c.w32(0, r67);
            c.logic(r67, 32);
            c.w64(11, c.r64(0));
            let r68 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r68);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb349); };
            c.w32(11, t69);
            c.w32(3, c.r32(6));
            let r70 = c.r32(8) | c.r32(11);
            c.w32(8, r70);
            c.logic(r70, 32);
            let r71 = c.r32(8) | c.r32(3);
            c.w32(8, r71);
            c.logic(r71, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fb358); };
            c.w64(7, t72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fb35c); };
            c.cmp(t73, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fb365); };
            c.w64(11, t74);
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fb369); };
            c.w32(3, t75);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r76 = c.r32(8) | c.r32(2);
            c.w32(8, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008fb37e); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fb383); }
            blk = 48; continue;
        }
        48 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb38a); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb38e); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fb392); };
            c.w64(8, t79);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t80) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fb39a); };
            c.w32(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb39f); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fb3a3); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fb3a7); };
            c.w64(5, t83);
            c.w64(12, c.r64(7));
            let r84 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r84);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fb3b4); };
            c.w64(11, t85);
            let Some(t86) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fb3bb); };
            c.w64(3, t86);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008fb3c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008fb3c6); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fb3ca); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008fb3d1); };
            c.cmp(t88, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fb3e0, 0x70008fb3db);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb3e3); };
            c.w64(7, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb3e7); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb3eb); };
            c.w64(0, t91);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008fb3f1); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb3f8); };
            c.w64(3, t92);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008fb3ff); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fb40b); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r93 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r93);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fb41d); }
            return c.call(0x70008708b4, 0x70008fb427, 0x70008fb422);
        }
        55 => {
            let Some(t94) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fb427); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r95);
            blk = 38; continue;
        }
        56 => {
            let r96 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fb437); }
            return c.call(0x70008708b4, 0x70008fb441, 0x70008fb43c);
        }
        57 => {
            let Some(t97) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fb441); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r98);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x70008fb450);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fb454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fb454); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fb458); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fb45e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb462); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fb466); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fb46c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fb473); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fb477); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fb47b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fb482); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb488); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fb48c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fb490); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008fb494); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fb498); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fb49c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008fb4a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008fb4a4); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fb4a8); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fb4b1); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fb4bc); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008fb4c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008fb4c6); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fb4cc); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb4d9); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb4e1); };
            c.cmp(t16, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fb4ee); };
            c.w32(8, t17);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fb501); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fb510, 0x70008fb50b);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb519); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fb51d); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb521); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fb525); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb529); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008fb52d); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fb531); };
            c.w64(1, t24);
            blk = 3; continue;
        }
        10 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fb537); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb54a); };
            c.w32(8, t27);
            let r28 = c.r32(8) | 0x2u64;
            c.w32(8, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb55d); };
            c.w32(8, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fb561); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb56d); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb574); };
            c.cmp(t33, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fb57d); };
            c.w32(11, t34);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fb593); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008fb597); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fb5a3, 0x70008fb59e);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fb5a8); };
            c.w64(1, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb5ac); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fb5b0); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb5b4); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fb5b8); };
            c.w64(2, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb5bc); };
            c.w64(3, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb5c0); };
            c.w64(12, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb5c4); };
            c.w64(8, t42);
            blk = 22; continue;
        }
        18 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fb5ca); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r45 = c.r32(8) | 0x40u64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fb5ff); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t46) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb618); };
            c.w32(8, t46);
            let r47 = c.r32(8) | 0xeu64;
            c.w32(8, r47);
            c.logic(r47, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fb623); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008fb627); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb634); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb63a); };
            c.cmp(t50, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fb647); };
            c.w32(1, t51);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fb657); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fb663, 0x70008fb65e);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fb668); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb66c); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb670); };
            c.w64(3, t54);
            blk = 26; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fb676); };
            c.w32(0, t55);
            let r56 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r56);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r57 = c.r32(8) | 0x2u64;
            c.w32(8, r57);
            c.logic(r57, 32);
            let r58 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r59 = c.r32(8) | 0x40u64;
            c.w32(8, r59);
            c.logic(r59, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb69e); };
            c.w64(8, t60);
            c.w64(1, c.r64(11));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fb6a5); };
            c.w64(11, t61);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r62 = c.r32(6) | c.r32(8);
            c.w32(6, r62);
            c.logic(r62, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fb6b4); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fb6b8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fb6ba); }
            c.w32(11, 0x12u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fb6c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fb6c3); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fb6cf); };
            c.w64(3, t63);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008fb6de, 0x70008fb6d9);
        }
        36 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb6de); };
            c.w64(8, t64);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fb6e4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r65 = c.r32(8) | 0xeu64;
            c.w32(8, r65);
            c.logic(r65, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.w64(1, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb707); };
            c.w64(11, t66);
            let r67 = c.shl(c.r32(11), c.r8(1), 32).unwrap_or(c.r32(11));
            c.w32(11, r67);
            c.w64(0, c.r64(11));
            let r68 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r68);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb730); };
            c.w32(11, t69);
            c.w32(3, c.r32(6));
            let r70 = c.r32(8) | c.r32(11);
            c.w32(8, r70);
            c.logic(r70, 32);
            let r71 = c.r32(8) | c.r32(3);
            c.w32(8, r71);
            c.logic(r71, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fb73f); };
            c.w64(7, t72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fb743); };
            c.cmp(t73, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fb74c); };
            c.w64(11, t74);
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fb750); };
            c.w32(3, t75);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r76 = c.r32(8) | c.r32(2);
            c.w32(8, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008fb765); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fb76a); }
            blk = 48; continue;
        }
        48 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb771); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb775); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fb779); };
            c.w64(8, t79);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t80) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fb781); };
            c.w32(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb786); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fb78a); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fb78e); };
            c.w64(5, t83);
            c.w64(12, c.r64(7));
            let r84 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r84);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fb79b); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008fb7a2); };
            c.w64(3, t86);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008fb7a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008fb7ad); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fb7b1); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008fb7b8); };
            c.cmp(t88, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fb7c7, 0x70008fb7c2);
        }
        51 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb7c7); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb7cb); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb7cf); };
            c.w64(8, t91);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008fb7d5); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb7dc); };
            c.w64(3, t92);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008fb7e3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fb7ef); }
            blk = 41; continue;
        }
        54 => {
            let r93 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r93);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fb7fd); }
            return c.call(0x70008708b4, 0x70008fb807, 0x70008fb802);
        }
        55 => {
            let Some(t94) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fb807); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r95);
            blk = 38; continue;
        }
        56 => {
            let r96 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fb817); }
            return c.call(0x70008708b4, 0x70008fb821, 0x70008fb81c);
        }
        57 => {
            let Some(t97) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fb821); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r98);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x70008fb830);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fb834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fb834); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fb838); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fb83e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb842); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fb846); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fb84c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fb853); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fb857); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fb85b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fb862); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb868); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fb86c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fb870); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008fb874); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fb878); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fb87c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008fb880); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008fb884); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fb888); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fb891); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fb89c); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008fb8a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008fb8a6); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fb8ac); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb8b9); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb8c1); };
            c.cmp(t16, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fb8ce); };
            c.w32(8, t17);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fb8e1); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fb8f0, 0x70008fb8eb);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb8f9); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fb8fd); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb901); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fb905); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb909); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008fb90d); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fb911); };
            c.w64(1, t24);
            blk = 3; continue;
        }
        10 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fb917); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb92a); };
            c.w32(8, t27);
            let r28 = c.r32(8) | 0x2u64;
            c.w32(8, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb93d); };
            c.w32(8, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fb941); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fb94d); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fb954); };
            c.cmp(t33, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fb95d); };
            c.w32(11, t34);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fb973); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008fb977); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fb983, 0x70008fb97e);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fb988); };
            c.w64(1, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fb98c); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fb990); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fb994); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fb998); };
            c.w64(2, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fb99c); };
            c.w64(3, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fb9a0); };
            c.w64(12, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb9a4); };
            c.w64(8, t42);
            blk = 22; continue;
        }
        18 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fb9aa); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r45 = c.r32(8) | 0x40u64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fb9df); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t46) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fb9f8); };
            c.w32(8, t46);
            let r47 = c.r32(8) | 0xeu64;
            c.w32(8, r47);
            c.logic(r47, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fba03); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008fba07); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fba14); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fba1a); };
            c.cmp(t50, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fba27); };
            c.w32(1, t51);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fba37); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fba43, 0x70008fba3e);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fba48); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fba4c); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fba50); };
            c.w64(3, t54);
            blk = 26; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fba56); };
            c.w32(0, t55);
            let r56 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r56);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r57 = c.r32(8) | 0x2u64;
            c.w32(8, r57);
            c.logic(r57, 32);
            let r58 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r59 = c.r32(8) | 0x40u64;
            c.w32(8, r59);
            c.logic(r59, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fba7e); };
            c.w64(8, t60);
            c.w64(1, c.r64(11));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fba85); };
            c.w64(11, t61);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r62 = c.r32(6) | c.r32(8);
            c.w32(6, r62);
            c.logic(r62, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fba94); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fba98); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fba9a); }
            c.w32(11, 0x14u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fbaa1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fbaa3); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fbaaf); };
            c.w64(3, t63);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008fbabe, 0x70008fbab9);
        }
        36 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fbabe); };
            c.w64(8, t64);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fbac4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r65 = c.r32(8) | 0xeu64;
            c.w32(8, r65);
            c.logic(r65, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.w64(1, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fbae7); };
            c.w64(11, t66);
            let r67 = c.sar(c.r32(11), c.r8(1), 32).unwrap_or(c.r32(11));
            c.w32(11, r67);
            c.w64(0, c.r64(11));
            let r68 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r68);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fbb10); };
            c.w32(11, t69);
            c.w32(3, c.r32(6));
            let r70 = c.r32(8) | c.r32(11);
            c.w32(8, r70);
            c.logic(r70, 32);
            let r71 = c.r32(8) | c.r32(3);
            c.w32(8, r71);
            c.logic(r71, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fbb1f); };
            c.w64(7, t72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fbb23); };
            c.cmp(t73, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fbb2c); };
            c.w64(11, t74);
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fbb30); };
            c.w32(3, t75);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r76 = c.r32(8) | c.r32(2);
            c.w32(8, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008fbb45); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fbb4a); }
            blk = 48; continue;
        }
        48 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbb51); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbb55); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fbb59); };
            c.w64(8, t79);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t80) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fbb61); };
            c.w32(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fbb66); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fbb6a); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fbb6e); };
            c.w64(5, t83);
            c.w64(12, c.r64(7));
            let r84 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r84);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fbb7b); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008fbb82); };
            c.w64(3, t86);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008fbb89); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008fbb8d); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fbb91); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008fbb98); };
            c.cmp(t88, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fbba7, 0x70008fbba2);
        }
        51 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fbba7); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbbab); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fbbaf); };
            c.w64(8, t91);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008fbbb5); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fbbbc); };
            c.w64(3, t92);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008fbbc3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fbbcf); }
            blk = 41; continue;
        }
        54 => {
            let r93 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r93);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fbbdd); }
            return c.call(0x70008708b4, 0x70008fbbe7, 0x70008fbbe2);
        }
        55 => {
            let Some(t94) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fbbe7); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r95);
            blk = 38; continue;
        }
        56 => {
            let r96 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fbbf7); }
            return c.call(0x70008708b4, 0x70008fbc01, 0x70008fbbfc);
        }
        57 => {
            let Some(t97) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fbc01); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r98);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x70008fbc10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fbc14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fbc14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fbc18); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fbc1e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fbc22); };
            c.w64(8, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fbc26); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fbc2c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fbc33); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fbc37); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fbc3b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fbc42); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fbc48); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fbc4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fbc50); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008fbc54); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fbc58); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fbc5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008fbc60); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008fbc64); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fbc68); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fbc71); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fbc7c); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008fbc82); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008fbc86); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fbc8c); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fbc99); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fbca1); };
            c.cmp(t16, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fbcae); };
            c.w32(8, t17);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fbcc1); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fbcd0, 0x70008fbccb);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fbcd9); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fbcdd); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbce1); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fbce5); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fbce9); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008fbced); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fbcf1); };
            c.w64(1, t24);
            blk = 3; continue;
        }
        10 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fbcf7); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fbd0a); };
            c.w32(8, t27);
            let r28 = c.r32(8) | 0x2u64;
            c.w32(8, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r29);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fbd1d); };
            c.w32(8, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fbd21); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fbd2d); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fbd34); };
            c.cmp(t33, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t34) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fbd3d); };
            c.w32(11, t34);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fbd53); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008fbd57); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fbd63, 0x70008fbd5e);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fbd68); };
            c.w64(1, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fbd6c); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fbd70); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbd74); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fbd78); };
            c.w64(2, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fbd7c); };
            c.w64(3, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fbd80); };
            c.w64(12, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fbd84); };
            c.w64(8, t42);
            blk = 22; continue;
        }
        18 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fbd8a); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r45 = c.r32(8) | 0x40u64;
            c.w32(8, r45);
            c.logic(r45, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fbdbf); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t46) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fbdd8); };
            c.w32(8, t46);
            let r47 = c.r32(8) | 0xeu64;
            c.w32(8, r47);
            c.logic(r47, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fbde3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008fbde7); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fbdf4); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fbdfa); };
            c.cmp(t50, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fbe07); };
            c.w32(1, t51);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fbe17); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70008fbe23, 0x70008fbe1e);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008fbe28); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbe2c); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fbe30); };
            c.w64(3, t54);
            blk = 26; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fbe36); };
            c.w32(0, t55);
            let r56 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r56);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r57 = c.r32(8) | 0x2u64;
            c.w32(8, r57);
            c.logic(r57, 32);
            let r58 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r59 = c.r32(8) | 0x40u64;
            c.w32(8, r59);
            c.logic(r59, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fbe5e); };
            c.w64(8, t60);
            c.w64(1, c.r64(11));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fbe65); };
            c.w64(11, t61);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r62 = c.r32(6) | c.r32(8);
            c.w32(6, r62);
            c.logic(r62, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008fbe74); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fbe78); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fbe7a); }
            c.w32(11, 0x16u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008fbe81); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fbe83); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fbe8f); };
            c.w64(3, t63);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008fbe9e, 0x70008fbe99);
        }
        36 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fbe9e); };
            c.w64(8, t64);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fbea4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r65 = c.r32(8) | 0xeu64;
            c.w32(8, r65);
            c.logic(r65, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.w64(1, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fbec7); };
            c.w64(11, t66);
            let r67 = c.shr(c.r32(11), c.r8(1), 32).unwrap_or(c.r32(11));
            c.w32(11, r67);
            c.cmp(c.r32(11), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t68) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fbef2); };
            c.w32(11, t68);
            c.w32(3, c.r32(6));
            let r69 = c.r32(8) | c.r32(11);
            c.w32(8, r69);
            c.logic(r69, 32);
            let r70 = c.r32(8) | c.r32(3);
            c.w32(8, r70);
            c.logic(r70, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fbf01); };
            c.w64(7, t71);
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fbf05); };
            c.cmp(t72, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fbf0e); };
            c.w64(11, t73);
            let Some(t74) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fbf12); };
            c.w32(3, t74);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r75 = c.r32(8) | c.r32(2);
            c.w32(8, r75);
            c.logic(r75, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008fbf27); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fbf2c); }
            blk = 48; continue;
        }
        48 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbf33); };
            c.w64(7, t76);
            let Some(t77) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbf37); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fbf3b); };
            c.w64(8, t78);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t79) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fbf43); };
            c.w32(8, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fbf48); };
            c.w64(15, t80);
            let Some(t81) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fbf4c); };
            c.w64(1, t81);
            let Some(t82) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fbf50); };
            c.w64(5, t82);
            c.w64(12, c.r64(7));
            let r83 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r83);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fbf5d); };
            c.w64(0, t84);
            let Some(t85) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008fbf64); };
            c.w64(3, t85);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008fbf6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008fbf6f); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fbf73); };
            c.w64(1, t86);
            let Some(t87) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008fbf7a); };
            c.cmp(t87, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fbf89, 0x70008fbf84);
        }
        51 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fbf89); };
            c.w64(11, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fbf8d); };
            c.w64(7, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fbf91); };
            c.w64(8, t90);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008fbf97); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fbf9e); };
            c.w64(3, t91);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008fbfa5); }
            c.w32(10, c.r32(11));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fbfb4); }
            blk = 41; continue;
        }
        54 => {
            let r92 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r92);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fbfc2); }
            return c.call(0x70008708b4, 0x70008fbfcc, 0x70008fbfc7);
        }
        55 => {
            let Some(t93) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fbfcc); };
            c.w32(0, t93);
            let r94 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            blk = 38; continue;
        }
        56 => {
            let r95 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r95);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fbfdc); }
            return c.call(0x70008708b4, 0x70008fbfe6, 0x70008fbfe1);
        }
        57 => {
            let Some(t96) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fbfe6); };
            c.w32(12, t96);
            let r97 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x70008fbff4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fc014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc017); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fc01b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fc022); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fc026); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fc02d); };
            c.w32(3, (t6 as u8 as i8 as i64 as u64));
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fc033); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc039); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fc03d); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fc046); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            c.w32(1, c.r32(0));
            c.w32(6, c.r32(3));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fc053); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let r13 = c.add(c.r32(6), c.r32(1), 32);
            c.w32(6, r13);
            if c.cc_o() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc074); };
            c.w32(11, t14);
            c.w32(0, c.r32(11));
            let r15 = c.r32(0) | 0x2u64;
            c.w32(0, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fc087); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc08c); }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc097); };
            c.w64(11, t16);
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(0).wrapping_add(0x3));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008fc0a2); };
            c.w32(0, t17);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fc0b6); };
            c.w64(1, t18);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(8)) { return RegfileExit::fault(0x70008fc0c2); }
            let r19 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r19);
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008fc0ce); };
            c.w32(7, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fc0d3); };
            c.w64(1, t21);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fc0df); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fc0e3); }
            let r22 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r22);
            let r23 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r23);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fc0f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fc0f7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fc0fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008fc0ff); }
            blk = 14; continue;
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fc105); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fc109); }
            let r24 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r24);
            let Some(t25) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc10f); };
            c.w32(1, t25);
            let r26 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r26);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fc115); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fc119); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fc11d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fc121); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008fc125); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008fc129); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008fc12d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008fc131); }
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc135); };
            c.cmp(t27, c.r32(1), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fc142); }
            blk = 14; continue;
        }
        14 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008fc147); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc152); };
            c.w32(11, t28);
            c.w32(0, c.r32(11));
            let r29 = c.r32(0) | 0xeu64;
            c.w32(0, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fc165); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc16a); }
            blk = 17; continue;
        }
        17 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fc172); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008fc179); };
            c.w64(11, t31);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fc180); };
            c.w64(2, t32);
            let Some(t33) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008fc187); };
            c.cmp(t33, c.r64(0), 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fc196, 0x70008fc191);
        }
        19 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fc196); };
            c.w64(3, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc19a); };
            c.w64(15, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc19e); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc1a2); };
            c.w64(9, t37);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008fc1a6); }
            blk = 21; continue;
        }
        20 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008fc1ad); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 21; continue;
        }
        21 => {
            let r38 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r38);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc1c5); };
            c.w64(8, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008fc1cc); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008fc1d0); }
            c.w64(8, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc1db); }
            blk = 8; continue;
        }
        22 => {
            let Some(t40) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fc1e1); };
            c.w32(1, t40);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008fc1f4); }
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 38; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t41) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc21f); };
            c.w32(1, t41);
            let r42 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r42);
            let Some(t43) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fc225); };
            c.cmp(t43, 0x41u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fc230); }
            c.w64(6, c.r64(11));
            return c.call(0x700089b334, 0x70008fc23c, 0x70008fc237);
        }
        28 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t44) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fc240); };
            c.cmp(t44, 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc246); };
            c.w64(8, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fc24a); };
            c.w64(2, t46);
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc24e); };
            c.w32(7, t47);
            c.w32(11, c.r32(7));
            let r48 = c.r32(11) | 0x40u64;
            c.w32(11, r48);
            c.logic(r48, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008fc262); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc267); }
            blk = 32; continue;
        }
        32 => {
            c.w64(8, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc272); };
            c.w64(15, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc276); };
            c.w64(7, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc27a); };
            c.w64(9, t51);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc281); }
            blk = 8; continue;
        }
        33 => {
            let Some(t52) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fc287); };
            c.cmp(t52, 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc28d); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fc291); };
            c.w64(7, t54);
            let Some(t55) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc295); };
            c.w32(11, t55);
            c.w32(0, c.r32(11));
            let r56 = c.r32(0) | 0xfeu64;
            c.w32(0, r56);
            c.logic(r56, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fc2aa); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc2af); }
            blk = 36; continue;
        }
        36 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc2b7); };
            c.w64(11, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fc2bb); };
            c.w64(2, t58);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008fc2bf); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fc2c8); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008fc2c9); };
            c.w64(3, t59);
            c.w32(0, 0x1u64);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fc2d5); };
            c.w64(6, t60);
            return c.call(0x700086ed74, 0x70008fc2de, 0x70008fc2d9);
        }
        37 => {
            return RegfileExit::fault(0x70008fc2de);
        }
        38 => {
            let Some(t61) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc2df); };
            c.w32(1, t61);
            let r62 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r62);
            let Some(t63) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fc2e5); };
            c.cmp(t63, 0x40u64, 16);
            if c.cc_ae() { blk = 47; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc2f2); };
            c.w64(1, t64);
            let Some(t65) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc2f6); };
            c.w32(6, t65);
            c.w32(12, c.r32(6));
            let r66 = c.r32(12) | 0x20u64;
            c.w32(12, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(6, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x70008fc309); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc30d); }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fc314); }
            c.w64(6, c.r64(11));
            return c.call(0x700086f9d4, 0x70008fc320, 0x70008fc31b);
        }
        43 => {
            c.w64(8, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc323); };
            c.w64(15, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc327); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc32b); };
            c.w64(9, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc332); }
            blk = 8; continue;
        }
        44 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t70) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc33d); };
            c.w32(1, t70);
            let r71 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r71);
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc343); };
            c.cmp(t72, c.r32(1), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t73) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fc34c); };
            c.cmp(t73, 0x43u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(1, 0xfeu64);
            blk = 49; continue;
        }
        48 => {
            c.w32(1, 0x1eu64);
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc365); };
            c.w64(6, t74);
            let Some(t75) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc369); };
            c.w32(12, t75);
            c.w32(1, c.r32(1));
            c.w32(11, c.r32(12));
            let r76 = c.r32(1) | c.r32(11);
            c.w32(1, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(12), c.r32(1), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fc37e); }
            if !c.st32(c.r64(6).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc383); }
            blk = 52; continue;
        }
        52 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fc38a); }
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fc38e); };
            c.w64(6, t77);
            return c.call(0x70008ad794, 0x70008fc397, 0x70008fc392);
        }
        53 => {
            c.w64(8, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc39a); };
            c.w64(15, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc39e); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc3a2); };
            c.w64(9, t80);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc3a9); }
            blk = 8; continue;
        }
        54 => {
            return RegfileExit::jump(0x70008fc3b0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fc3b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc3b7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fc3bb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fc3c2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fc3c6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fc3cd); };
            c.w32(3, (t6 as u8 as i8 as i64 as u64));
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fc3d3); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc3d9); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fc3dd); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fc3e6); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            c.w32(1, c.r32(0));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fc3f1); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(11, c.r32(3));
            let r13 = c.sub(c.r32(1), c.r32(11), 32);
            c.w32(1, r13);
            if c.cc_o() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(0, 0x2u64);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc421); };
            c.w32(3, t14);
            c.w32(0, c.r32(0));
            c.w32(1, c.r32(3));
            let r15 = c.r32(0) | c.r32(1);
            c.w32(0, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fc433); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc438); }
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc440); };
            c.w64(8, t16);
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(0).wrapping_add(0x3));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008fc44b); };
            c.w32(0, t17);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fc45f); };
            c.w64(1, t18);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008fc46b); }
            let r19 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r19);
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008fc477); };
            c.w32(7, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fc47c); };
            c.w64(1, t21);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fc488); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fc48c); }
            let r22 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r22);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fc492); };
            c.w64(11, t23);
            let Some(t24) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fc499); };
            c.w64(1, t24);
            let r25 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r25);
            c.w64(12, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fc4a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fc4a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fc4aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008fc4ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fc4b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008fc4b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008fc4ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008fc4be); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008fc4c2); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fc4c6); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008fc4cd); };
            c.cmp(t27, c.r64(12), 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fc4df, 0x70008fc4da);
        }
        13 => {
            c.w64(11, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc4e2); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc4e6); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc4ea); };
            c.w64(9, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fc4ee); };
            c.w64(0, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fc4f2); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fc4f6); };
            c.w64(2, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fc4fa); };
            c.w64(6, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc4fe); };
            c.w64(3, t35);
            blk = 15; continue;
        }
        14 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x70008fc504); }
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fc507); };
            c.w64(11, t36);
            let r37 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r37);
            blk = 15; continue;
        }
        15 => {
            let r38 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r38);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc525); };
            c.w64(0, t39);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008fc52c); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008fc530); }
            c.w32(0, 0xeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc53e); }
            blk = 5; continue;
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fc544); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fc548); }
            let r40 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc54e); };
            c.w32(1, t41);
            let r42 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fc554); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008fc558); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008fc55c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008fc560); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008fc564); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fc568); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008fc56c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008fc570); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc574); };
            c.cmp(t43, c.r32(1), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            let r44 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r44);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fc583); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008fc590); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x70008fc595); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t45) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc59f); };
            c.w32(11, t45);
            c.w32(0, c.r32(11));
            let r46 = c.r32(0) | 0xeu64;
            c.w32(0, r46);
            c.logic(r46, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fc5b2); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc5b7); }
            blk = 20; continue;
        }
        20 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fc5bf); };
            c.w64(8, t47);
            let Some(t48) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008fc5c6); };
            c.w64(11, t48);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fc5cd); };
            c.w64(3, t49);
            let Some(t50) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fc5d4); };
            c.cmp(t50, c.r64(0), 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fc5e3, 0x70008fc5de);
        }
        22 => {
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008fc5e3); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008fc5e8); }
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008fc5ef); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 24; continue;
        }
        24 => {
            c.arith_sd(1, 0, c.xmm_u64(1));
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc5fd); };
            c.w64(7, t51);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008fc604); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fc607); }
            c.w64(11, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc60f); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc613); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc617); };
            c.w64(9, t54);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc61e); }
            blk = 8; continue;
        }
        25 => {
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t55) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fc628); };
            c.w32(1, t55);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008fc62c); }
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t56) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc645); };
            c.w32(1, t56);
            let r57 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r57);
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc64b); };
            c.cmp(t58, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(1, 0x1eu64);
            blk = 47; continue;
        }
        30 => {
            let Some(t59) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fc65e); };
            c.cmp(t59, 0x43u64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(1, 0x1eu64);
            blk = 47; continue;
        }
        32 => {
            c.w32(1, 0x1eu64);
            blk = 47; continue;
        }
        33 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc686); };
            c.w32(1, t60);
            let r61 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r61);
            let Some(t62) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fc68c); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc69c); };
            c.w32(1, t63);
            c.w32(0, c.r32(1));
            let r64 = c.r32(0) | 0x40u64;
            c.w32(0, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fc6ad); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc6b2); }
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fc6ba); }
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fc6be); };
            c.w64(0, t65);
            c.w64(6, c.r64(11));
            return c.call(0x700089bcd4, 0x70008fc6ca, 0x70008fc6c5);
        }
        39 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(11, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc6d1); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc6d5); };
            c.w64(7, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc6d9); };
            c.w64(9, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc6e0); }
            blk = 8; continue;
        }
        41 => {
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fc6e6); };
            c.cmp(t69, 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fc6ec); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fc6f0); };
            c.w64(2, t71);
            let Some(t72) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc6f4); };
            c.w32(7, t72);
            c.w32(11, c.r32(7));
            let r73 = c.r32(11) | 0xfeu64;
            c.w32(11, r73);
            c.logic(r73, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008fc70b); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc710); }
            blk = 44; continue;
        }
        44 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc718); };
            c.w64(7, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fc71c); };
            c.w64(11, t75);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fc720); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fc729); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008fc72a); };
            c.w64(3, t76);
            c.w32(0, 0x1u64);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc736); };
            c.w64(6, t77);
            return c.call(0x700086ed74, 0x70008fc73f, 0x70008fc73a);
        }
        45 => {
            return RegfileExit::fault(0x70008fc73f);
        }
        46 => {
            c.w32(1, 0xfeu64);
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc74a); };
            c.w32(6, t78);
            c.w32(1, c.r32(1));
            c.w32(11, c.r32(6));
            let r79 = c.r32(1) | c.r32(11);
            c.w32(1, r79);
            c.logic(r79, 32);
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fc75e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc763); }
            blk = 50; continue;
        }
        50 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fc76b); }
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc76f); };
            c.w64(6, t80);
            return c.call(0x70008adb14, 0x70008fc778, 0x70008fc773);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc77b); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc77f); };
            c.w64(7, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc783); };
            c.w64(9, t83);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc78a); }
            blk = 8; continue;
        }
        52 => {
            return RegfileExit::jump(0x70008fc794);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fc7b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc7b7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fc7bb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fc7c2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fc7c6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fc7cd); };
            c.w32(3, (t6 as u8 as i8 as i64 as u64));
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fc7d3); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc7d9); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fc7dd); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fc7e6); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fc7ef); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(11, c.r64(3));
            let r13 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r13);
            let r14 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r14);
            c.w64(6, c.r64(0));
            let r15 = c.imul2(c.r32(6), c.r32(11), 32);
            c.w32(6, r15);
            if c.cc_o() { blk = 22; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(11, c.r64(6));
            let r16 = c.add(c.r32(11), c.r32(6), 32);
            c.w32(11, r16);
            if c.cc_o() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 10; continue;
        }
        7 => {
            let r17 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r17);
            let r18 = c.r32(3) | c.r32(0);
            c.w32(3, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008fc83d); };
            c.w64(11, t19);
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(0, 0xeu64);
            blk = 13; continue;
        }
        12 => {
            c.w32(0, 0x2u64);
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fc85b); };
            c.w32(3, t20);
            c.w32(0, c.r32(0));
            c.w32(1, c.r32(3));
            let r21 = c.r32(0) | c.r32(1);
            c.w32(0, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fc86d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fc872); }
            blk = 16; continue;
        }
        16 => {
            let Some(t22) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc87a); };
            c.w64(7, t22);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fc885); };
            c.w32(8, t23);
            let Some(t24) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fc88a); };
            c.w64(1, t24);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fc896); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fc89a); }
            let r25 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r25);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fc8a0); };
            c.w64(11, t26);
            let Some(t27) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fc8a7); };
            c.w64(0, t27);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fc8ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fc8b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fc8b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008fc8ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008fc8be); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008fc8c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008fc8c6); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fc8ca); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fc8d1); };
            c.cmp(t29, c.r64(3), 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fc8e4, 0x70008fc8df);
        }
        19 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fc8e4); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc8e8); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc8ec); };
            c.w64(9, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc8f0); };
            c.w64(7, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc8f4); };
            c.w64(8, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fc8f8); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fc8fc); };
            c.w64(1, t36);
            blk = 21; continue;
        }
        20 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008fc902); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(6) as u32 as i32 as i64));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc914); };
            c.w64(11, t37);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008fc91b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fc91f); }
            c.w64(11, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc92a); }
            blk = 10; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fc930); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fc934); }
            let r38 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fc93a); };
            c.w64(3, t39);
            let Some(t40) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fc941); };
            c.w64(6, t40);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fc948); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fc94c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fc950); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008fc954); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008fc958); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008fc95c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008fc960); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008fc964); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70008fc968); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fc96c); };
            c.w64(6, t41);
            let Some(t42) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008fc973); };
            c.cmp(t42, c.r64(12), 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(2, 0xcu64);
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70008fc985, 0x70008fc980);
        }
        24 => {
            c.w64(3, c.r64(0));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fc988); };
            c.w64(15, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fc98c); };
            c.w64(9, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fc990); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fc994); };
            c.w64(8, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fc998); };
            c.w64(2, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fc99c); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fc9a0); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fc9a4); };
            c.w64(11, t50);
            blk = 26; continue;
        }
        25 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x70008fc9aa); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fc9ad); };
            c.w64(3, t51);
            let r52 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r52);
            blk = 26; continue;
        }
        26 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fc9ca); };
            c.w64(11, t53);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008fc9d1); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fc9d5); }
            c.w64(11, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fc9e0); }
            blk = 10; continue;
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fc9e6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fc9ea); }
            let r54 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fc9f0); };
            c.w32(6, t55);
            let r56 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r56);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fc9f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fc9fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fc9fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fca02); }
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fca06); };
            c.cmp(t57, c.r32(6), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            let r58 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r58);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fca15); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008fca22); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x70008fca27); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t59) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fca31); };
            c.w32(11, t59);
            c.w32(0, c.r32(11));
            let r60 = c.r32(0) | 0xeu64;
            c.w32(0, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fca44); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fca49); }
            blk = 31; continue;
        }
        31 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fca51); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008fca58); };
            c.w64(11, t62);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fca5f); };
            c.w64(3, t63);
            let Some(t64) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fca66); };
            c.cmp(t64, c.r64(0), 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fca75, 0x70008fca70);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008fca75); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008fca7a); }
            blk = 35; continue;
        }
        34 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008fca81); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 35; continue;
        }
        35 => {
            c.arith_sd(2, 1, c.xmm_u64(0));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fca8f); };
            c.w64(7, t65);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008fca96); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008fca99); }
            c.w64(11, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fcaa1); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fcaa5); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fcaa9); };
            c.w64(7, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fcab0); }
            blk = 16; continue;
        }
        36 => {
            let Some(t69) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fcab6); };
            c.w32(6, t69);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t70) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fcad3); };
            c.w32(6, t70);
            let r71 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r71);
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fcad9); };
            c.cmp(t72, c.r32(6), 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(6, 0x1eu64);
            blk = 52; continue;
        }
        41 => {
            let Some(t73) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fcaec); };
            c.cmp(t73, 0x43u64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0x1eu64);
            blk = 52; continue;
        }
        43 => {
            c.w32(6, 0x1eu64);
            blk = 52; continue;
        }
        44 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t74) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fcb10); };
            c.w32(6, t74);
            let r75 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r75);
            let Some(t76) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fcb16); };
            c.cmp(t76, 0x41u64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t77) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fcb22); };
            c.w32(1, t77);
            c.w32(6, c.r32(1));
            let r78 = c.r32(6) | 0x40u64;
            c.w32(6, r78);
            c.logic(r78, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70008fcb33); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fcb38); }
            blk = 49; continue;
        }
        49 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fcb40); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fcb44); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008fcb45); }
            c.w32(8, 0x4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fcb4c); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fcb4e); };
            c.w64(3, t79);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008fcb62, 0x70008fcb5d);
        }
        50 => {
            c.w64(11, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fcb65); };
            c.w64(15, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fcb69); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fcb6d); };
            c.w64(7, t82);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fcb74); }
            blk = 16; continue;
        }
        51 => {
            c.w32(6, 0xfeu64);
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t83) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fcb84); };
            c.w32(1, t83);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r84 = c.r32(6) | c.r32(11);
            c.w32(6, r84);
            c.logic(r84, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fcb98); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fcb9d); }
            blk = 55; continue;
        }
        55 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fcba5); }
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fcba9); };
            c.w64(6, t85);
            return c.call(0x70008adcd4, 0x70008fcbb2, 0x70008fcbad);
        }
        56 => {
            c.w64(11, c.r64(0));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fcbb5); };
            c.w64(15, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fcbb9); };
            c.w64(9, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fcbbd); };
            c.w64(7, t88);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fcbc4); }
            blk = 16; continue;
        }
        57 => {
            return RegfileExit::jump(0x70008fcbcc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fcbd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fcbd7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fcbdb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fcbe2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fcbe6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fcbed); };
            c.w32(3, (t6 as u8 as i8 as i64 as u64));
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fcbf3); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fcbf9); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fcbfd); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fcc06); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fcc0f); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(11, c.r64(3));
            let r13 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r13);
            c.w64(12, c.r64(0));
            let r14 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r14);
            c.w32(2, 0);
            c.logic(0, 32);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(11), 0xffffffffffffffffu64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0xc0000000u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(12));
            c.cdq();
            c.idiv(c.r32(11), 32);
            c.w64(3, c.r64(0));
            let r15 = c.imul2(c.r32(3), c.r32(11), 32);
            c.w32(3, r15);
            c.cmp(c.r32(3), c.r32(12), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(0, 0x2u64);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fcc79); };
            c.w32(3, t16);
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(3));
            let r17 = c.r32(0) | c.r32(2);
            c.w32(0, r17);
            c.logic(r17, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fcc8b); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fcc90); }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fcc98); };
            c.w64(7, t18);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fcca3); };
            c.w32(8, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fcca8); };
            c.w64(1, t20);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fccb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fccb8); }
            let r21 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r21);
            blk = 18; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fccc0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fccc4); }
            let r22 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r22);
            blk = 18; continue;
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fcccc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fccd0); }
            let r23 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r23);
            blk = 18; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fccd8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fccdc); }
            let r24 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r24);
            blk = 18; continue;
        }
        18 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fcce2); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008fcce9); };
            c.w64(3, t26);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fccf0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fccf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fccf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008fccfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fcd00); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008fcd04); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fcd08); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008fcd0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008fcd10); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fcd14); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fcd1b); };
            c.cmp(t28, c.r64(2), 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fcd2a, 0x70008fcd25);
        }
        20 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fcd2a); };
            c.w64(11, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fcd2e); };
            c.w64(15, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fcd32); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fcd36); };
            c.w64(7, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fcd3a); };
            c.w64(8, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fcd3e); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fcd42); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fcd46); };
            c.w64(12, t36);
            blk = 22; continue;
        }
        21 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008fcd4c); }
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fcd4f); };
            c.w64(0, t37);
            let r38 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r38);
            blk = 22; continue;
        }
        22 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fcd70); };
            c.w64(11, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008fcd77); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008fcd7b); }
            c.w64(11, c.r64(0));
            c.w32(0, 0x6u64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fcd8b); }
            blk = 10; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fcd91); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fcd95); }
            let r40 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fcd9b); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fcda1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fcda5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fcda9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fcdad); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fcdb1); };
            c.cmp(t43, c.r32(2), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 24; continue;
        }
        24 => {
            let r44 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r44);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fcdc0); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008fcdcd); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x70008fcdd2); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t45) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fcddc); };
            c.w32(11, t45);
            c.w32(0, c.r32(11));
            let r46 = c.r32(0) | 0xeu64;
            c.w32(0, r46);
            c.logic(r46, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fcdef); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fcdf4); }
            blk = 27; continue;
        }
        27 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fcdfc); };
            c.w64(8, t47);
            let Some(t48) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008fce03); };
            c.w64(11, t48);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fce0a); };
            c.w64(3, t49);
            let Some(t50) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fce11); };
            c.cmp(t50, c.r64(0), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fce20, 0x70008fce1b);
        }
        29 => {
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008fce20); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008fce25); }
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008fce2c); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 31; continue;
        }
        31 => {
            c.arith_sd(3, 0, c.xmm_u64(1));
            c.mov128_rr(0, 0);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fce3d); };
            c.w64(7, t51);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008fce44); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fce47); }
            c.w64(11, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fce4f); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fce53); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fce57); };
            c.w64(7, t54);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fce5e); }
            blk = 13; continue;
        }
        32 => {
            let Some(t55) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fce64); };
            c.w32(2, t55);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 34; continue;
        }
        34 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t56) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fce81); };
            c.w32(2, t56);
            let r57 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r57);
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fce87); };
            c.cmp(t58, c.r32(2), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0x1eu64);
            blk = 48; continue;
        }
        37 => {
            let Some(t59) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fce9a); };
            c.cmp(t59, 0x43u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0x1eu64);
            blk = 48; continue;
        }
        39 => {
            c.w32(2, 0x1eu64);
            blk = 48; continue;
        }
        40 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fcebe); };
            c.w32(2, t60);
            let r61 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r61);
            let Some(t62) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fcec4); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fced0); };
            c.w32(2, t63);
            c.w32(6, c.r32(2));
            let r64 = c.r32(6) | 0x40u64;
            c.w32(6, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70008fcee1); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fcee6); }
            blk = 45; continue;
        }
        45 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fceee); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fcef2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008fcef3); }
            c.w32(8, 0x6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fcefa); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fcefc); };
            c.w64(3, t65);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008fcf10, 0x70008fcf0b);
        }
        46 => {
            c.w64(11, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fcf13); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fcf17); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fcf1b); };
            c.w64(7, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fcf22); }
            blk = 13; continue;
        }
        47 => {
            c.w32(2, 0xfeu64);
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fcf32); };
            c.w32(6, t69);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r70 = c.r32(2) | c.r32(11);
            c.w32(2, r70);
            c.logic(r70, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fcf46); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fcf4b); }
            blk = 51; continue;
        }
        51 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fcf53); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fcf57); };
            c.w64(6, t71);
            return c.call(0x70008adf94, 0x70008fcf60, 0x70008fcf5b);
        }
        52 => {
            c.w64(11, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fcf63); };
            c.w64(15, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fcf67); };
            c.w64(9, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fcf6b); };
            c.w64(7, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fcf72); }
            blk = 13; continue;
        }
        53 => {
            return RegfileExit::jump(0x70008fcf7c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fcf94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fcf97); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fcf9b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fcfa2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fcfa6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fcfad); };
            c.w32(3, (t6 as u8 as i8 as i64 as u64));
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fcfb3); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fcfb9); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fcfbd); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fcfc6); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fcfcf); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 3; continue;
        }
        3 => {
            let r13 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r13);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 4; continue;
        }
        4 => {
            let r14 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r14);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cdq();
            c.idiv(c.r32(3), 32);
            c.w32(11, c.r64(2).wrapping_add(c.r64(2)));
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(0, 0xeu64);
            blk = 9; continue;
        }
        8 => {
            c.w32(0, 0x2u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd016); };
            c.w32(3, t15);
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(3));
            let r16 = c.r32(0) | c.r32(2);
            c.w32(0, r16);
            c.logic(r16, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fd028); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd02d); }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd035); };
            c.w64(7, t17);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fd040); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fd045); };
            c.w64(1, t19);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fd051); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fd055); }
            let r20 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r20);
            c.cdq();
            c.idiv(c.r32(3), 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fd05e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fd062); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fd066); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008fd06a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fd06e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008fd072); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008fd076); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(11, c.r64(2));
            let r21 = c.add(c.r32(11), c.r32(2), 32);
            c.w32(11, r21);
            if c.cc_o() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 21; continue;
        }
        16 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fd093); };
            c.w64(11, t22);
            let Some(t23) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fd09a); };
            c.w64(0, t23);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fd0a1); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fd0a8); };
            c.cmp(t25, c.r64(3), 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fd0bb, 0x70008fd0b6);
        }
        18 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fd0bb); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd0bf); };
            c.w64(15, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fd0c3); };
            c.w64(9, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd0c7); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fd0cb); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fd0cf); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fd0d3); };
            c.w64(6, t32);
            blk = 20; continue;
        }
        19 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008fd0d9); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 20; continue;
        }
        20 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd0eb); };
            c.w64(11, t33);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008fd0f2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fd0f6); }
            blk = 21; continue;
        }
        21 => {
            c.w64(11, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fd101); }
            blk = 6; continue;
        }
        22 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70008fd107); };
            c.w64(11, t34);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fd111); }
            blk = 6; continue;
        }
        23 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x70008fd117); };
            c.w64(11, t35);
            blk = 6; continue;
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fd123); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fd127); }
            let r36 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r36);
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd12d); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fd133); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fd137); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fd13b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fd13f); }
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd143); };
            c.cmp(t39, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 25; continue;
        }
        25 => {
            let r40 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r40);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fd152); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008fd157); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008fd15b); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd165); };
            c.w32(11, t41);
            c.w32(0, c.r32(11));
            let r42 = c.r32(0) | 0xeu64;
            c.w32(0, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fd178); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd17d); }
            blk = 28; continue;
        }
        28 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fd185); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008fd18c); };
            c.w64(11, t44);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fd193); };
            c.w64(2, t45);
            let Some(t46) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008fd19a); };
            c.cmp(t46, c.r64(0), 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fd1a9, 0x70008fd1a4);
        }
        30 => {
            c.w64(8, c.r64(0));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd1ac); };
            c.w64(15, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fd1b0); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd1b4); };
            c.w64(7, t49);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008fd1b8); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fd1bd); };
            c.w64(3, t50);
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008fd1c3); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 32; continue;
        }
        32 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            let r51 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r51);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x70008fd1d6); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x70008fd1db); }
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fd1de); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x70008fd1e3); }
            blk = 33; continue;
        }
        33 => {
            c.fprem();
            c.fnstsw_ax();
            let r52 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r52);
            let r53 = c.r32(0) & 0xffu64;
            c.w32(0, r53);
            c.logic(r53, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fd1f2); }
            if !c.popfq() { return RegfileExit::fault(0x70008fd1f3); }
            if c.cc_p() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x70008fd1f8); }
            if !c.movsd_rm(0, c.r64(4)) { return RegfileExit::fault(0x70008fd1fb); }
            let r54 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd204); };
            c.w64(11, t55);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008fd20b); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fd20f); }
            c.w64(11, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fd21b); }
            blk = 12; continue;
        }
        35 => {
            let Some(t56) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fd221); };
            c.w32(2, t56);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t57) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd23e); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd244); };
            c.cmp(t59, c.r32(2), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(2, 0x1eu64);
            blk = 51; continue;
        }
        40 => {
            let Some(t60) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fd257); };
            c.cmp(t60, 0x43u64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(2, 0x1eu64);
            blk = 51; continue;
        }
        42 => {
            c.w32(2, 0x1eu64);
            blk = 51; continue;
        }
        43 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t61) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd27b); };
            c.w32(2, t61);
            let r62 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r62);
            let Some(t63) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fd281); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd28d); };
            c.w32(2, t64);
            c.w32(6, c.r32(2));
            let r65 = c.r32(6) | 0x40u64;
            c.w32(6, r65);
            c.logic(r65, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70008fd29e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd2a3); }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fd2ab); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fd2af); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008fd2b0); }
            c.w32(8, 0x8u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fd2b7); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fd2b9); };
            c.w64(3, t66);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008fd2cd, 0x70008fd2c8);
        }
        49 => {
            c.w64(11, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd2d0); };
            c.w64(15, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fd2d4); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd2d8); };
            c.w64(7, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fd2df); }
            blk = 12; continue;
        }
        50 => {
            c.w32(2, 0xfeu64);
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd2ef); };
            c.w32(6, t70);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r71 = c.r32(2) | c.r32(11);
            c.w32(2, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fd303); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd308); }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fd310); }
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fd314); };
            c.w64(6, t72);
            return c.call(0x70008ae1b4, 0x70008fd31d, 0x70008fd318);
        }
        55 => {
            c.w64(11, c.r64(0));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd320); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fd324); };
            c.w64(9, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd328); };
            c.w64(7, t75);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008fd32f); }
            blk = 12; continue;
        }
        56 => {
            return RegfileExit::jump(0x70008fd338);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fd354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fd354); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fd358); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fd35e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd362); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fd366); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fd36d); };
            c.w64(11, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fd371); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fd378); };
            c.w32(3, (t8 as u8 as i8 as i64 as u64));
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fd37e); };
            c.w32(2, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd384); };
            c.w32(1, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fd388); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008fd38c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fd390); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fd394); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008fd398); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fd39c); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fd3a5); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            let r13 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fd3ae); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008fd3bb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008fd3bf); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fd3cb); };
            c.w64(11, t15);
            let Some(t16) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008fd3d2); };
            c.w64(6, t16);
            let r17 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r17);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fd3db); };
            c.w64(0, t18);
            let Some(t19) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008fd3e2); };
            c.cmp(t19, c.r64(12), 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x70008fd3eb); }
            c.w64(11, c.r64(6).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fd3f2); };
            c.w64(0, t20);
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd408); };
            c.w64(0, t22);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008fd40f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fd413); }
            c.w64(10, c.r64(4));
            let r23 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r23);
            let r24 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r24);
            c.logic(r24, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008fd422); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70008fd427); };
            c.w64(0, t25);
            c.w64(10, 0x70008fd435u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008fd435); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008fd439); }
            return c.call(c.r64(0), 0x70008fd43f, 0x70008fd43d);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008fd43f); }
            let Some(t26) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008fd447); };
            c.w64(4, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fd44c); };
            c.w64(0, t27);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fd450); }
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fd455); };
            c.cmp(t28, 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fd45b); };
            c.w64(8, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fd45f); };
            c.w64(7, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd463); };
            c.w32(11, t31);
            c.w32(3, c.r32(11));
            let r32 = c.r32(3) | 0xeu64;
            c.w32(3, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fd476); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd47b); }
            blk = 9; continue;
        }
        9 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd483); };
            c.w64(7, t33);
            let Some(t34) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd487); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fd48b); };
            c.w64(8, t35);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t36) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fd493); };
            c.w32(8, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd498); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fd49c); };
            c.w64(1, t38);
            let Some(t39) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fd4a0); };
            c.w64(5, t39);
            c.w64(12, c.r64(7));
            let r40 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r40);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008fd4b2); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008fd4be, 0x70008fd4b9);
        }
        11 => {
            c.w64(11, c.r64(0));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008fd4c1); };
            c.w64(3, t41);
            blk = 5; continue;
        }
        12 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd4ca); };
            c.w32(6, t42);
            let r43 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fd4d0); }
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd4d4); };
            c.cmp(t44, c.r32(6), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 13; continue;
        }
        13 => {
            let r45 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r45);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fd4e3); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x70008fd4f0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x70008fd4f5); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd4ff); };
            c.w32(11, t46);
            c.w32(0, c.r32(11));
            let r47 = c.r32(0) | 0xeu64;
            c.w32(0, r47);
            c.logic(r47, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fd512); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd517); }
            blk = 16; continue;
        }
        16 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fd51f); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008fd526); };
            c.w64(11, t49);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fd52d); };
            c.w64(3, t50);
            let Some(t51) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008fd534); };
            c.cmp(t51, c.r64(0), 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008fd543, 0x70008fd53e);
        }
        18 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x70008fd543); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008fd548); }
            blk = 20; continue;
        }
        19 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008fd54f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        20 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd559); };
            c.w64(7, t52);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008fd560); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008fd563); }
            c.w64(10, c.r64(4));
            let r53 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r53);
            let r54 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r54);
            c.logic(r54, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008fd572); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70008fd577); };
            c.w64(0, t55);
            c.w64(10, 0x70008fd585u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008fd585); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008fd589); }
            return c.call(c.r64(0), 0x70008fd58f, 0x70008fd58d);
        }
        21 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008fd58f); }
            let Some(t56) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008fd597); };
            c.w64(4, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fd59c); };
            c.w64(0, t57);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fd5a0); }
            blk = 9; continue;
        }
        22 => {
            let Some(t58) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fd5aa); };
            c.w32(6, t58);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t59) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd5c7); };
            c.w32(6, t59);
            let r60 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r60);
            let Some(t61) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd5cd); };
            c.cmp(t61, c.r32(6), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(6, 0x1eu64);
            blk = 38; continue;
        }
        27 => {
            let Some(t62) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fd5e0); };
            c.cmp(t62, 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(6, 0x1eu64);
            blk = 38; continue;
        }
        29 => {
            c.w32(6, 0x1eu64);
            blk = 38; continue;
        }
        30 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t63) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd5fe); };
            c.w32(6, t63);
            let r64 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r64);
            let Some(t65) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fd604); };
            c.cmp(t65, 0x41u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd610); };
            c.w32(1, t66);
            c.w32(6, c.r32(1));
            let r67 = c.r32(6) | 0x40u64;
            c.w32(6, r67);
            c.logic(r67, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70008fd621); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd626); }
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fd62e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008fd632); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008fd633); }
            c.w32(8, 0xau64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008fd63a); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70008fd63c); };
            c.w64(3, t68);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008fd650, 0x70008fd64b);
        }
        36 => {
            blk = 9; continue;
        }
        37 => {
            c.w32(6, 0xfeu64);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd65f); };
            c.w32(1, t69);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r70 = c.r32(6) | c.r32(11);
            c.w32(6, r70);
            c.logic(r70, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70008fd673); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd678); }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008fd680); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fd684); };
            c.w64(6, t71);
            return c.call(0x70008ae454, 0x70008fd68d, 0x70008fd688);
        }
        42 => {
            blk = 9; continue;
        }
        43 => {
            return RegfileExit::jump(0x70008fd694);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fd6b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fd6b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fd6b8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fd6be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd6c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fd6c6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fd6cd); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fd6d4); };
            c.w32(11, (t7 as u8 as i8 as i64 as u64));
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fd6da); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd6e0); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fd6e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fd6e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fd6ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fd6f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008fd6f4); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fd6f8); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fd701); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fd708); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fd70c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fd719); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fd71d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fd723); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd72b); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd732); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fd73f); };
            c.w32(12, t17);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(6, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008fd757); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x70008fd763, 0x70008fd75e);
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fd763); };
            c.w64(3, t18);
            c.w32(6, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd76c); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fd770); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd774); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fd778); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fd77c); };
            c.w64(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fd780); };
            c.w64(2, t24);
            blk = 3; continue;
        }
        9 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fd786); };
            c.w32(0, t25);
            let r26 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r26);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r27 = c.r32(2) | 0x2u64;
            c.w32(2, r27);
            c.logic(r27, 32);
            let r28 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r28);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(1, c.r64(9));
            c.w32(11, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t29) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fd7a6); };
            c.cmp(t29, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd7ae); };
            c.w32(1, t30);
            let r31 = c.r32(0) | 0x40u64;
            c.w32(0, r31);
            c.logic(r31, 32);
            c.w32(6, c.r32(1));
            let r32 = c.r32(0) | c.r32(6);
            c.w32(0, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fd7c1); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd7c6); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fd7ce); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fd7d7); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008fd7d8); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008fd7ec, 0x70008fd7e7);
        }
        15 => {
            return RegfileExit::fault(0x70008fd7ec);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fd7ed); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r34 = c.r32(2) | 0xeu64;
            c.w32(2, r34);
            c.logic(r34, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r35 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r35);
            let r36 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r36);
            let r37 = c.r32(11) | c.r32(0);
            c.w32(11, r37);
            c.logic(r37, 32);
            c.w64(0, c.r64(11));
            let r38 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r38);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, 0xeu64);
            blk = 23; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            blk = 23; continue;
        }
        23 => {
            let Some(t39) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fd831); };
            c.cmp(t39, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fd837); };
            c.w32(1, t40);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r41 = c.r32(2) | c.r32(0);
            c.w32(2, r41);
            c.logic(r41, 32);
            let r42 = c.r32(2) | c.r32(6);
            c.w32(2, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fd84d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fd852); }
            blk = 26; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd85a); };
            c.w64(7, t43);
            let r44 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fd862); };
            c.w32(8, t45);
            let Some(t46) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fd867); };
            c.w64(1, t46);
            let Some(t47) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fd86b); };
            c.w64(5, t47);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r48 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r48);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fd87b); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008fd882); };
            c.w64(1, t50);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fd889); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fd88d); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fd891); };
            c.w64(12, t51);
            let Some(t52) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fd898); };
            c.cmp(t52, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70008fd8ab, 0x70008fd8a6);
        }
        29 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fd8ab); };
            c.w64(3, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd8af); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fd8b3); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd8b7); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fd8bb); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fd8bf); };
            c.w64(2, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fd8c3); };
            c.w64(11, t59);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70008fd8c9); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd8d0); };
            c.w64(1, t60);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008fd8d7); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fd8e3); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r61 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fd8f4); }
            return c.call(0x70008708b4, 0x70008fd8fe, 0x70008fd8f9);
        }
        33 => {
            let Some(t62) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fd8fe); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x70008fd90c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008fd914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008fd914); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008fd918); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fd91e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd922); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008fd926); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008fd92d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008fd934); };
            c.w32(11, (t7 as u8 as i8 as i64 as u64));
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008fd93a); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd940); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008fd944); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008fd948); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008fd94c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fd950); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008fd954); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008fd958); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fd961); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008fd968); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008fd96c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008fd979); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fd97d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008fd983); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008fd98b); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fd992); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008fd99f); };
            c.w32(12, t17);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(6, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008fd9b7); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x70008fd9c3, 0x70008fd9be);
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fd9c3); };
            c.w64(3, t18);
            c.w32(6, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fd9cc); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fd9d0); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fd9d4); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fd9d8); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fd9dc); };
            c.w64(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fd9e0); };
            c.w64(2, t24);
            blk = 3; continue;
        }
        9 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008fd9e6); };
            c.w32(0, t25);
            let r26 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r26);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r27 = c.r32(2) | 0x2u64;
            c.w32(2, r27);
            c.logic(r27, 32);
            let r28 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r28);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(1, c.r64(9));
            c.w32(11, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t29) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fda06); };
            c.cmp(t29, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fda0e); };
            c.w32(1, t30);
            let r31 = c.r32(0) | 0x40u64;
            c.w32(0, r31);
            c.logic(r31, 32);
            c.w32(6, c.r32(1));
            let r32 = c.r32(0) | c.r32(6);
            c.w32(0, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fda21); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fda26); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008fda2e); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008fda37); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008fda38); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008fda4c, 0x70008fda47);
        }
        15 => {
            return RegfileExit::fault(0x70008fda4c);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008fda4d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r34 = c.r32(2) | 0xeu64;
            c.w32(2, r34);
            c.logic(r34, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r35 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r35);
            let r36 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r36);
            let r37 = c.r32(11) ^ c.r32(0);
            c.w32(11, r37);
            c.logic(r37, 32);
            c.w64(0, c.r64(11));
            let r38 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r38);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, 0xeu64);
            blk = 23; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            blk = 23; continue;
        }
        23 => {
            let Some(t39) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008fda91); };
            c.cmp(t39, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008fda97); };
            c.w32(1, t40);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r41 = c.r32(2) | c.r32(0);
            c.w32(2, r41);
            c.logic(r41, 32);
            let r42 = c.r32(2) | c.r32(6);
            c.w32(2, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70008fdaad); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70008fdab2); }
            blk = 26; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fdaba); };
            c.w64(7, t43);
            let r44 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008fdac2); };
            c.w32(8, t45);
            let Some(t46) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008fdac7); };
            c.w64(1, t46);
            let Some(t47) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008fdacb); };
            c.w64(5, t47);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r48 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r48);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008fdadb); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008fdae2); };
            c.w64(1, t50);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008fdae9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008fdaed); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008fdaf1); };
            c.w64(12, t51);
            let Some(t52) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008fdaf8); };
            c.cmp(t52, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70008fdb0b, 0x70008fdb06);
        }
        29 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008fdb0b); };
            c.w64(3, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008fdb0f); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008fdb13); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008fdb17); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008fdb1b); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008fdb1f); };
            c.w64(2, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008fdb23); };
            c.w64(11, t59);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70008fdb29); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008fdb30); };
            c.w64(1, t60);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008fdb37); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008fdb43); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r61 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r61);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008fdb54); }
            return c.call(0x70008708b4, 0x70008fdb5e, 0x70008fdb59);
        }
        33 => {
            let Some(t62) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008fdb5e); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x70008fdb6c);
        }
            _ => unreachable!(),
        }
    }
}
