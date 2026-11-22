#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70007fa8f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.st32(c.r64(2).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x70007fa8f4); }
            c.w64(7, c.r64(2));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(7).wrapping_add(0x17));
            let r0 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r0);
            c.logic(r0, 64);
            let Some(t1) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fa90c); };
            c.logic(t1 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r2 = c.r64(0) & c.r64(7);
            c.w64(0, r2);
            c.logic(r2, 64);
            let Some(t3) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fa91c); };
            c.logic(t3 & 0x4u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return c.call(0x70007f1594, 0x70007fa927, 0x70007fa922);
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fa927); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fa92d); };
            c.w32(6, t6);
            let r7 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2e28)) else { return RegfileExit::fault(0x70007fa933); };
            c.w64(10, t8);
            let Some(t9) = c.ld8(c.r64(10)) else { return RegfileExit::fault(0x70007fa93a); };
            c.cmp(t9, 0x0u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2e40)) else { return RegfileExit::fault(0x70007fa944); };
            c.w64(10, t10);
            let Some(t11) = c.ld64(c.r64(10)) else { return RegfileExit::fault(0x70007fa94b); };
            c.cmp(c.r64(2), t11, 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fa954); };
            c.cmp(c.r64(4), t12, 64);
            if c.cc_b() { blk = 18; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fa95e); }
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fa95f); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld16(c.r64(1).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007fa965); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007fa969); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            blk = 8; continue;
        }
        8 => {
            let r18 = c.dec(c.r64(1), 64);
            c.w64(1, r18);
            if c.cc_l() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fa974); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fa97c); }
            blk = 8; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fa980); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fa987); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fa989); }
            let Some(t23) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fa98a); };
            c.w32(0, t23);
            let r24 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r24);
            let Some(t25) = c.ld16(c.r64(0).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007fa990); };
            c.w32(0, t25);
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fa994); };
            c.w32(1, t26);
            let r27 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r27);
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fa99a); };
            c.logic(t28 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let r29 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r29);
            blk = 13; continue;
        }
        12 => {
            let Some(t30) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fa9b0); };
            c.w32(1, t30);
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fa9b3); };
            c.w64(1, t31);
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa9bd); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fa9c1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fa9c3); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fa9c4); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007fa9c5); };
            if !c.push64(t32) { return RegfileExit::fault(0x70007fa9c5); }
            c.w32(0, 0x2u64);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1ec0)) else { return RegfileExit::fault(0x70007fa9d1); };
            c.w64(3, t33);
            return c.call(0x700086ed74, 0x70007fa9dd, 0x70007fa9d8);
        }
        15 => {
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fa9dd); }
            let Some(t34) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fa9de); };
            c.w32(7, t34);
            let r35 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r35);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fa9e7); }
            blk = 6; continue;
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa9ed); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fa9f1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fa9f3); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1ed0)) else { return RegfileExit::fault(0x70007fa9f6); };
            c.w64(3, t36);
            return c.call(0x700086ed74, 0x70007faa02, 0x70007fa9fd);
        }
        17 => {
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007faa02); }
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007faa03); };
            c.w32(7, t37);
            let r38 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007faa0c); }
            blk = 6; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007faa12); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007faa16); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007faa1a); };
            c.w64(3, t39);
            return c.call(0x700086ed74, 0x70007faa26, 0x70007faa21);
        }
        19 => {
            return RegfileExit::fault(0x70007faa26);
        }
        20 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007faa2a); }
            return RegfileExit::fault(0x70007faa2b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007faa34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007faa34); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faa3a); };
            c.w32(7, t2);
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faa3d); };
            c.w32(1, t3);
            c.logic(c.r8(1) & 0x7u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0));
            c.w64(9, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            blk = 2; continue;
        }
        2 => {
            c.w64(11, c.r64(9));
            let r4 = c.r32(11) & 0x7u64;
            c.w32(11, r4);
            c.logic(r4, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 49; continue;
        }
        7 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 49; continue;
        }
        10 => {
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007faa8e); };
            c.w32(9, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007faa92); };
            c.cmp(t6, c.r32(9), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007faa9f); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007faaa6); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faaad); };
            c.w32(9, t11);
            blk = 2; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007faab4); };
            c.w32(9, t12);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r13 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007faac1); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007faac8); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            let Some(t18) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faacf); };
            c.w32(9, t18);
            blk = 2; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007faad9); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007faae0); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faae7); };
            c.w32(9, t23);
            blk = 2; continue;
        }
        14 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 17; continue;
        }
        15 => {
            c.w32(11, 0x1u64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 17; continue;
        }
        16 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(6));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 18; continue;
        }
        18 => {
            let r24 = c.add(c.r64(9), 0xbu64, 64);
            c.w64(9, r24);
            blk = 19; continue;
        }
        19 => {
            let r25 = c.add(c.r64(8), c.r64(3), 64);
            c.w64(8, r25);
            let r26 = c.r32(1) & 0x8u64;
            c.w32(1, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t27) = c.ld16(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fab46); };
            c.w32(1, t27);
            blk = 22; continue;
        }
        21 => {
            let Some(t28) = c.ld8(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fab4d); };
            c.w32(1, t28);
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(1));
            let r29 = c.r32(8) & 0xfc00u64;
            c.w32(8, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(8), 0xd800u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ge() { blk = 48; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t30) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fab76); };
            c.w32(7, t30);
            let r31 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r31);
            let Some(t32) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fab7c); };
            c.w32(7, t32);
            c.logic(c.r8(7) & 0x7u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(6));
            c.w64(9, c.r64(0));
            blk = 26; continue;
        }
        26 => {
            c.w64(12, c.r64(11));
            let r33 = c.r32(12) & 0x7u64;
            c.w32(12, r33);
            c.logic(r33, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 66; continue;
        }
        31 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 66; continue;
        }
        34 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fabcc); };
            c.w32(11, t34);
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fabd0); };
            c.cmp(t35, c.r32(11), 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t36) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fabdd); };
            c.w32(9, t36);
            let r37 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r37);
            let Some(t38) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fabe4); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            let Some(t40) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fabeb); };
            c.w32(11, t40);
            blk = 26; continue;
        }
        36 => {
            let Some(t41) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fabf2); };
            c.w32(11, t41);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r42 = c.add(c.r64(7), c.r64(11), 64);
            c.w64(7, r42);
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fabff); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fac06); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fac0d); };
            c.w32(11, t47);
            blk = 26; continue;
        }
        37 => {
            let Some(t48) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fac17); };
            c.w32(9, t48);
            let r49 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r49);
            let Some(t50) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fac1e); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            let Some(t52) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fac25); };
            c.w32(11, t52);
            blk = 26; continue;
        }
        38 => {
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 41; continue;
        }
        39 => {
            c.w32(12, 0x1u64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 41; continue;
        }
        40 => {
            c.w64(9, c.r64(6));
            c.w64(11, c.r64(0));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 42; continue;
        }
        42 => {
            let r53 = c.add(c.r64(11), 0xbu64, 64);
            c.w64(11, r53);
            blk = 43; continue;
        }
        43 => {
            let r54 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r54);
            let r55 = c.r32(7) & 0x8u64;
            c.w32(7, r55);
            c.logic(r55, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t56) = c.ld16(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fac82); };
            c.w32(7, t56);
            blk = 46; continue;
        }
        45 => {
            let Some(t57) = c.ld8(c.r64(8).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70007fac89); };
            c.w32(7, t57);
            blk = 46; continue;
        }
        46 => {
            c.w64(8, c.r64(7));
            let r58 = c.r32(8) & 0xfc00u64;
            c.w32(8, r58);
            c.logic(r58, 32);
            c.cmp(c.r32(8), 0xdc00u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let r59 = c.shl(c.r32(1), 0xau64, 32).unwrap_or(c.r32(1));
            c.w32(1, r59);
            c.w32(1, c.r64(1).wrapping_add(c.r64(7)).wrapping_sub(0x35fdc00));
            blk = 48; continue;
        }
        48 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            return c.ret(0x0, 0x70007facae);
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007facaf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007facb3); }
            let r60 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r60);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70007facb9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70007facbd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70007facc1); }
            blk = 53; continue;
        }
        50 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007facc7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007faccb); }
            let r61 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70007facd1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70007facd5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70007facd9); }
            blk = 53; continue;
        }
        51 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007facdf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007face3); }
            let r62 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r62);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70007face9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70007faced); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70007facf1); }
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t63) = c.ld64(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007facfa); };
            c.w64(9, t63);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fad01); }
            blk = 19; continue;
        }
        53 => {
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(1, c.r64(3).wrapping_add(c.r64(3)));
            blk = 64; continue;
        }
        55 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fad15); };
            c.w64(1, t64);
            let Some(t65) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007fad1c); };
            c.w64(8, t65);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fad23); };
            c.w64(11, t66);
            let Some(t67) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007fad2a); };
            c.cmp(t67, c.r64(9), 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70007fad39, 0x70007fad34);
        }
        57 => {
            c.w64(1, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fad3c); };
            c.w64(0, t68);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fad42); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fad46); };
            c.w64(3, t70);
            blk = 59; continue;
        }
        58 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70007fad4c); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 59; continue;
        }
        59 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 63; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            let r71 = c.r64(10) | 0x1u64;
            c.w64(10, r71);
            c.logic(r71, 64);
            blk = 62; continue;
        }
        62 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 63; continue;
        }
        63 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fad7a); };
            c.w64(8, t72);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70007fad81); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007fad85); }
            blk = 64; continue;
        }
        64 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fad8a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fad8b); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70007fad8c); };
            c.w64(1, t73);
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70007fada3, 0x70007fad9e);
        }
        65 => {
            let r74 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r74);
            c.w64(1, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fada8); };
            c.w64(0, t75);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fadae); };
            c.w64(7, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fadb2); };
            c.w64(3, t77);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fadb9); }
            blk = 22; continue;
        }
        66 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fadbf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fadc3); }
            let r78 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r78);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007fadc9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70007fadcd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fadd1); }
            blk = 70; continue;
        }
        67 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fadd7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007faddb); }
            let r79 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r79);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007fade1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70007fade5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fade9); }
            blk = 70; continue;
        }
        68 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fadef); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fadf3); }
            let r80 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r80);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007fadf9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70007fadfd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fae01); }
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t81) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fae0b); };
            c.w64(11, t81);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fae12); }
            blk = 43; continue;
        }
        70 => {
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w32(7, c.r64(8).wrapping_add(c.r64(8)));
            blk = 81; continue;
        }
        72 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fae27); };
            c.w64(7, t82);
            let Some(t83) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fae2e); };
            c.w64(9, t83);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fae35); };
            c.w64(12, t84);
            let Some(t85) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007fae3c); };
            c.cmp(t85, c.r64(11), 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70007fae4c, 0x70007fae47);
        }
        74 => {
            c.w64(7, c.r64(0));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fae4f); };
            c.w64(1, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fae53); };
            c.w64(0, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fae57); };
            c.w64(8, t88);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 76; continue;
        }
        75 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70007fae5f); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 76; continue;
        }
        76 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fae66); };
            c.w64(9, t89);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70007fae6d); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(8) as i64));
            c.logic(c.r64(8) & c.r64(8), 64);
            if c.cc_ns() { blk = 80; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(10, c.r64(8));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let r90 = c.r64(10) | 0x1u64;
            c.w64(10, r90);
            c.logic(r90, 64);
            blk = 79; continue;
        }
        79 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 80; continue;
        }
        80 => {
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007fae98); }
            blk = 81; continue;
        }
        81 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fae9d); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fae9e); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70007fae9f); };
            c.w64(3, t91);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007faeb0, 0x70007faeab);
        }
        82 => {
            let r92 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r92);
            c.w64(7, c.r64(0));
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007faeb5); };
            c.w64(1, t93);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007faebc); }
            blk = 46; continue;
        }
        83 => {
            return RegfileExit::jump(0x70007faec4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007faed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007faed4); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faeda); };
            c.w32(7, t2);
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faedd); };
            c.w32(1, t3);
            c.logic(c.r8(1) & 0x7u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0));
            c.w64(9, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            blk = 2; continue;
        }
        2 => {
            c.w64(11, c.r64(9));
            let r4 = c.r32(11) & 0x7u64;
            c.w32(11, r4);
            c.logic(r4, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 55; continue;
        }
        7 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 55; continue;
        }
        10 => {
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007faf2e); };
            c.w32(9, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007faf32); };
            c.cmp(t6, c.r32(9), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007faf3f); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007faf46); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faf4d); };
            c.w32(9, t11);
            blk = 2; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007faf54); };
            c.w32(9, t12);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r13 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007faf61); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007faf68); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            let Some(t18) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faf6f); };
            c.w32(9, t18);
            blk = 2; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007faf79); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007faf80); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007faf87); };
            c.w32(9, t23);
            blk = 2; continue;
        }
        14 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 17; continue;
        }
        15 => {
            c.w32(11, 0x1u64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 17; continue;
        }
        16 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(6));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 18; continue;
        }
        18 => {
            let r24 = c.add(c.r64(9), 0xbu64, 64);
            c.w64(9, r24);
            blk = 19; continue;
        }
        19 => {
            let r25 = c.add(c.r64(8), c.r64(3), 64);
            c.w64(8, r25);
            let r26 = c.r32(1) & 0x8u64;
            c.w32(1, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t27) = c.ld16(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fafe6); };
            c.w32(1, t27);
            blk = 22; continue;
        }
        21 => {
            let Some(t28) = c.ld8(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fafed); };
            c.w32(1, t28);
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(1));
            let r29 = c.r32(8) & 0xfc00u64;
            c.w32(8, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(8), 0xd800u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ge() { blk = 48; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t30) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb016); };
            c.w32(7, t30);
            let r31 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r31);
            let Some(t32) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb01c); };
            c.w32(7, t32);
            c.logic(c.r8(7) & 0x7u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(6));
            c.w64(9, c.r64(0));
            blk = 26; continue;
        }
        26 => {
            c.w64(12, c.r64(11));
            let r33 = c.r32(12) & 0x7u64;
            c.w32(12, r33);
            c.logic(r33, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 72; continue;
        }
        31 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 72; continue;
        }
        34 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb06c); };
            c.w32(11, t34);
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb070); };
            c.cmp(t35, c.r32(11), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t36) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb07d); };
            c.w32(9, t36);
            let r37 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r37);
            let Some(t38) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb084); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            let Some(t40) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb08b); };
            c.w32(11, t40);
            blk = 26; continue;
        }
        36 => {
            let Some(t41) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb092); };
            c.w32(11, t41);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r42 = c.add(c.r64(7), c.r64(11), 64);
            c.w64(7, r42);
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb09f); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb0a6); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb0ad); };
            c.w32(11, t47);
            blk = 26; continue;
        }
        37 => {
            let Some(t48) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb0b7); };
            c.w32(9, t48);
            let r49 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r49);
            let Some(t50) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb0be); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            let Some(t52) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb0c5); };
            c.w32(11, t52);
            blk = 26; continue;
        }
        38 => {
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 41; continue;
        }
        39 => {
            c.w32(12, 0x1u64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 41; continue;
        }
        40 => {
            c.w64(9, c.r64(6));
            c.w64(11, c.r64(0));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 42; continue;
        }
        42 => {
            let r53 = c.add(c.r64(11), 0xbu64, 64);
            c.w64(11, r53);
            blk = 43; continue;
        }
        43 => {
            let r54 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r54);
            let r55 = c.r32(7) & 0x8u64;
            c.w32(7, r55);
            c.logic(r55, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t56) = c.ld16(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fb122); };
            c.w32(7, t56);
            blk = 46; continue;
        }
        45 => {
            let Some(t57) = c.ld8(c.r64(8).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70007fb129); };
            c.w32(7, t57);
            blk = 46; continue;
        }
        46 => {
            c.w64(8, c.r64(7));
            let r58 = c.r32(8) & 0xfc00u64;
            c.w32(8, r58);
            c.logic(r58, 32);
            c.cmp(c.r32(8), 0xdc00u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let r59 = c.shl(c.r32(7), 0x10u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r59);
            let r60 = c.r32(7) | c.r32(1);
            c.w32(7, r60);
            c.logic(r60, 32);
            c.w64(1, c.r64(7));
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(1), 0x10000u64, 32);
            if c.cc_b() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fb151); };
            c.w64(7, t61);
            let Some(t62) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fb158); };
            c.w64(8, t62);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fb15f); };
            c.w64(11, t63);
            let Some(t64) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007fb166); };
            c.cmp(t64, c.r64(9), 64);
            if c.cc_be() { blk = 89; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70007fb16f); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 51; continue;
        }
        51 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70007fb176); };
            c.w64(8, t65);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70007fb17d); }
            if !c.st32(c.r64(7).wrapping_add(0x7), 0x2u64) { return RegfileExit::fault(0x70007fb181); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fb188); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70007fb18f); }
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70007fb195);
        }
        52 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 91; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70007fb1aa); };
            c.w64(8, t66);
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb1b1); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r68);
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fb1b9); };
            c.cmp(t69, c.r32(0), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 54; continue;
        }
        54 => {
            return c.ret(0x0, 0x70007fb1c6);
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb1c7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb1cb); }
            let r70 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r70);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fb1d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70007fb1d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70007fb1d9); }
            blk = 59; continue;
        }
        56 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb1df); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb1e3); }
            let r71 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r71);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fb1e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70007fb1ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70007fb1f1); }
            blk = 59; continue;
        }
        57 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb1f7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb1fb); }
            let r72 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r72);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fb201); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70007fb205); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70007fb209); }
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t73) = c.ld64(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb212); };
            c.w64(9, t73);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fb219); }
            blk = 19; continue;
        }
        59 => {
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(1, c.r64(3).wrapping_add(c.r64(3)));
            blk = 70; continue;
        }
        61 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fb22d); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007fb234); };
            c.w64(8, t75);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fb23b); };
            c.w64(11, t76);
            let Some(t77) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007fb242); };
            c.cmp(t77, c.r64(9), 64);
            if c.cc_a() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70007fb251, 0x70007fb24c);
        }
        63 => {
            c.w64(1, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fb254); };
            c.w64(0, t78);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fb25a); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fb25e); };
            c.w64(3, t80);
            blk = 65; continue;
        }
        64 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70007fb264); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 65; continue;
        }
        65 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 69; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            let r81 = c.r64(10) | 0x1u64;
            c.w64(10, r81);
            c.logic(r81, 64);
            blk = 68; continue;
        }
        68 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 69; continue;
        }
        69 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fb292); };
            c.w64(8, t82);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70007fb299); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007fb29d); }
            blk = 70; continue;
        }
        70 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fb2a2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fb2a3); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70007fb2a4); };
            c.w64(1, t83);
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70007fb2bb, 0x70007fb2b6);
        }
        71 => {
            let r84 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r84);
            c.w64(1, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fb2c0); };
            c.w64(0, t85);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fb2c6); };
            c.w64(7, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fb2ca); };
            c.w64(3, t87);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fb2d1); }
            blk = 22; continue;
        }
        72 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb2d7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb2db); }
            let r88 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r88);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007fb2e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fb2e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fb2e9); }
            blk = 76; continue;
        }
        73 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb2ef); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb2f3); }
            let r89 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r89);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007fb2f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fb2fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fb301); }
            blk = 76; continue;
        }
        74 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb307); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb30b); }
            let r90 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r90);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007fb311); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fb315); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fb319); }
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t91) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb323); };
            c.w64(11, t91);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fb32a); }
            blk = 43; continue;
        }
        76 => {
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(7, c.r64(8).wrapping_add(c.r64(8)));
            blk = 87; continue;
        }
        78 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fb33f); };
            c.w64(7, t92);
            let Some(t93) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fb346); };
            c.w64(9, t93);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fb34d); };
            c.w64(12, t94);
            let Some(t95) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007fb354); };
            c.cmp(t95, c.r64(11), 64);
            if c.cc_a() { blk = 81; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70007fb364, 0x70007fb35f);
        }
        80 => {
            c.w64(7, c.r64(0));
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fb367); };
            c.w64(1, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fb36b); };
            c.w64(0, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fb36f); };
            c.w64(8, t98);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 82; continue;
        }
        81 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70007fb377); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 82; continue;
        }
        82 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fb37e); };
            c.w64(9, t99);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70007fb385); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(8) as i64));
            c.logic(c.r64(8) & c.r64(8), 64);
            if c.cc_ns() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w64(10, c.r64(8));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            let r100 = c.r64(10) | 0x1u64;
            c.w64(10, r100);
            c.logic(r100, 64);
            blk = 85; continue;
        }
        85 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 86; continue;
        }
        86 => {
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007fb3b0); }
            blk = 87; continue;
        }
        87 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fb3b5); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fb3b6); }
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70007fb3b7); };
            c.w64(3, t101);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007fb3c8, 0x70007fb3c3);
        }
        88 => {
            let r102 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r102);
            c.w64(7, c.r64(0));
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fb3cd); };
            c.w64(1, t103);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fb3d4); }
            blk = 46; continue;
        }
        89 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb3da); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb3de); }
            let r104 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r104);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007fb3e9); }
            return c.call(0x70007febf4, 0x70007fb3f2, 0x70007fb3ed);
        }
        90 => {
            c.w64(7, c.r64(0));
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fb3f5); };
            c.w64(1, t105);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fb3fc); }
            blk = 51; continue;
        }
        91 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb402); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb406); }
            let r106 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r106);
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fb40c); };
            c.w64(7, t107);
            let Some(t108) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fb413); };
            c.w64(8, t108);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007fb41a); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fb41e); };
            c.w64(11, t109);
            let Some(t110) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007fb425); };
            c.cmp(t110, c.r64(9), 64);
            if c.cc_a() { blk = 94; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70007fb434, 0x70007fb42f);
        }
        93 => {
            blk = 95; continue;
        }
        94 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70007fb436); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 95; continue;
        }
        95 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70007fb440); };
            c.w64(1, t111);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007fb447); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70007fb44a); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fb451); }
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fb458); };
            c.w64(1, t112);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x70007fb45c); }
            blk = 96; continue;
        }
        96 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fb463); }
            return c.ret(0x0, 0x70007fb464);
        }
        97 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fb465); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fb469); }
            let r113 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r113);
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fb46f); };
            c.w64(9, t114);
            let Some(t115) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007fb476); };
            c.w64(11, t115);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007fb47d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007fb481); }
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fb485); };
            c.w64(15, t116);
            let Some(t117) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007fb48c); };
            c.cmp(t117, c.r64(12), 64);
            if c.cc_a() { blk = 100; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70007fb49b, 0x70007fb496);
        }
        99 => {
            blk = 101; continue;
        }
        100 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70007fb49d); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 101; continue;
        }
        101 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70007fb4a7); };
            c.w64(1, t118);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007fb4ae); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70007fb4b1); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fb4b8); }
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fb4bf); };
            c.w64(1, t119);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70007fb4c3); }
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70007fb4c6); };
            c.w64(8, t120);
            let Some(t121) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fb4cd); };
            c.w32(7, t121);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            return RegfileExit::fault(0x70007fb4dc);
        }
        103 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fb4dd); };
            c.w64(7, t122);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70007fb4e1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r123 = c.r64(1) & c.r64(8);
            c.w64(1, r123);
            c.logic(r123, 64);
            let Some(t124) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fb4f3); };
            c.logic(t124 & 0x4u64, 8);
            if c.cc_ne() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 96; continue;
        }
        106 => {
            c.w32(0, c.r32(0));
            let r125 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r125);
            c.w64(1, 0xfffffffffffc0000u64);
            let r126 = c.r64(1) & c.r64(0);
            c.w64(1, r126);
            c.logic(r126, 64);
            let Some(t127) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fb50d); };
            c.logic(t127 & 0x2u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fb517); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fb518); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70007fb525, 0x70007fb520);
        }
        108 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fb525); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fb527); }
            blk = 105; continue;
        }
        109 => {
            return RegfileExit::jump(0x70007fb52c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fb534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb534); };
            c.w32(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb537); };
            c.w32(7, t1);
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ne() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb543); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb549); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb550); };
            c.w32(7, t6);
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb554); };
            c.w32(8, t7);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(9, c.r64(8));
            let r8 = c.shl(c.r32(9), 0x8u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r8);
            let r9 = c.r32(9) | c.r32(7);
            c.w32(9, r9);
            c.logic(r9, 32);
            c.logic(c.r16(9) & 0x2020u64, 16);
            if c.cc_e() { blk = 52; continue; }
            blk = 3; continue;
        }
        3 => {
            let r10 = c.r32(7) & 0x7u64;
            c.w32(7, r10);
            c.logic(r10, 32);
            let r11 = c.r32(8) & 0x7u64;
            c.w32(8, r11);
            c.logic(r11, 32);
            c.logic(c.r16(9) & 0x1111u64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 4; continue;
        }
        4 => {
            let r12 = c.r32(9) & 0x808u64;
            c.w32(9, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(8), 0x0u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(9), 0x808u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb5ca); };
            c.w64(2, t13);
            blk = 10; continue;
        }
        9 => {
            let r14 = c.add(c.r64(2), 0xbu64, 64);
            c.w64(2, r14);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb5da); };
            c.w64(7, t15);
            blk = 13; continue;
        }
        12 => {
            c.w64(7, c.r64(0).wrapping_add(0xb));
            blk = 13; continue;
        }
        13 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        14 => {
            let r16 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r16);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_e() { blk = 51; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t17) = c.ld16(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fb5f6); };
            c.w32(8, t17);
            let Some(t18) = c.ld8(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fb5fb); };
            c.w32(11, t18);
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 14; continue;
        }
        18 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t19) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb610); };
            c.w64(2, t19);
            blk = 21; continue;
        }
        20 => {
            let r20 = c.add(c.r64(2), 0xbu64, 64);
            c.w64(2, r20);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t21) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb620); };
            c.w64(7, t21);
            blk = 24; continue;
        }
        23 => {
            c.w64(7, c.r64(0).wrapping_add(0xb));
            blk = 24; continue;
        }
        24 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        25 => {
            let r22 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r22);
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_e() { blk = 51; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld8(c.r64(2).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fb63c); };
            c.w32(8, t23);
            let Some(t24) = c.ld16(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fb641); };
            c.w32(11, t24);
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 25; continue;
        }
        29 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t25) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb656); };
            c.w64(2, t25);
            blk = 32; continue;
        }
        31 => {
            let r26 = c.add(c.r64(2), 0xbu64, 64);
            c.w64(2, r26);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t27) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb666); };
            c.w64(7, t27);
            blk = 35; continue;
        }
        34 => {
            c.w64(7, c.r64(0).wrapping_add(0xb));
            blk = 35; continue;
        }
        35 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        36 => {
            let r28 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r28);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_e() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t29) = c.ld16(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fb67e); };
            c.w32(8, t29);
            let Some(t30) = c.ld16(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fb683); };
            c.cmp(t30, c.r16(8), 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 36; continue;
        }
        40 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t31) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb691); };
            c.w64(2, t31);
            blk = 43; continue;
        }
        42 => {
            let r32 = c.add(c.r64(2), 0xbu64, 64);
            c.w64(2, r32);
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t33) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fb6a1); };
            c.w64(7, t33);
            blk = 46; continue;
        }
        45 => {
            c.w64(7, c.r64(0).wrapping_add(0xb));
            blk = 46; continue;
        }
        46 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 48; continue;
        }
        47 => {
            let r34 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r34);
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t35) = c.ld8(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fb6b9); };
            c.w32(8, t35);
            let Some(t36) = c.ld8(c.r64(2).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fb6be); };
            c.cmp(t36, c.r8(8), 8);
            if c.cc_ne() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 47; continue;
        }
        51 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007fb6c6); };
            c.w64(0, t37);
            return c.ret(0x0, 0x70007fb6cd);
        }
        52 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007fb6ce); };
            c.w64(0, t38);
            return c.ret(0x0, 0x70007fb6d5);
        }
        53 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb6ec); };
            c.w32(7, t39);
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb6ef); };
            c.cmp(t40, c.r32(7), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb704); };
            c.w32(7, t41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb707); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb710); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            blk = 1; continue;
        }
        60 => {
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb71b); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_ne() { blk = 1; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb731); };
            c.w32(7, t47);
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb734); };
            c.cmp(t48, c.r32(7), 32);
            if c.cc_ne() { blk = 1; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb741); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            blk = 1; continue;
        }
        64 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fb74c); }
            let Some(t51) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fb74d); };
            if !c.push64(t51) { return RegfileExit::fault(0x70007fb74d); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2778)) else { return RegfileExit::fault(0x70007fb751); };
            c.w64(3, t52);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007fb75f); }
            return RegfileExit::jump(0x700086ed74);
        }
        65 => {
            return RegfileExit::jump(0x70007fb76c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fb774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb77c); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb782); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb788); };
            c.w32(1, t4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb78c); };
            c.w32(7, t5);
            c.w64(8, c.r64(1));
            let r6 = c.shl(c.r32(8), 0x8u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r6);
            let r7 = c.r32(8) | c.r32(7);
            c.w32(8, r7);
            c.logic(r7, 32);
            let r8 = c.r32(8) & 0xf0fu64;
            c.w32(8, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(8), 0x808u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let r9 = c.r32(1) & 0x7u64;
            c.w32(1, r9);
            c.logic(r9, 32);
            let r10 = c.r32(7) & 0x7u64;
            c.w32(7, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb7c9); };
            c.w32(7, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb7cc); };
            c.cmp(t12, c.r32(7), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb7e0); };
            c.w32(1, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb7e3); };
            c.cmp(t14, c.r32(1), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb7ec); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            blk = 0; continue;
        }
        9 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb7f4); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r18);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 0; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb809); };
            c.w32(1, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb80c); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_ne() { blk = 0; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb819); };
            c.w32(0, t21);
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            blk = 0; continue;
        }
        13 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fb824); }
            let Some(t23) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fb825); };
            if !c.push64(t23) { return RegfileExit::fault(0x70007fb825); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2788)) else { return RegfileExit::fault(0x70007fb829); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007fb837); }
            return RegfileExit::jump(0x700086ed74);
        }
        14 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb841); };
            c.w32(1, t25);
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb844); };
            c.w32(7, t26);
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_be() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(7));
            blk = 17; continue;
        }
        16 => {
            c.w64(8, c.r64(1));
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r27);
            c.w32(9, 0xbu64);
            blk = 19; continue;
        }
        18 => {
            let r28 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r28);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld8(c.r64(2).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fb868); };
            c.w32(11, t29);
            let Some(t30) = c.ld8(c.r64(0).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fb86d); };
            c.w32(12, t30);
            c.cmp(c.r8(11), c.r8(12), 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r8(11), c.r8(12), 8);
            if c.cc_b() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_b() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007fb887); };
            c.w64(0, t31);
            return c.ret(0x0, 0x70007fb88e);
        }
        26 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007fb88f); };
            c.w64(0, t32);
            return c.ret(0x0, 0x70007fb896);
        }
        27 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007fb897); };
            c.w64(0, t33);
            return c.ret(0x0, 0x70007fb89e);
        }
        28 => {
            return RegfileExit::jump(0x70007fb8a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fb8b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb8bc); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb8c2); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb8c8); };
            c.w32(1, t4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb8cc); };
            c.w32(7, t5);
            c.w64(8, c.r64(1));
            let r6 = c.shl(c.r32(8), 0x8u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r6);
            let r7 = c.r32(8) | c.r32(7);
            c.w32(8, r7);
            c.logic(r7, 32);
            let r8 = c.r32(8) & 0xf0fu64;
            c.w32(8, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(8), 0x808u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let r9 = c.r32(1) & 0x7u64;
            c.w32(1, r9);
            c.logic(r9, 32);
            let r10 = c.r32(7) & 0x7u64;
            c.w32(7, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb909); };
            c.w32(7, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb90c); };
            c.cmp(t12, c.r32(7), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb920); };
            c.w32(1, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb923); };
            c.cmp(t14, c.r32(1), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb92c); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            blk = 0; continue;
        }
        9 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb934); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r18);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 0; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fb949); };
            c.w32(1, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fb94c); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_ne() { blk = 0; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fb959); };
            c.w32(0, t21);
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            blk = 0; continue;
        }
        13 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fb964); }
            let Some(t23) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fb965); };
            if !c.push64(t23) { return RegfileExit::fault(0x70007fb965); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2790)) else { return RegfileExit::fault(0x70007fb969); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007fb977); }
            return RegfileExit::jump(0x700086ed74);
        }
        14 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb981); };
            c.w32(1, t25);
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fb984); };
            c.w32(7, t26);
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_be() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(7));
            blk = 17; continue;
        }
        16 => {
            c.w64(8, c.r64(1));
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r27);
            c.w32(9, 0xbu64);
            blk = 19; continue;
        }
        18 => {
            let r28 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r28);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld8(c.r64(2).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fb9a8); };
            c.w32(11, t29);
            let Some(t30) = c.ld8(c.r64(0).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fb9ad); };
            c.w32(12, t30);
            c.cmp(c.r8(11), c.r8(12), 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r8(11), c.r8(12), 8);
            if c.cc_b() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_b() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007fb9c7); };
            c.w64(0, t31);
            return c.ret(0x0, 0x70007fb9ce);
        }
        26 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007fb9cf); };
            c.w64(0, t32);
            return c.ret(0x0, 0x70007fb9d6);
        }
        27 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007fb9d7); };
            c.w64(0, t33);
            return c.ret(0x0, 0x70007fb9de);
        }
        28 => {
            return RegfileExit::jump(0x70007fb9e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fb9f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fb9fc); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fba02); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fba08); };
            c.w32(1, t4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fba0c); };
            c.w32(7, t5);
            c.w64(8, c.r64(1));
            let r6 = c.shl(c.r32(8), 0x8u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r6);
            let r7 = c.r32(8) | c.r32(7);
            c.w32(8, r7);
            c.logic(r7, 32);
            let r8 = c.r32(8) & 0xf0fu64;
            c.w32(8, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(8), 0x808u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let r9 = c.r32(1) & 0x7u64;
            c.w32(1, r9);
            c.logic(r9, 32);
            let r10 = c.r32(7) & 0x7u64;
            c.w32(7, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fba49); };
            c.w32(7, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fba4c); };
            c.cmp(t12, c.r32(7), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fba60); };
            c.w32(1, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fba63); };
            c.cmp(t14, c.r32(1), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fba6c); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            blk = 0; continue;
        }
        9 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fba74); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r18);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 0; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fba89); };
            c.w32(1, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fba8c); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_ne() { blk = 0; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fba99); };
            c.w32(0, t21);
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            blk = 0; continue;
        }
        13 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fbaa4); }
            let Some(t23) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fbaa5); };
            if !c.push64(t23) { return RegfileExit::fault(0x70007fbaa5); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x27a0)) else { return RegfileExit::fault(0x70007fbaa9); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007fbab7); }
            return RegfileExit::jump(0x700086ed74);
        }
        14 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbac1); };
            c.w32(1, t25);
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbac4); };
            c.w32(7, t26);
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_be() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(7));
            blk = 17; continue;
        }
        16 => {
            c.w64(8, c.r64(1));
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r27);
            c.w32(9, 0xbu64);
            blk = 19; continue;
        }
        18 => {
            let r28 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r28);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld8(c.r64(2).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fbae8); };
            c.w32(11, t29);
            let Some(t30) = c.ld8(c.r64(0).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fbaed); };
            c.w32(12, t30);
            c.cmp(c.r8(11), c.r8(12), 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r8(11), c.r8(12), 8);
            if c.cc_b() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_b() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007fbb07); };
            c.w64(0, t31);
            return c.ret(0x0, 0x70007fbb0e);
        }
        26 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007fbb0f); };
            c.w64(0, t32);
            return c.ret(0x0, 0x70007fbb16);
        }
        27 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007fbb17); };
            c.w64(0, t33);
            return c.ret(0x0, 0x70007fbb1e);
        }
        28 => {
            return RegfileExit::jump(0x70007fbb20);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fbb34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fbb3c); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fbb42); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbb48); };
            c.w32(1, t4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbb4c); };
            c.w32(7, t5);
            c.w64(8, c.r64(1));
            let r6 = c.shl(c.r32(8), 0x8u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r6);
            let r7 = c.r32(8) | c.r32(7);
            c.w32(8, r7);
            c.logic(r7, 32);
            let r8 = c.r32(8) & 0xf0fu64;
            c.w32(8, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(8), 0x808u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let r9 = c.r32(1) & 0x7u64;
            c.w32(1, r9);
            c.logic(r9, 32);
            let r10 = c.r32(7) & 0x7u64;
            c.w32(7, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fbb89); };
            c.w32(7, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fbb8c); };
            c.cmp(t12, c.r32(7), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fbba0); };
            c.w32(1, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fbba3); };
            c.cmp(t14, c.r32(1), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fbbac); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            blk = 0; continue;
        }
        9 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fbbb4); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r18);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 0; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fbbc9); };
            c.w32(1, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fbbcc); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_ne() { blk = 0; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fbbd9); };
            c.w32(0, t21);
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            blk = 0; continue;
        }
        13 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fbbe4); }
            let Some(t23) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fbbe5); };
            if !c.push64(t23) { return RegfileExit::fault(0x70007fbbe5); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x27a8)) else { return RegfileExit::fault(0x70007fbbe9); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007fbbf7); }
            return RegfileExit::jump(0x700086ed74);
        }
        14 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbc01); };
            c.w32(1, t25);
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbc04); };
            c.w32(7, t26);
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_be() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(7));
            blk = 17; continue;
        }
        16 => {
            c.w64(8, c.r64(1));
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r27);
            c.w32(9, 0xbu64);
            blk = 19; continue;
        }
        18 => {
            let r28 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r28);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld8(c.r64(2).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fbc28); };
            c.w32(11, t29);
            let Some(t30) = c.ld8(c.r64(0).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fbc2d); };
            c.w32(12, t30);
            c.cmp(c.r8(11), c.r8(12), 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r8(11), c.r8(12), 8);
            if c.cc_b() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_b() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007fbc47); };
            c.w64(0, t31);
            return c.ret(0x0, 0x70007fbc4e);
        }
        26 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007fbc4f); };
            c.w64(0, t32);
            return c.ret(0x0, 0x70007fbc56);
        }
        27 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007fbc57); };
            c.w64(0, t33);
            return c.ret(0x0, 0x70007fbc5e);
        }
        28 => {
            return RegfileExit::jump(0x70007fbc60);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fbc74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fbc74); };
            c.w32(7, t0);
            let r1 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1);
            c.w64(8, c.r64(1));
            let r2 = c.sub(c.r64(8), c.r64(3), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbc80); };
            c.w32(9, t3);
            let Some(t4) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbc84); };
            c.w32(7, t4);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_be() { blk = 117; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_e() { blk = 116; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(7) & 0x7u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_e() { blk = 91; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(0));
            blk = 5; continue;
        }
        5 => {
            c.w64(11, c.r64(9));
            let r5 = c.r32(11) & 0x7u64;
            c.w32(11, r5);
            c.logic(r5, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 17; continue;
        }
        10 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 17; continue;
        }
        13 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fbcf5); };
            c.w32(9, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fbcf9); };
            c.cmp(t7, c.r32(9), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fbd02); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fbd08); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbd0f); };
            c.w32(9, t12);
            blk = 5; continue;
        }
        15 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fbd16); };
            c.w32(9, t13);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r14 = c.add(c.r64(6), c.r64(9), 64);
            c.w64(6, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fbd23); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fbd29); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbd30); };
            c.w32(9, t19);
            blk = 5; continue;
        }
        16 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fbd37); };
            c.w32(7, t20);
            let r21 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r21);
            let Some(t22) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fbd3d); };
            c.w32(9, t22);
            let r23 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r23);
            let Some(t24) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fbd44); };
            c.w32(9, t24);
            blk = 5; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fbd4e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fbd52); }
            let r25 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            blk = 119; continue;
        }
        18 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(10));
            blk = 22; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fbd6b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fbd6f); }
            let r26 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r26);
            blk = 119; continue;
        }
        20 => {
            c.w32(11, 0x1u64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(10));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(8), 0xdu64, 64);
            if c.cc_l() { blk = 30; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(6)));
            let r27 = c.r32(7) & 0x8u64;
            c.w32(7, r27);
            c.logic(r27, 32);
            let r28 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r28);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fbdab); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fbdb2); };
            c.w64(11, t30);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fbdb9); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007fbdc0); };
            c.cmp(t32, c.r64(12), 64);
            if c.cc_be() { blk = 121; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70007fbdc9); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70007fbdd0); };
            c.w64(11, t33);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70007fbdd7); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fbddb); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007fbde2); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70007fbde6); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70007fbdea); }
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70007fbdf0);
        }
        27 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fbdf1); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fbdf8); };
            c.w64(11, t35);
            c.w64(15, c.r64(11).wrapping_add(0x14));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fbdff); };
            c.w64(12, t36);
            let Some(t37) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007fbe06); };
            c.cmp(t37, c.r64(15), 64);
            if c.cc_be() { blk = 123; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x70007fbe10); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70007fbe17); };
            c.w64(11, t38);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70007fbe1e); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fbe22); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007fbe29); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70007fbe2d); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70007fbe31); }
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70007fbe37);
        }
        30 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(1, c.r32(8));
            let r39 = c.r32(7) & 0x8u64;
            c.w32(7, r39);
            c.logic(r39, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(1, c.r32(8));
            c.w64(7, c.r64(1));
            let r40 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r40);
            if c.cc_o() { blk = 40; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r41 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r41);
            if c.cc_o() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fbe7a); };
            c.w64(7, t42);
            let Some(t43) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fbe81); };
            c.w64(7, t43);
            let r44 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r44);
            let r45 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r45);
            c.logic(r45, 64);
            c.w64(11, c.r64(7).wrapping_add(c.r64(1)));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fbe90); };
            c.w64(12, t46);
            let Some(t47) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007fbe97); };
            c.cmp(t47, c.r64(11), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 126; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fbeae); };
            c.w64(1, t48);
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x70007fbeb5); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 38; continue;
        }
        38 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70007fbebc); };
            c.w64(7, t49);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007fbec3); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fbec6); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007fbecd); }
            c.w64(0, c.r64(1));
            blk = 42; continue;
        }
        39 => {
            return RegfileExit::fault(0x70007fbed6);
        }
        40 => {
            return RegfileExit::fault(0x70007fbed7);
        }
        41 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fbed8); };
            c.w64(0, t50);
            blk = 42; continue;
        }
        42 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(6)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(8, 0xbu64);
            blk = 45; continue;
        }
        44 => {
            let r51 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r51);
            c.w64(1, c.r64(11));
            blk = 45; continue;
        }
        45 => {
            c.w64(11, c.r64(1).wrapping_add(0x2));
            let Some(t52) = c.ld16(c.r64(9).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70007fbf07); };
            c.w32(1, t52);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70007fbf0c); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 44; continue; }
            blk = 46; continue;
        }
        46 => {
            return c.ret(0x0, 0x70007fbf16);
        }
        47 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r53 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r53);
            if c.cc_o() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fbf29); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fbf30); };
            c.w64(7, t55);
            let r56 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r56);
            let r57 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r57);
            c.logic(r57, 64);
            c.w64(11, c.r64(7).wrapping_add(c.r64(1)));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fbf3f); };
            c.w64(12, t58);
            let Some(t59) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007fbf46); };
            c.cmp(t59, c.r64(11), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 130; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fbf5d); };
            c.w64(1, t60);
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x70007fbf64); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 52; continue;
        }
        52 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70007fbf6b); };
            c.w64(7, t61);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007fbf72); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fbf75); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007fbf7c); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        53 => {
            return RegfileExit::fault(0x70007fbf85);
        }
        54 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fbf86); };
            c.w64(0, t62);
            blk = 55; continue;
        }
        55 => {
            c.w64(1, c.r64(6).wrapping_add(c.r64(3)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(8, 0xbu64);
            blk = 58; continue;
        }
        57 => {
            let r63 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r63);
            c.w64(1, c.r64(11));
            blk = 58; continue;
        }
        58 => {
            c.w64(11, c.r64(1).wrapping_add(0x1));
            let Some(t64) = c.ld8(c.r64(9).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70007fbfb2); };
            c.w32(1, t64);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70007fbfb7); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 57; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 46; continue;
        }
        60 => {
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 90; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w32(1, c.r32(8));
            let Some(t65) = c.ld64(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fbfd2); };
            c.w64(9, t65);
            let r66 = c.r32(7) & 0x8u64;
            c.w32(7, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(1, c.r32(8));
            c.w64(7, c.r64(1));
            let r67 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r67);
            if c.cc_o() { blk = 70; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r68 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r68);
            if c.cc_o() { blk = 69; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fc00b); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fc012); };
            c.w64(7, t70);
            let r71 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r71);
            let r72 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r72);
            c.logic(r72, 64);
            c.w64(11, c.r64(7).wrapping_add(c.r64(1)));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fc021); };
            c.w64(12, t73);
            let Some(t74) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007fc028); };
            c.cmp(t74, c.r64(11), 64);
            if c.cc_be() { blk = 133; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 134; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fc03f); };
            c.w64(1, t75);
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x70007fc046); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 68; continue;
        }
        68 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70007fc04d); };
            c.w64(7, t76);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007fc054); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fc057); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007fc05e); }
            c.w64(0, c.r64(1));
            blk = 72; continue;
        }
        69 => {
            return RegfileExit::fault(0x70007fc067);
        }
        70 => {
            return RegfileExit::fault(0x70007fc068);
        }
        71 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fc069); };
            c.w64(0, t77);
            blk = 72; continue;
        }
        72 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(6)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(8, 0xbu64);
            blk = 75; continue;
        }
        74 => {
            let r78 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r78);
            c.w64(1, c.r64(11));
            blk = 75; continue;
        }
        75 => {
            c.w64(11, c.r64(1).wrapping_add(0x2));
            let Some(t79) = c.ld16(c.r64(1).wrapping_add(c.r64(9)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70007fc09c); };
            c.w32(1, t79);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70007fc0a2); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 74; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 46; continue;
        }
        77 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r80 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r80);
            if c.cc_o() { blk = 83; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fc0c3); };
            c.w64(7, t81);
            let Some(t82) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fc0ca); };
            c.w64(7, t82);
            let r83 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r83);
            let r84 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r84);
            c.logic(r84, 64);
            c.w64(11, c.r64(7).wrapping_add(c.r64(1)));
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fc0d9); };
            c.w64(12, t85);
            let Some(t86) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007fc0e0); };
            c.cmp(t86, c.r64(11), 64);
            if c.cc_be() { blk = 137; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 138; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fc0f7); };
            c.w64(1, t87);
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x70007fc0fe); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 82; continue;
        }
        82 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70007fc105); };
            c.w64(7, t88);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007fc10c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fc10f); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007fc116); }
            c.w64(0, c.r64(1));
            blk = 85; continue;
        }
        83 => {
            return RegfileExit::fault(0x70007fc11f);
        }
        84 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fc120); };
            c.w64(0, t89);
            blk = 85; continue;
        }
        85 => {
            c.w64(1, c.r64(6).wrapping_add(c.r64(3)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w32(8, 0xbu64);
            blk = 88; continue;
        }
        87 => {
            let r90 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r90);
            c.w64(1, c.r64(11));
            blk = 88; continue;
        }
        88 => {
            c.w64(11, c.r64(1).wrapping_add(0x1));
            let Some(t91) = c.ld8(c.r64(1).wrapping_add(c.r64(9)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70007fc14c); };
            c.w32(1, t91);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70007fc152); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 87; continue; }
            blk = 89; continue;
        }
        89 => {
            blk = 46; continue;
        }
        90 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc160); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc164); }
            let r92 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r92);
            blk = 119; continue;
        }
        91 => {
            c.w32(1, c.r16(7));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(0));
            blk = 93; continue;
        }
        93 => {
            c.w64(8, c.r64(7));
            let r93 = c.r32(8) & 0x7u64;
            c.w32(8, r93);
            c.logic(r93, 32);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 98; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 97; continue;
        }
        97 => {
            blk = 141; continue;
        }
        98 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            blk = 141; continue;
        }
        101 => {
            let Some(t94) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fc1bd); };
            c.w32(7, t94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fc1c0); };
            c.cmp(t95, c.r32(7), 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t96) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fc1cd); };
            c.w32(1, t96);
            let r97 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r97);
            let Some(t98) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fc1d3); };
            c.w32(7, t98);
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            let Some(t100) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fc1d9); };
            c.w32(7, t100);
            blk = 93; continue;
        }
        103 => {
            let Some(t101) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fc1df); };
            c.w32(7, t101);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r102 = c.add(c.r64(6), c.r64(7), 64);
            c.w64(6, r102);
            let Some(t103) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fc1eb); };
            c.w32(1, t103);
            let r104 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r104);
            let Some(t105) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fc1f1); };
            c.w32(7, t105);
            let r106 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r106);
            let Some(t107) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fc1f7); };
            c.w32(7, t107);
            blk = 93; continue;
        }
        104 => {
            let Some(t108) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fc1fd); };
            c.w32(1, t108);
            let r109 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r109);
            let Some(t110) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fc203); };
            c.w32(7, t110);
            let r111 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r111);
            let Some(t112) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fc209); };
            c.w32(7, t112);
            blk = 93; continue;
        }
        105 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 108; continue;
        }
        106 => {
            c.w32(11, 0x1u64);
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 108; continue;
        }
        107 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 109; continue;
        }
        109 => {
            let r113 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r113);
            blk = 110; continue;
        }
        110 => {
            c.w64(8, c.r64(3).wrapping_add(c.r64(6)));
            let r114 = c.r32(1) & 0x8u64;
            c.w32(1, r114);
            c.logic(r114, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t115) = c.ld16(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x70007fc253); };
            c.w32(1, t115);
            blk = 113; continue;
        }
        112 => {
            let Some(t116) = c.ld8(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70007fc25a); };
            c.w32(1, t116);
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 158; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70007fc273); };
            c.w64(8, t117);
            let Some(t118) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fc27a); };
            c.w32(0, t118);
            let r119 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r119);
            let Some(t120) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fc282); };
            c.cmp(t120, c.r32(0), 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 46; continue;
        }
        116 => {
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007fc294); };
            c.w64(0, t121);
            return c.ret(0x0, 0x70007fc29b);
        }
        117 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 118; continue;
        }
        118 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc2a6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc2aa); }
            let r122 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r122);
            blk = 119; continue;
        }
        119 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            let r123 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r123);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fc2b5); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc2b6); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fc2b7); }
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70007fc2b8); };
            c.w64(3, t124);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007fc2cb, 0x70007fc2c6);
        }
        120 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc2ce); }
            return c.ret(0x0, 0x70007fc2cf);
        }
        121 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc2d0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc2d4); }
            let r125 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r125);
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fc2df); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70007fc2e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007fc2e7); }
            return c.call(0x70007febf4, 0x70007fc2f0, 0x70007fc2eb);
        }
        122 => {
            c.w64(7, c.r64(0));
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc2f3); };
            c.w64(8, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fc2f7); };
            c.w64(9, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fc2fb); };
            c.w64(1, t128);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc302); }
            blk = 26; continue;
        }
        123 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc308); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc30c); }
            let r129 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r129);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fc312); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70007fc316); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007fc31a); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70007fc328, 0x70007fc323);
        }
        124 => {
            c.w64(7, c.r64(0));
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc32b); };
            c.w64(8, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fc32f); };
            c.w64(9, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fc333); };
            c.w64(1, t132);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc33a); }
            blk = 29; continue;
        }
        125 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc340); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc344); }
            let r133 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r133);
            blk = 127; continue;
        }
        126 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc34c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc350); }
            let r134 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r134);
            blk = 127; continue;
        }
        127 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70007fc356); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fc35a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70007fc35e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70007fc362); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70007fc36e, 0x70007fc369);
        }
        128 => {
            c.w64(1, c.r64(0));
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fc371); };
            c.w64(3, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc375); };
            c.w64(8, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fc379); };
            c.w64(6, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fc37d); };
            c.w64(9, t138);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc384); }
            blk = 38; continue;
        }
        129 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc38a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc38e); }
            let r139 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r139);
            blk = 131; continue;
        }
        130 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc396); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc39a); }
            let r140 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r140);
            blk = 131; continue;
        }
        131 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70007fc3a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fc3a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70007fc3a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70007fc3ac); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70007fc3b8, 0x70007fc3b3);
        }
        132 => {
            c.w64(1, c.r64(0));
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fc3bb); };
            c.w64(3, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc3bf); };
            c.w64(8, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fc3c3); };
            c.w64(6, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fc3c7); };
            c.w64(9, t144);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc3ce); }
            blk = 52; continue;
        }
        133 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc3d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc3d8); }
            let r145 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r145);
            blk = 135; continue;
        }
        134 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc3e0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc3e4); }
            let r146 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r146);
            blk = 135; continue;
        }
        135 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70007fc3ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70007fc3ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fc3f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70007fc3f6); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70007fc402, 0x70007fc3fd);
        }
        136 => {
            c.w64(1, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fc405); };
            c.w64(3, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fc409); };
            c.w64(9, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc40d); };
            c.w64(8, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fc411); };
            c.w64(6, t150);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc418); }
            blk = 68; continue;
        }
        137 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc41e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc422); }
            let r151 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r151);
            blk = 139; continue;
        }
        138 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc42a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc42e); }
            let r152 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r152);
            blk = 139; continue;
        }
        139 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70007fc434); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70007fc438); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fc43c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70007fc440); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70007fc44c, 0x70007fc447);
        }
        140 => {
            c.w64(1, c.r64(0));
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fc44f); };
            c.w64(3, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fc453); };
            c.w64(9, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc457); };
            c.w64(8, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fc45b); };
            c.w64(6, t156);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc462); }
            blk = 82; continue;
        }
        141 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc468); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc46c); }
            let r157 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r157);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fc472); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70007fc476); }
            blk = 145; continue;
        }
        142 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc47c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc480); }
            let r158 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r158);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fc486); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70007fc48a); }
            blk = 145; continue;
        }
        143 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc490); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc494); }
            let r159 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r159);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fc49a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70007fc49e); }
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t160) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fc4a7); };
            c.w64(7, t160);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc4ae); }
            blk = 110; continue;
        }
        145 => {
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w32(1, c.r64(3).wrapping_add(c.r64(3)));
            blk = 156; continue;
        }
        147 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fc4c2); };
            c.w64(1, t161);
            let Some(t162) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007fc4c9); };
            c.w64(7, t162);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fc4d0); };
            c.w64(9, t163);
            let Some(t164) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007fc4d7); };
            c.cmp(t164, c.r64(8), 64);
            if c.cc_a() { blk = 150; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70007fc4e6, 0x70007fc4e1);
        }
        149 => {
            c.w64(1, c.r64(0));
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fc4e9); };
            c.w64(0, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fc4ed); };
            c.w64(3, t166);
            blk = 151; continue;
        }
        150 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70007fc4f3); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 151; continue;
        }
        151 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 155; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            let r167 = c.r64(10) | 0x1u64;
            c.w64(10, r167);
            c.logic(r167, 64);
            blk = 154; continue;
        }
        154 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 155; continue;
        }
        155 => {
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fc521); };
            c.w64(7, t168);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007fc528); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007fc52b); }
            blk = 156; continue;
        }
        156 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fc530); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fc531); }
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70007fc532); };
            c.w64(3, t169);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007fc545, 0x70007fc540);
        }
        157 => {
            let r170 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r170);
            c.w64(1, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc54d); }
            blk = 113; continue;
        }
        158 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc553); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc557); }
            let r171 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r171);
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fc55d); };
            c.w64(7, t172);
            let Some(t173) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fc564); };
            c.w64(8, t173);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007fc56b); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fc56f); };
            c.w64(11, t174);
            let Some(t175) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007fc576); };
            c.cmp(t175, c.r64(9), 64);
            if c.cc_a() { blk = 161; continue; }
            blk = 159; continue;
        }
        159 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70007fc585, 0x70007fc580);
        }
        160 => {
            blk = 162; continue;
        }
        161 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70007fc587); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 162; continue;
        }
        162 => {
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70007fc591); };
            c.w64(1, t176);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007fc598); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70007fc59b); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fc5a2); }
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fc5a9); };
            c.w64(1, t177);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x70007fc5ad); }
            blk = 120; continue;
        }
        163 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc5b6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fc5ba); }
            let r178 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r178);
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fc5c0); };
            c.w64(9, t179);
            let Some(t180) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007fc5c7); };
            c.w64(11, t180);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007fc5ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007fc5d2); }
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fc5d6); };
            c.w64(15, t181);
            let Some(t182) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007fc5dd); };
            c.cmp(t182, c.r64(12), 64);
            if c.cc_a() { blk = 166; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70007fc5ec, 0x70007fc5e7);
        }
        165 => {
            blk = 167; continue;
        }
        166 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70007fc5ee); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 167; continue;
        }
        167 => {
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70007fc5f8); };
            c.w64(1, t183);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007fc5ff); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70007fc602); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007fc609); }
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fc610); };
            c.w64(1, t184);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70007fc614); }
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70007fc617); };
            c.w64(8, t185);
            let Some(t186) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fc61e); };
            c.w32(7, t186);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            return RegfileExit::fault(0x70007fc62d);
        }
        169 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fc62e); };
            c.w64(7, t187);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70007fc632); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r188 = c.r64(1) & c.r64(8);
            c.w64(1, r188);
            c.logic(r188, 64);
            let Some(t189) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fc644); };
            c.logic(t189 & 0x4u64, 8);
            if c.cc_ne() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            blk = 120; continue;
        }
        172 => {
            c.w32(0, c.r32(0));
            let r190 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r190);
            c.w64(1, 0xfffffffffffc0000u64);
            let r191 = c.r64(1) & c.r64(0);
            c.w64(1, r191);
            c.logic(r191, 64);
            let Some(t192) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fc65e); };
            c.logic(t192 & 0x2u64, 8);
            if c.cc_e() { blk = 171; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc668); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fc669); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70007fc676, 0x70007fc671);
        }
        174 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fc676); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fc678); }
            blk = 171; continue;
        }
        175 => {
            return RegfileExit::jump(0x70007fc67c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fc694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_le() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fc699); };
            c.w32(2, t0);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(3));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 4; continue;
        }
        3 => {
            let r1 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r1);
            let r2 = c.sub(c.r32(1), 0x2u64, 32);
            c.w32(1, r2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_ge() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fc6be); };
            c.w32(7, t3);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(6).wrapping_add(0x3));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fc6d0); };
            c.w32(7, t4);
            c.cmp(c.r32(3), c.r32(7), 32);
            if c.cc_g() { blk = 3; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70007fc6db);
        }
        8 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70007fc6de);
        }
        9 => {
            return RegfileExit::fault(0x70007fc6df);
        }
        10 => {
            return RegfileExit::jump(0x70007fc6e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fc6f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fc6f4); };
            c.w32(10, t0);
            let r1 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r1);
            let Some(t2) = c.ld32(c.r64(10).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fc6fb); };
            c.w32(12, t2);
            let r3 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fc702); };
            c.w32(10, t4);
            let r5 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r5);
            let Some(t6) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fc70a); };
            c.cmp(t6, 0x5eu64, 16);
            if c.cc_e() { blk = 69; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fc716); };
            c.cmp(t7, 0x69u64, 16);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fc71e); };
            c.w32(12, t8);
            let r9 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r9);
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fc726); };
            c.w32(10, t10);
            let r11 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r11);
            let Some(t12) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fc72e); };
            c.cmp(t12, 0x85u64, 16);
            if c.cc_ne() { blk = 30; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fc73b); };
            c.w32(3, t13);
            let r14 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fc741); };
            c.w32(3, t15);
            let r16 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fc747); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fc74d); };
            c.cmp(t19, 0xa7u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fc759); };
            c.w32(1, t20);
            c.logic(c.r8(1) & 0x1eu64, 8);
            if c.cc_ne() { blk = 36; continue; }
            blk = 6; continue;
        }
        6 => {
            let ea21 = c.r64(3).wrapping_add(0x7);
            let Some(t22) = c.ld32(ea21) else { return RegfileExit::fault(0x70007fc765); };
            let r23 = c.inc(t22, 32);
            if !c.st32(ea21, r23) { return RegfileExit::fault(0x70007fc765); }
            blk = 7; continue;
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc768); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fc76c); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc76d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fc76e); }
            if !c.st16(c.r64(12).wrapping_add(0x1f), 0x0u64) { return RegfileExit::fault(0x70007fc76f); }
            c.w32(9, 0x21u64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fc77d); }
            c.w32(1, c.r32(9));
            let r24 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r24);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fc784); }
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fc785); };
            c.w32(1, t25);
            c.w64(0, c.r64(4));
            let r26 = c.sub(c.r64(0), c.r64(1), 64);
            c.w64(0, r26);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fc790); };
            c.cmp(c.r64(0), t27, 64);
            if c.cc_b() { blk = 83; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fc79a); };
            c.w64(0, t28);
            blk = 10; continue;
        }
        9 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fc7a3); }
            blk = 10; continue;
        }
        10 => {
            let r29 = c.sub(c.r64(1), 0x8u64, 64);
            c.w64(1, r29);
            if c.cc_ge() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fc7aa); };
            c.w64(1, (t30 as u32 as i32 as i64 as u64));
            c.logic(c.r32(1) & c.r32(1), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_add(c.r64(1).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x70007fc7b3); }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007fc7b8); };
            c.cmp(c.r64(4), t31, 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70007fc7c2); };
            c.w64(15, t32);
            let Some(t33) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fc7c9); };
            c.w32(10, t33);
            let Some(t34) = c.ld64(c.r64(15).wrapping_add(c.r64(10).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fc7ce); };
            c.w64(1, t34);
            return c.call(c.r64(1), 0x70007fc7d4, 0x70007fc7d2);
        }
        15 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc7d4); };
            c.w64(12, t35);
            let Some(t36) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fc7d8); };
            c.w64(9, (t36 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t37) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fc7df); };
            c.w32(3, t37);
            c.w64(8, c.r64(9));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1268)) else { return RegfileExit::fault(0x70007fc7e7); };
            c.w64(1, t38);
            c.cmp(c.r8(3), 0x3u64, 8);
            if c.cc_a() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            let r39 = c.inc(c.r32(9), 32);
            c.w32(9, r39);
            c.logic(c.r8(3) & 0x1u64, 8);
            let Some(t40) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fc7f9); };
            c.w32(3, t40);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let r41 = c.add(c.r64(1), 0xc5u64, 64);
            c.w64(1, r41);
            blk = 19; continue;
        }
        18 => {
            let r42 = c.add(c.r64(1), 0x18au64, 64);
            c.w64(1, r42);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r8(3), 0xffffffffffffffa8u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r8(3), 0xffffffffffffffaeu64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r8(3), 0xffffffffffffff88u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(9, c.r64(8));
            blk = 24; continue;
        }
        23 => {
            let Some(t43) = c.ld8(c.r64(1).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70007fc82c); };
            c.w32(10, t43);
            let r44 = c.add(c.r32(9), c.r32(10), 32);
            c.w32(9, r44);
            blk = 24; continue;
        }
        24 => {
            blk = 14; continue;
        }
        25 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc836); };
            c.w64(3, t45);
            let Some(t46) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fc83a); };
            c.w32(3, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fc83d); };
            c.w64(1, t47);
            c.w64(1, c.r64(1).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_ge() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(3, c.r64(1));
            blk = 27; continue;
        }
        27 => {
            if !c.leave() { return RegfileExit::fault(0x70007fc851); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fc852); }
            let r48 = c.add(c.r64(4), c.r64(3), 64);
            c.w64(4, r48);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fc856); }
            return c.ret(0x0, 0x70007fc857);
        }
        28 => {
            c.w32(10, 0x40u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fc85e); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70007fc864); };
            c.w64(3, t49);
            return c.call(0x700086ed74, 0x70007fc870, 0x70007fc86b);
        }
        29 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fc870); };
            c.w64(12, t50);
            c.w32(9, 0x21u64);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fc87a); };
            c.w64(0, t51);
            c.w32(1, c.r32(12));
            let r52 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70007fc886); }
            blk = 14; continue;
        }
        30 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc88f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fc893); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc895); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fc896); }
            let r53 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r53);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fc899); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc89a); }
            c.w32(0, 0x1u64);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1e10)) else { return RegfileExit::fault(0x70007fc8a0); };
            c.w64(3, t54);
            return c.call(0x700086ed74, 0x70007fc8ac, 0x70007fc8a7);
        }
        31 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fc8af); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fc8b6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fc8b7); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc8bb); }
            let Some(t55) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fc8bc); };
            c.logic(t55 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let r56 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r56);
            blk = 34; continue;
        }
        33 => {
            let Some(t57) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fc8d2); };
            c.w32(1, t57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fc8d5); };
            c.w64(1, t58);
            blk = 34; continue;
        }
        34 => {
            return RegfileExit::jump(c.r64(1));
        }
        35 => {
            return RegfileExit::fault(0x70007fc8df);
        }
        36 => {
            c.logic(c.r8(1) & 0x6u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 37; continue;
        }
        37 => {
            let r59 = c.r64(1) & 0x7u64;
            c.w64(1, r59);
            c.logic(r59, 64);
            c.cmp(c.r32(1), 0x4u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 38; continue;
        }
        38 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc8f6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fc8fa); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc8fc); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fc8fd); }
            let r60 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r60);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fc900); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc901); }
            c.w32(0, 0x1u64);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1e40)) else { return RegfileExit::fault(0x70007fc907); };
            c.w64(3, t61);
            return c.call(0x700086ed74, 0x70007fc913, 0x70007fc90e);
        }
        39 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fc916); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fc91d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fc91e); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc922); }
            let Some(t62) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fc923); };
            c.logic(t62 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            let r63 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r63);
            blk = 42; continue;
        }
        41 => {
            let Some(t64) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fc939); };
            c.w32(1, t64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fc93c); };
            c.w64(1, t65);
            blk = 42; continue;
        }
        42 => {
            return RegfileExit::jump(c.r64(1));
        }
        43 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc94f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fc953); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc955); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fc956); }
            let r66 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r66);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fc959); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc95a); }
            c.w32(0, 0x1u64);
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1e28)) else { return RegfileExit::fault(0x70007fc960); };
            c.w64(3, t67);
            return c.call(0x700086ed74, 0x70007fc96c, 0x70007fc967);
        }
        45 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fc96f); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fc976); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fc977); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc97b); }
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fc97c); };
            c.logic(t68 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let r69 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r69);
            blk = 48; continue;
        }
        47 => {
            let Some(t70) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fc992); };
            c.w32(1, t70);
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fc995); };
            c.w64(1, t71);
            blk = 48; continue;
        }
        48 => {
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fc9a8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fc9ac); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc9ae); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fc9af); }
            let r72 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r72);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fc9b2); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fc9b3); }
            c.w32(0, 0x1u64);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1e20)) else { return RegfileExit::fault(0x70007fc9b9); };
            c.w64(3, t73);
            return c.call(0x700086ed74, 0x70007fc9c5, 0x70007fc9c0);
        }
        51 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fc9c8); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fc9cf); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fc9d0); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fc9d4); }
            let Some(t74) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fc9d5); };
            c.logic(t74 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let r75 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r75);
            blk = 54; continue;
        }
        53 => {
            let Some(t76) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fc9eb); };
            c.w32(1, t76);
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fc9ee); };
            c.w64(1, t77);
            blk = 54; continue;
        }
        54 => {
            return RegfileExit::jump(c.r64(1));
        }
        55 => {
            let Some(t78) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fc9f8); };
            c.w32(1, t78);
            let r79 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r79);
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 56; continue;
        }
        56 => {
            let r80 = c.r64(1) & 0xfffffffffffffffdu64;
            c.w64(1, r80);
            c.logic(r80, 64);
            let Some(t81) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fca0b); };
            c.w32(9, t81);
            let r82 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r82);
            let Some(t83) = c.ld8(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fca12); };
            c.logic(t83 & 0x1u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70007fca1d); }
            c.w64(9, c.r64(1));
            c.w64(8, c.r64(7).wrapping_add(0x17));
            let r84 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fca2e); };
            c.logic(t85 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r86 = c.r64(9) & c.r64(7);
            c.w64(9, r86);
            c.logic(r86, 64);
            let Some(t87) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fca3f); };
            c.logic(t87 & 0x4u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return c.call(0x70007f1834, 0x70007fca4b, 0x70007fca46);
        }
        60 => {
            let Some(t88) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fca4b); };
            c.logic(t88 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            let r89 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r89);
            blk = 63; continue;
        }
        62 => {
            let Some(t90) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fca61); };
            c.w32(1, t90);
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fca64); };
            c.w64(1, t91);
            blk = 63; continue;
        }
        63 => {
            return RegfileExit::jump(c.r64(1));
        }
        64 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fca6e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fca72); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fca74); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fca75); }
            let r92 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r92);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fca78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fca79); }
            c.w32(0, 0x1u64);
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x1e38)) else { return RegfileExit::fault(0x70007fca7f); };
            c.w64(3, t93);
            return c.call(0x700086ed74, 0x70007fca8b, 0x70007fca86);
        }
        65 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fca8e); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fca95); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fca96); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fca9a); }
            let Some(t94) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fca9b); };
            c.logic(t94 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let r95 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r95);
            blk = 68; continue;
        }
        67 => {
            let Some(t96) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fcab1); };
            c.w32(1, t96);
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fcab4); };
            c.w64(1, t97);
            blk = 68; continue;
        }
        68 => {
            return RegfileExit::jump(c.r64(1));
        }
        69 => {
            let Some(t98) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fcabe); };
            c.w32(3, t98);
            let r99 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r99);
            let Some(t100) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fcac4); };
            c.w32(3, t100);
            let r101 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r101);
            let Some(t102) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fcaca); };
            c.w32(1, t102);
            let r103 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r103);
            let Some(t104) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fcad0); };
            c.cmp(t104, 0xa7u64, 16);
            if c.cc_ne() { blk = 78; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t105) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fcadc); };
            c.w32(1, t105);
            c.logic(c.r8(1) & 0x1eu64, 8);
            if c.cc_ne() { blk = 36; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t106) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fcae8); };
            c.w32(1, t106);
            let r107 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r107);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70007fcaf0); }
            c.w64(12, c.r64(1));
            c.w64(8, c.r64(7).wrapping_add(0x17));
            let r108 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r108);
            c.logic(r108, 64);
            let Some(t109) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fcb01); };
            c.logic(t109 & 0x2u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r110 = c.r64(12) & c.r64(7);
            c.w64(12, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fcb13); };
            c.logic(t111 & 0x4u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            return c.call(0x70007f1834, 0x70007fcb20, 0x70007fcb1b);
        }
        74 => {
            let Some(t112) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fcb20); };
            c.logic(t112 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            let r113 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r113);
            blk = 77; continue;
        }
        76 => {
            let Some(t114) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fcb36); };
            c.w32(1, t114);
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fcb39); };
            c.w64(1, t115);
            blk = 77; continue;
        }
        77 => {
            return RegfileExit::jump(c.r64(1));
        }
        78 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fcb43); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fcb47); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fcb49); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fcb4a); }
            let r116 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r116);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fcb4d); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fcb4e); }
            c.w32(0, 0x1u64);
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x1e30)) else { return RegfileExit::fault(0x70007fcb54); };
            c.w64(3, t117);
            return c.call(0x700086ed74, 0x70007fcb60, 0x70007fcb5b);
        }
        79 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fcb63); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fcb6a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fcb6b); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fcb6f); }
            let Some(t118) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fcb70); };
            c.logic(t118 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            let r119 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r119);
            blk = 82; continue;
        }
        81 => {
            let Some(t120) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fcb86); };
            c.w32(1, t120);
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fcb89); };
            c.w64(1, t121);
            blk = 82; continue;
        }
        82 => {
            return RegfileExit::jump(c.r64(1));
        }
        83 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fcb95); };
            c.w64(3, t122);
            return c.call(0x700086ed74, 0x70007fcba1, 0x70007fcb9c);
        }
        84 => {
            return RegfileExit::fault(0x70007fcba1);
        }
        85 => {
            return RegfileExit::fault(0x70007fcba2);
        }
        86 => {
            return RegfileExit::fault(0x70007fcba3);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fcbb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(1, c.r64(0).wrapping_add(0x1));
            c.w64(10, c.r64(4));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fcbba); };
            let r1 = c.sub(c.r64(10), t0, 64);
            c.w64(10, r1);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(1), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fcbcb); }
            c.w64(2, c.r64(1));
            let r2 = c.neg(c.r64(2), 64);
            c.w64(2, r2);
            c.w64(3, c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x8));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fcbdf); };
            if !c.push64(t3) { return RegfileExit::fault(0x70007fcbdf); }
            let r4 = c.inc(c.r64(2), 64);
            c.w64(2, r4);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_l() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fcbea); }
            return RegfileExit::jump(0x70007f2274);
        }
        5 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fcbf3); };
            c.w64(3, t5);
            return RegfileExit::jump(0x700086ed74);
        }
        6 => {
            return RegfileExit::fault(0x70007fcbff);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fcc14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(1, c.r64(0).wrapping_add(0x1));
            c.w64(10, c.r64(4));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fcc1a); };
            let r1 = c.sub(c.r64(10), t0, 64);
            c.w64(10, r1);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(1), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fcc2b); }
            let r2 = c.dec(c.r64(1), 64);
            c.w64(1, r2);
            c.w64(2, c.r64(1));
            let r3 = c.neg(c.r64(2), 64);
            c.w64(2, r3);
            c.w64(3, c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x8));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fcc42); };
            if !c.push64(t4) { return RegfileExit::fault(0x70007fcc42); }
            let r5 = c.inc(c.r64(2), 64);
            c.w64(2, r5);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_l() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fcc4d); };
            if !c.push64(t6) { return RegfileExit::fault(0x70007fcc4d); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fcc54); }
            return RegfileExit::jump(0x70007f2134);
        }
        5 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fcc5d); };
            c.w64(3, t7);
            return RegfileExit::jump(0x700086ed74);
        }
        6 => {
            return RegfileExit::fault(0x70007fcc69);
        }
        7 => {
            return RegfileExit::fault(0x70007fcc6a);
        }
        8 => {
            return RegfileExit::fault(0x70007fcc6b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fcc74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.dec(c.r32(0), 32);
            c.w32(0, r0);
            c.w32(1, c.r64(0).wrapping_add(0x1));
            c.w64(10, c.r64(4));
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fcc7c); };
            let r2 = c.sub(c.r64(10), t1, 64);
            c.w64(10, r2);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(1), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fcc8d); }
            c.w64(2, c.r64(1));
            let r3 = c.neg(c.r64(2), 64);
            c.w64(2, r3);
            c.w64(3, c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x8));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fcca1); };
            if !c.push64(t4) { return RegfileExit::fault(0x70007fcca1); }
            let r5 = c.inc(c.r64(2), 64);
            c.w64(2, r5);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_l() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007fccac); };
            c.w64(3, t6);
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fccb0); }
            return RegfileExit::jump(0x70007f4db4);
        }
        5 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fccb9); };
            c.w64(3, t7);
            return RegfileExit::jump(0x700086ed74);
        }
        6 => {
            return RegfileExit::fault(0x70007fccc5);
        }
        7 => {
            return RegfileExit::fault(0x70007fccc6);
        }
        8 => {
            return RegfileExit::fault(0x70007fccc7);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fccd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(10, c.r64(4));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fccd7); };
            let r1 = c.sub(c.r64(10), t0, 64);
            c.w64(10, r1);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(0), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fcce8); }
            c.w64(8, c.r64(0));
            let r2 = c.neg(c.r64(8), 64);
            c.w64(8, r2);
            c.w64(1, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fccfd); };
            if !c.push64(t3) { return RegfileExit::fault(0x70007fccfd); }
            let r4 = c.inc(c.r64(8), 64);
            c.w64(8, r4);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(8), c.r64(0), 64);
            if c.cc_l() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fcd09); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fcd0b); }
            return RegfileExit::jump(0x70007f7214);
        }
        5 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fcd14); };
            c.w64(3, t5);
            return RegfileExit::jump(0x700086ed74);
        }
        6 => {
            return RegfileExit::fault(0x70007fcd20);
        }
        7 => {
            return RegfileExit::fault(0x70007fcd21);
        }
        8 => {
            return RegfileExit::fault(0x70007fcd22);
        }
        9 => {
            return RegfileExit::fault(0x70007fcd23);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fcd34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(10, c.r64(4));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fcd37); };
            let r1 = c.sub(c.r64(10), t0, 64);
            c.w64(10, r1);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(0), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fcd48); }
            c.w64(8, c.r64(0));
            let r2 = c.neg(c.r64(8), 64);
            c.w64(8, r2);
            c.w64(1, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fcd5d); };
            if !c.push64(t3) { return RegfileExit::fault(0x70007fcd5d); }
            let r4 = c.inc(c.r64(8), 64);
            c.w64(8, r4);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(8), c.r64(0), 64);
            if c.cc_l() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fcd69); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fcd6b); }
            return RegfileExit::jump(0x7000824374);
        }
        5 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fcd74); };
            c.w64(3, t5);
            return RegfileExit::jump(0x700086ed74);
        }
        6 => {
            return RegfileExit::fault(0x70007fcd80);
        }
        7 => {
            return RegfileExit::fault(0x70007fcd81);
        }
        8 => {
            return RegfileExit::fault(0x70007fcd82);
        }
        9 => {
            return RegfileExit::fault(0x70007fcd83);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fcd94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(10, c.r64(4));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fcd97); };
            let r1 = c.sub(c.r64(10), t0, 64);
            c.w64(10, r1);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(0), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fcda8); }
            let r2 = c.dec(c.r32(0), 32);
            c.w32(0, r2);
            c.w64(8, c.r64(0));
            let r3 = c.neg(c.r64(8), 64);
            c.w64(8, r3);
            c.w64(1, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fcdbf); };
            if !c.push64(t4) { return RegfileExit::fault(0x70007fcdbf); }
            let r5 = c.inc(c.r64(8), 64);
            c.w64(8, r5);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(8), c.r64(0), 64);
            if c.cc_l() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fcdcb); }
            let Some(t6) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007fcdcd); };
            c.w64(3, t6);
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fcdd1); }
            return RegfileExit::jump(0x70007f7354);
        }
        5 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fcdda); };
            c.w64(3, t7);
            return RegfileExit::jump(0x700086ed74);
        }
        6 => {
            return RegfileExit::fault(0x70007fcde6);
        }
        7 => {
            return RegfileExit::fault(0x70007fcde7);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fcdf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fcdf4); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fcdf8); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fcdfe); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fce04); };
            c.w32(10, t5);
            let r6 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r6);
            let Some(t7) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fce0b); };
            c.cmp(t7, 0x69u64, 16);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fce13); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            c.w64(3, c.r64(3).wrapping_add(0x3f));
            blk = 3; continue;
        }
        2 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2d60)) else { return RegfileExit::fault(0x70007fce1f); };
            c.w64(10, t10);
            let Some(t11) = c.ld64(c.r64(10)) else { return RegfileExit::fault(0x70007fce26); };
            c.w64(3, t11);
            blk = 3; continue;
        }
        3 => {
            let r12 = c.add(c.r64(3), 0xe0u64, 64);
            c.w64(3, r12);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fce30); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70007fce31); };
            c.w64(15, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fce38); };
            c.w64(12, t14);
            let Some(t15) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fce3c); };
            c.w64(9, (t15 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t16) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fce43); };
            c.w32(10, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(10).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fce48); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::fault(0x70007fce4e);
        }
        5 => {
            return RegfileExit::fault(0x70007fce4f);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fce54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fce54); };
            c.w64(12, t0);
            let Some(t1) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fce58); };
            c.w64(9, (t1 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x20u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fce69); };
            c.w32(3, t2);
            c.w64(8, c.r64(9));
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1268)) else { return RegfileExit::fault(0x70007fce71); };
            c.w64(1, t3);
            c.cmp(c.r8(3), 0x3u64, 8);
            if c.cc_a() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let r4 = c.inc(c.r32(9), 32);
            c.w32(9, r4);
            c.logic(c.r8(3) & 0x1u64, 8);
            let Some(t5) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fce83); };
            c.w32(3, t5);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let r6 = c.add(c.r64(1), 0xc5u64, 64);
            c.w64(1, r6);
            blk = 5; continue;
        }
        4 => {
            let r7 = c.add(c.r64(1), 0x18au64, 64);
            c.w64(1, r7);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r8(3), 0xffffffffffffffa8u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r8(3), 0xffffffffffffffaeu64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r8(3), 0xffffffffffffff88u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, c.r64(8));
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld8(c.r64(1).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70007fceb6); };
            c.w32(10, t8);
            let r9 = c.add(c.r32(9), c.r32(10), 32);
            c.w32(9, r9);
            blk = 10; continue;
        }
        10 => {
            let r10 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r10);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70007fcec1); }
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fcec5); };
            c.w64(3, t11);
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fcec9); };
            c.w32(3, t12);
            let r13 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fcecf); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fced5); };
            c.w32(10, t16);
            let r17 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r17);
            let Some(t18) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fcedc); };
            c.cmp(t18, 0x69u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fcee4); };
            c.w32(3, t19);
            let r20 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r20);
            c.w64(3, c.r64(3).wrapping_add(0x3f));
            blk = 13; continue;
        }
        12 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2d60)) else { return RegfileExit::fault(0x70007fcef0); };
            c.w64(10, t21);
            let Some(t22) = c.ld64(c.r64(10)) else { return RegfileExit::fault(0x70007fcef7); };
            c.w64(3, t22);
            blk = 13; continue;
        }
        13 => {
            let r23 = c.add(c.r64(3), 0xe0u64, 64);
            c.w64(3, r23);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fcf01); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70007fcf02); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fcf09); };
            c.w64(12, t25);
            let Some(t26) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fcf0d); };
            c.w64(9, (t26 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t27) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007fcf14); };
            c.w32(10, t27);
            let Some(t28) = c.ld64(c.r64(15).wrapping_add(c.r64(10).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fcf19); };
            c.w64(1, t28);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            c.w32(9, 0x21u64);
            blk = 10; continue;
        }
        15 => {
            c.w32(2, 0x18u64);
            return c.call(0x7000824214, 0x70007fcf31, 0x70007fcf2c);
        }
        16 => {
            return RegfileExit::fault(0x70007fcf31);
        }
        17 => {
            return RegfileExit::fault(0x70007fcf32);
        }
        18 => {
            return RegfileExit::fault(0x70007fcf33);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fcf54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fcf54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fcf58); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1e08)) else { return RegfileExit::fault(0x70007fcf5c); };
            c.w64(3, t0);
            return c.call(0x700086ed74, 0x70007fcf68, 0x70007fcf63);
        }
        1 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fcf6b); }
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return c.ret(0x0, 0x70007fcf71);
        }
        3 => {
            if !c.leave() { return RegfileExit::fault(0x70007fcf72); }
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fcf73); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fcf79); };
            c.w64(3, (t3 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(0, c.r64(0).wrapping_add(c.r64(3)).wrapping_add(0x3f));
            if !c.st64(c.r64(4), c.r64(0)) { return RegfileExit::fault(0x70007fcf85); }
            return c.ret(0x0, 0x70007fcf89);
        }
        4 => {
            return RegfileExit::fault(0x70007fcf8a);
        }
        5 => {
            return RegfileExit::fault(0x70007fcf8b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fcf94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fcf94); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fcf9b); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fcfa2); };
            c.w32(1, t4);
            c.logic(c.r8(1) & 0x1eu64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let ea5 = c.r64(8).wrapping_add(0x7);
            let Some(t6) = c.ld32(ea5) else { return RegfileExit::fault(0x70007fcfaf); };
            let r7 = c.inc(t6, 32);
            if !c.st32(ea5, r7) { return RegfileExit::fault(0x70007fcfaf); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fcfb3); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fcfb5); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fcfb9); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fcfba); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fcfbb); }
            if !c.st16(c.r64(12).wrapping_add(0x1f), 0x0u64) { return RegfileExit::fault(0x70007fcfbc); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fcfc4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fcfc6); }
            c.w64(10, c.r64(4));
            let r8 = c.sub(c.r64(10), c.r64(3), 64);
            c.w64(10, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007fcfce); };
            c.cmp(c.r64(10), t9, 64);
            if c.cc_b() { blk = 37; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fcfd8); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fcfda); };
            c.w64(0, t10);
            return c.ret(0x0, 0x70007fcfe1);
        }
        3 => {
            let r11 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            c.logic(c.r8(1) & 0x6u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 4; continue;
        }
        4 => {
            let r12 = c.r64(1) & 0x7u64;
            c.w64(1, r12);
            c.logic(r12, 64);
            c.cmp(c.r32(1), 0x4u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fcffc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd000); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd002); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fd003); }
            let r13 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r13);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd006); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd007); }
            c.w32(0, 0x1u64);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1e40)) else { return RegfileExit::fault(0x70007fd00d); };
            c.w64(3, t14);
            return c.call(0x700086ed74, 0x70007fd019, 0x70007fd014);
        }
        6 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd01c); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fd023); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fd024); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd028); }
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd029); };
            c.logic(t15 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let r16 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r16);
            blk = 9; continue;
        }
        8 => {
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd03f); };
            c.w32(1, t17);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd042); };
            c.w64(1, t18);
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd055); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd059); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd05b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fd05c); }
            let r19 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r19);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd05f); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd060); }
            c.w32(0, 0x1u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x1e28)) else { return RegfileExit::fault(0x70007fd066); };
            c.w64(3, t20);
            return c.call(0x700086ed74, 0x70007fd072, 0x70007fd06d);
        }
        12 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd075); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fd07c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fd07d); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd081); }
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd082); };
            c.logic(t21 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let r22 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r22);
            blk = 15; continue;
        }
        14 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd098); };
            c.w32(1, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd09b); };
            c.w64(1, t24);
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::jump(c.r64(1));
        }
        16 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd0ae); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd0b2); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd0b4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fd0b5); }
            let r25 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r25);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd0b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd0b9); }
            c.w32(0, 0x1u64);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1e20)) else { return RegfileExit::fault(0x70007fd0bf); };
            c.w64(3, t26);
            return c.call(0x700086ed74, 0x70007fd0cb, 0x70007fd0c6);
        }
        18 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd0ce); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fd0d5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fd0d6); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd0da); }
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd0db); };
            c.logic(t27 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let r28 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r28);
            blk = 21; continue;
        }
        20 => {
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd0f1); };
            c.w32(1, t29);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd0f4); };
            c.w64(1, t30);
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::jump(c.r64(1));
        }
        22 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fd0fe); };
            c.w32(1, t31);
            let r32 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r32);
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 23; continue;
        }
        23 => {
            let r33 = c.r64(1) & 0xfffffffffffffffdu64;
            c.w64(1, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fd112); };
            c.w32(9, t34);
            let r35 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r35);
            let Some(t36) = c.ld8(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd119); };
            c.logic(t36 & 0x1u64, 8);
            if c.cc_ne() { blk = 31; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70007fd124); }
            c.w64(9, c.r64(1));
            c.w64(8, c.r64(7).wrapping_add(0x17));
            let r37 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r37);
            c.logic(r37, 64);
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd135); };
            c.logic(t38 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r39 = c.r64(9) & c.r64(7);
            c.w64(9, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd146); };
            c.logic(t40 & 0x4u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            return c.call(0x70007f1834, 0x70007fd152, 0x70007fd14d);
        }
        27 => {
            let Some(t41) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd152); };
            c.logic(t41 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let r42 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r42);
            blk = 30; continue;
        }
        29 => {
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd168); };
            c.w32(1, t43);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd16b); };
            c.w64(1, t44);
            blk = 30; continue;
        }
        30 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fd173); }
            return c.ret(0x0, 0x70007fd174);
        }
        31 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd175); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd179); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd17b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fd17c); }
            let r45 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r45);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd17f); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd180); }
            c.w32(0, 0x1u64);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1e38)) else { return RegfileExit::fault(0x70007fd186); };
            c.w64(3, t46);
            return c.call(0x700086ed74, 0x70007fd192, 0x70007fd18d);
        }
        32 => {
            c.w64(1, c.r64(0));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd195); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fd19c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fd19d); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd1a1); }
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd1a2); };
            c.logic(t47 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let r48 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r48);
            blk = 35; continue;
        }
        34 => {
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd1b8); };
            c.w32(1, t49);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd1bb); };
            c.w64(1, t50);
            blk = 35; continue;
        }
        35 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fd1c3); }
            return c.ret(0x0, 0x70007fd1c4);
        }
        36 => {
            return RegfileExit::fault(0x70007fd1c5);
        }
        37 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fd1c6); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd1c8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd1cc); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fd1ce); }
            let r51 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r51);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fd1d1); }
            c.w32(0, 0x1u64);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x21b0)) else { return RegfileExit::fault(0x70007fd1d7); };
            c.w64(3, t52);
            return c.call(0x700086ed74, 0x70007fd1e3, 0x70007fd1de);
        }
        38 => {
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fd1e3); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd1e7); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fd1e8); };
            c.w64(0, t53);
            return c.ret(0x0, 0x70007fd1ef);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fd1f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007fd1f4); };
            c.w64(6, t0);
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd1f8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd1fc); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x1e08)) else { return RegfileExit::fault(0x70007fd200); };
            c.w64(3, t1);
            return c.call(0x700086ed74, 0x70007fd20c, 0x70007fd207);
        }
        1 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd20f); }
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return c.ret(0x0, 0x70007fd215);
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd216); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fd21c); };
            c.w64(3, (t4 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(0, c.r64(0).wrapping_add(c.r64(3)).wrapping_add(0x3f));
            if !c.st64(c.r64(4), c.r64(0)) { return RegfileExit::fault(0x70007fd228); }
            return c.ret(0x0, 0x70007fd22c);
        }
        4 => {
            return RegfileExit::fault(0x70007fd22d);
        }
        5 => {
            return RegfileExit::fault(0x70007fd22e);
        }
        6 => {
            return RegfileExit::fault(0x70007fd22f);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fd234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fd234); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fd238); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let ea3 = c.r64(8).wrapping_add(0x7);
            let Some(t4) = c.ld32(ea3) else { return RegfileExit::fault(0x70007fd23f); };
            let r5 = c.add(t4, c.r32(1), 32);
            if !c.st32(ea3, r5) { return RegfileExit::fault(0x70007fd23f); }
            if c.cc_ge() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let r6 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r6);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fd24b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd24c); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007fd24d); };
            c.w64(6, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fd251); };
            if !c.push64(t8) { return RegfileExit::fault(0x70007fd251); }
            c.w32(0, 0x1u64);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2140)) else { return RegfileExit::fault(0x70007fd259); };
            c.w64(3, t9);
            return c.call(0x700086ed74, 0x70007fd265, 0x70007fd260);
        }
        2 => {
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd265); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fd266); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fd26d); };
            c.w64(8, t10);
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_ge() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(3, c.r64(8));
            blk = 5; continue;
        }
        5 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd280); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fd281); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fd288); }
            return c.ret(0x0, 0x70007fd28a);
        }
        6 => {
            return RegfileExit::fault(0x70007fd28b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fd294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fd294); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fd298); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd29e); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fd2a4); };
            c.w32(10, t5);
            let r6 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r6);
            let Some(t7) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd2ab); };
            c.cmp(t7, 0x5eu64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::jump(0x70007fcdf4);
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd2bc); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fd2c2); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd2c9); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fd2d0); };
            c.w32(10, t14);
            let r15 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r15);
            let Some(t16) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd2d7); };
            c.cmp(t16, 0xa7u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t17) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fd2e4); };
            c.w64(9, (t17 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70007fd2eb); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x1258)) else { return RegfileExit::fault(0x70007fd2ef); };
            c.w64(11, t18);
            c.cmp(c.r64(9), 0x20u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let r19 = c.sub(c.r64(9), 0x21u64, 64);
            c.w64(9, r19);
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fd304); };
            c.w64(12, t20);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd308); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd309); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd30d); }
            c.w64(10, c.r64(4));
            let r21 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r21);
            let r22 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r22);
            c.logic(r22, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007fd31a); }
            c.w64(1, c.r64(3));
            c.w64(2, c.r64(9));
            c.w64(8, c.r64(12));
            c.w64(10, 0x70007fd32fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fd32f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fd333); }
            return c.call(c.r64(11), 0x70007fd33a, 0x70007fd337);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fd33a); }
            let Some(t23) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fd342); };
            c.w64(4, t23);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd34a); }
            c.w64(3, c.r64(3).wrapping_add(c.r64(0)).wrapping_add(0x3f));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd350); }
            return RegfileExit::jump(c.r64(3));
        }
        7 => {
            return RegfileExit::fault(0x70007fd353);
        }
        8 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd359); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd35d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd35f); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd360); }
            c.w32(0, 0x1u64);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x1e30)) else { return RegfileExit::fault(0x70007fd366); };
            c.w64(3, t24);
            return c.call(0x700086ed74, 0x70007fd372, 0x70007fd36d);
        }
        10 => {
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd372); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd376); }
            blk = 0; continue;
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fd394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fd394); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fd398); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd39e); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fd3a4); };
            c.w32(10, t5);
            let r6 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r6);
            let Some(t7) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd3ab); };
            c.cmp(t7, 0x5eu64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::jump(0x70007fce54);
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd3bc); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fd3c2); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd3c9); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fd3d0); };
            c.w32(10, t14);
            let r15 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r15);
            let Some(t16) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd3d7); };
            c.cmp(t16, 0xa7u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t17) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fd3e4); };
            c.w64(9, (t17 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70007fd3eb); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x1260)) else { return RegfileExit::fault(0x70007fd3ef); };
            c.w64(11, t18);
            c.cmp(c.r64(9), 0x20u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let r19 = c.sub(c.r64(9), 0x21u64, 64);
            c.w64(9, r19);
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fd404); };
            c.w64(12, t20);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd408); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd409); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd40d); }
            c.w64(10, c.r64(4));
            let r21 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r21);
            let r22 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r22);
            c.logic(r22, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007fd41a); }
            c.w64(1, c.r64(3));
            c.w64(2, c.r64(9));
            c.w64(8, c.r64(12));
            c.w64(10, 0x70007fd42fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fd42f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fd433); }
            return c.call(c.r64(11), 0x70007fd43a, 0x70007fd437);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fd43a); }
            let Some(t23) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fd442); };
            c.w64(4, t23);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd44a); }
            c.w64(3, c.r64(3).wrapping_add(c.r64(0)).wrapping_add(0x3f));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd450); }
            return RegfileExit::jump(c.r64(3));
        }
        7 => {
            return RegfileExit::fault(0x70007fd453);
        }
        8 => {
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x1258)) else { return RegfileExit::fault(0x70007fd457); };
            c.w64(11, t24);
            blk = 5; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd460); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd464); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd466); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd467); }
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1e30)) else { return RegfileExit::fault(0x70007fd46d); };
            c.w64(3, t25);
            return c.call(0x700086ed74, 0x70007fd479, 0x70007fd474);
        }
        10 => {
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd479); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd47d); }
            blk = 0; continue;
        }
        11 => {
            return RegfileExit::fault(0x70007fd483);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fd494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fd494); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fd498); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd49e); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd4a4); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fd4aa); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd4b1); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fd4b8); };
            c.w32(10, t11);
            let r12 = c.add(c.r64(10), c.r64(14), 64);
            c.w64(10, r12);
            let Some(t13) = c.ld16(c.r64(10).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd4bf); };
            c.cmp(t13, 0xa7u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fd4cc); };
            c.w64(9, (t14 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70007fd4d3); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1260)) else { return RegfileExit::fault(0x70007fd4d7); };
            c.w64(11, t15);
            let r16 = c.sub(c.r64(9), 0x21u64, 64);
            c.w64(9, r16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fd4e2); };
            c.w64(12, t17);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd4e6); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd4e7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd4eb); }
            c.w64(10, c.r64(4));
            let r18 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r18);
            let r19 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r19);
            c.logic(r19, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007fd4f8); }
            c.w64(1, c.r64(3));
            c.w64(2, c.r64(9));
            c.w64(8, c.r64(12));
            c.w64(10, 0x70007fd50du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fd50d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fd511); }
            return c.call(c.r64(11), 0x70007fd518, 0x70007fd515);
        }
        2 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fd518); }
            let Some(t20) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fd520); };
            c.w64(4, t20);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd528); }
            c.w64(3, c.r64(3).wrapping_add(c.r64(0)).wrapping_add(0x3f));
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd52e); }
            if !c.st16(c.r64(12).wrapping_add(0x1f), 0x0u64) { return RegfileExit::fault(0x70007fd52f); }
            if !c.st64(c.r64(4), c.r64(3)) { return RegfileExit::fault(0x70007fd537); }
            return c.ret(0x0, 0x70007fd53b);
        }
        3 => {
            return RegfileExit::fault(0x70007fd53c);
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd53d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fd541); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fd543); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd544); }
            c.w32(0, 0x1u64);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x1e30)) else { return RegfileExit::fault(0x70007fd54a); };
            c.w64(3, t21);
            return c.call(0x700086ed74, 0x70007fd556, 0x70007fd551);
        }
        5 => {
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fd556); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fd55a); }
            blk = 0; continue;
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fd574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd574); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fd578); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fd57e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd585); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007fd58c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70007fd590); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007fd594); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70007fd598); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(8), 0xaau64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(13).wrapping_add(0x63f0));
            let Some(t5) = c.ld64(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4))) else { return RegfileExit::fault(0x70007fd5b9); };
            c.w64(8, t5);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 21; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fd5c5); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd5cc); };
            c.w32(9, t8);
            c.cmp(c.r32(9), 0x5cu64, 32);
            if c.cc_l() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x85u64, 32);
            if c.cc_l() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x85u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(9), 0x9du64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x9eu64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), 0x5cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(9), 0x5eu64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(9), 0x4au64, 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x4au64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(9), 0x53u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(9), 0x48u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x49u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd62d); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            blk = 21; continue;
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70007fd636); };
            c.w64(8, t11);
            blk = 21; continue;
        }
        17 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd63f); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            blk = 21; continue;
        }
        18 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70007fd648); };
            c.w64(8, t14);
            blk = 21; continue;
        }
        19 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fd651); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            blk = 21; continue;
        }
        20 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70007fd65a); };
            c.w64(8, t17);
            blk = 21; continue;
        }
        21 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fd661); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fd668); };
            c.w32(11, t20);
            let r21 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r21);
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fd66f); };
            c.cmp(t22, c.r32(11), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t23) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fd67c); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd683); };
            c.cmp(t25, 0x81u64, 16);
            if c.cc_e() { blk = 57; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fd691); };
            c.w32(12, t26);
            c.logic(c.r8(12) & 0x1eu64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(12) & 0x6u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fd6a9); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            let r29 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r29);
            c.logic(r29, 64);
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fd6ba); };
            c.w32(9, t30);
            let r31 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r31);
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fd6c1); };
            c.logic(t32 & 0x1u64, 8);
            if c.cc_ne() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70007fd6c8); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r33 = c.r64(9) & c.r64(7);
            c.w64(9, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd6dc); };
            c.logic(t34 & 0x4u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t35) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fd6e7); };
            c.w64(5, t35);
            c.w64(1, c.r64(8));
            let r36 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r36);
            let Some(t37) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd6f2); };
            c.logic(t37 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let r38 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r38);
            blk = 32; continue;
        }
        31 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd701); };
            c.w32(1, t39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd704); };
            c.w64(1, t40);
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::jump(c.r64(1));
        }
        33 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd70e); }
            c.w32(0, 0x1u64);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x1e38)) else { return RegfileExit::fault(0x70007fd714); };
            c.w64(3, t41);
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x70007fd723, 0x70007fd71e);
        }
        34 => {
            let Some(t42) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fd723); };
            c.w64(5, t42);
            c.w64(1, c.r64(0));
            let Some(t43) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fd72a); };
            c.w64(7, t43);
            let Some(t44) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fd72f); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd734); };
            c.w64(0, t45);
            let Some(t46) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fd739); };
            c.w64(6, t46);
            let r47 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r47);
            let Some(t48) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd741); };
            c.logic(t48 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            let r49 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r49);
            blk = 37; continue;
        }
        36 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd750); };
            c.w32(1, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd753); };
            c.w64(1, t51);
            blk = 37; continue;
        }
        37 => {
            return RegfileExit::jump(c.r64(1));
        }
        38 => {
            let r52 = c.r32(12) & 0x7u64;
            c.w32(12, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x70007fd777);
        }
        42 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd778); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1e20)) else { return RegfileExit::fault(0x70007fd779); };
            c.w64(3, t53);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007fd78a, 0x70007fd785);
        }
        43 => {
            let Some(t54) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fd78a); };
            c.w64(5, t54);
            c.w64(1, c.r64(0));
            let Some(t55) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fd791); };
            c.w64(7, t55);
            let Some(t56) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fd796); };
            c.w64(2, t56);
            let Some(t57) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd79b); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fd7a0); };
            c.w64(6, t58);
            let r59 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r59);
            let Some(t60) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd7a8); };
            c.logic(t60 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let r61 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r61);
            blk = 46; continue;
        }
        45 => {
            let Some(t62) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd7b7); };
            c.w32(1, t62);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd7ba); };
            c.w64(1, t63);
            blk = 46; continue;
        }
        46 => {
            return RegfileExit::jump(c.r64(1));
        }
        47 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd7c4); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1e28)) else { return RegfileExit::fault(0x70007fd7c5); };
            c.w64(3, t64);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007fd7d6, 0x70007fd7d1);
        }
        48 => {
            let Some(t65) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fd7d6); };
            c.w64(5, t65);
            c.w64(1, c.r64(0));
            let Some(t66) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fd7dd); };
            c.w64(7, t66);
            let Some(t67) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fd7e2); };
            c.w64(2, t67);
            let Some(t68) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd7e7); };
            c.w64(0, t68);
            let Some(t69) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fd7ec); };
            c.w64(6, t69);
            let r70 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r70);
            let Some(t71) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd7f4); };
            c.logic(t71 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let r72 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r72);
            blk = 51; continue;
        }
        50 => {
            let Some(t73) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd803); };
            c.w32(1, t73);
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd806); };
            c.w64(1, t74);
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::jump(c.r64(1));
        }
        52 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd810); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1e40)) else { return RegfileExit::fault(0x70007fd811); };
            c.w64(3, t75);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007fd822, 0x70007fd81d);
        }
        53 => {
            let Some(t76) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fd822); };
            c.w64(5, t76);
            c.w64(1, c.r64(0));
            let Some(t77) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fd829); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fd82e); };
            c.w64(2, t78);
            let Some(t79) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd833); };
            c.w64(0, t79);
            let Some(t80) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fd838); };
            c.w64(6, t80);
            let r81 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r81);
            let Some(t82) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd840); };
            c.logic(t82 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let r83 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r83);
            blk = 56; continue;
        }
        55 => {
            let Some(t84) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd84f); };
            c.w32(1, t84);
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd852); };
            c.w64(1, t85);
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::jump(c.r64(1));
        }
        57 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70007fd85c); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r86 = c.r64(12) & c.r64(7);
            c.w64(12, r86);
            c.logic(r86, 64);
            let Some(t87) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd870); };
            c.logic(t87 & 0x4u64, 8);
            if c.cc_ne() { blk = 72; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(9), 0x5eu64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t88) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fd882); };
            c.w32(9, t88);
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007fd886); };
            c.cmp(t89, c.r32(9), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd88f); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x1e30)) else { return RegfileExit::fault(0x70007fd890); };
            c.w64(3, t90);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007fd8a1, 0x70007fd89c);
        }
        62 => {
            c.w64(8, c.r64(0));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fd8a4); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fd8a8); };
            c.w64(2, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fd8ac); };
            c.w64(0, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fd8b0); };
            c.w64(6, t94);
            blk = 63; continue;
        }
        63 => {
            let Some(t95) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fd8b4); };
            c.w64(5, t95);
            c.w64(1, c.r64(8));
            let r96 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r96);
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd8bf); };
            c.logic(t97 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            let r98 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r98);
            blk = 66; continue;
        }
        65 => {
            let Some(t99) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd8ce); };
            c.w32(1, t99);
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd8d1); };
            c.w64(1, t100);
            blk = 66; continue;
        }
        66 => {
            return RegfileExit::jump(c.r64(1));
        }
        67 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fd8db); }
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x1e10)) else { return RegfileExit::fault(0x70007fd8dc); };
            c.w64(3, t101);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007fd8ed, 0x70007fd8e8);
        }
        68 => {
            let Some(t102) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fd8ed); };
            c.w64(5, t102);
            c.w64(1, c.r64(0));
            let Some(t103) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fd8f4); };
            c.w64(7, t103);
            let Some(t104) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fd8f9); };
            c.w64(2, t104);
            let Some(t105) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd8fe); };
            c.w64(0, t105);
            let Some(t106) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fd903); };
            c.w64(6, t106);
            let r107 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r107);
            let Some(t108) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fd90b); };
            c.logic(t108 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let r109 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r109);
            blk = 71; continue;
        }
        70 => {
            let Some(t110) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fd91a); };
            c.w32(1, t110);
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fd91d); };
            c.w64(1, t111);
            blk = 71; continue;
        }
        71 => {
            return RegfileExit::jump(c.r64(1));
        }
        72 => {
            c.w32(8, c.r32(8));
            let r112 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r112);
            c.w64(12, 0xfffffffffffc0000u64);
            let r113 = c.r64(12) & c.r64(8);
            c.w64(12, r113);
            c.logic(r113, 64);
            let Some(t114) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd937); };
            c.logic(t114 & 0x2u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(15, c.r64(7).wrapping_add(0x17));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fd947); }
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007fd951, 0x70007fd94c);
        }
        74 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fd951); }
            blk = 59; continue;
        }
        75 => {
            c.w32(8, c.r32(8));
            let r115 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r115);
            c.w64(9, 0xfffffffffffc0000u64);
            let r116 = c.r64(9) & c.r64(8);
            c.w64(9, r116);
            c.logic(r116, 64);
            let Some(t117) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd968); };
            c.logic(t117 & 0x2u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(11, c.r64(7).wrapping_add(0x17));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fd977); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70007fd981, 0x70007fd97c);
        }
        77 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fd981); }
            blk = 29; continue;
        }
        78 => {
            return RegfileExit::jump(0x70007fd98c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fd994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70007fd994); };
            c.w64(3, t0);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70007fd99b); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r1 = c.r64(1) & c.r64(7);
            c.w64(1, r1);
            c.logic(r1, 64);
            let Some(t2) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd9ad); };
            c.logic(t2 & 0x4u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::jump(0x70007fd574);
        }
        3 => {
            c.w32(3, c.r32(3));
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            c.w64(1, 0xfffffffffffc0000u64);
            let r4 = c.r64(1) & c.r64(3);
            c.w64(1, r4);
            c.logic(r4, 64);
            let Some(t5) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fd9c7); };
            c.logic(t5 & 0x2u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(8, c.r64(7).wrapping_add(0x17));
            return c.call(0x70007f1594, 0x70007fd9d6, 0x70007fd9d1);
        }
        5 => {
            blk = 2; continue;
        }
        6 => {
            return RegfileExit::jump(0x70007fd9dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fd9f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fd9f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fd9f8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007fda01); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70007fda05); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007fda09); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70007fda0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70007fda11); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(5));
            let Some(t1) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fda1e); };
            c.w64(9, t1);
            blk = 3; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fda24); };
            c.w64(9, t2);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(11, c.r64(5));
            let r3 = c.add(c.r64(11), 0x18u64, 64);
            c.w64(11, r3);
            let Some(t4) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fda38); };
            c.w64(11, t4);
            blk = 6; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fda3e); };
            c.w64(11, t5);
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(12, c.r64(5));
            let r6 = c.add(c.r64(12), 0x18u64, 64);
            c.w64(12, r6);
            let Some(t7) = c.ld64(c.r64(12).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fda52); };
            c.w64(12, t7);
            blk = 9; continue;
        }
        8 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fda59); };
            c.w64(12, t8);
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fda60); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007fda61); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007fda63); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fda65); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1e48)) else { return RegfileExit::fault(0x70007fda67); };
            c.w64(3, t9);
            c.w32(0, 0x4u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x70007fda7b, 0x70007fda76);
        }
        10 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fda7f); };
            c.w64(7, t10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fda83); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007fda8a); };
            c.w32(8, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fda8f); };
            c.w64(9, t14);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fda98); };
            c.w64(8, t15);
            let r16 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fdaa3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fdaa4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fdaaa); }
            return c.ret(0x0, 0x70007fdaac);
        }
        13 => {
            let r17 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r17);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fdab4); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fdab5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fdabb); }
            return c.ret(0x0, 0x70007fdabd);
        }
        14 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fdabe); };
            c.w64(7, t18);
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fdac2); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fdac8); };
            c.w64(5, t21);
            let Some(t22) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fdacc); };
            c.w64(6, t22);
            let Some(t23) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fdad1); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fdad6); };
            c.w64(2, t24);
            let r25 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r25);
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fdadf); };
            c.logic(t26 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let r27 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r27);
            blk = 17; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007fdaee); };
            c.w32(1, t28);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fdaf1); };
            c.w64(1, t29);
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            return RegfileExit::jump(0x70007fdafc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fdb14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fdb14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fdb18); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1e50)) else { return RegfileExit::fault(0x70007fdb1c); };
            c.w64(3, t0);
            return c.call(0x700086ed74, 0x70007fdb28, 0x70007fdb23);
        }
        1 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fdb2b); }
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fdb2c); };
            c.w64(0, t1);
            return c.ret(0x8, 0x70007fdb31);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fdb54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.sub(c.r64(4), 0x80u64, 64);
            c.w64(4, r0);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007fdb5b); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x8), 1) { return RegfileExit::fault(0x70007fdb60); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x10), 2) { return RegfileExit::fault(0x70007fdb66); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x18), 3) { return RegfileExit::fault(0x70007fdb6c); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x20), 4) { return RegfileExit::fault(0x70007fdb72); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x28), 5) { return RegfileExit::fault(0x70007fdb78); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x30), 6) { return RegfileExit::fault(0x70007fdb7e); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x38), 7) { return RegfileExit::fault(0x70007fdb84); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x40), 8) { return RegfileExit::fault(0x70007fdb8a); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x48), 9) { return RegfileExit::fault(0x70007fdb91); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x50), 10) { return RegfileExit::fault(0x70007fdb98); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x58), 11) { return RegfileExit::fault(0x70007fdb9f); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x60), 12) { return RegfileExit::fault(0x70007fdba6); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x68), 13) { return RegfileExit::fault(0x70007fdbad); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x70), 14) { return RegfileExit::fault(0x70007fdbb4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fdbbb); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fdbbc); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fdbbd); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fdbbe); }
            if !c.push64(c.r64(4)) { return RegfileExit::fault(0x70007fdbbf); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fdbc0); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fdbc1); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fdbc2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fdbc3); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007fdbc5); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fdbc7); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007fdbc9); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fdbcb); }
            if !c.push64(c.r64(13)) { return RegfileExit::fault(0x70007fdbcd); }
            if !c.push64(c.r64(14)) { return RegfileExit::fault(0x70007fdbcf); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fdbd1); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x70007fdbd3); }
            c.w32(8, 0xffffffffu64);
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x100)) else { return RegfileExit::fault(0x70007fdbe0); };
            c.w64(9, t1);
            c.w64(15, c.r64(4).wrapping_add(0x108));
            let r2 = c.sub(c.r64(15), c.r64(5), 64);
            c.w64(15, r2);
            let r3 = c.neg(c.r64(15), 64);
            c.w64(15, r3);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70007fdc01); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007fdc08); };
            c.w64(7, t6);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fdc16); };
            c.w64(0, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(0));
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(15)) { return RegfileExit::fault(0x70007fdc1f); }
            c.w64(15, c.r64(13));
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(15)) { return RegfileExit::fault(0x70007fdc27); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1410)) else { return RegfileExit::fault(0x70007fdc2c); };
            c.w64(0, t8);
            c.w64(10, 0x70007fdc3au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fdc3a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fdc3e); }
            return c.call(c.r64(0), 0x70007fdc44, 0x70007fdc42);
        }
        3 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fdc44); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fdc4c); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(0).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fdc51); };
            c.w64(3, t10);
            let sp11 = c.r64(4);
            let Some(pv12) = c.ld64(sp11) else { return RegfileExit::fault(0x70007fdc55); };
            c.set_rsp(sp11.wrapping_add(8));
            let pea13 = c.r64(3).wrapping_add(0x88);
            if !c.st64(pea13, pv12) { c.set_rsp(sp11); return RegfileExit::fault(0x70007fdc55); }
            let sp14 = c.r64(4);
            let Some(pv15) = c.ld64(sp14) else { return RegfileExit::fault(0x70007fdc5b); };
            c.set_rsp(sp14.wrapping_add(8));
            let pea16 = c.r64(3).wrapping_add(0x80);
            if !c.st64(pea16, pv15) { c.set_rsp(sp14); return RegfileExit::fault(0x70007fdc5b); }
            let sp17 = c.r64(4);
            let Some(pv18) = c.ld64(sp17) else { return RegfileExit::fault(0x70007fdc61); };
            c.set_rsp(sp17.wrapping_add(8));
            let pea19 = c.r64(3).wrapping_add(0x78);
            if !c.st64(pea19, pv18) { c.set_rsp(sp17); return RegfileExit::fault(0x70007fdc61); }
            let sp20 = c.r64(4);
            let Some(pv21) = c.ld64(sp20) else { return RegfileExit::fault(0x70007fdc64); };
            c.set_rsp(sp20.wrapping_add(8));
            let pea22 = c.r64(3).wrapping_add(0x70);
            if !c.st64(pea22, pv21) { c.set_rsp(sp20); return RegfileExit::fault(0x70007fdc64); }
            let sp23 = c.r64(4);
            let Some(pv24) = c.ld64(sp23) else { return RegfileExit::fault(0x70007fdc67); };
            c.set_rsp(sp23.wrapping_add(8));
            let pea25 = c.r64(3).wrapping_add(0x68);
            if !c.st64(pea25, pv24) { c.set_rsp(sp23); return RegfileExit::fault(0x70007fdc67); }
            let sp26 = c.r64(4);
            let Some(pv27) = c.ld64(sp26) else { return RegfileExit::fault(0x70007fdc6a); };
            c.set_rsp(sp26.wrapping_add(8));
            let pea28 = c.r64(3).wrapping_add(0x60);
            if !c.st64(pea28, pv27) { c.set_rsp(sp26); return RegfileExit::fault(0x70007fdc6a); }
            let sp29 = c.r64(4);
            let Some(pv30) = c.ld64(sp29) else { return RegfileExit::fault(0x70007fdc6d); };
            c.set_rsp(sp29.wrapping_add(8));
            let pea31 = c.r64(3).wrapping_add(0x58);
            if !c.st64(pea31, pv30) { c.set_rsp(sp29); return RegfileExit::fault(0x70007fdc6d); }
            let sp32 = c.r64(4);
            let Some(pv33) = c.ld64(sp32) else { return RegfileExit::fault(0x70007fdc70); };
            c.set_rsp(sp32.wrapping_add(8));
            let pea34 = c.r64(3).wrapping_add(0x50);
            if !c.st64(pea34, pv33) { c.set_rsp(sp32); return RegfileExit::fault(0x70007fdc70); }
            let sp35 = c.r64(4);
            let Some(pv36) = c.ld64(sp35) else { return RegfileExit::fault(0x70007fdc73); };
            c.set_rsp(sp35.wrapping_add(8));
            let pea37 = c.r64(3).wrapping_add(0x48);
            if !c.st64(pea37, pv36) { c.set_rsp(sp35); return RegfileExit::fault(0x70007fdc73); }
            let sp38 = c.r64(4);
            let Some(pv39) = c.ld64(sp38) else { return RegfileExit::fault(0x70007fdc76); };
            c.set_rsp(sp38.wrapping_add(8));
            let pea40 = c.r64(3).wrapping_add(0x40);
            if !c.st64(pea40, pv39) { c.set_rsp(sp38); return RegfileExit::fault(0x70007fdc76); }
            let sp41 = c.r64(4);
            let Some(pv42) = c.ld64(sp41) else { return RegfileExit::fault(0x70007fdc79); };
            c.set_rsp(sp41.wrapping_add(8));
            let pea43 = c.r64(3).wrapping_add(0x38);
            if !c.st64(pea43, pv42) { c.set_rsp(sp41); return RegfileExit::fault(0x70007fdc79); }
            let sp44 = c.r64(4);
            let Some(pv45) = c.ld64(sp44) else { return RegfileExit::fault(0x70007fdc7c); };
            c.set_rsp(sp44.wrapping_add(8));
            let pea46 = c.r64(3).wrapping_add(0x30);
            if !c.st64(pea46, pv45) { c.set_rsp(sp44); return RegfileExit::fault(0x70007fdc7c); }
            let sp47 = c.r64(4);
            let Some(pv48) = c.ld64(sp47) else { return RegfileExit::fault(0x70007fdc7f); };
            c.set_rsp(sp47.wrapping_add(8));
            let pea49 = c.r64(3).wrapping_add(0x28);
            if !c.st64(pea49, pv48) { c.set_rsp(sp47); return RegfileExit::fault(0x70007fdc7f); }
            let sp50 = c.r64(4);
            let Some(pv51) = c.ld64(sp50) else { return RegfileExit::fault(0x70007fdc82); };
            c.set_rsp(sp50.wrapping_add(8));
            let pea52 = c.r64(3).wrapping_add(0x20);
            if !c.st64(pea52, pv51) { c.set_rsp(sp50); return RegfileExit::fault(0x70007fdc82); }
            let sp53 = c.r64(4);
            let Some(pv54) = c.ld64(sp53) else { return RegfileExit::fault(0x70007fdc85); };
            c.set_rsp(sp53.wrapping_add(8));
            let pea55 = c.r64(3).wrapping_add(0x18);
            if !c.st64(pea55, pv54) { c.set_rsp(sp53); return RegfileExit::fault(0x70007fdc85); }
            let sp56 = c.r64(4);
            let Some(pv57) = c.ld64(sp56) else { return RegfileExit::fault(0x70007fdc88); };
            c.set_rsp(sp56.wrapping_add(8));
            let pea58 = c.r64(3).wrapping_add(0x10);
            if !c.st64(pea58, pv57) { c.set_rsp(sp56); return RegfileExit::fault(0x70007fdc88); }
            let sp59 = c.r64(4);
            let Some(pv60) = c.ld64(sp59) else { return RegfileExit::fault(0x70007fdc8b); };
            c.set_rsp(sp59.wrapping_add(8));
            let pea61 = c.r64(3).wrapping_add(0x90);
            if !c.st64(pea61, pv60) { c.set_rsp(sp59); return RegfileExit::fault(0x70007fdc8b); }
            let sp62 = c.r64(4);
            let Some(pv63) = c.ld64(sp62) else { return RegfileExit::fault(0x70007fdc91); };
            c.set_rsp(sp62.wrapping_add(8));
            let pea64 = c.r64(3).wrapping_add(0x98);
            if !c.st64(pea64, pv63) { c.set_rsp(sp62); return RegfileExit::fault(0x70007fdc91); }
            let sp65 = c.r64(4);
            let Some(pv66) = c.ld64(sp65) else { return RegfileExit::fault(0x70007fdc97); };
            c.set_rsp(sp65.wrapping_add(8));
            let pea67 = c.r64(3).wrapping_add(0xa0);
            if !c.st64(pea67, pv66) { c.set_rsp(sp65); return RegfileExit::fault(0x70007fdc97); }
            let sp68 = c.r64(4);
            let Some(pv69) = c.ld64(sp68) else { return RegfileExit::fault(0x70007fdc9d); };
            c.set_rsp(sp68.wrapping_add(8));
            let pea70 = c.r64(3).wrapping_add(0xa8);
            if !c.st64(pea70, pv69) { c.set_rsp(sp68); return RegfileExit::fault(0x70007fdc9d); }
            let sp71 = c.r64(4);
            let Some(pv72) = c.ld64(sp71) else { return RegfileExit::fault(0x70007fdca3); };
            c.set_rsp(sp71.wrapping_add(8));
            let pea73 = c.r64(3).wrapping_add(0xb0);
            if !c.st64(pea73, pv72) { c.set_rsp(sp71); return RegfileExit::fault(0x70007fdca3); }
            let sp74 = c.r64(4);
            let Some(pv75) = c.ld64(sp74) else { return RegfileExit::fault(0x70007fdca9); };
            c.set_rsp(sp74.wrapping_add(8));
            let pea76 = c.r64(3).wrapping_add(0xb8);
            if !c.st64(pea76, pv75) { c.set_rsp(sp74); return RegfileExit::fault(0x70007fdca9); }
            let sp77 = c.r64(4);
            let Some(pv78) = c.ld64(sp77) else { return RegfileExit::fault(0x70007fdcaf); };
            c.set_rsp(sp77.wrapping_add(8));
            let pea79 = c.r64(3).wrapping_add(0xc0);
            if !c.st64(pea79, pv78) { c.set_rsp(sp77); return RegfileExit::fault(0x70007fdcaf); }
            let sp80 = c.r64(4);
            let Some(pv81) = c.ld64(sp80) else { return RegfileExit::fault(0x70007fdcb5); };
            c.set_rsp(sp80.wrapping_add(8));
            let pea82 = c.r64(3).wrapping_add(0xc8);
            if !c.st64(pea82, pv81) { c.set_rsp(sp80); return RegfileExit::fault(0x70007fdcb5); }
            let sp83 = c.r64(4);
            let Some(pv84) = c.ld64(sp83) else { return RegfileExit::fault(0x70007fdcbb); };
            c.set_rsp(sp83.wrapping_add(8));
            let pea85 = c.r64(3).wrapping_add(0xd0);
            if !c.st64(pea85, pv84) { c.set_rsp(sp83); return RegfileExit::fault(0x70007fdcbb); }
            let sp86 = c.r64(4);
            let Some(pv87) = c.ld64(sp86) else { return RegfileExit::fault(0x70007fdcc1); };
            c.set_rsp(sp86.wrapping_add(8));
            let pea88 = c.r64(3).wrapping_add(0xd8);
            if !c.st64(pea88, pv87) { c.set_rsp(sp86); return RegfileExit::fault(0x70007fdcc1); }
            let sp89 = c.r64(4);
            let Some(pv90) = c.ld64(sp89) else { return RegfileExit::fault(0x70007fdcc7); };
            c.set_rsp(sp89.wrapping_add(8));
            let pea91 = c.r64(3).wrapping_add(0xe0);
            if !c.st64(pea91, pv90) { c.set_rsp(sp89); return RegfileExit::fault(0x70007fdcc7); }
            let sp92 = c.r64(4);
            let Some(pv93) = c.ld64(sp92) else { return RegfileExit::fault(0x70007fdccd); };
            c.set_rsp(sp92.wrapping_add(8));
            let pea94 = c.r64(3).wrapping_add(0xe8);
            if !c.st64(pea94, pv93) { c.set_rsp(sp92); return RegfileExit::fault(0x70007fdccd); }
            let sp95 = c.r64(4);
            let Some(pv96) = c.ld64(sp95) else { return RegfileExit::fault(0x70007fdcd3); };
            c.set_rsp(sp95.wrapping_add(8));
            let pea97 = c.r64(3).wrapping_add(0xf0);
            if !c.st64(pea97, pv96) { c.set_rsp(sp95); return RegfileExit::fault(0x70007fdcd3); }
            let sp98 = c.r64(4);
            let Some(pv99) = c.ld64(sp98) else { return RegfileExit::fault(0x70007fdcd9); };
            c.set_rsp(sp98.wrapping_add(8));
            let pea100 = c.r64(3).wrapping_add(0xf8);
            if !c.st64(pea100, pv99) { c.set_rsp(sp98); return RegfileExit::fault(0x70007fdcd9); }
            let sp101 = c.r64(4);
            let Some(pv102) = c.ld64(sp101) else { return RegfileExit::fault(0x70007fdcdf); };
            c.set_rsp(sp101.wrapping_add(8));
            let pea103 = c.r64(3).wrapping_add(0x100);
            if !c.st64(pea103, pv102) { c.set_rsp(sp101); return RegfileExit::fault(0x70007fdcdf); }
            let sp104 = c.r64(4);
            let Some(pv105) = c.ld64(sp104) else { return RegfileExit::fault(0x70007fdce5); };
            c.set_rsp(sp104.wrapping_add(8));
            let pea106 = c.r64(3).wrapping_add(0x108);
            if !c.st64(pea106, pv105) { c.set_rsp(sp104); return RegfileExit::fault(0x70007fdce5); }
            if !c.st8(c.r64(13).wrapping_add(0x9748), 0x0u64) { return RegfileExit::fault(0x70007fdceb); }
            let r107 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r107);
            let Some(t108) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fdcf7); };
            c.w64(1, t108);
            let r109 = c.add(c.r64(1), c.r64(4), 64);
            c.w64(1, r109);
            c.w64(2, c.r64(3).wrapping_add(0x140));
            blk = 5; continue;
        }
        4 => {
            let sp110 = c.r64(4);
            let Some(pv111) = c.ld64(sp110) else { return RegfileExit::fault(0x70007fdd09); };
            c.set_rsp(sp110.wrapping_add(8));
            let pea112 = c.r64(2);
            if !c.st64(pea112, pv111) { c.set_rsp(sp110); return RegfileExit::fault(0x70007fdd09); }
            let r113 = c.add(c.r64(2), 0x8u64, 64);
            c.w64(2, r113);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(1), c.r64(4), 64);
            if c.cc_ne() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fdd14); }
            c.w64(10, c.r64(4));
            let r114 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r114);
            let r115 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r115);
            c.logic(r115, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007fdd20); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(13));
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1280)) else { return RegfileExit::fault(0x70007fdd2b); };
            c.w64(0, t116);
            c.w64(10, 0x70007fdd39u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fdd39); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fdd3d); }
            return c.call(c.r64(0), 0x70007fdd43, 0x70007fdd41);
        }
        7 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fdd43); }
            let Some(t117) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fdd4b); };
            c.w64(4, t117);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fdd50); }
            let Some(t118) = c.ld64(c.r64(0).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fdd51); };
            c.w64(4, t118);
            let Some(t119) = c.ld32(c.r64(0).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fdd55); };
            c.w32(2, t119);
            let Some(t120) = c.ld64(c.r64(0).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007fdd58); };
            c.w64(0, t120);
            c.w64(2, c.r64(0).wrapping_add(c.r64(2).wrapping_mul(8)));
            blk = 12; continue;
        }
        8 => {
            let Some(t121) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70007fdd65); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fdd68); };
            c.w64(1, t122);
            blk = 10; continue;
        }
        9 => {
            let r123 = c.sub(c.r64(1), 0x8u64, 64);
            c.w64(1, r123);
            let Some(t124) = c.ld64(c.r64(3).wrapping_add(c.r64(1)).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fdd74); };
            if !c.push64(t124) { return RegfileExit::fault(0x70007fdd74); }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            let r125 = c.add(c.r64(0), 0x8u64, 64);
            c.w64(0, r125);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_b() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x90)) { return RegfileExit::fault(0x70007fdd89); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x98)) { return RegfileExit::fault(0x70007fdd91); }
            if !c.movsd_rm(2, c.r64(3).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007fdd99); }
            if !c.movsd_rm(3, c.r64(3).wrapping_add(0xa8)) { return RegfileExit::fault(0x70007fdda1); }
            if !c.movsd_rm(4, c.r64(3).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007fdda9); }
            if !c.movsd_rm(5, c.r64(3).wrapping_add(0xb8)) { return RegfileExit::fault(0x70007fddb1); }
            if !c.movsd_rm(6, c.r64(3).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007fddb9); }
            if !c.movsd_rm(7, c.r64(3).wrapping_add(0xc8)) { return RegfileExit::fault(0x70007fddc1); }
            if !c.movsd_rm(8, c.r64(3).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007fddc9); }
            if !c.movsd_rm(9, c.r64(3).wrapping_add(0xd8)) { return RegfileExit::fault(0x70007fddd2); }
            if !c.movsd_rm(10, c.r64(3).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007fdddb); }
            if !c.movsd_rm(11, c.r64(3).wrapping_add(0xe8)) { return RegfileExit::fault(0x70007fdde4); }
            if !c.movsd_rm(12, c.r64(3).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007fdded); }
            if !c.movsd_rm(13, c.r64(3).wrapping_add(0xf8)) { return RegfileExit::fault(0x70007fddf6); }
            if !c.movsd_rm(14, c.r64(3).wrapping_add(0x100)) { return RegfileExit::fault(0x70007fddff); }
            let Some(t126) = c.ld64(c.r64(3).wrapping_add(0x118)) else { return RegfileExit::fault(0x70007fde08); };
            if !c.push64(t126) { return RegfileExit::fault(0x70007fde08); }
            let Some(t127) = c.ld64(c.r64(3).wrapping_add(0x138)) else { return RegfileExit::fault(0x70007fde0e); };
            if !c.push64(t127) { return RegfileExit::fault(0x70007fde0e); }
            let Some(t128) = c.ld64(c.r64(3).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fde14); };
            if !c.push64(t128) { return RegfileExit::fault(0x70007fde14); }
            let Some(t129) = c.ld64(c.r64(3).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fde17); };
            if !c.push64(t129) { return RegfileExit::fault(0x70007fde17); }
            let Some(t130) = c.ld64(c.r64(3).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fde1a); };
            if !c.push64(t130) { return RegfileExit::fault(0x70007fde1a); }
            let Some(t131) = c.ld64(c.r64(3).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fde1d); };
            if !c.push64(t131) { return RegfileExit::fault(0x70007fde1d); }
            let Some(t132) = c.ld64(c.r64(3).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fde20); };
            if !c.push64(t132) { return RegfileExit::fault(0x70007fde20); }
            let Some(t133) = c.ld64(c.r64(3).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fde23); };
            if !c.push64(t133) { return RegfileExit::fault(0x70007fde23); }
            let Some(t134) = c.ld64(c.r64(3).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fde26); };
            if !c.push64(t134) { return RegfileExit::fault(0x70007fde26); }
            let Some(t135) = c.ld64(c.r64(3).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007fde29); };
            if !c.push64(t135) { return RegfileExit::fault(0x70007fde29); }
            let Some(t136) = c.ld64(c.r64(3).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fde2c); };
            if !c.push64(t136) { return RegfileExit::fault(0x70007fde2c); }
            let Some(t137) = c.ld64(c.r64(3).wrapping_add(0x58)) else { return RegfileExit::fault(0x70007fde2f); };
            if !c.push64(t137) { return RegfileExit::fault(0x70007fde2f); }
            let Some(t138) = c.ld64(c.r64(3).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007fde32); };
            if !c.push64(t138) { return RegfileExit::fault(0x70007fde32); }
            let Some(t139) = c.ld64(c.r64(3).wrapping_add(0x68)) else { return RegfileExit::fault(0x70007fde35); };
            if !c.push64(t139) { return RegfileExit::fault(0x70007fde35); }
            let Some(t140) = c.ld64(c.r64(3).wrapping_add(0x70)) else { return RegfileExit::fault(0x70007fde38); };
            if !c.push64(t140) { return RegfileExit::fault(0x70007fde38); }
            let Some(t141) = c.ld64(c.r64(3).wrapping_add(0x78)) else { return RegfileExit::fault(0x70007fde3b); };
            if !c.push64(t141) { return RegfileExit::fault(0x70007fde3b); }
            let Some(t142) = c.ld64(c.r64(3).wrapping_add(0x80)) else { return RegfileExit::fault(0x70007fde3e); };
            if !c.push64(t142) { return RegfileExit::fault(0x70007fde3e); }
            let Some(t143) = c.ld64(c.r64(3).wrapping_add(0x88)) else { return RegfileExit::fault(0x70007fde44); };
            if !c.push64(t143) { return RegfileExit::fault(0x70007fde44); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fde4a); }
            if !c.pop64_reg(14) { return RegfileExit::fault(0x70007fde4c); }
            if !c.pop64_reg(13) { return RegfileExit::fault(0x70007fde4e); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fde50); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007fde52); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fde54); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007fde56); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fde58); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fde5a); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fde5b); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fde5c); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fde5d); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fde5e); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fde5f); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fde60); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fde61); }
            if !c.st8(c.r64(13).wrapping_add(0x9748), 0x1u64) { return RegfileExit::fault(0x70007fde62); }
            return c.ret(0x0, 0x70007fde6a);
        }
        14 => {
            return RegfileExit::fault(0x70007fde6b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fde74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.sub(c.r64(4), 0x80u64, 64);
            c.w64(4, r0);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007fde7b); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x8), 1) { return RegfileExit::fault(0x70007fde80); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x10), 2) { return RegfileExit::fault(0x70007fde86); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x18), 3) { return RegfileExit::fault(0x70007fde8c); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x20), 4) { return RegfileExit::fault(0x70007fde92); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x28), 5) { return RegfileExit::fault(0x70007fde98); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x30), 6) { return RegfileExit::fault(0x70007fde9e); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x38), 7) { return RegfileExit::fault(0x70007fdea4); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x40), 8) { return RegfileExit::fault(0x70007fdeaa); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x48), 9) { return RegfileExit::fault(0x70007fdeb1); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x50), 10) { return RegfileExit::fault(0x70007fdeb8); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x58), 11) { return RegfileExit::fault(0x70007fdebf); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x60), 12) { return RegfileExit::fault(0x70007fdec6); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x68), 13) { return RegfileExit::fault(0x70007fdecd); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x70), 14) { return RegfileExit::fault(0x70007fded4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fdedb); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fdedc); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fdedd); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fdede); }
            if !c.push64(c.r64(4)) { return RegfileExit::fault(0x70007fdedf); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fdee0); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fdee1); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fdee2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fdee3); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007fdee5); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fdee7); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007fdee9); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fdeeb); }
            if !c.push64(c.r64(13)) { return RegfileExit::fault(0x70007fdeed); }
            if !c.push64(c.r64(14)) { return RegfileExit::fault(0x70007fdeef); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fdef1); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x70007fdef3); }
            c.w32(8, 0xffffffffu64);
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x100)) else { return RegfileExit::fault(0x70007fdf00); };
            c.w64(9, t1);
            c.w64(15, c.r64(4).wrapping_add(0x108));
            let r2 = c.sub(c.r64(15), c.r64(5), 64);
            c.w64(15, r2);
            let r3 = c.neg(c.r64(15), 64);
            c.w64(15, r3);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70007fdf21); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007fdf28); };
            c.w64(7, t6);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fdf36); };
            c.w64(0, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(0));
            c.w32(2, 0x1u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(15)) { return RegfileExit::fault(0x70007fdf42); }
            c.w64(15, c.r64(13));
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(15)) { return RegfileExit::fault(0x70007fdf4a); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1410)) else { return RegfileExit::fault(0x70007fdf4f); };
            c.w64(0, t8);
            c.w64(10, 0x70007fdf5du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fdf5d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fdf61); }
            return c.call(c.r64(0), 0x70007fdf67, 0x70007fdf65);
        }
        3 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fdf67); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fdf6f); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(0).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fdf74); };
            c.w64(3, t10);
            let sp11 = c.r64(4);
            let Some(pv12) = c.ld64(sp11) else { return RegfileExit::fault(0x70007fdf78); };
            c.set_rsp(sp11.wrapping_add(8));
            let pea13 = c.r64(3).wrapping_add(0x88);
            if !c.st64(pea13, pv12) { c.set_rsp(sp11); return RegfileExit::fault(0x70007fdf78); }
            let sp14 = c.r64(4);
            let Some(pv15) = c.ld64(sp14) else { return RegfileExit::fault(0x70007fdf7e); };
            c.set_rsp(sp14.wrapping_add(8));
            let pea16 = c.r64(3).wrapping_add(0x80);
            if !c.st64(pea16, pv15) { c.set_rsp(sp14); return RegfileExit::fault(0x70007fdf7e); }
            let sp17 = c.r64(4);
            let Some(pv18) = c.ld64(sp17) else { return RegfileExit::fault(0x70007fdf84); };
            c.set_rsp(sp17.wrapping_add(8));
            let pea19 = c.r64(3).wrapping_add(0x78);
            if !c.st64(pea19, pv18) { c.set_rsp(sp17); return RegfileExit::fault(0x70007fdf84); }
            let sp20 = c.r64(4);
            let Some(pv21) = c.ld64(sp20) else { return RegfileExit::fault(0x70007fdf87); };
            c.set_rsp(sp20.wrapping_add(8));
            let pea22 = c.r64(3).wrapping_add(0x70);
            if !c.st64(pea22, pv21) { c.set_rsp(sp20); return RegfileExit::fault(0x70007fdf87); }
            let sp23 = c.r64(4);
            let Some(pv24) = c.ld64(sp23) else { return RegfileExit::fault(0x70007fdf8a); };
            c.set_rsp(sp23.wrapping_add(8));
            let pea25 = c.r64(3).wrapping_add(0x68);
            if !c.st64(pea25, pv24) { c.set_rsp(sp23); return RegfileExit::fault(0x70007fdf8a); }
            let sp26 = c.r64(4);
            let Some(pv27) = c.ld64(sp26) else { return RegfileExit::fault(0x70007fdf8d); };
            c.set_rsp(sp26.wrapping_add(8));
            let pea28 = c.r64(3).wrapping_add(0x60);
            if !c.st64(pea28, pv27) { c.set_rsp(sp26); return RegfileExit::fault(0x70007fdf8d); }
            let sp29 = c.r64(4);
            let Some(pv30) = c.ld64(sp29) else { return RegfileExit::fault(0x70007fdf90); };
            c.set_rsp(sp29.wrapping_add(8));
            let pea31 = c.r64(3).wrapping_add(0x58);
            if !c.st64(pea31, pv30) { c.set_rsp(sp29); return RegfileExit::fault(0x70007fdf90); }
            let sp32 = c.r64(4);
            let Some(pv33) = c.ld64(sp32) else { return RegfileExit::fault(0x70007fdf93); };
            c.set_rsp(sp32.wrapping_add(8));
            let pea34 = c.r64(3).wrapping_add(0x50);
            if !c.st64(pea34, pv33) { c.set_rsp(sp32); return RegfileExit::fault(0x70007fdf93); }
            let sp35 = c.r64(4);
            let Some(pv36) = c.ld64(sp35) else { return RegfileExit::fault(0x70007fdf96); };
            c.set_rsp(sp35.wrapping_add(8));
            let pea37 = c.r64(3).wrapping_add(0x48);
            if !c.st64(pea37, pv36) { c.set_rsp(sp35); return RegfileExit::fault(0x70007fdf96); }
            let sp38 = c.r64(4);
            let Some(pv39) = c.ld64(sp38) else { return RegfileExit::fault(0x70007fdf99); };
            c.set_rsp(sp38.wrapping_add(8));
            let pea40 = c.r64(3).wrapping_add(0x40);
            if !c.st64(pea40, pv39) { c.set_rsp(sp38); return RegfileExit::fault(0x70007fdf99); }
            let sp41 = c.r64(4);
            let Some(pv42) = c.ld64(sp41) else { return RegfileExit::fault(0x70007fdf9c); };
            c.set_rsp(sp41.wrapping_add(8));
            let pea43 = c.r64(3).wrapping_add(0x38);
            if !c.st64(pea43, pv42) { c.set_rsp(sp41); return RegfileExit::fault(0x70007fdf9c); }
            let sp44 = c.r64(4);
            let Some(pv45) = c.ld64(sp44) else { return RegfileExit::fault(0x70007fdf9f); };
            c.set_rsp(sp44.wrapping_add(8));
            let pea46 = c.r64(3).wrapping_add(0x30);
            if !c.st64(pea46, pv45) { c.set_rsp(sp44); return RegfileExit::fault(0x70007fdf9f); }
            let sp47 = c.r64(4);
            let Some(pv48) = c.ld64(sp47) else { return RegfileExit::fault(0x70007fdfa2); };
            c.set_rsp(sp47.wrapping_add(8));
            let pea49 = c.r64(3).wrapping_add(0x28);
            if !c.st64(pea49, pv48) { c.set_rsp(sp47); return RegfileExit::fault(0x70007fdfa2); }
            let sp50 = c.r64(4);
            let Some(pv51) = c.ld64(sp50) else { return RegfileExit::fault(0x70007fdfa5); };
            c.set_rsp(sp50.wrapping_add(8));
            let pea52 = c.r64(3).wrapping_add(0x20);
            if !c.st64(pea52, pv51) { c.set_rsp(sp50); return RegfileExit::fault(0x70007fdfa5); }
            let sp53 = c.r64(4);
            let Some(pv54) = c.ld64(sp53) else { return RegfileExit::fault(0x70007fdfa8); };
            c.set_rsp(sp53.wrapping_add(8));
            let pea55 = c.r64(3).wrapping_add(0x18);
            if !c.st64(pea55, pv54) { c.set_rsp(sp53); return RegfileExit::fault(0x70007fdfa8); }
            let sp56 = c.r64(4);
            let Some(pv57) = c.ld64(sp56) else { return RegfileExit::fault(0x70007fdfab); };
            c.set_rsp(sp56.wrapping_add(8));
            let pea58 = c.r64(3).wrapping_add(0x10);
            if !c.st64(pea58, pv57) { c.set_rsp(sp56); return RegfileExit::fault(0x70007fdfab); }
            let sp59 = c.r64(4);
            let Some(pv60) = c.ld64(sp59) else { return RegfileExit::fault(0x70007fdfae); };
            c.set_rsp(sp59.wrapping_add(8));
            let pea61 = c.r64(3).wrapping_add(0x90);
            if !c.st64(pea61, pv60) { c.set_rsp(sp59); return RegfileExit::fault(0x70007fdfae); }
            let sp62 = c.r64(4);
            let Some(pv63) = c.ld64(sp62) else { return RegfileExit::fault(0x70007fdfb4); };
            c.set_rsp(sp62.wrapping_add(8));
            let pea64 = c.r64(3).wrapping_add(0x98);
            if !c.st64(pea64, pv63) { c.set_rsp(sp62); return RegfileExit::fault(0x70007fdfb4); }
            let sp65 = c.r64(4);
            let Some(pv66) = c.ld64(sp65) else { return RegfileExit::fault(0x70007fdfba); };
            c.set_rsp(sp65.wrapping_add(8));
            let pea67 = c.r64(3).wrapping_add(0xa0);
            if !c.st64(pea67, pv66) { c.set_rsp(sp65); return RegfileExit::fault(0x70007fdfba); }
            let sp68 = c.r64(4);
            let Some(pv69) = c.ld64(sp68) else { return RegfileExit::fault(0x70007fdfc0); };
            c.set_rsp(sp68.wrapping_add(8));
            let pea70 = c.r64(3).wrapping_add(0xa8);
            if !c.st64(pea70, pv69) { c.set_rsp(sp68); return RegfileExit::fault(0x70007fdfc0); }
            let sp71 = c.r64(4);
            let Some(pv72) = c.ld64(sp71) else { return RegfileExit::fault(0x70007fdfc6); };
            c.set_rsp(sp71.wrapping_add(8));
            let pea73 = c.r64(3).wrapping_add(0xb0);
            if !c.st64(pea73, pv72) { c.set_rsp(sp71); return RegfileExit::fault(0x70007fdfc6); }
            let sp74 = c.r64(4);
            let Some(pv75) = c.ld64(sp74) else { return RegfileExit::fault(0x70007fdfcc); };
            c.set_rsp(sp74.wrapping_add(8));
            let pea76 = c.r64(3).wrapping_add(0xb8);
            if !c.st64(pea76, pv75) { c.set_rsp(sp74); return RegfileExit::fault(0x70007fdfcc); }
            let sp77 = c.r64(4);
            let Some(pv78) = c.ld64(sp77) else { return RegfileExit::fault(0x70007fdfd2); };
            c.set_rsp(sp77.wrapping_add(8));
            let pea79 = c.r64(3).wrapping_add(0xc0);
            if !c.st64(pea79, pv78) { c.set_rsp(sp77); return RegfileExit::fault(0x70007fdfd2); }
            let sp80 = c.r64(4);
            let Some(pv81) = c.ld64(sp80) else { return RegfileExit::fault(0x70007fdfd8); };
            c.set_rsp(sp80.wrapping_add(8));
            let pea82 = c.r64(3).wrapping_add(0xc8);
            if !c.st64(pea82, pv81) { c.set_rsp(sp80); return RegfileExit::fault(0x70007fdfd8); }
            let sp83 = c.r64(4);
            let Some(pv84) = c.ld64(sp83) else { return RegfileExit::fault(0x70007fdfde); };
            c.set_rsp(sp83.wrapping_add(8));
            let pea85 = c.r64(3).wrapping_add(0xd0);
            if !c.st64(pea85, pv84) { c.set_rsp(sp83); return RegfileExit::fault(0x70007fdfde); }
            let sp86 = c.r64(4);
            let Some(pv87) = c.ld64(sp86) else { return RegfileExit::fault(0x70007fdfe4); };
            c.set_rsp(sp86.wrapping_add(8));
            let pea88 = c.r64(3).wrapping_add(0xd8);
            if !c.st64(pea88, pv87) { c.set_rsp(sp86); return RegfileExit::fault(0x70007fdfe4); }
            let sp89 = c.r64(4);
            let Some(pv90) = c.ld64(sp89) else { return RegfileExit::fault(0x70007fdfea); };
            c.set_rsp(sp89.wrapping_add(8));
            let pea91 = c.r64(3).wrapping_add(0xe0);
            if !c.st64(pea91, pv90) { c.set_rsp(sp89); return RegfileExit::fault(0x70007fdfea); }
            let sp92 = c.r64(4);
            let Some(pv93) = c.ld64(sp92) else { return RegfileExit::fault(0x70007fdff0); };
            c.set_rsp(sp92.wrapping_add(8));
            let pea94 = c.r64(3).wrapping_add(0xe8);
            if !c.st64(pea94, pv93) { c.set_rsp(sp92); return RegfileExit::fault(0x70007fdff0); }
            let sp95 = c.r64(4);
            let Some(pv96) = c.ld64(sp95) else { return RegfileExit::fault(0x70007fdff6); };
            c.set_rsp(sp95.wrapping_add(8));
            let pea97 = c.r64(3).wrapping_add(0xf0);
            if !c.st64(pea97, pv96) { c.set_rsp(sp95); return RegfileExit::fault(0x70007fdff6); }
            let sp98 = c.r64(4);
            let Some(pv99) = c.ld64(sp98) else { return RegfileExit::fault(0x70007fdffc); };
            c.set_rsp(sp98.wrapping_add(8));
            let pea100 = c.r64(3).wrapping_add(0xf8);
            if !c.st64(pea100, pv99) { c.set_rsp(sp98); return RegfileExit::fault(0x70007fdffc); }
            let sp101 = c.r64(4);
            let Some(pv102) = c.ld64(sp101) else { return RegfileExit::fault(0x70007fe002); };
            c.set_rsp(sp101.wrapping_add(8));
            let pea103 = c.r64(3).wrapping_add(0x100);
            if !c.st64(pea103, pv102) { c.set_rsp(sp101); return RegfileExit::fault(0x70007fe002); }
            let sp104 = c.r64(4);
            let Some(pv105) = c.ld64(sp104) else { return RegfileExit::fault(0x70007fe008); };
            c.set_rsp(sp104.wrapping_add(8));
            let pea106 = c.r64(3).wrapping_add(0x108);
            if !c.st64(pea106, pv105) { c.set_rsp(sp104); return RegfileExit::fault(0x70007fe008); }
            if !c.st8(c.r64(13).wrapping_add(0x9748), 0x0u64) { return RegfileExit::fault(0x70007fe00e); }
            let r107 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r107);
            let Some(t108) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fe01a); };
            c.w64(1, t108);
            let r109 = c.add(c.r64(1), c.r64(4), 64);
            c.w64(1, r109);
            c.w64(2, c.r64(3).wrapping_add(0x140));
            blk = 5; continue;
        }
        4 => {
            let sp110 = c.r64(4);
            let Some(pv111) = c.ld64(sp110) else { return RegfileExit::fault(0x70007fe02c); };
            c.set_rsp(sp110.wrapping_add(8));
            let pea112 = c.r64(2);
            if !c.st64(pea112, pv111) { c.set_rsp(sp110); return RegfileExit::fault(0x70007fe02c); }
            let r113 = c.add(c.r64(2), 0x8u64, 64);
            c.w64(2, r113);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(1), c.r64(4), 64);
            if c.cc_ne() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fe037); }
            c.w64(10, c.r64(4));
            let r114 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r114);
            let r115 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r115);
            c.logic(r115, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007fe043); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(13));
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1280)) else { return RegfileExit::fault(0x70007fe04e); };
            c.w64(0, t116);
            c.w64(10, 0x70007fe05cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fe05c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fe060); }
            return c.call(c.r64(0), 0x70007fe066, 0x70007fe064);
        }
        7 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fe066); }
            let Some(t117) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fe06e); };
            c.w64(4, t117);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe073); }
            let Some(t118) = c.ld64(c.r64(0).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fe074); };
            c.w64(4, t118);
            let Some(t119) = c.ld32(c.r64(0).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fe078); };
            c.w32(2, t119);
            let Some(t120) = c.ld64(c.r64(0).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007fe07b); };
            c.w64(0, t120);
            c.w64(2, c.r64(0).wrapping_add(c.r64(2).wrapping_mul(8)));
            blk = 12; continue;
        }
        8 => {
            let Some(t121) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70007fe088); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fe08b); };
            c.w64(1, t122);
            blk = 10; continue;
        }
        9 => {
            let r123 = c.sub(c.r64(1), 0x8u64, 64);
            c.w64(1, r123);
            let Some(t124) = c.ld64(c.r64(3).wrapping_add(c.r64(1)).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fe097); };
            if !c.push64(t124) { return RegfileExit::fault(0x70007fe097); }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            let r125 = c.add(c.r64(0), 0x8u64, 64);
            c.w64(0, r125);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_b() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x90)) { return RegfileExit::fault(0x70007fe0ac); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x98)) { return RegfileExit::fault(0x70007fe0b4); }
            if !c.movsd_rm(2, c.r64(3).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007fe0bc); }
            if !c.movsd_rm(3, c.r64(3).wrapping_add(0xa8)) { return RegfileExit::fault(0x70007fe0c4); }
            if !c.movsd_rm(4, c.r64(3).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007fe0cc); }
            if !c.movsd_rm(5, c.r64(3).wrapping_add(0xb8)) { return RegfileExit::fault(0x70007fe0d4); }
            if !c.movsd_rm(6, c.r64(3).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007fe0dc); }
            if !c.movsd_rm(7, c.r64(3).wrapping_add(0xc8)) { return RegfileExit::fault(0x70007fe0e4); }
            if !c.movsd_rm(8, c.r64(3).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007fe0ec); }
            if !c.movsd_rm(9, c.r64(3).wrapping_add(0xd8)) { return RegfileExit::fault(0x70007fe0f5); }
            if !c.movsd_rm(10, c.r64(3).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007fe0fe); }
            if !c.movsd_rm(11, c.r64(3).wrapping_add(0xe8)) { return RegfileExit::fault(0x70007fe107); }
            if !c.movsd_rm(12, c.r64(3).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007fe110); }
            if !c.movsd_rm(13, c.r64(3).wrapping_add(0xf8)) { return RegfileExit::fault(0x70007fe119); }
            if !c.movsd_rm(14, c.r64(3).wrapping_add(0x100)) { return RegfileExit::fault(0x70007fe122); }
            let Some(t126) = c.ld64(c.r64(3).wrapping_add(0x118)) else { return RegfileExit::fault(0x70007fe12b); };
            if !c.push64(t126) { return RegfileExit::fault(0x70007fe12b); }
            let Some(t127) = c.ld64(c.r64(3).wrapping_add(0x138)) else { return RegfileExit::fault(0x70007fe131); };
            if !c.push64(t127) { return RegfileExit::fault(0x70007fe131); }
            let Some(t128) = c.ld64(c.r64(3).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fe137); };
            if !c.push64(t128) { return RegfileExit::fault(0x70007fe137); }
            let Some(t129) = c.ld64(c.r64(3).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fe13a); };
            if !c.push64(t129) { return RegfileExit::fault(0x70007fe13a); }
            let Some(t130) = c.ld64(c.r64(3).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fe13d); };
            if !c.push64(t130) { return RegfileExit::fault(0x70007fe13d); }
            let Some(t131) = c.ld64(c.r64(3).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fe140); };
            if !c.push64(t131) { return RegfileExit::fault(0x70007fe140); }
            let Some(t132) = c.ld64(c.r64(3).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fe143); };
            if !c.push64(t132) { return RegfileExit::fault(0x70007fe143); }
            let Some(t133) = c.ld64(c.r64(3).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fe146); };
            if !c.push64(t133) { return RegfileExit::fault(0x70007fe146); }
            let Some(t134) = c.ld64(c.r64(3).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fe149); };
            if !c.push64(t134) { return RegfileExit::fault(0x70007fe149); }
            let Some(t135) = c.ld64(c.r64(3).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007fe14c); };
            if !c.push64(t135) { return RegfileExit::fault(0x70007fe14c); }
            let Some(t136) = c.ld64(c.r64(3).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fe14f); };
            if !c.push64(t136) { return RegfileExit::fault(0x70007fe14f); }
            let Some(t137) = c.ld64(c.r64(3).wrapping_add(0x58)) else { return RegfileExit::fault(0x70007fe152); };
            if !c.push64(t137) { return RegfileExit::fault(0x70007fe152); }
            let Some(t138) = c.ld64(c.r64(3).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007fe155); };
            if !c.push64(t138) { return RegfileExit::fault(0x70007fe155); }
            let Some(t139) = c.ld64(c.r64(3).wrapping_add(0x68)) else { return RegfileExit::fault(0x70007fe158); };
            if !c.push64(t139) { return RegfileExit::fault(0x70007fe158); }
            let Some(t140) = c.ld64(c.r64(3).wrapping_add(0x70)) else { return RegfileExit::fault(0x70007fe15b); };
            if !c.push64(t140) { return RegfileExit::fault(0x70007fe15b); }
            let Some(t141) = c.ld64(c.r64(3).wrapping_add(0x78)) else { return RegfileExit::fault(0x70007fe15e); };
            if !c.push64(t141) { return RegfileExit::fault(0x70007fe15e); }
            let Some(t142) = c.ld64(c.r64(3).wrapping_add(0x80)) else { return RegfileExit::fault(0x70007fe161); };
            if !c.push64(t142) { return RegfileExit::fault(0x70007fe161); }
            let Some(t143) = c.ld64(c.r64(3).wrapping_add(0x88)) else { return RegfileExit::fault(0x70007fe167); };
            if !c.push64(t143) { return RegfileExit::fault(0x70007fe167); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fe16d); }
            if !c.pop64_reg(14) { return RegfileExit::fault(0x70007fe16f); }
            if !c.pop64_reg(13) { return RegfileExit::fault(0x70007fe171); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fe173); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007fe175); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fe177); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007fe179); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fe17b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fe17d); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fe17e); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fe17f); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe180); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe181); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fe182); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fe183); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe184); }
            if !c.st8(c.r64(13).wrapping_add(0x9748), 0x1u64) { return RegfileExit::fault(0x70007fe185); }
            return c.ret(0x0, 0x70007fe18d);
        }
        14 => {
            return RegfileExit::fault(0x70007fe18e);
        }
        15 => {
            return RegfileExit::fault(0x70007fe18f);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fe194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.sub(c.r64(4), 0x80u64, 64);
            c.w64(4, r0);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007fe19b); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x8), 1) { return RegfileExit::fault(0x70007fe1a0); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x10), 2) { return RegfileExit::fault(0x70007fe1a6); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x18), 3) { return RegfileExit::fault(0x70007fe1ac); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x20), 4) { return RegfileExit::fault(0x70007fe1b2); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x28), 5) { return RegfileExit::fault(0x70007fe1b8); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x30), 6) { return RegfileExit::fault(0x70007fe1be); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x38), 7) { return RegfileExit::fault(0x70007fe1c4); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x40), 8) { return RegfileExit::fault(0x70007fe1ca); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x48), 9) { return RegfileExit::fault(0x70007fe1d1); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x50), 10) { return RegfileExit::fault(0x70007fe1d8); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x58), 11) { return RegfileExit::fault(0x70007fe1df); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x60), 12) { return RegfileExit::fault(0x70007fe1e6); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x68), 13) { return RegfileExit::fault(0x70007fe1ed); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x70), 14) { return RegfileExit::fault(0x70007fe1f4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fe1fb); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fe1fc); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fe1fd); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fe1fe); }
            if !c.push64(c.r64(4)) { return RegfileExit::fault(0x70007fe1ff); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fe200); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fe201); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fe202); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fe203); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007fe205); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fe207); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007fe209); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fe20b); }
            if !c.push64(c.r64(13)) { return RegfileExit::fault(0x70007fe20d); }
            if !c.push64(c.r64(14)) { return RegfileExit::fault(0x70007fe20f); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fe211); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x70007fe213); }
            c.w32(8, 0xffffffffu64);
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x100)) else { return RegfileExit::fault(0x70007fe220); };
            c.w64(9, t1);
            c.w64(15, c.r64(4).wrapping_add(0x108));
            let r2 = c.sub(c.r64(15), c.r64(5), 64);
            c.w64(15, r2);
            let r3 = c.neg(c.r64(15), 64);
            c.w64(15, r3);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70007fe241); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007fe248); };
            c.w64(7, t6);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fe256); };
            c.w64(0, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(0));
            c.w32(2, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(15)) { return RegfileExit::fault(0x70007fe262); }
            c.w64(15, c.r64(13));
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(15)) { return RegfileExit::fault(0x70007fe26a); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1410)) else { return RegfileExit::fault(0x70007fe26f); };
            c.w64(0, t8);
            c.w64(10, 0x70007fe27du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fe27d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fe281); }
            return c.call(c.r64(0), 0x70007fe287, 0x70007fe285);
        }
        3 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fe287); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fe28f); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(0).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fe294); };
            c.w64(3, t10);
            let sp11 = c.r64(4);
            let Some(pv12) = c.ld64(sp11) else { return RegfileExit::fault(0x70007fe298); };
            c.set_rsp(sp11.wrapping_add(8));
            let pea13 = c.r64(3).wrapping_add(0x88);
            if !c.st64(pea13, pv12) { c.set_rsp(sp11); return RegfileExit::fault(0x70007fe298); }
            let sp14 = c.r64(4);
            let Some(pv15) = c.ld64(sp14) else { return RegfileExit::fault(0x70007fe29e); };
            c.set_rsp(sp14.wrapping_add(8));
            let pea16 = c.r64(3).wrapping_add(0x80);
            if !c.st64(pea16, pv15) { c.set_rsp(sp14); return RegfileExit::fault(0x70007fe29e); }
            let sp17 = c.r64(4);
            let Some(pv18) = c.ld64(sp17) else { return RegfileExit::fault(0x70007fe2a4); };
            c.set_rsp(sp17.wrapping_add(8));
            let pea19 = c.r64(3).wrapping_add(0x78);
            if !c.st64(pea19, pv18) { c.set_rsp(sp17); return RegfileExit::fault(0x70007fe2a4); }
            let sp20 = c.r64(4);
            let Some(pv21) = c.ld64(sp20) else { return RegfileExit::fault(0x70007fe2a7); };
            c.set_rsp(sp20.wrapping_add(8));
            let pea22 = c.r64(3).wrapping_add(0x70);
            if !c.st64(pea22, pv21) { c.set_rsp(sp20); return RegfileExit::fault(0x70007fe2a7); }
            let sp23 = c.r64(4);
            let Some(pv24) = c.ld64(sp23) else { return RegfileExit::fault(0x70007fe2aa); };
            c.set_rsp(sp23.wrapping_add(8));
            let pea25 = c.r64(3).wrapping_add(0x68);
            if !c.st64(pea25, pv24) { c.set_rsp(sp23); return RegfileExit::fault(0x70007fe2aa); }
            let sp26 = c.r64(4);
            let Some(pv27) = c.ld64(sp26) else { return RegfileExit::fault(0x70007fe2ad); };
            c.set_rsp(sp26.wrapping_add(8));
            let pea28 = c.r64(3).wrapping_add(0x60);
            if !c.st64(pea28, pv27) { c.set_rsp(sp26); return RegfileExit::fault(0x70007fe2ad); }
            let sp29 = c.r64(4);
            let Some(pv30) = c.ld64(sp29) else { return RegfileExit::fault(0x70007fe2b0); };
            c.set_rsp(sp29.wrapping_add(8));
            let pea31 = c.r64(3).wrapping_add(0x58);
            if !c.st64(pea31, pv30) { c.set_rsp(sp29); return RegfileExit::fault(0x70007fe2b0); }
            let sp32 = c.r64(4);
            let Some(pv33) = c.ld64(sp32) else { return RegfileExit::fault(0x70007fe2b3); };
            c.set_rsp(sp32.wrapping_add(8));
            let pea34 = c.r64(3).wrapping_add(0x50);
            if !c.st64(pea34, pv33) { c.set_rsp(sp32); return RegfileExit::fault(0x70007fe2b3); }
            let sp35 = c.r64(4);
            let Some(pv36) = c.ld64(sp35) else { return RegfileExit::fault(0x70007fe2b6); };
            c.set_rsp(sp35.wrapping_add(8));
            let pea37 = c.r64(3).wrapping_add(0x48);
            if !c.st64(pea37, pv36) { c.set_rsp(sp35); return RegfileExit::fault(0x70007fe2b6); }
            let sp38 = c.r64(4);
            let Some(pv39) = c.ld64(sp38) else { return RegfileExit::fault(0x70007fe2b9); };
            c.set_rsp(sp38.wrapping_add(8));
            let pea40 = c.r64(3).wrapping_add(0x40);
            if !c.st64(pea40, pv39) { c.set_rsp(sp38); return RegfileExit::fault(0x70007fe2b9); }
            let sp41 = c.r64(4);
            let Some(pv42) = c.ld64(sp41) else { return RegfileExit::fault(0x70007fe2bc); };
            c.set_rsp(sp41.wrapping_add(8));
            let pea43 = c.r64(3).wrapping_add(0x38);
            if !c.st64(pea43, pv42) { c.set_rsp(sp41); return RegfileExit::fault(0x70007fe2bc); }
            let sp44 = c.r64(4);
            let Some(pv45) = c.ld64(sp44) else { return RegfileExit::fault(0x70007fe2bf); };
            c.set_rsp(sp44.wrapping_add(8));
            let pea46 = c.r64(3).wrapping_add(0x30);
            if !c.st64(pea46, pv45) { c.set_rsp(sp44); return RegfileExit::fault(0x70007fe2bf); }
            let sp47 = c.r64(4);
            let Some(pv48) = c.ld64(sp47) else { return RegfileExit::fault(0x70007fe2c2); };
            c.set_rsp(sp47.wrapping_add(8));
            let pea49 = c.r64(3).wrapping_add(0x28);
            if !c.st64(pea49, pv48) { c.set_rsp(sp47); return RegfileExit::fault(0x70007fe2c2); }
            let sp50 = c.r64(4);
            let Some(pv51) = c.ld64(sp50) else { return RegfileExit::fault(0x70007fe2c5); };
            c.set_rsp(sp50.wrapping_add(8));
            let pea52 = c.r64(3).wrapping_add(0x20);
            if !c.st64(pea52, pv51) { c.set_rsp(sp50); return RegfileExit::fault(0x70007fe2c5); }
            let sp53 = c.r64(4);
            let Some(pv54) = c.ld64(sp53) else { return RegfileExit::fault(0x70007fe2c8); };
            c.set_rsp(sp53.wrapping_add(8));
            let pea55 = c.r64(3).wrapping_add(0x18);
            if !c.st64(pea55, pv54) { c.set_rsp(sp53); return RegfileExit::fault(0x70007fe2c8); }
            let sp56 = c.r64(4);
            let Some(pv57) = c.ld64(sp56) else { return RegfileExit::fault(0x70007fe2cb); };
            c.set_rsp(sp56.wrapping_add(8));
            let pea58 = c.r64(3).wrapping_add(0x10);
            if !c.st64(pea58, pv57) { c.set_rsp(sp56); return RegfileExit::fault(0x70007fe2cb); }
            let sp59 = c.r64(4);
            let Some(pv60) = c.ld64(sp59) else { return RegfileExit::fault(0x70007fe2ce); };
            c.set_rsp(sp59.wrapping_add(8));
            let pea61 = c.r64(3).wrapping_add(0x90);
            if !c.st64(pea61, pv60) { c.set_rsp(sp59); return RegfileExit::fault(0x70007fe2ce); }
            let sp62 = c.r64(4);
            let Some(pv63) = c.ld64(sp62) else { return RegfileExit::fault(0x70007fe2d4); };
            c.set_rsp(sp62.wrapping_add(8));
            let pea64 = c.r64(3).wrapping_add(0x98);
            if !c.st64(pea64, pv63) { c.set_rsp(sp62); return RegfileExit::fault(0x70007fe2d4); }
            let sp65 = c.r64(4);
            let Some(pv66) = c.ld64(sp65) else { return RegfileExit::fault(0x70007fe2da); };
            c.set_rsp(sp65.wrapping_add(8));
            let pea67 = c.r64(3).wrapping_add(0xa0);
            if !c.st64(pea67, pv66) { c.set_rsp(sp65); return RegfileExit::fault(0x70007fe2da); }
            let sp68 = c.r64(4);
            let Some(pv69) = c.ld64(sp68) else { return RegfileExit::fault(0x70007fe2e0); };
            c.set_rsp(sp68.wrapping_add(8));
            let pea70 = c.r64(3).wrapping_add(0xa8);
            if !c.st64(pea70, pv69) { c.set_rsp(sp68); return RegfileExit::fault(0x70007fe2e0); }
            let sp71 = c.r64(4);
            let Some(pv72) = c.ld64(sp71) else { return RegfileExit::fault(0x70007fe2e6); };
            c.set_rsp(sp71.wrapping_add(8));
            let pea73 = c.r64(3).wrapping_add(0xb0);
            if !c.st64(pea73, pv72) { c.set_rsp(sp71); return RegfileExit::fault(0x70007fe2e6); }
            let sp74 = c.r64(4);
            let Some(pv75) = c.ld64(sp74) else { return RegfileExit::fault(0x70007fe2ec); };
            c.set_rsp(sp74.wrapping_add(8));
            let pea76 = c.r64(3).wrapping_add(0xb8);
            if !c.st64(pea76, pv75) { c.set_rsp(sp74); return RegfileExit::fault(0x70007fe2ec); }
            let sp77 = c.r64(4);
            let Some(pv78) = c.ld64(sp77) else { return RegfileExit::fault(0x70007fe2f2); };
            c.set_rsp(sp77.wrapping_add(8));
            let pea79 = c.r64(3).wrapping_add(0xc0);
            if !c.st64(pea79, pv78) { c.set_rsp(sp77); return RegfileExit::fault(0x70007fe2f2); }
            let sp80 = c.r64(4);
            let Some(pv81) = c.ld64(sp80) else { return RegfileExit::fault(0x70007fe2f8); };
            c.set_rsp(sp80.wrapping_add(8));
            let pea82 = c.r64(3).wrapping_add(0xc8);
            if !c.st64(pea82, pv81) { c.set_rsp(sp80); return RegfileExit::fault(0x70007fe2f8); }
            let sp83 = c.r64(4);
            let Some(pv84) = c.ld64(sp83) else { return RegfileExit::fault(0x70007fe2fe); };
            c.set_rsp(sp83.wrapping_add(8));
            let pea85 = c.r64(3).wrapping_add(0xd0);
            if !c.st64(pea85, pv84) { c.set_rsp(sp83); return RegfileExit::fault(0x70007fe2fe); }
            let sp86 = c.r64(4);
            let Some(pv87) = c.ld64(sp86) else { return RegfileExit::fault(0x70007fe304); };
            c.set_rsp(sp86.wrapping_add(8));
            let pea88 = c.r64(3).wrapping_add(0xd8);
            if !c.st64(pea88, pv87) { c.set_rsp(sp86); return RegfileExit::fault(0x70007fe304); }
            let sp89 = c.r64(4);
            let Some(pv90) = c.ld64(sp89) else { return RegfileExit::fault(0x70007fe30a); };
            c.set_rsp(sp89.wrapping_add(8));
            let pea91 = c.r64(3).wrapping_add(0xe0);
            if !c.st64(pea91, pv90) { c.set_rsp(sp89); return RegfileExit::fault(0x70007fe30a); }
            let sp92 = c.r64(4);
            let Some(pv93) = c.ld64(sp92) else { return RegfileExit::fault(0x70007fe310); };
            c.set_rsp(sp92.wrapping_add(8));
            let pea94 = c.r64(3).wrapping_add(0xe8);
            if !c.st64(pea94, pv93) { c.set_rsp(sp92); return RegfileExit::fault(0x70007fe310); }
            let sp95 = c.r64(4);
            let Some(pv96) = c.ld64(sp95) else { return RegfileExit::fault(0x70007fe316); };
            c.set_rsp(sp95.wrapping_add(8));
            let pea97 = c.r64(3).wrapping_add(0xf0);
            if !c.st64(pea97, pv96) { c.set_rsp(sp95); return RegfileExit::fault(0x70007fe316); }
            let sp98 = c.r64(4);
            let Some(pv99) = c.ld64(sp98) else { return RegfileExit::fault(0x70007fe31c); };
            c.set_rsp(sp98.wrapping_add(8));
            let pea100 = c.r64(3).wrapping_add(0xf8);
            if !c.st64(pea100, pv99) { c.set_rsp(sp98); return RegfileExit::fault(0x70007fe31c); }
            let sp101 = c.r64(4);
            let Some(pv102) = c.ld64(sp101) else { return RegfileExit::fault(0x70007fe322); };
            c.set_rsp(sp101.wrapping_add(8));
            let pea103 = c.r64(3).wrapping_add(0x100);
            if !c.st64(pea103, pv102) { c.set_rsp(sp101); return RegfileExit::fault(0x70007fe322); }
            let sp104 = c.r64(4);
            let Some(pv105) = c.ld64(sp104) else { return RegfileExit::fault(0x70007fe328); };
            c.set_rsp(sp104.wrapping_add(8));
            let pea106 = c.r64(3).wrapping_add(0x108);
            if !c.st64(pea106, pv105) { c.set_rsp(sp104); return RegfileExit::fault(0x70007fe328); }
            if !c.st8(c.r64(13).wrapping_add(0x9748), 0x0u64) { return RegfileExit::fault(0x70007fe32e); }
            let r107 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r107);
            let Some(t108) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fe33a); };
            c.w64(1, t108);
            let r109 = c.add(c.r64(1), c.r64(4), 64);
            c.w64(1, r109);
            c.w64(2, c.r64(3).wrapping_add(0x140));
            blk = 5; continue;
        }
        4 => {
            let sp110 = c.r64(4);
            let Some(pv111) = c.ld64(sp110) else { return RegfileExit::fault(0x70007fe34c); };
            c.set_rsp(sp110.wrapping_add(8));
            let pea112 = c.r64(2);
            if !c.st64(pea112, pv111) { c.set_rsp(sp110); return RegfileExit::fault(0x70007fe34c); }
            let r113 = c.add(c.r64(2), 0x8u64, 64);
            c.w64(2, r113);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(1), c.r64(4), 64);
            if c.cc_ne() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fe357); }
            c.w64(10, c.r64(4));
            let r114 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r114);
            let r115 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r115);
            c.logic(r115, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007fe363); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(13));
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1280)) else { return RegfileExit::fault(0x70007fe36e); };
            c.w64(0, t116);
            c.w64(10, 0x70007fe37cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fe37c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fe380); }
            return c.call(c.r64(0), 0x70007fe386, 0x70007fe384);
        }
        7 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fe386); }
            let Some(t117) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fe38e); };
            c.w64(4, t117);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe393); }
            let Some(t118) = c.ld64(c.r64(0).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fe394); };
            c.w64(4, t118);
            let Some(t119) = c.ld32(c.r64(0).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fe398); };
            c.w32(2, t119);
            let Some(t120) = c.ld64(c.r64(0).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007fe39b); };
            c.w64(0, t120);
            c.w64(2, c.r64(0).wrapping_add(c.r64(2).wrapping_mul(8)));
            blk = 12; continue;
        }
        8 => {
            let Some(t121) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70007fe3a8); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fe3ab); };
            c.w64(1, t122);
            blk = 10; continue;
        }
        9 => {
            let r123 = c.sub(c.r64(1), 0x8u64, 64);
            c.w64(1, r123);
            let Some(t124) = c.ld64(c.r64(3).wrapping_add(c.r64(1)).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fe3b7); };
            if !c.push64(t124) { return RegfileExit::fault(0x70007fe3b7); }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            let r125 = c.add(c.r64(0), 0x8u64, 64);
            c.w64(0, r125);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_b() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x90)) { return RegfileExit::fault(0x70007fe3cc); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x98)) { return RegfileExit::fault(0x70007fe3d4); }
            if !c.movsd_rm(2, c.r64(3).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007fe3dc); }
            if !c.movsd_rm(3, c.r64(3).wrapping_add(0xa8)) { return RegfileExit::fault(0x70007fe3e4); }
            if !c.movsd_rm(4, c.r64(3).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007fe3ec); }
            if !c.movsd_rm(5, c.r64(3).wrapping_add(0xb8)) { return RegfileExit::fault(0x70007fe3f4); }
            if !c.movsd_rm(6, c.r64(3).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007fe3fc); }
            if !c.movsd_rm(7, c.r64(3).wrapping_add(0xc8)) { return RegfileExit::fault(0x70007fe404); }
            if !c.movsd_rm(8, c.r64(3).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007fe40c); }
            if !c.movsd_rm(9, c.r64(3).wrapping_add(0xd8)) { return RegfileExit::fault(0x70007fe415); }
            if !c.movsd_rm(10, c.r64(3).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007fe41e); }
            if !c.movsd_rm(11, c.r64(3).wrapping_add(0xe8)) { return RegfileExit::fault(0x70007fe427); }
            if !c.movsd_rm(12, c.r64(3).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007fe430); }
            if !c.movsd_rm(13, c.r64(3).wrapping_add(0xf8)) { return RegfileExit::fault(0x70007fe439); }
            if !c.movsd_rm(14, c.r64(3).wrapping_add(0x100)) { return RegfileExit::fault(0x70007fe442); }
            let Some(t126) = c.ld64(c.r64(3).wrapping_add(0x118)) else { return RegfileExit::fault(0x70007fe44b); };
            if !c.push64(t126) { return RegfileExit::fault(0x70007fe44b); }
            let Some(t127) = c.ld64(c.r64(3).wrapping_add(0x138)) else { return RegfileExit::fault(0x70007fe451); };
            if !c.push64(t127) { return RegfileExit::fault(0x70007fe451); }
            let Some(t128) = c.ld64(c.r64(3).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fe457); };
            if !c.push64(t128) { return RegfileExit::fault(0x70007fe457); }
            let Some(t129) = c.ld64(c.r64(3).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fe45a); };
            if !c.push64(t129) { return RegfileExit::fault(0x70007fe45a); }
            let Some(t130) = c.ld64(c.r64(3).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fe45d); };
            if !c.push64(t130) { return RegfileExit::fault(0x70007fe45d); }
            let Some(t131) = c.ld64(c.r64(3).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fe460); };
            if !c.push64(t131) { return RegfileExit::fault(0x70007fe460); }
            let Some(t132) = c.ld64(c.r64(3).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fe463); };
            if !c.push64(t132) { return RegfileExit::fault(0x70007fe463); }
            let Some(t133) = c.ld64(c.r64(3).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fe466); };
            if !c.push64(t133) { return RegfileExit::fault(0x70007fe466); }
            let Some(t134) = c.ld64(c.r64(3).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fe469); };
            if !c.push64(t134) { return RegfileExit::fault(0x70007fe469); }
            let Some(t135) = c.ld64(c.r64(3).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007fe46c); };
            if !c.push64(t135) { return RegfileExit::fault(0x70007fe46c); }
            let Some(t136) = c.ld64(c.r64(3).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fe46f); };
            if !c.push64(t136) { return RegfileExit::fault(0x70007fe46f); }
            let Some(t137) = c.ld64(c.r64(3).wrapping_add(0x58)) else { return RegfileExit::fault(0x70007fe472); };
            if !c.push64(t137) { return RegfileExit::fault(0x70007fe472); }
            let Some(t138) = c.ld64(c.r64(3).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007fe475); };
            if !c.push64(t138) { return RegfileExit::fault(0x70007fe475); }
            let Some(t139) = c.ld64(c.r64(3).wrapping_add(0x68)) else { return RegfileExit::fault(0x70007fe478); };
            if !c.push64(t139) { return RegfileExit::fault(0x70007fe478); }
            let Some(t140) = c.ld64(c.r64(3).wrapping_add(0x70)) else { return RegfileExit::fault(0x70007fe47b); };
            if !c.push64(t140) { return RegfileExit::fault(0x70007fe47b); }
            let Some(t141) = c.ld64(c.r64(3).wrapping_add(0x78)) else { return RegfileExit::fault(0x70007fe47e); };
            if !c.push64(t141) { return RegfileExit::fault(0x70007fe47e); }
            let Some(t142) = c.ld64(c.r64(3).wrapping_add(0x80)) else { return RegfileExit::fault(0x70007fe481); };
            if !c.push64(t142) { return RegfileExit::fault(0x70007fe481); }
            let Some(t143) = c.ld64(c.r64(3).wrapping_add(0x88)) else { return RegfileExit::fault(0x70007fe487); };
            if !c.push64(t143) { return RegfileExit::fault(0x70007fe487); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fe48d); }
            if !c.pop64_reg(14) { return RegfileExit::fault(0x70007fe48f); }
            if !c.pop64_reg(13) { return RegfileExit::fault(0x70007fe491); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fe493); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007fe495); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fe497); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007fe499); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fe49b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fe49d); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fe49e); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fe49f); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe4a0); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe4a1); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fe4a2); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fe4a3); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe4a4); }
            if !c.st8(c.r64(13).wrapping_add(0x9748), 0x1u64) { return RegfileExit::fault(0x70007fe4a5); }
            return c.ret(0x0, 0x70007fe4ad);
        }
        14 => {
            return RegfileExit::fault(0x70007fe4ae);
        }
        15 => {
            return RegfileExit::fault(0x70007fe4af);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fe4b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.sub(c.r64(4), 0x80u64, 64);
            c.w64(4, r0);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007fe4bb); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x8), 1) { return RegfileExit::fault(0x70007fe4c0); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x10), 2) { return RegfileExit::fault(0x70007fe4c6); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x18), 3) { return RegfileExit::fault(0x70007fe4cc); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x20), 4) { return RegfileExit::fault(0x70007fe4d2); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x28), 5) { return RegfileExit::fault(0x70007fe4d8); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x30), 6) { return RegfileExit::fault(0x70007fe4de); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x38), 7) { return RegfileExit::fault(0x70007fe4e4); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x40), 8) { return RegfileExit::fault(0x70007fe4ea); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x48), 9) { return RegfileExit::fault(0x70007fe4f1); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x50), 10) { return RegfileExit::fault(0x70007fe4f8); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x58), 11) { return RegfileExit::fault(0x70007fe4ff); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x60), 12) { return RegfileExit::fault(0x70007fe506); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x68), 13) { return RegfileExit::fault(0x70007fe50d); }
            if !c.movsd_mr(c.r64(4).wrapping_add(0x70), 14) { return RegfileExit::fault(0x70007fe514); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fe51b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fe51c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fe51d); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fe51e); }
            if !c.push64(c.r64(4)) { return RegfileExit::fault(0x70007fe51f); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fe520); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fe521); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fe522); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fe523); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007fe525); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fe527); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007fe529); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fe52b); }
            if !c.push64(c.r64(13)) { return RegfileExit::fault(0x70007fe52d); }
            if !c.push64(c.r64(14)) { return RegfileExit::fault(0x70007fe52f); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fe531); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x70007fe533); }
            c.w32(8, 0xffffffffu64);
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x100)) else { return RegfileExit::fault(0x70007fe540); };
            c.w64(9, t1);
            c.w64(15, c.r64(4).wrapping_add(0x108));
            let r2 = c.sub(c.r64(15), c.r64(5), 64);
            c.w64(15, r2);
            let r3 = c.neg(c.r64(15), 64);
            c.w64(15, r3);
            c.w64(10, c.r64(4));
            let r4 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r4);
            let r5 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r5);
            c.logic(r5, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70007fe561); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007fe568); };
            c.w64(7, t6);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fe576); };
            c.w64(0, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(0));
            c.w32(2, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(15)) { return RegfileExit::fault(0x70007fe582); }
            c.w64(15, c.r64(13));
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(15)) { return RegfileExit::fault(0x70007fe58a); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1410)) else { return RegfileExit::fault(0x70007fe58f); };
            c.w64(0, t8);
            c.w64(10, 0x70007fe59du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fe59d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fe5a1); }
            return c.call(c.r64(0), 0x70007fe5a7, 0x70007fe5a5);
        }
        3 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fe5a7); }
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fe5af); };
            c.w64(4, t9);
            let Some(t10) = c.ld64(c.r64(0).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fe5b4); };
            c.w64(3, t10);
            let sp11 = c.r64(4);
            let Some(pv12) = c.ld64(sp11) else { return RegfileExit::fault(0x70007fe5b8); };
            c.set_rsp(sp11.wrapping_add(8));
            let pea13 = c.r64(3).wrapping_add(0x88);
            if !c.st64(pea13, pv12) { c.set_rsp(sp11); return RegfileExit::fault(0x70007fe5b8); }
            let sp14 = c.r64(4);
            let Some(pv15) = c.ld64(sp14) else { return RegfileExit::fault(0x70007fe5be); };
            c.set_rsp(sp14.wrapping_add(8));
            let pea16 = c.r64(3).wrapping_add(0x80);
            if !c.st64(pea16, pv15) { c.set_rsp(sp14); return RegfileExit::fault(0x70007fe5be); }
            let sp17 = c.r64(4);
            let Some(pv18) = c.ld64(sp17) else { return RegfileExit::fault(0x70007fe5c4); };
            c.set_rsp(sp17.wrapping_add(8));
            let pea19 = c.r64(3).wrapping_add(0x78);
            if !c.st64(pea19, pv18) { c.set_rsp(sp17); return RegfileExit::fault(0x70007fe5c4); }
            let sp20 = c.r64(4);
            let Some(pv21) = c.ld64(sp20) else { return RegfileExit::fault(0x70007fe5c7); };
            c.set_rsp(sp20.wrapping_add(8));
            let pea22 = c.r64(3).wrapping_add(0x70);
            if !c.st64(pea22, pv21) { c.set_rsp(sp20); return RegfileExit::fault(0x70007fe5c7); }
            let sp23 = c.r64(4);
            let Some(pv24) = c.ld64(sp23) else { return RegfileExit::fault(0x70007fe5ca); };
            c.set_rsp(sp23.wrapping_add(8));
            let pea25 = c.r64(3).wrapping_add(0x68);
            if !c.st64(pea25, pv24) { c.set_rsp(sp23); return RegfileExit::fault(0x70007fe5ca); }
            let sp26 = c.r64(4);
            let Some(pv27) = c.ld64(sp26) else { return RegfileExit::fault(0x70007fe5cd); };
            c.set_rsp(sp26.wrapping_add(8));
            let pea28 = c.r64(3).wrapping_add(0x60);
            if !c.st64(pea28, pv27) { c.set_rsp(sp26); return RegfileExit::fault(0x70007fe5cd); }
            let sp29 = c.r64(4);
            let Some(pv30) = c.ld64(sp29) else { return RegfileExit::fault(0x70007fe5d0); };
            c.set_rsp(sp29.wrapping_add(8));
            let pea31 = c.r64(3).wrapping_add(0x58);
            if !c.st64(pea31, pv30) { c.set_rsp(sp29); return RegfileExit::fault(0x70007fe5d0); }
            let sp32 = c.r64(4);
            let Some(pv33) = c.ld64(sp32) else { return RegfileExit::fault(0x70007fe5d3); };
            c.set_rsp(sp32.wrapping_add(8));
            let pea34 = c.r64(3).wrapping_add(0x50);
            if !c.st64(pea34, pv33) { c.set_rsp(sp32); return RegfileExit::fault(0x70007fe5d3); }
            let sp35 = c.r64(4);
            let Some(pv36) = c.ld64(sp35) else { return RegfileExit::fault(0x70007fe5d6); };
            c.set_rsp(sp35.wrapping_add(8));
            let pea37 = c.r64(3).wrapping_add(0x48);
            if !c.st64(pea37, pv36) { c.set_rsp(sp35); return RegfileExit::fault(0x70007fe5d6); }
            let sp38 = c.r64(4);
            let Some(pv39) = c.ld64(sp38) else { return RegfileExit::fault(0x70007fe5d9); };
            c.set_rsp(sp38.wrapping_add(8));
            let pea40 = c.r64(3).wrapping_add(0x40);
            if !c.st64(pea40, pv39) { c.set_rsp(sp38); return RegfileExit::fault(0x70007fe5d9); }
            let sp41 = c.r64(4);
            let Some(pv42) = c.ld64(sp41) else { return RegfileExit::fault(0x70007fe5dc); };
            c.set_rsp(sp41.wrapping_add(8));
            let pea43 = c.r64(3).wrapping_add(0x38);
            if !c.st64(pea43, pv42) { c.set_rsp(sp41); return RegfileExit::fault(0x70007fe5dc); }
            let sp44 = c.r64(4);
            let Some(pv45) = c.ld64(sp44) else { return RegfileExit::fault(0x70007fe5df); };
            c.set_rsp(sp44.wrapping_add(8));
            let pea46 = c.r64(3).wrapping_add(0x30);
            if !c.st64(pea46, pv45) { c.set_rsp(sp44); return RegfileExit::fault(0x70007fe5df); }
            let sp47 = c.r64(4);
            let Some(pv48) = c.ld64(sp47) else { return RegfileExit::fault(0x70007fe5e2); };
            c.set_rsp(sp47.wrapping_add(8));
            let pea49 = c.r64(3).wrapping_add(0x28);
            if !c.st64(pea49, pv48) { c.set_rsp(sp47); return RegfileExit::fault(0x70007fe5e2); }
            let sp50 = c.r64(4);
            let Some(pv51) = c.ld64(sp50) else { return RegfileExit::fault(0x70007fe5e5); };
            c.set_rsp(sp50.wrapping_add(8));
            let pea52 = c.r64(3).wrapping_add(0x20);
            if !c.st64(pea52, pv51) { c.set_rsp(sp50); return RegfileExit::fault(0x70007fe5e5); }
            let sp53 = c.r64(4);
            let Some(pv54) = c.ld64(sp53) else { return RegfileExit::fault(0x70007fe5e8); };
            c.set_rsp(sp53.wrapping_add(8));
            let pea55 = c.r64(3).wrapping_add(0x18);
            if !c.st64(pea55, pv54) { c.set_rsp(sp53); return RegfileExit::fault(0x70007fe5e8); }
            let sp56 = c.r64(4);
            let Some(pv57) = c.ld64(sp56) else { return RegfileExit::fault(0x70007fe5eb); };
            c.set_rsp(sp56.wrapping_add(8));
            let pea58 = c.r64(3).wrapping_add(0x10);
            if !c.st64(pea58, pv57) { c.set_rsp(sp56); return RegfileExit::fault(0x70007fe5eb); }
            let sp59 = c.r64(4);
            let Some(pv60) = c.ld64(sp59) else { return RegfileExit::fault(0x70007fe5ee); };
            c.set_rsp(sp59.wrapping_add(8));
            let pea61 = c.r64(3).wrapping_add(0x90);
            if !c.st64(pea61, pv60) { c.set_rsp(sp59); return RegfileExit::fault(0x70007fe5ee); }
            let sp62 = c.r64(4);
            let Some(pv63) = c.ld64(sp62) else { return RegfileExit::fault(0x70007fe5f4); };
            c.set_rsp(sp62.wrapping_add(8));
            let pea64 = c.r64(3).wrapping_add(0x98);
            if !c.st64(pea64, pv63) { c.set_rsp(sp62); return RegfileExit::fault(0x70007fe5f4); }
            let sp65 = c.r64(4);
            let Some(pv66) = c.ld64(sp65) else { return RegfileExit::fault(0x70007fe5fa); };
            c.set_rsp(sp65.wrapping_add(8));
            let pea67 = c.r64(3).wrapping_add(0xa0);
            if !c.st64(pea67, pv66) { c.set_rsp(sp65); return RegfileExit::fault(0x70007fe5fa); }
            let sp68 = c.r64(4);
            let Some(pv69) = c.ld64(sp68) else { return RegfileExit::fault(0x70007fe600); };
            c.set_rsp(sp68.wrapping_add(8));
            let pea70 = c.r64(3).wrapping_add(0xa8);
            if !c.st64(pea70, pv69) { c.set_rsp(sp68); return RegfileExit::fault(0x70007fe600); }
            let sp71 = c.r64(4);
            let Some(pv72) = c.ld64(sp71) else { return RegfileExit::fault(0x70007fe606); };
            c.set_rsp(sp71.wrapping_add(8));
            let pea73 = c.r64(3).wrapping_add(0xb0);
            if !c.st64(pea73, pv72) { c.set_rsp(sp71); return RegfileExit::fault(0x70007fe606); }
            let sp74 = c.r64(4);
            let Some(pv75) = c.ld64(sp74) else { return RegfileExit::fault(0x70007fe60c); };
            c.set_rsp(sp74.wrapping_add(8));
            let pea76 = c.r64(3).wrapping_add(0xb8);
            if !c.st64(pea76, pv75) { c.set_rsp(sp74); return RegfileExit::fault(0x70007fe60c); }
            let sp77 = c.r64(4);
            let Some(pv78) = c.ld64(sp77) else { return RegfileExit::fault(0x70007fe612); };
            c.set_rsp(sp77.wrapping_add(8));
            let pea79 = c.r64(3).wrapping_add(0xc0);
            if !c.st64(pea79, pv78) { c.set_rsp(sp77); return RegfileExit::fault(0x70007fe612); }
            let sp80 = c.r64(4);
            let Some(pv81) = c.ld64(sp80) else { return RegfileExit::fault(0x70007fe618); };
            c.set_rsp(sp80.wrapping_add(8));
            let pea82 = c.r64(3).wrapping_add(0xc8);
            if !c.st64(pea82, pv81) { c.set_rsp(sp80); return RegfileExit::fault(0x70007fe618); }
            let sp83 = c.r64(4);
            let Some(pv84) = c.ld64(sp83) else { return RegfileExit::fault(0x70007fe61e); };
            c.set_rsp(sp83.wrapping_add(8));
            let pea85 = c.r64(3).wrapping_add(0xd0);
            if !c.st64(pea85, pv84) { c.set_rsp(sp83); return RegfileExit::fault(0x70007fe61e); }
            let sp86 = c.r64(4);
            let Some(pv87) = c.ld64(sp86) else { return RegfileExit::fault(0x70007fe624); };
            c.set_rsp(sp86.wrapping_add(8));
            let pea88 = c.r64(3).wrapping_add(0xd8);
            if !c.st64(pea88, pv87) { c.set_rsp(sp86); return RegfileExit::fault(0x70007fe624); }
            let sp89 = c.r64(4);
            let Some(pv90) = c.ld64(sp89) else { return RegfileExit::fault(0x70007fe62a); };
            c.set_rsp(sp89.wrapping_add(8));
            let pea91 = c.r64(3).wrapping_add(0xe0);
            if !c.st64(pea91, pv90) { c.set_rsp(sp89); return RegfileExit::fault(0x70007fe62a); }
            let sp92 = c.r64(4);
            let Some(pv93) = c.ld64(sp92) else { return RegfileExit::fault(0x70007fe630); };
            c.set_rsp(sp92.wrapping_add(8));
            let pea94 = c.r64(3).wrapping_add(0xe8);
            if !c.st64(pea94, pv93) { c.set_rsp(sp92); return RegfileExit::fault(0x70007fe630); }
            let sp95 = c.r64(4);
            let Some(pv96) = c.ld64(sp95) else { return RegfileExit::fault(0x70007fe636); };
            c.set_rsp(sp95.wrapping_add(8));
            let pea97 = c.r64(3).wrapping_add(0xf0);
            if !c.st64(pea97, pv96) { c.set_rsp(sp95); return RegfileExit::fault(0x70007fe636); }
            let sp98 = c.r64(4);
            let Some(pv99) = c.ld64(sp98) else { return RegfileExit::fault(0x70007fe63c); };
            c.set_rsp(sp98.wrapping_add(8));
            let pea100 = c.r64(3).wrapping_add(0xf8);
            if !c.st64(pea100, pv99) { c.set_rsp(sp98); return RegfileExit::fault(0x70007fe63c); }
            let sp101 = c.r64(4);
            let Some(pv102) = c.ld64(sp101) else { return RegfileExit::fault(0x70007fe642); };
            c.set_rsp(sp101.wrapping_add(8));
            let pea103 = c.r64(3).wrapping_add(0x100);
            if !c.st64(pea103, pv102) { c.set_rsp(sp101); return RegfileExit::fault(0x70007fe642); }
            let sp104 = c.r64(4);
            let Some(pv105) = c.ld64(sp104) else { return RegfileExit::fault(0x70007fe648); };
            c.set_rsp(sp104.wrapping_add(8));
            let pea106 = c.r64(3).wrapping_add(0x108);
            if !c.st64(pea106, pv105) { c.set_rsp(sp104); return RegfileExit::fault(0x70007fe648); }
            if !c.st8(c.r64(13).wrapping_add(0x9748), 0x0u64) { return RegfileExit::fault(0x70007fe64e); }
            let r107 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r107);
            let Some(t108) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fe65a); };
            c.w64(1, t108);
            let r109 = c.add(c.r64(1), c.r64(4), 64);
            c.w64(1, r109);
            c.w64(2, c.r64(3).wrapping_add(0x140));
            blk = 5; continue;
        }
        4 => {
            let sp110 = c.r64(4);
            let Some(pv111) = c.ld64(sp110) else { return RegfileExit::fault(0x70007fe66c); };
            c.set_rsp(sp110.wrapping_add(8));
            let pea112 = c.r64(2);
            if !c.st64(pea112, pv111) { c.set_rsp(sp110); return RegfileExit::fault(0x70007fe66c); }
            let r113 = c.add(c.r64(2), 0x8u64, 64);
            c.w64(2, r113);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(1), c.r64(4), 64);
            if c.cc_ne() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fe677); }
            c.w64(10, c.r64(4));
            let r114 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r114);
            let r115 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r115);
            c.logic(r115, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007fe683); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(13));
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1280)) else { return RegfileExit::fault(0x70007fe68e); };
            c.w64(0, t116);
            c.w64(10, 0x70007fe69cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007fe69c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007fe6a0); }
            return c.call(c.r64(0), 0x70007fe6a6, 0x70007fe6a4);
        }
        7 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fe6a6); }
            let Some(t117) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fe6ae); };
            c.w64(4, t117);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe6b3); }
            let Some(t118) = c.ld64(c.r64(0).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fe6b4); };
            c.w64(4, t118);
            let Some(t119) = c.ld32(c.r64(0).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fe6b8); };
            c.w32(2, t119);
            let Some(t120) = c.ld64(c.r64(0).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007fe6bb); };
            c.w64(0, t120);
            c.w64(2, c.r64(0).wrapping_add(c.r64(2).wrapping_mul(8)));
            blk = 12; continue;
        }
        8 => {
            let Some(t121) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70007fe6c8); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fe6cb); };
            c.w64(1, t122);
            blk = 10; continue;
        }
        9 => {
            let r123 = c.sub(c.r64(1), 0x8u64, 64);
            c.w64(1, r123);
            let Some(t124) = c.ld64(c.r64(3).wrapping_add(c.r64(1)).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007fe6d7); };
            if !c.push64(t124) { return RegfileExit::fault(0x70007fe6d7); }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            let r125 = c.add(c.r64(0), 0x8u64, 64);
            c.w64(0, r125);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_b() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x90)) { return RegfileExit::fault(0x70007fe6ec); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x98)) { return RegfileExit::fault(0x70007fe6f4); }
            if !c.movsd_rm(2, c.r64(3).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007fe6fc); }
            if !c.movsd_rm(3, c.r64(3).wrapping_add(0xa8)) { return RegfileExit::fault(0x70007fe704); }
            if !c.movsd_rm(4, c.r64(3).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007fe70c); }
            if !c.movsd_rm(5, c.r64(3).wrapping_add(0xb8)) { return RegfileExit::fault(0x70007fe714); }
            if !c.movsd_rm(6, c.r64(3).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007fe71c); }
            if !c.movsd_rm(7, c.r64(3).wrapping_add(0xc8)) { return RegfileExit::fault(0x70007fe724); }
            if !c.movsd_rm(8, c.r64(3).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007fe72c); }
            if !c.movsd_rm(9, c.r64(3).wrapping_add(0xd8)) { return RegfileExit::fault(0x70007fe735); }
            if !c.movsd_rm(10, c.r64(3).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007fe73e); }
            if !c.movsd_rm(11, c.r64(3).wrapping_add(0xe8)) { return RegfileExit::fault(0x70007fe747); }
            if !c.movsd_rm(12, c.r64(3).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007fe750); }
            if !c.movsd_rm(13, c.r64(3).wrapping_add(0xf8)) { return RegfileExit::fault(0x70007fe759); }
            if !c.movsd_rm(14, c.r64(3).wrapping_add(0x100)) { return RegfileExit::fault(0x70007fe762); }
            let Some(t126) = c.ld64(c.r64(3).wrapping_add(0x118)) else { return RegfileExit::fault(0x70007fe76b); };
            if !c.push64(t126) { return RegfileExit::fault(0x70007fe76b); }
            let Some(t127) = c.ld64(c.r64(3).wrapping_add(0x138)) else { return RegfileExit::fault(0x70007fe771); };
            if !c.push64(t127) { return RegfileExit::fault(0x70007fe771); }
            let Some(t128) = c.ld64(c.r64(3).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007fe777); };
            if !c.push64(t128) { return RegfileExit::fault(0x70007fe777); }
            let Some(t129) = c.ld64(c.r64(3).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007fe77a); };
            if !c.push64(t129) { return RegfileExit::fault(0x70007fe77a); }
            let Some(t130) = c.ld64(c.r64(3).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fe77d); };
            if !c.push64(t130) { return RegfileExit::fault(0x70007fe77d); }
            let Some(t131) = c.ld64(c.r64(3).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fe780); };
            if !c.push64(t131) { return RegfileExit::fault(0x70007fe780); }
            let Some(t132) = c.ld64(c.r64(3).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fe783); };
            if !c.push64(t132) { return RegfileExit::fault(0x70007fe783); }
            let Some(t133) = c.ld64(c.r64(3).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fe786); };
            if !c.push64(t133) { return RegfileExit::fault(0x70007fe786); }
            let Some(t134) = c.ld64(c.r64(3).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fe789); };
            if !c.push64(t134) { return RegfileExit::fault(0x70007fe789); }
            let Some(t135) = c.ld64(c.r64(3).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007fe78c); };
            if !c.push64(t135) { return RegfileExit::fault(0x70007fe78c); }
            let Some(t136) = c.ld64(c.r64(3).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fe78f); };
            if !c.push64(t136) { return RegfileExit::fault(0x70007fe78f); }
            let Some(t137) = c.ld64(c.r64(3).wrapping_add(0x58)) else { return RegfileExit::fault(0x70007fe792); };
            if !c.push64(t137) { return RegfileExit::fault(0x70007fe792); }
            let Some(t138) = c.ld64(c.r64(3).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007fe795); };
            if !c.push64(t138) { return RegfileExit::fault(0x70007fe795); }
            let Some(t139) = c.ld64(c.r64(3).wrapping_add(0x68)) else { return RegfileExit::fault(0x70007fe798); };
            if !c.push64(t139) { return RegfileExit::fault(0x70007fe798); }
            let Some(t140) = c.ld64(c.r64(3).wrapping_add(0x70)) else { return RegfileExit::fault(0x70007fe79b); };
            if !c.push64(t140) { return RegfileExit::fault(0x70007fe79b); }
            let Some(t141) = c.ld64(c.r64(3).wrapping_add(0x78)) else { return RegfileExit::fault(0x70007fe79e); };
            if !c.push64(t141) { return RegfileExit::fault(0x70007fe79e); }
            let Some(t142) = c.ld64(c.r64(3).wrapping_add(0x80)) else { return RegfileExit::fault(0x70007fe7a1); };
            if !c.push64(t142) { return RegfileExit::fault(0x70007fe7a1); }
            let Some(t143) = c.ld64(c.r64(3).wrapping_add(0x88)) else { return RegfileExit::fault(0x70007fe7a7); };
            if !c.push64(t143) { return RegfileExit::fault(0x70007fe7a7); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fe7ad); }
            if !c.pop64_reg(14) { return RegfileExit::fault(0x70007fe7af); }
            if !c.pop64_reg(13) { return RegfileExit::fault(0x70007fe7b1); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fe7b3); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007fe7b5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fe7b7); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007fe7b9); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fe7bb); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fe7bd); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fe7be); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fe7bf); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe7c0); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe7c1); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fe7c2); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fe7c3); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe7c4); }
            if !c.st8(c.r64(13).wrapping_add(0x9748), 0x1u64) { return RegfileExit::fault(0x70007fe7c5); }
            return c.ret(0x0, 0x70007fe7cd);
        }
        14 => {
            return RegfileExit::fault(0x70007fe7ce);
        }
        15 => {
            return RegfileExit::fault(0x70007fe7cf);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fe7d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fe7d4); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fe7d6); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007fe7d8); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007fe7da); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fe7dc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fe7de); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fe7df); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fe7e0); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fe7e1); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe7e2); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe7e3); }
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fe7e4); };
            c.w64(5, t0);
            let sp1 = c.r64(4);
            let Some(pv2) = c.ld64(sp1) else { return RegfileExit::fault(0x70007fe7e9); };
            c.set_rsp(sp1.wrapping_add(8));
            let pea3 = c.r64(4).wrapping_add(0x20);
            if !c.st64(pea3, pv2) { c.set_rsp(sp1); return RegfileExit::fault(0x70007fe7e9); }
            let r4 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r4);
            let Some(t5) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fe7f1); };
            c.w64(10, t5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(c.r64(10).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fe7f5); };
            c.w64(10, t6);
            if !c.st64(c.r64(4), c.r64(10)) { return RegfileExit::fault(0x70007fe7fd); }
            return c.ret(0x0, 0x70007fe801);
        }
        1 => {
            return RegfileExit::fault(0x70007fe802);
        }
        2 => {
            return RegfileExit::fault(0x70007fe803);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fe814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.st64(c.r64(4).wrapping_add(0x90), c.r64(0)) { return RegfileExit::fault(0x70007fe814); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fe81c); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fe81e); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007fe820); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007fe822); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fe824); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fe826); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fe827); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fe828); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fe829); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe82a); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe82b); }
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fe82c); };
            c.w64(5, t0);
            let sp1 = c.r64(4);
            let Some(pv2) = c.ld64(sp1) else { return RegfileExit::fault(0x70007fe831); };
            c.set_rsp(sp1.wrapping_add(8));
            let pea3 = c.r64(4).wrapping_add(0x20);
            if !c.st64(pea3, pv2) { c.set_rsp(sp1); return RegfileExit::fault(0x70007fe831); }
            let r4 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r4);
            let Some(t5) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fe839); };
            c.w64(10, t5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(c.r64(10).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fe83d); };
            c.w64(10, t6);
            if !c.st64(c.r64(4), c.r64(10)) { return RegfileExit::fault(0x70007fe845); }
            return c.ret(0x0, 0x70007fe849);
        }
        1 => {
            return RegfileExit::fault(0x70007fe84a);
        }
        2 => {
            return RegfileExit::fault(0x70007fe84b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fe854(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fe854); }
    if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fe856); }
    if !c.pop64_reg(11) { return RegfileExit::fault(0x70007fe858); }
    if !c.pop64_reg(9) { return RegfileExit::fault(0x70007fe85a); }
    if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fe85c); }
    if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fe85e); }
    if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fe85f); }
    if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fe860); }
    if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fe861); }
    if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe862); }
    if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe863); }
    c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
    if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
    let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fe86a); };
    c.w64(5, t0);
    let sp1 = c.r64(4);
    let Some(pv2) = c.ld64(sp1) else { return RegfileExit::fault(0x70007fe86f); };
    c.set_rsp(sp1.wrapping_add(8));
    let pea3 = c.r64(4).wrapping_add(0x20);
    if !c.st64(pea3, pv2) { c.set_rsp(sp1); return RegfileExit::fault(0x70007fe86f); }
    let r4 = c.add(c.r64(4), 0x20u64, 64);
    c.w64(4, r4);
    let Some(t5) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fe877); };
    c.w64(10, t5);
    let Some(t6) = c.ld64(c.r64(13).wrapping_add(c.r64(10).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fe87b); };
    c.w64(10, t6);
    if !c.st64(c.r64(4), c.r64(10)) { return RegfileExit::fault(0x70007fe883); }
    return c.ret(0x0, 0x70007fe887);
}

pub unsafe extern "C" fn rf_70007fe894(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    c.w64(10, c.r64(0));
    if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fe897); }
    if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fe899); }
    if !c.pop64_reg(11) { return RegfileExit::fault(0x70007fe89b); }
    if !c.pop64_reg(9) { return RegfileExit::fault(0x70007fe89d); }
    if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fe89f); }
    if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fe8a1); }
    if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fe8a2); }
    if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fe8a3); }
    if !c.pop64_reg(2) { return RegfileExit::fault(0x70007fe8a4); }
    if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fe8a5); }
    if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe8a6); }
    c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
    if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
    if !c.st64(c.r64(4).wrapping_add(c.r64(0).wrapping_mul(8)).wrapping_add(0x38), c.r64(10)) { return RegfileExit::fault(0x70007fe8ad); }
    let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fe8b2); };
    c.w64(5, t0);
    let sp1 = c.r64(4);
    let Some(pv2) = c.ld64(sp1) else { return RegfileExit::fault(0x70007fe8b7); };
    c.set_rsp(sp1.wrapping_add(8));
    let pea3 = c.r64(4).wrapping_add(0x20);
    if !c.st64(pea3, pv2) { c.set_rsp(sp1); return RegfileExit::fault(0x70007fe8b7); }
    let r4 = c.add(c.r64(4), 0x20u64, 64);
    c.w64(4, r4);
    let Some(t5) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007fe8bf); };
    c.w64(10, t5);
    let Some(t6) = c.ld64(c.r64(13).wrapping_add(c.r64(10).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007fe8c3); };
    c.w64(10, t6);
    if !c.st64(c.r64(4), c.r64(10)) { return RegfileExit::fault(0x70007fe8cb); }
    return c.ret(0x0, 0x70007fe8cf);
}

pub unsafe extern "C" fn rf_70007fe8d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fe8d4); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fe8d5); };
            c.w64(10, t0);
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fe8dc); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fe8de); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fe8df); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fe8e1); }
            c.w64(10, c.r64(13));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fe8e6); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fe8e8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fe8e9); }
            c.w64(3, c.r64(4).wrapping_add(0x8));
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fe8ef); }
            c.w64(5, c.r64(4));
            if !c.push64(0x6u64) { return RegfileExit::fault(0x70007fe8f3); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fe8f5); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x70007fe8f7); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x70007fe8fe); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x70007fe905); }
            let r1 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r1);
            let r2 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r2);
            c.logic(r2, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x70007fe914); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(3)) { return RegfileExit::fault(0x70007fe918); }
            c.w64(3, c.r64(3).wrapping_add(0x38));
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(3)) { return RegfileExit::fault(0x70007fe921); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(1)) { return RegfileExit::fault(0x70007fe926); }
            c.w64(10, c.r64(1).wrapping_mul(8).wrapping_add(0x38));
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(10)) { return RegfileExit::fault(0x70007fe933); }
            c.w64(1, c.r64(4).wrapping_add(0x20));
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2de0)) else { return RegfileExit::fault(0x70007fe93d); };
            c.w64(15, t3);
            let Some(t4) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007fe944); };
            c.w64(12, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fe947); };
            c.w64(3, t5);
            let ea6 = c.r64(15).wrapping_add(0x10);
            let Some(t7) = c.ld32(ea6) else { return RegfileExit::fault(0x70007fe94b); };
            let r8 = c.add(t7, 0x1u64, 32);
            if !c.st32(ea6, r8) { return RegfileExit::fault(0x70007fe94b); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2e38)) else { return RegfileExit::fault(0x70007fe950); };
            c.w64(0, t9);
            let Some(t10) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x70007fe957); };
            c.cmp(t10, 0x0u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1208)) else { return RegfileExit::fault(0x70007fe960); };
            c.w64(0, t11);
            let Some(t12) = c.ld32(c.r64(0)) else { return RegfileExit::fault(0x70007fe967); };
            c.cmp(t12, 0x0u64, 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(0, c.r64(2));
            blk = 4; continue;
        }
        3 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x13a0)) else { return RegfileExit::fault(0x70007fe978); };
            c.w64(0, t13);
            blk = 4; continue;
        }
        4 => {
            return c.call(c.r64(0), 0x70007fe981, 0x70007fe97f);
        }
        5 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fe981); };
            c.w64(0, t14);
            let ea15 = c.r64(15).wrapping_add(0x10);
            let Some(t16) = c.ld32(ea15) else { return RegfileExit::fault(0x70007fe985); };
            let r17 = c.sub(t16, 0x1u64, 32);
            if !c.st32(ea15, r17) { return RegfileExit::fault(0x70007fe985); }
            if !c.st64(c.r64(15), c.r64(12)) { return RegfileExit::fault(0x70007fe98a); }
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fe98d); };
            c.cmp(c.r64(3), t18, 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t19) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fe997); };
            c.w64(3, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fe99f); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x70007fe9a0); };
            c.w64(6, t20);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x70007fe9a7); }
            c.w64(7, c.r64(13).wrapping_add(0x3040));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007fe9b9); };
            c.w64(10, t21);
            let Some(t22) = c.ld32(c.r64(7)) else { return RegfileExit::fault(0x70007fe9c0); };
            c.cmp(t22, c.r32(10), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007fe9c9); }
            let r23 = c.add(c.r64(4), c.r64(3), 64);
            c.w64(4, r23);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2180)) else { return RegfileExit::fault(0x70007fe9d1); };
            c.w64(3, t24);
            return RegfileExit::jump(0x700086ed74);
        }
        9 => {
            if !c.st64(c.r64(15).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x70007fe9dd); }
            c.w64(3, c.r64(0));
            c.w64(1, c.r64(13));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x12a8)) else { return RegfileExit::fault(0x70007fe9e7); };
            c.w64(0, t25);
            return c.call(c.r64(0), 0x70007fe9f0, 0x70007fe9ee);
        }
        10 => {
            c.w64(0, c.r64(3));
            blk = 6; continue;
        }
        11 => {
            return RegfileExit::fault(0x70007fe9f5);
        }
        12 => {
            return RegfileExit::fault(0x70007fe9f6);
        }
        13 => {
            return RegfileExit::fault(0x70007fe9f7);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fea14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007fea14); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007fea15); }
            let Some(t0) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007fea16); };
            c.w32(15, t0);
            let r1 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fea1d); }
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fea1f); };
            c.w64(10, t2);
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fea26); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fea28); }
            c.w64(10, c.r64(13));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fea2d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fea2f); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fea30); }
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fea32); };
            c.w32(15, t3);
            let r4 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r4);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fea39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fea3b); }
            c.w64(0, c.r64(4).wrapping_add(0x10));
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fea41); }
            c.w64(5, c.r64(4));
            if !c.push64(0x6u64) { return RegfileExit::fault(0x70007fea45); }
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fea47); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), c.r64(5)) { return RegfileExit::fault(0x70007fea49); }
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x70007fea50); }
            if !c.st64(c.r64(13).wrapping_add(0x3058), c.r64(3)) { return RegfileExit::fault(0x70007fea57); }
            let r5 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r5);
            let r6 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r6);
            c.logic(r6, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(4)) { return RegfileExit::fault(0x70007fea66); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(0)) { return RegfileExit::fault(0x70007fea6a); }
            c.w64(1, c.r64(0).wrapping_sub(0x8));
            c.w64(2, c.r64(4).wrapping_add(0x20));
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007fea78); };
            c.w32(0, t7);
            let r8 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r8);
            let Some(t9) = c.ld64(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fea7e); };
            c.w64(8, t9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2de0)) else { return RegfileExit::fault(0x70007fea82); };
            c.w64(15, t10);
            let Some(t11) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007fea89); };
            c.w64(12, t11);
            let Some(t12) = c.ld64(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fea8c); };
            c.w64(3, t12);
            let ea13 = c.r64(15).wrapping_add(0x10);
            let Some(t14) = c.ld32(ea13) else { return RegfileExit::fault(0x70007fea90); };
            let r15 = c.add(t14, 0x1u64, 32);
            if !c.st32(ea13, r15) { return RegfileExit::fault(0x70007fea90); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2e38)) else { return RegfileExit::fault(0x70007fea95); };
            c.w64(0, t16);
            let Some(t17) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x70007fea9c); };
            c.cmp(t17, 0x0u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x1208)) else { return RegfileExit::fault(0x70007feaa5); };
            c.w64(0, t18);
            let Some(t19) = c.ld32(c.r64(0)) else { return RegfileExit::fault(0x70007feaac); };
            c.cmp(t19, 0x0u64, 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(0, c.r64(8));
            blk = 4; continue;
        }
        3 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x1398)) else { return RegfileExit::fault(0x70007feabd); };
            c.w64(0, t20);
            blk = 4; continue;
        }
        4 => {
            return c.call(c.r64(0), 0x70007feac6, 0x70007feac4);
        }
        5 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007feac6); };
            c.w64(0, t21);
            let ea22 = c.r64(15).wrapping_add(0x10);
            let Some(t23) = c.ld32(ea22) else { return RegfileExit::fault(0x70007feaca); };
            let r24 = c.sub(t23, 0x1u64, 32);
            if !c.st32(ea22, r24) { return RegfileExit::fault(0x70007feaca); }
            if !c.st64(c.r64(15), c.r64(12)) { return RegfileExit::fault(0x70007feacf); }
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fead2); };
            c.cmp(c.r64(3), t25, 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007feadf); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x70007feae0); };
            c.w64(6, t26);
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x70007feae7); }
            c.w64(7, c.r64(13).wrapping_add(0x3040));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007feaf9); };
            c.w64(10, t27);
            let Some(t28) = c.ld32(c.r64(7)) else { return RegfileExit::fault(0x70007feb00); };
            c.cmp(t28, c.r32(10), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x40, 0x70007feb09);
        }
        8 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2180)) else { return RegfileExit::fault(0x70007feb0e); };
            c.w64(3, t29);
            return RegfileExit::jump(0x700086ed74);
        }
        9 => {
            if !c.st64(c.r64(15).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x70007feb1a); }
            c.w64(3, c.r64(0));
            c.w64(1, c.r64(13));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x12a8)) else { return RegfileExit::fault(0x70007feb24); };
            c.w64(0, t30);
            return c.call(c.r64(0), 0x70007feb2d, 0x70007feb2b);
        }
        10 => {
            c.w64(0, c.r64(3));
            blk = 6; continue;
        }
        11 => {
            return RegfileExit::fault(0x70007feb32);
        }
        12 => {
            return RegfileExit::fault(0x70007feb33);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007feb54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1690)) else { return RegfileExit::fault(0x70007feb54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70007feb74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1698)) else { return RegfileExit::fault(0x70007feb74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70007feb94(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16a0)) else { return RegfileExit::fault(0x70007feb94); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70007febb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r64(2), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r0 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r0);
            let r1 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1);
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007febc3); };
            if !c.push64(t2) { return RegfileExit::fault(0x70007febc3); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70007febc7); };
            c.w64(3, t3);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x4u64) { return RegfileExit::fault(0x70007febd3); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007febde); }
            return RegfileExit::jump(0x700086ed74);
        }
        2 => {
            return RegfileExit::fault(0x70007febe8);
        }
        3 => {
            return RegfileExit::jump(0x70007febec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007febf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r64(2), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r0 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r0);
            let r1 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1);
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fec03); };
            if !c.push64(t2) { return RegfileExit::fault(0x70007fec03); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70007fec07); };
            c.w64(3, t3);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007fec15); }
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(6)) { return RegfileExit::fault(0x70007fec1a); }
            return RegfileExit::jump(0x700086ed74);
        }
        2 => {
            return RegfileExit::fault(0x70007fec24);
        }
        3 => {
            return RegfileExit::jump(0x70007fec28);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fec34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r64(2), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r0 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r0);
            let r1 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1);
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fec43); };
            if !c.push64(t2) { return RegfileExit::fault(0x70007fec43); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x20e0)) else { return RegfileExit::fault(0x70007fec47); };
            c.w64(3, t3);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x4u64) { return RegfileExit::fault(0x70007fec53); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007fec5e); }
            return RegfileExit::jump(0x700086ed74);
        }
        2 => {
            return RegfileExit::fault(0x70007fec68);
        }
        3 => {
            return RegfileExit::jump(0x70007fec6c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fec74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r64(2), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r0 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r0);
            let r1 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1);
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fec83); };
            if !c.push64(t2) { return RegfileExit::fault(0x70007fec83); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x20e0)) else { return RegfileExit::fault(0x70007fec87); };
            c.w64(3, t3);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70007fec95); }
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(6)) { return RegfileExit::fault(0x70007fec9a); }
            return RegfileExit::jump(0x700086ed74);
        }
        2 => {
            return RegfileExit::fault(0x70007feca4);
        }
        3 => {
            return RegfileExit::jump(0x70007feca8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fecb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fecb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fecb8); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fecbe); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fecc4); };
            c.w32(9, t3);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007feccb); };
            c.w32(11, t4);
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fecd5); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70007fece3); };
            c.cmp(t5, c.r32(11), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007fecec); };
            c.w64(11, t6);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fed04); };
            c.w64(9, t7);
            let Some(t8) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007fed0b); };
            c.w64(15, t8);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(3, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fed16); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007fed1d); };
            c.cmp(t10, c.r64(3), 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x70007fed33); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            let r11 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r11);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70007fed3d); }
            c.w64(11, c.r64(9));
            let r12 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r12);
            c.logic(r12, 64);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70007fed4b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70007fed4f); }
            let Some(t13) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fed53); };
            c.logic(t13 & 0x4u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(11, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(12, 0x7u64);
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70007fed6a); };
            c.w32(15, t14);
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x70007fed6e); }
            let r15 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r15);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ne() { blk = 10; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 15; continue;
        }
        12 => {
            c.w64(2, c.r64(7).wrapping_add(0x7));
            c.w64(1, c.r64(9).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r16 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r16);
            let r17 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r17);
            c.logic(r17, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007fed90); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70007fed95); };
            c.w64(0, t18);
            c.w64(10, 0x70007feda3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007feda3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007feda7); }
            return c.call(c.r64(0), 0x70007fedad, 0x70007fedab);
        }
        13 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007fedad); }
            let Some(t19) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007fedb5); };
            c.w64(4, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fedba); };
            c.w64(0, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fedbe); };
            c.w64(9, t21);
            blk = 15; continue;
        }
        14 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007fedc4); };
            c.w64(7, t22);
            c.w64(9, c.r64(7));
            blk = 15; continue;
        }
        15 => {
            c.w64(7, c.r64(0));
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70007fedd1); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r23 = c.r64(11) & c.r64(7);
            c.w64(11, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fede5); };
            c.logic(t24 & 0x4u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fedf2); }
            return c.ret(0x0, 0x70007fedf3);
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007fedf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70007fedf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70007fedfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70007fee00); }
            return c.call(0x70007febb4, 0x70007fee09, 0x70007fee04);
        }
        19 => {
            c.w64(9, c.r64(0));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fee0c); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fee10); };
            c.w64(8, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fee14); };
            c.w64(7, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fee18); };
            c.w64(11, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007fee1c); };
            c.w64(12, t29);
            blk = 7; continue;
        }
        20 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007fee25); };
            c.w64(3, t30);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007fee35, 0x70007fee30);
        }
        21 => {
            return RegfileExit::fault(0x70007fee35);
        }
        22 => {
            c.w32(9, c.r32(9));
            let r31 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r31);
            c.w64(11, 0xfffffffffffc0000u64);
            let r32 = c.r64(11) & c.r64(9);
            c.w64(11, r32);
            c.logic(r32, 64);
            let Some(t33) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fee46); };
            c.logic(t33 & 0x2u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(12, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fee51); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70007fee5b, 0x70007fee56);
        }
        24 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007fee5b); }
            blk = 17; continue;
        }
        25 => {
            return RegfileExit::jump(0x70007fee60);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fee74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fee74); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fee7a); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(1));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r3 = c.add(c.r64(8), 0x400u64, 64);
            c.w64(8, r3);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            let r4 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r4);
            c.w64(8, c.r64(7));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(7, c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x3fffu64, 64);
            if c.cc_ae() { blk = 23; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(7), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(8, c.r64(7).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007feeda); };
            c.w64(9, t5);
            let Some(t6) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007feee1); };
            c.w64(11, t6);
            c.w64(12, c.r64(8).wrapping_add(c.r64(11)));
            let Some(t7) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007feee8); };
            c.cmp(t7, c.r64(12), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70007feef2); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            c.w32(9, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70007feefd); };
            c.w64(11, t8);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70007fef04); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70007fef08); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007fef0c); };
            c.w64(9, t9);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70007fef13); }
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let r10 = c.sub(c.r64(7), 0x8u64, 64);
            c.w64(7, r10);
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8)), 0) { return RegfileExit::fault(0x70007fef32); }
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ne() { blk = 7; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(7, 0xfff7fffffff7ffffu64);
            blk = 10; continue;
        }
        10 => {
            let r11 = c.sub(c.r64(1), 0x8u64, 64);
            c.w64(1, r11);
            let Some(t12) = c.ld64(c.r64(1).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70007fef51); };
            c.cmp(t12, c.r64(7), 64);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(c.r64(2))) { return RegfileExit::fault(0x70007fef57); }
            if !c.movsd_mr(c.r64(1).wrapping_add(c.r64(8)), 0) { return RegfileExit::fault(0x70007fef5c); }
            blk = 13; continue;
        }
        12 => {
            if !c.st64(c.r64(1).wrapping_add(c.r64(8)), c.r64(7)) { return RegfileExit::fault(0x70007fef64); }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 10; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007fef6e); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r13 = c.r64(2) & c.r64(0);
            c.w64(2, r13);
            c.logic(r13, 64);
            let Some(t14) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007fef82); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70007fef8f);
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fef90); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fef94); }
            let r15 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r15);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70007fef9e); };
            c.w64(3, t16);
            return c.call(0x700086ed74, 0x70007fefaa, 0x70007fefa5);
        }
        18 => {
            return RegfileExit::fault(0x70007fefaa);
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fefab); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fefaf); }
            let r17 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r17);
            let r18 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r18);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007fefb8); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fefbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007fefbd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70007fefc1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70007fefc5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70007fefc9); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70007fefcd); };
            c.w64(3, t19);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70007fefe1, 0x70007fefdc);
        }
        20 => {
            c.w64(8, c.r64(0));
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fefe4); };
            c.w64(0, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fefe8); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fefec); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007feff0); };
            c.w64(1, t23);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007feff7); }
            blk = 6; continue;
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007feffd); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff001); }
            let r24 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r24);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007ff007); };
            c.w64(3, t25);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007ff017, 0x70007ff012);
        }
        22 => {
            return RegfileExit::fault(0x70007ff017);
        }
        23 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007ff018); }
            let Some(t26) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007ff019); };
            if !c.push64(t26) { return RegfileExit::fault(0x70007ff019); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(0)) { return RegfileExit::fault(0x70007ff01f); }
            c.w32(0, 0x2u64);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x70007ff029); };
            c.w64(3, t27);
            return RegfileExit::jump(0x700086ed74);
        }
        24 => {
            c.w32(8, c.r32(8));
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            c.w64(2, 0xfffffffffffc0000u64);
            let r29 = c.r64(2) & c.r64(8);
            c.w64(2, r29);
            c.logic(r29, 64);
            let Some(t30) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007ff045); };
            c.logic(t30 & 0x2u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(1, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007ff053); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007ff054); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70007ff061, 0x70007ff05c);
        }
        26 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007ff061); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007ff063); }
            blk = 16; continue;
        }
        27 => {
            return RegfileExit::jump(0x70007ff06c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff074); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff07a); };
            c.w32(7, t2);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r3 = c.add(c.r64(9), 0x400u64, 64);
            c.w64(9, r3);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let r4 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r4);
            c.w64(9, c.r64(8));
            if let Some(sr) = c.shr(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(8, c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x10));
            c.cmp(c.r64(8), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007ff0c5); };
            c.w64(9, t5);
            let Some(t6) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007ff0cc); };
            c.w64(11, t6);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(12, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007ff0db); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007ff0e2); };
            c.cmp(t8, c.r64(12), 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70007ff0eb); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007ff0f6); };
            c.w64(12, t9);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70007ff0fd); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70007ff101); }
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007ff11a); };
            c.w64(11, t10);
            blk = 7; continue;
        }
        7 => {
            let r11 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r11);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x70007ff125); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 7; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let r12 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70007ff138); };
            c.w32(8, t13);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70007ff13c); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70007ff146); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r14 = c.r64(1) & c.r64(0);
            c.w64(1, r14);
            c.logic(r14, 64);
            let Some(t15) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007ff15a); };
            c.logic(t15 & 0x4u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x70007ff163);
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff164); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff168); }
            let r16 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r16);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007ff16e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007ff172); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007ff176); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70007ff17a); }
            return c.call(0x70007febf4, 0x70007ff183, 0x70007ff17e);
        }
        14 => {
            c.w64(9, c.r64(0));
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007ff186); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007ff18a); };
            c.w64(1, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007ff18e); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007ff192); };
            c.w64(7, t20);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff199); }
            blk = 5; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff19f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff1a3); }
            let r21 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r21);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007ff1ab); };
            c.w64(3, t22);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007ff1b9, 0x70007ff1b4);
        }
        16 => {
            return RegfileExit::fault(0x70007ff1b9);
        }
        17 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007ff1ba); }
            let Some(t23) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007ff1bb); };
            if !c.push64(t23) { return RegfileExit::fault(0x70007ff1bb); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(0)) { return RegfileExit::fault(0x70007ff1c1); }
            c.w32(0, 0x2u64);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x70007ff1cb); };
            c.w64(3, t24);
            return RegfileExit::jump(0x700086ed74);
        }
        18 => {
            c.w32(9, c.r32(9));
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            c.w64(1, 0xfffffffffffc0000u64);
            let r26 = c.r64(1) & c.r64(9);
            c.w64(1, r26);
            c.logic(r26, 64);
            let Some(t27) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007ff1e7); };
            c.logic(t27 & 0x2u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(7, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007ff1f5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007ff1f6); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x70007ff203, 0x70007ff1fe);
        }
        20 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007ff203); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007ff205); }
            blk = 12; continue;
        }
        21 => {
            return RegfileExit::jump(0x70007ff20c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff214); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007ff218); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007ff219); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff21a); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70007ff21f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70007ff223); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70007ff227); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70007ff22b); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007ff22f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 31; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007ff239); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007ff240); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70007ff247); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff251); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let Some(t8) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff258); };
            c.cmp(t8, 0x64u64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007ff260); };
            c.w32(9, t9);
            c.w32(9, c.r32(9));
            let r10 = c.r32(9) & 0x10u64;
            c.w32(9, r10);
            c.logic(r10, 32);
            let r11 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r11);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007ff274); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1ea8)) else { return RegfileExit::fault(0x70007ff275); };
            c.w64(3, t12);
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x70007ff289, 0x70007ff284);
        }
        5 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007ff289); };
            c.w64(7, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007ff28d); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007ff291); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007ff295); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007ff299); };
            c.w64(8, t17);
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff29d); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(13).wrapping_add(0x63f0));
            let Some(t20) = c.ld64(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4))) else { return RegfileExit::fault(0x70007ff2b4); };
            c.w64(8, t20);
            blk = 27; continue;
        }
        8 => {
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff2bd); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff2c4); };
            c.w32(9, t23);
            c.cmp(c.r32(9), 0x5cu64, 32);
            if c.cc_l() { blk = 15; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(9), 0x85u64, 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(9), 0x85u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x9du64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(9), 0x9eu64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(9), 0x5cu64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x5eu64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x4au64, 32);
            if c.cc_l() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(9), 0x4au64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(9), 0x53u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(9), 0x48u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(9), 0x49u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t24) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff321); };
            c.w32(8, t24);
            let r25 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r25);
            blk = 27; continue;
        }
        21 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70007ff32a); };
            c.w64(8, t26);
            blk = 27; continue;
        }
        22 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff333); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            blk = 27; continue;
        }
        23 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70007ff33c); };
            c.w64(8, t29);
            blk = 27; continue;
        }
        24 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70007ff345); };
            c.w64(8, t30);
            blk = 27; continue;
        }
        25 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007ff34e); };
            c.w32(8, t31);
            let r32 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r32);
            blk = 27; continue;
        }
        26 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70007ff357); };
            c.w64(8, t33);
            blk = 27; continue;
        }
        27 => {
            let Some(t34) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007ff35e); };
            c.w64(5, t34);
            c.w64(1, c.r64(8));
            let r35 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r35);
            let Some(t36) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007ff369); };
            c.logic(t36 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let r37 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r37);
            blk = 30; continue;
        }
        29 => {
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007ff378); };
            c.w32(1, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007ff37b); };
            c.w64(1, t39);
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::jump(c.r64(1));
        }
        31 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70007ff385); };
            c.w64(3, t40);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70007ff396, 0x70007ff391);
        }
        32 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007ff396); };
            c.w64(7, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007ff39a); };
            c.w64(2, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007ff39e); };
            c.w64(0, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007ff3a2); };
            c.w64(6, t44);
            blk = 1; continue;
        }
        33 => {
            return RegfileExit::jump(0x70007ff3ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff3b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return c.ret(0x0, 0x70007ff3b8);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff3b9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff3bd); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff3c3); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70007ff3c9); }
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff3cd); };
            c.cmp(t3, c.r32(2), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff3d9); }
            return c.ret(0x0, 0x70007ff3da);
        }
        4 => {
            let Some(t4) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff3db); };
            c.cmp(t4, 0xbau64, 16);
            if c.cc_b() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.call(0x70008a2474, 0x70007ff3e8, 0x70007ff3e3);
        }
        6 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff3ec); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff3f2); };
            c.cmp(t7, c.r32(2), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007ff3fb); };
            c.w64(6, t8);
            blk = 9; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff3ff); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff405); };
            c.w32(2, t11);
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_b() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff415); }
            c.w32(0, 0x1u64);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70007ff41b); };
            c.w64(3, t12);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70007ff42a, 0x70007ff425);
        }
        12 => {
            blk = 3; continue;
        }
        13 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007ff42c); };
            c.w32(0, t13);
            let r14 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r14);
            blk = 3; continue;
        }
        14 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff434); };
            c.w32(2, t15);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r16 = c.r64(2) & 0xffffffu64;
            c.w64(2, r16);
            c.logic(r16, 64);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            blk = 3; continue;
        }
        16 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff44f); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70007ff450); };
            c.w64(3, t17);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007ff463, 0x70007ff45e);
        }
        17 => {
            blk = 3; continue;
        }
        18 => {
            return RegfileExit::jump(0x70007ff46c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(1, 0x2u64);
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007ff480); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007ff484); };
            c.w32(7, t1);
            c.w32(1, c.r32(1));
            c.w32(8, c.r32(7));
            let r2 = c.r32(1) | c.r32(8);
            c.w32(1, r2);
            c.logic(r2, 32);
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70007ff497); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007ff49b); }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x0, 0x70007ff4a2);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff4a3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff4a7); }
            let r3 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff4ad); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70007ff4b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70007ff4b7); }
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff4bb); };
            c.cmp(t6, c.r32(1), 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(1, 0xeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff4cc); }
            blk = 2; continue;
        }
        7 => {
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007ff4cf); };
            c.w64(7, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70007ff4d3); }
            let Some(t8) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff4d7); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(6, c.r64(7));
            return c.call(0x70008a2474, 0x70007ff4e7, 0x70007ff4e2);
        }
        9 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff4ef); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff4f5); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007ff502); };
            c.w64(7, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007ff506); };
            c.w64(3, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007ff50a); };
            c.w64(2, t14);
            blk = 12; continue;
        }
        12 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff50e); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            let Some(t17) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff514); };
            c.w32(1, t17);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff524); }
            c.w64(6, c.r64(7));
            c.w32(0, 0x1u64);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70007ff52d); };
            c.w64(3, t18);
            c.w64(1, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70007ff53f, 0x70007ff53a);
        }
        15 => {
            blk = 20; continue;
        }
        16 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007ff541); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            blk = 20; continue;
        }
        17 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff549); };
            c.w32(1, t21);
            c.logic(c.r32(1) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            if let Some(sr) = c.shr(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            let r22 = c.r64(1) & 0xffffffu64;
            c.w64(1, r22);
            c.logic(r22, 64);
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 20; continue;
        }
        19 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff564); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70007ff565); };
            c.w64(1, t23);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x70007ff57b, 0x70007ff576);
        }
        20 => {
            c.w32(1, 0xfeu64);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007ff580); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007ff584); };
            c.w64(2, t25);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff58b); }
            blk = 2; continue;
        }
        21 => {
            return RegfileExit::jump(0x70007ff594);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff5b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(1, 0x2u64);
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007ff5c0); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007ff5c4); };
            c.w32(7, t1);
            c.w32(1, c.r32(1));
            c.w32(8, c.r32(7));
            let r2 = c.r32(1) | c.r32(8);
            c.w32(1, r2);
            c.logic(r2, 32);
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70007ff5d7); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007ff5db); }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x0, 0x70007ff5e2);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff5e3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff5e7); }
            let r3 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff5ed); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70007ff5f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70007ff5f7); }
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff5fb); };
            c.cmp(t6, c.r32(1), 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(1, 0xeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff60c); }
            blk = 2; continue;
        }
        7 => {
            let Some(t7) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff60f); };
            c.w32(1, t7);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007ff61d); };
            c.w64(7, t8);
            c.w32(1, c.r16(1));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70007ff624); }
            c.cmp(c.r32(1), 0xbau64, 32);
            if c.cc_ge() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(2, c.r64(1));
            c.w32(1, 0x1u64);
            blk = 17; continue;
        }
        10 => {
            c.w64(6, c.r64(7));
            return c.call(0x70008a2474, 0x70007ff642, 0x70007ff63d);
        }
        11 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff64a); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff650); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(7, 0x1u64);
            blk = 15; continue;
        }
        14 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        15 => {
            let Some(t12) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff662); };
            c.w32(2, t12);
            c.cmp(c.r16(2), 0x41u64, 16);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.w32(1, 0x1u64);
            let r13 = c.r32(8) | c.r32(7);
            c.w32(8, r13);
            c.logic(r13, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(2, c.r16(2));
            blk = 17; continue;
        }
        17 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff683); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            let Some(t16) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff689); };
            c.w32(7, t16);
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(2), 0x41u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff69e); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x24c0)) else { return RegfileExit::fault(0x70007ff69f); };
            c.w64(3, t17);
            c.w64(0, c.r64(1));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007ff6a9); };
            c.w64(6, t18);
            return c.call(0x700086ed74, 0x70007ff6b2, 0x70007ff6ad);
        }
        21 => {
            blk = 26; continue;
        }
        22 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007ff6b4); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            blk = 26; continue;
        }
        23 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff6bc); };
            c.w32(2, t21);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r22 = c.r64(2) & 0xffffffu64;
            c.w64(2, r22);
            c.logic(r22, 64);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            blk = 26; continue;
        }
        25 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff6d7); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70007ff6d8); };
            c.w64(3, t23);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(1));
            return c.call(0x700086ed74, 0x70007ff6e9, 0x70007ff6e4);
        }
        26 => {
            c.w32(1, 0xfeu64);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007ff6ee); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007ff6f2); };
            c.w64(2, t25);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff6f9); }
            blk = 2; continue;
        }
        27 => {
            c.w32(1, 0x40u64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff707); }
            blk = 2; continue;
        }
        28 => {
            return RegfileExit::jump(0x70007ff710);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff71a); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff720); };
            c.cmp(t2, c.r32(2), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return c.ret(0x0, 0x70007ff729);
        }
        4 => {
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff72a); };
            c.w32(2, t3);
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_b() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70007ff73a);
        }
        7 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007ff73b); };
            c.w32(0, t4);
            let r5 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r5);
            return c.ret(0x0, 0x70007ff741);
        }
        8 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff742); };
            c.w32(2, t6);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r7 = c.r64(2) & 0xffffffu64;
            c.w64(2, r7);
            c.logic(r7, 64);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            return c.ret(0x0, 0x70007ff75b);
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff75c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff760); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff762); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70007ff763); };
            c.w64(3, t8);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007ff776, 0x70007ff771);
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff779); }
            return c.ret(0x0, 0x70007ff77a);
        }
        12 => {
            return RegfileExit::jump(0x70007ff77c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return c.ret(0x0, 0x70007ff798);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff799); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff79d); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff7a3); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70007ff7a9); }
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff7ad); };
            c.cmp(t3, c.r32(2), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff7b9); }
            return c.ret(0x0, 0x70007ff7ba);
        }
        4 => {
            let Some(t4) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff7bb); };
            c.w32(2, t4);
            c.cmp(c.r16(2), 0xbau64, 16);
            if c.cc_b() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.call(0x70008a2474, 0x70007ff7cb, 0x70007ff7c6);
        }
        6 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff7cf); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff7d5); };
            c.cmp(t7, c.r32(2), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t8) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff7de); };
            c.w32(2, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007ff7e2); };
            c.w64(6, t9);
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff7e6); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff7ec); };
            c.w32(1, t12);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(2), 0x41u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff801); }
            c.w32(0, 0x1u64);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70007ff807); };
            c.w64(3, t13);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70007ff816, 0x70007ff811);
        }
        13 => {
            blk = 3; continue;
        }
        14 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff818); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1d60)) else { return RegfileExit::fault(0x70007ff819); };
            c.w64(3, t14);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007ff82a, 0x70007ff825);
        }
        15 => {
            blk = 3; continue;
        }
        16 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007ff82c); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            blk = 3; continue;
        }
        17 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff834); };
            c.w32(2, t17);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r18 = c.r64(2) & 0xffffffu64;
            c.w64(2, r18);
            c.logic(r18, 64);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            blk = 3; continue;
        }
        19 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff852); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70007ff853); };
            c.w64(3, t19);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007ff866, 0x70007ff861);
        }
        20 => {
            blk = 3; continue;
        }
        21 => {
            return RegfileExit::jump(0x70007ff86c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff87d); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ff883); };
            c.cmp(t2, c.r32(2), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff88c); };
            c.w32(1, t3);
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007ff896); };
            c.w32(3, t4);
            let r5 = c.r32(3) & 0x12u64;
            c.w32(3, r5);
            c.logic(r5, 32);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xb30)) else { return RegfileExit::fault(0x70007ff8a7); };
            c.w64(0, t6);
            return c.ret(0x0, 0x70007ff8ae);
        }
        6 => {
            c.cmp(c.r16(1), 0xbau64, 16);
            if c.cc_ae() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 11; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xaa8)) else { return RegfileExit::fault(0x70007ff8c2); };
            c.w64(0, t7);
            return c.ret(0x0, 0x70007ff8c9);
        }
        10 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x550)) else { return RegfileExit::fault(0x70007ff8ca); };
            c.w64(0, t8);
            return c.ret(0x0, 0x70007ff8d1);
        }
        11 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x70007ff8d2); };
            c.w64(0, t9);
            return c.ret(0x0, 0x70007ff8d9);
        }
        12 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x918)) else { return RegfileExit::fault(0x70007ff8da); };
            c.w64(0, t10);
            return c.ret(0x0, 0x70007ff8e1);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x730)) else { return RegfileExit::fault(0x70007ff8e2); };
            c.w64(0, t11);
            return c.ret(0x0, 0x70007ff8e9);
        }
        14 => {
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007ff8ea); };
            c.w32(0, t12);
            let r13 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r13);
            return c.ret(0x0, 0x70007ff8f0);
        }
        15 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x70007ff8f1); };
            c.w64(0, t14);
            return c.ret(0x0, 0x70007ff8f8);
        }
        16 => {
            return RegfileExit::jump(0x70007ff8fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ff914); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ff918); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ff91e); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff924); };
            c.cmp(t2, 0x41u64, 16);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff92b); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70007ff92c); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007ff93d, 0x70007ff938);
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ff93d); };
            c.w32(2, t4);
            c.logic(c.r32(2) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ff948); };
            c.w64(0, t5);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let r6 = c.neg(c.r64(0), 64);
            c.w64(0, r6);
            blk = 6; continue;
        }
        6 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ff957); }
            return c.ret(0x0, 0x70007ff958);
        }
        7 => {
            blk = 6; continue;
        }
        8 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 6; continue;
        }
        9 => {
            c.w32(2, 0x28u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007ff964); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ff965); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70007ff966); };
            c.w64(3, t7);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007ff977, 0x70007ff972);
        }
        10 => {
            return RegfileExit::fault(0x70007ff977);
        }
        11 => {
            return RegfileExit::jump(0x70007ff97c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            return RegfileExit::fault(0x70007ff994);
        }
        1 => {
            return RegfileExit::jump(0x70007ff998);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ff9b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007ff9ba); };
            c.w64(3, t0);
            let Some(t1) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007ff9c1); };
            c.w64(1, t1);
            c.w64(6, c.r64(1).wrapping_add(0x10));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007ff9c8); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007ff9cf); };
            c.cmp(t3, c.r64(6), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70007ff9d4); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70007ff9db); };
            c.w64(1, t4);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007ff9e2); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007ff9eb); }
            let r5 = c.neg(c.r64(0), 64);
            c.w64(0, r5);
            if !c.st64(c.r64(3).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x70007ff9f5); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70007ff9fc);
        }
        5 => {
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70007ff9fd); }
            if !c.st64(c.r64(3).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x70007ffa04); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70007ffa0b);
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007ffa0c); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007ffa13); };
            c.w64(1, t7);
            c.w64(7, c.r64(1).wrapping_add(0x8));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007ffa1a); };
            c.w64(6, t8);
            let Some(t9) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70007ffa21); };
            c.cmp(t9, c.r64(7), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70007ffa26); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70007ffa2d); };
            c.w64(1, t10);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007ffa34); }
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70007ffa37); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70007ffa41);
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ffa42); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ffa46); }
            let r11 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007ffa51); }
            return c.call(0x70007febb4, 0x70007ffa5a, 0x70007ffa55);
        }
        10 => {
            c.w64(3, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007ffa5d); };
            c.w64(0, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ffa64); }
            blk = 3; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ffa6a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ffa6e); }
            let r13 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r13);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70007ffa7e, 0x70007ffa79);
        }
        12 => {
            c.w64(3, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ffa84); }
            blk = 8; continue;
        }
        13 => {
            return RegfileExit::jump(0x70007ffa88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ffa94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            return RegfileExit::fault(0x70007ffa94);
        }
        1 => {
            return RegfileExit::jump(0x70007ffa98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ffab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007ffab4); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007ffab9); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(3));
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70007ffac9); };
            c.cmp(t2, c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffad2); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007ffad8); };
            c.logic(t5 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ffade); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ffae7); };
            c.cmp(t7, 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 8; continue;
        }
        7 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007ffaf5); };
            c.w64(0, t8);
            return c.ret(0x8, 0x70007ffafc);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70007ffaff); };
            c.w64(0, t9);
            return c.ret(0x8, 0x70007ffb06);
        }
        10 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ffb09); };
            c.logic(t10 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70007ffb14); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x70007ffb28);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007ffb34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffb39); };
            c.w32(8, t0);
            blk = 3; continue;
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ffb3f); };
            c.w64(8, t1);
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007ffb46); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            let Some(t4) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ffb4e); };
            c.w32(11, t4);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r5 = c.sub(c.r64(11), 0x2u64, 64);
            c.w64(11, r5);
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11).wrapping_mul(4));
            c.w32(15, c.r32(8));
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ffb67); };
            c.w32(7, t6);
            let r7 = c.r32(15) | 0x2u64;
            c.w32(15, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let r8 = c.sub(c.r64(11), 0x2u64, 64);
            c.w64(11, r8);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ge() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 28; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007ffb84); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 68; continue; }
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
            c.w64(12, c.r64(11));
            let r11 = c.r64(12) & 0xfu64;
            c.w64(12, r11);
            c.logic(r11, 64);
            c.cmp(c.r64(12), 0x4u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x5u64, 64);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(12), 0xdu64, 64);
            if c.cc_ne() { blk = 159; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007ffbbf); };
            c.w64(0, t12);
            return c.ret(0x0, 0x70007ffbc6);
        }
        13 => {
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x70007ffbca);
        }
        14 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            if c.cc_ne() { blk = 29; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(11));
            if let Some(sr) = c.shr(c.r64(8), 0x9u64, 64) { c.w64(8, sr); }
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let r13 = c.r64(8) & 0x7ffu64;
            c.w64(8, r13);
            c.logic(r13, 64);
            c.logic(c.r8(11) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ffbf7); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007ffc04); };
            c.w64(9, t16);
            blk = 18; continue;
        }
        18 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffc0b); };
            c.w32(8, t17);
            let r18 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r18);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70007ffc1c);
        }
        20 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffc23); };
            c.w32(9, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ffc27); };
            c.cmp(t20, c.r32(9), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70007ffc30); }
            blk = 63; continue;
        }
        23 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffc41); };
            c.w32(0, t21);
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            return c.ret(0x0, 0x70007ffc49);
        }
        25 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffc4a); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t25) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffc58); };
            c.w32(9, t25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ffc5c); };
            c.cmp(t26, c.r32(9), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70007ffc65); }
            blk = 63; continue;
        }
        28 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007ffc70); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007ffc71); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007ffc72); }
            let Some(t27) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007ffc73); };
            if !c.push64(t27) { return RegfileExit::fault(0x70007ffc73); }
            c.w32(0, 0x4u64);
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2018)) else { return RegfileExit::fault(0x70007ffc7c); };
            c.w64(3, t28);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x70007ffc83); }
            return RegfileExit::jump(0x700086ed74);
        }
        29 => {
            c.w64(8, c.r64(11));
            let r29 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r29);
            let r30 = c.r32(8) & 0xfu64;
            c.w32(8, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 323; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 306; continue;
        }
        35 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 306; continue;
        }
        38 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 306; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 306; continue;
        }
        46 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r31 = c.r64(11) & 0xfffffu64;
            c.w64(11, r31);
            c.logic(r31, 64);
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffd1d); };
            c.w32(8, (t32 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x70007ffd27);
        }
        47 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r33 = c.r64(11) & 0xfffffu64;
            c.w64(11, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld16(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffd33); };
            c.w32(8, (t34 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x70007ffd3d);
        }
        48 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r35 = c.r64(11) & 0xfffffu64;
            c.w64(11, r35);
            c.logic(r35, 64);
            let Some(t36) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffd49); };
            c.w32(8, t36);
            c.w64(9, c.r64(8));
            let r37 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r37);
            if c.cc_o() { blk = 307; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70007ffd5d);
        }
        50 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r38 = c.r64(11) & 0xfffffu64;
            c.w64(11, r38);
            c.logic(r38, 64);
            let Some(t39) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffd69); };
            c.w32(8, t39);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 312; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            return c.ret(0x0, 0x70007ffd7f);
        }
        52 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r40 = c.r64(11) & 0xfffffu64;
            c.w64(11, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld64(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffd8b); };
            c.w64(8, t41);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 58; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007ffd96); };
            c.w64(9, t42);
            let Some(t43) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007ffd9d); };
            c.w64(11, t43);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007ffda4); };
            c.w64(15, t44);
            let Some(t45) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007ffdab); };
            c.cmp(t45, c.r64(12), 64);
            if c.cc_be() { blk = 317; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70007ffdb4); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70007ffdbb); };
            c.w64(11, t46);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70007ffdc2); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70007ffdcc); }
            let r47 = c.neg(c.r64(8), 64);
            c.w64(8, r47);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70007ffdd7); }
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x70007ffdde);
        }
        57 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70007ffddf); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70007ffde7); }
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x70007ffdee);
        }
        58 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007ffdef); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70007ffdf6); };
            c.w64(9, t49);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007ffdfd); };
            c.w64(11, t50);
            let Some(t51) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007ffe04); };
            c.cmp(t51, c.r64(12), 64);
            if c.cc_be() { blk = 319; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70007ffe0d); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 60; continue;
        }
        60 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70007ffe14); };
            c.w64(9, t52);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70007ffe1b); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70007ffe1f); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70007ffe2a);
        }
        61 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r53 = c.r64(11) & 0xfffffu64;
            c.w64(11, r53);
            c.logic(r53, 64);
            if !c.movss_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70007ffe36); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 63; continue;
        }
        62 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r54 = c.r64(11) & 0xfffffu64;
            c.w64(11, r54);
            c.logic(r54, 64);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70007ffe4e); }
            blk = 63; continue;
        }
        63 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007ffe55); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70007ffe5c); };
            c.w64(9, t56);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007ffe63); };
            c.w64(12, t57);
            let Some(t58) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007ffe6a); };
            c.cmp(t58, c.r64(11), 64);
            if c.cc_be() { blk = 321; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70007ffe74); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 65; continue;
        }
        65 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007ffe7b); };
            c.w64(9, t59);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70007ffe82); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007ffe86); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70007ffe8f);
        }
        66 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r60 = c.r64(11) & 0xfffffu64;
            c.w64(11, r60);
            c.logic(r60, 64);
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffe9b); };
            c.w32(0, t61);
            let r62 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r62);
            return c.ret(0x0, 0x70007ffea3);
        }
        67 => {
            blk = 28; continue;
        }
        68 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007ffea9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007ffead); }
            let r63 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r63);
            let Some(t64) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007ffeb3); };
            c.w32(9, t64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007ffeb7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70007ffebb); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70007ffebf); }
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70007ffec3); };
            c.cmp(t65, c.r32(9), 32);
            if c.cc_e() { blk = 155; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007ffed0); };
            c.w32(9, t66);
            let r67 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r67);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t68) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ffedf); };
            c.cmp(t68, c.r32(11), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007ffee8); }
            blk = 28; continue;
        }
        72 => {
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007ffeee); };
            c.w32(9, t69);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70007ffef8); }
            c.logic(c.r8(12) & 0x30u64, 8);
            if c.cc_e() { blk = 147; continue; }
            blk = 73; continue;
        }
        73 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 136; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t70) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fff10); };
            c.w32(12, t70);
            let r71 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r71);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70007fff1d); };
            c.w64(12, t72);
            blk = 76; continue;
        }
        76 => {
            let Some(t73) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fff24); };
            c.w32(15, t73);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r74 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r74);
            let Some(t75) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fff33); };
            c.w32(7, t75);
            let r76 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r76);
            let r77 = c.r64(7) & c.r64(15);
            c.w64(7, r77);
            c.logic(r77, 64);
            c.w64(0, c.r64(11));
            blk = 77; continue;
        }
        77 => {
            c.w64(3, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(9, c.r64(3).wrapping_mul(4));
            let Some(t78) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fff4c); };
            c.w32(6, t78);
            let Some(t79) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fff51); };
            c.cmp(t79, c.r32(6), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            let r80 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r80);
            c.w64(9, c.r64(0).wrapping_add(c.r64(7)));
            c.w64(3, c.r64(15));
            let r81 = c.r64(3) & c.r64(9);
            c.w64(3, r81);
            c.logic(r81, 64);
            c.w64(7, c.r64(3));
            blk = 77; continue;
        }
        80 => {
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fff75); };
            c.w32(8, t82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007fff7a); };
            c.w32(15, t83);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_b() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            return RegfileExit::fault(0x70007fff8e);
        }
        82 => {
            let Some(t84) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fff8f); };
            c.w32(0, t84);
            let r85 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r85);
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t86) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fffa1); };
            c.w32(8, t86);
            let Some(t87) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70007fffa5); };
            c.cmp(t87, c.r32(8), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t88) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fffb2); };
            c.w32(8, t88);
            let r89 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r89);
            let Some(t90) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fffb9); };
            c.w32(9, t90);
            let r91 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r91);
            let Some(t92) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007fffc0); };
            c.logic(t92 & 0x2u64, 8);
            if c.cc_ne() { blk = 107; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70007fffcb); };
            c.cmp(t93, c.r32(9), 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fffd4); };
            c.w64(0, t94);
            blk = 87; continue;
        }
        87 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fffde); }
            return c.ret(0x0, 0x70007fffdf);
        }
        88 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70007fffe0); };
            c.w64(9, t95);
            let Some(t96) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x70007fffe7); };
            c.cmp(t96, 0x20u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t97) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x70007ffff1); };
            c.w32(9, t97);
            let Some(t98) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007ffff5); };
            c.cmp(t98, c.r32(9), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007ffffe); };
            c.w64(0, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800002); };
            c.w64(3, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800006); };
            c.w64(6, t101);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080000d); }
            blk = 28; continue;
        }
        91 => {
            let Some(t102) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800013); };
            c.w32(9, t102);
            let r103 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r103);
            let Some(t104) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080001a); };
            c.w32(12, t104);
            let Some(t105) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700080001e); };
            c.cmp(t105, c.r32(12), 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t106) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000800027); };
            c.w32(9, t106);
            let r107 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r107);
            let Some(t108) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080002e); };
            c.w32(12, t108);
            let Some(t109) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000800032); };
            c.cmp(t109, c.r32(12), 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t110) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800045); };
            c.w32(12, t110);
            let r111 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r111);
            let Some(t112) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080004c); };
            c.w32(15, t112);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 100; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(9, c.r16(15));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t113) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700080006b); };
            c.cmp(t113, c.r32(12), 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(9, c.r64(2));
            blk = 100; continue;
        }
        98 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800079); };
            c.w64(0, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080007d); };
            c.w64(3, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800081); };
            c.w64(6, t116);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800088); }
            blk = 28; continue;
        }
        99 => {
            let Some(t117) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080008e); };
            c.w32(9, t117);
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            blk = 100; continue;
        }
        100 => {
            let Some(t119) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000800095); };
            c.w32(9, t119);
            let r120 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r120);
            let Some(t121) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080009c); };
            c.w32(9, t121);
            let r122 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r122);
            let Some(t123) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008000a3); };
            c.w32(12, t123);
            let r124 = c.sub(c.r32(12), 0x89u64, 32);
            c.w32(12, r124);
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_a() { blk = 103; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t125) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008000b5); };
            c.w32(6, t125);
            let r126 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r126);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008000bc); }
            c.w64(2, c.r64(8));
            c.w64(1, c.r64(11));
            return c.call(0x70007f6fb4, 0x70008000c8, 0x70008000c3);
        }
        102 => {
            blk = 87; continue;
        }
        103 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008000cd); };
            c.w64(0, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008000d1); };
            c.w64(3, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008000d5); };
            c.w64(6, t129);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008000dc); }
            blk = 28; continue;
        }
        104 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008000e2); };
            c.w64(0, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008000e6); };
            c.w64(3, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008000ea); };
            c.w64(6, t132);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008000f1); }
            blk = 28; continue;
        }
        105 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008000f7); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008000f8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008000f9); }
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008000fa); };
            c.w64(3, t133);
            c.w32(0, 0x3u64);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800106); };
            c.w64(6, t134);
            return c.call(0x700086ed74, 0x700080010f, 0x700080010a);
        }
        106 => {
            blk = 87; continue;
        }
        107 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000800114); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(8));
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080011a); };
            c.w64(6, t135);
            return c.call(0x70007f2274, 0x7000800123, 0x700080011e);
        }
        108 => {
            blk = 87; continue;
        }
        109 => {
            let Some(t136) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800128); };
            c.w32(8, t136);
            let r137 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r137);
            let Some(t138) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080012f); };
            c.w32(9, t138);
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 132; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w32(11, c.r64(9).wrapping_sub(0x80c));
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 120; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_e() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800159); };
            c.w64(0, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080015d); };
            c.w64(3, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800161); };
            c.w64(6, t141);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800168); }
            blk = 28; continue;
        }
        113 => {
            let Some(t142) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080016e); };
            c.w32(8, t142);
            let Some(t143) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000800172); };
            c.cmp(t143, c.r32(8), 32);
            if c.cc_e() { blk = 115; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080017b); };
            c.w64(0, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080017f); };
            c.w64(3, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800183); };
            c.w64(6, t146);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080018a); }
            blk = 28; continue;
        }
        115 => {
            let Some(t147) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800190); };
            c.w32(8, t147);
            let r148 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r148);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080019d); };
            c.w64(0, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008001a1); };
            c.w64(3, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008001a5); };
            c.w64(6, t151);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008001ac); }
            blk = 28; continue;
        }
        117 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008001b2); };
            c.w32(9, t152);
            let r153 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r153);
            let Some(t154) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008001b9); };
            c.cmp(t154, 0x40u64, 16);
            if c.cc_b() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008001c1); };
            c.w64(0, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008001c5); };
            c.w64(3, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008001c9); };
            c.w64(6, t157);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008001d0); }
            blk = 28; continue;
        }
        119 => {
            let Some(t158) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008001d6); };
            c.w32(8, t158);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 87; continue;
        }
        120 => {
            let Some(t159) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008001e3); };
            c.w32(9, t159);
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008001e7); };
            c.cmp(t160, c.r32(9), 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008001f0); };
            c.w64(0, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008001f4); };
            c.w64(3, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008001f8); };
            c.w64(6, t163);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008001ff); }
            blk = 28; continue;
        }
        122 => {
            let Some(t164) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000800205); };
            c.w32(8, t164);
            c.w64(9, c.r64(8));
            let r165 = c.r32(9) & 0xc0u64;
            c.w32(9, r165);
            c.logic(r165, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t166) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080021d); };
            c.w32(9, t166);
            let r167 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r167);
            let Some(t168) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000800224); };
            c.w32(9, t168);
            let r169 = c.r32(9) & 0x1fu64;
            c.w32(9, r169);
            c.logic(r169, 32);
            let r170 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r170);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_be() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800236); };
            c.w64(0, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080023a); };
            c.w64(3, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080023e); };
            c.w64(6, t173);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800245); }
            blk = 28; continue;
        }
        125 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t174) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000800251); };
            c.w32(0, t174);
            let r175 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r175);
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000800257); };
            c.cmp(t176, c.r32(0), 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t177) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800260); };
            c.w32(8, t177);
            let Some(t178) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000800264); };
            c.cmp(t178, c.r32(8), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 87; continue;
        }
        129 => {
            let Some(t179) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000800272); };
            c.w32(0, t179);
            let r180 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r180);
            blk = 87; continue;
        }
        130 => {
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080027d); };
            c.w64(0, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800281); };
            c.w64(3, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000800285); };
            c.w64(6, t183);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080028c); }
            blk = 28; continue;
        }
        131 => {
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800292); };
            c.w64(0, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800296); };
            c.w64(3, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080029a); };
            c.w64(6, t186);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008002a1); }
            blk = 28; continue;
        }
        132 => {
            let Some(t187) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008002a7); };
            c.w32(8, t187);
            let Some(t188) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008002ab); };
            c.cmp(t188, c.r32(8), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008002b4); };
            c.w64(0, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008002b8); };
            c.w64(3, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008002bc); };
            c.w64(6, t191);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008002c3); }
            blk = 28; continue;
        }
        134 => {
            let Some(t192) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008002c9); };
            c.w32(0, t192);
            let r193 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r193);
            blk = 87; continue;
        }
        135 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008002d4); };
            c.w64(0, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008002d8); };
            c.w64(3, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008002dc); };
            c.w64(6, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008002e0); };
            c.w64(9, t197);
            blk = 147; continue;
        }
        136 => {
            let Some(t198) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008002e9); };
            c.w32(12, t198);
            let r199 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r199);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 137; continue;
        }
        137 => {
            let r200 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r200);
            c.logic(r200, 64);
            let Some(t201) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008002fe); };
            c.w32(15, t201);
            let r202 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r202);
            let Some(t203) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000800305); };
            c.w32(15, t203);
            let r204 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r204);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t205) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800311); };
            c.w32(7, t205);
            let r206 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r206);
            let Some(t207) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800317); };
            c.cmp(t207, 0xbeu64, 16);
            if c.cc_e() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800322); }
            blk = 28; continue;
        }
        140 => {
            let Some(t208) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800328); };
            c.w32(7, t208);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xa9u64, 64);
            if c.cc_a() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            return RegfileExit::fault(0x700080033c);
        }
        142 => {
            let Some(t209) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080033d); };
            c.w32(7, t209);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t210) = c.ld32(c.r64(12).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x7000800344); };
            c.w32(12, t210);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xa9u64, 64);
            if c.cc_a() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            return RegfileExit::fault(0x7000800358);
        }
        144 => {
            let Some(t211) = c.ld32(c.r64(15).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x7000800359); };
            c.cmp(t211, c.r32(12), 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800365); }
            blk = 28; continue;
        }
        146 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080036e); }
            blk = 28; continue;
        }
        147 => {
            let Some(t212) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800374); };
            c.w32(12, t212);
            let r213 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r213);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t214) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000800381); };
            c.cmp(t214, c.r32(12), 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            let r215 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r215);
            c.logic(r215, 64);
            blk = 153; continue;
        }
        151 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800399); }
            blk = 28; continue;
        }
        152 => {
            c.w64(12, c.r64(2));
            blk = 153; continue;
        }
        153 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008003ab); }
            blk = 9; continue;
        }
        154 => {
            c.w64(0, c.r64(12));
            blk = 87; continue;
        }
        155 => {
            let Some(t216) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008003b9); };
            c.w64(5, t216);
            let r217 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r217);
            let Some(t218) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008003c1); };
            c.logic(t218 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            let r219 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r219);
            blk = 158; continue;
        }
        157 => {
            let Some(t220) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008003d1); };
            c.w32(8, t220);
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008003d5); };
            c.w64(8, t221);
            blk = 158; continue;
        }
        158 => {
            return RegfileExit::jump(c.r64(8));
        }
        159 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008003e0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008003e4); }
            let r222 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r222);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008003ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008003ee); }
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_e() { blk = 246; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r64(12), 0x6u64, 64);
            if c.cc_e() { blk = 244; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 241; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r64(12), 0x8u64, 64);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r64(12), 0x9u64, 64);
            if c.cc_e() { blk = 230; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_e() { blk = 175; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 243; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r64(12), 0xcu64, 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r64(12), 0xeu64, 64);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t223) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000800454); };
            c.w64(5, t223);
            let r224 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r224);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080045c); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700080045d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080045f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000800460); }
            let Some(t225) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000800461); };
            if !c.push64(t225) { return RegfileExit::fault(0x7000800461); }
            c.w32(0, 0x5u64);
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700080046a); };
            c.w64(3, t226);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x7000800471); }
            return RegfileExit::jump(0x700086ed74);
        }
        169 => {
            let Some(t227) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080047b); };
            c.w32(8, t227);
            let r228 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r228);
            let Some(t229) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800482); };
            c.w32(8, t229);
            let r230 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r230);
            let Some(t231) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800489); };
            c.w32(9, t231);
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r232 = c.r64(11) & 0x1ffffffu64;
            c.w64(11, r232);
            c.logic(r232, 64);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            return RegfileExit::fault(0x70008004a3);
        }
        171 => {
            let Some(t233) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008004a4); };
            c.w32(8, t233);
            let r234 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r234);
            let Some(t235) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008004ac); };
            c.w32(0, t235);
            let r236 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r236);
            let Some(t237) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008004b3); };
            c.cmp(t237, c.r32(0), 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t238) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008004c0); };
            c.w64(5, t238);
            let r239 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r239);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008004c8); }
            let Some(t240) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008004c9); };
            if !c.push64(t240) { return RegfileExit::fault(0x70008004c9); }
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008004cd); };
            c.w64(3, t241);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x174u64) { return RegfileExit::fault(0x70008004d9); }
            return RegfileExit::jump(0x700086ed74);
        }
        173 => {
            let Some(t242) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008004e7); };
            c.w64(5, t242);
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(2));
            c.w32(2, 0x2u64);
            c.w64(0, c.r64(9));
            let r243 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r243);
            return RegfileExit::jump(0x70008bc174);
        }
        174 => {
            blk = 243; continue;
        }
        175 => {
            let Some(t244) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800507); };
            c.w32(8, t244);
            let r245 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r245);
            let Some(t246) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080050e); };
            c.w32(11, t246);
            let Some(t247) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000800512); };
            c.cmp(t247, c.r32(8), 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 176; continue;
        }
        176 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 228; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t248) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800529); };
            c.w32(11, t248);
            let r249 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r249);
            let Some(t250) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000800530); };
            c.cmp(t250, c.r32(11), 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t251) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080053d); };
            c.w32(11, t251);
            let r252 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r252);
            let Some(t253) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800544); };
            c.w32(12, t253);
            let r254 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r254);
            let Some(t255) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700080054b); };
            c.logic(t255 & 0x2u64, 8);
            if c.cc_ne() { blk = 200; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t256) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000800557); };
            c.cmp(t256, c.r32(12), 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000800560); };
            c.w64(0, t257);
            blk = 87; continue;
        }
        181 => {
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700080056c); };
            c.w64(12, t258);
            let Some(t259) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x7000800573); };
            c.cmp(t259, 0x20u64, 8);
            if c.cc_e() { blk = 198; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t260) = c.ld32(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080057e); };
            c.w32(12, t260);
            let Some(t261) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000800582); };
            c.cmp(t261, c.r32(12), 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080058e); }
            blk = 28; continue;
        }
        184 => {
            let Some(t262) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800594); };
            c.w32(12, t262);
            let r263 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r263);
            let Some(t264) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080059b); };
            c.w32(15, t264);
            let Some(t265) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008005a0); };
            c.cmp(t265, c.r32(15), 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t266) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008005a9); };
            c.w32(12, t266);
            let r267 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r267);
            let Some(t268) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008005b1); };
            c.w32(15, t268);
            let Some(t269) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008005b6); };
            c.cmp(t269, c.r32(15), 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 186; continue;
        }
        186 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 197; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t270) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008005c9); };
            c.w32(15, t270);
            let r271 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r271);
            let Some(t272) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008005d1); };
            c.w32(7, t272);
            c.w32(0, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 192; continue; }
            blk = 188; continue;
        }
        188 => {
            c.w32(12, c.r16(7));
            c.cmp(c.r32(12), 0x82au64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t273) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008005ee); };
            c.cmp(t273, c.r32(15), 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008005f7); };
            c.w64(0, t274);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008005fe); }
            blk = 28; continue;
        }
        191 => {
            c.w64(9, c.r64(8));
            blk = 193; continue;
        }
        192 => {
            c.w64(9, c.r64(12));
            blk = 193; continue;
        }
        193 => {
            let Some(t275) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080060c); };
            c.w32(8, t275);
            let r276 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r276);
            let Some(t277) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800613); };
            c.w32(8, t277);
            let r278 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r278);
            let Some(t279) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080061a); };
            c.w32(9, t279);
            let r280 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r280);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 196; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t281) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700080062c); };
            c.w32(6, t281);
            let r282 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r282);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000800633); }
            c.w64(2, c.r64(11));
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700080063e, 0x7000800639);
        }
        195 => {
            blk = 87; continue;
        }
        196 => {
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800643); };
            c.w64(0, t283);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080064a); }
            blk = 28; continue;
        }
        197 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800653); }
            blk = 28; continue;
        }
        198 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000800659); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700080065b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080065c); }
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700080065d); };
            c.w64(3, t284);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700080066e, 0x7000800669);
        }
        199 => {
            blk = 87; continue;
        }
        200 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000800673); }
            c.w64(7, c.r64(11));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x700080067e, 0x7000800679);
        }
        201 => {
            blk = 87; continue;
        }
        202 => {
            let Some(t285) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800683); };
            c.w32(12, t285);
            let r286 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r286);
            let Some(t287) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080068a); };
            c.w32(15, t287);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 225; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w32(7, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 213; continue; }
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_e() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008006b3); }
            blk = 28; continue;
        }
        206 => {
            let Some(t288) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008006b9); };
            c.w32(9, t288);
            let Some(t289) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008006bd); };
            c.cmp(t289, c.r32(9), 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008006c9); }
            blk = 28; continue;
        }
        208 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 210; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008006d8); }
            blk = 28; continue;
        }
        210 => {
            let Some(t290) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008006de); };
            c.cmp(t290, 0x40u64, 16);
            if c.cc_b() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008006e9); }
            blk = 28; continue;
        }
        212 => {
            let Some(t291) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008006ef); };
            c.w32(8, t291);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 87; continue;
        }
        213 => {
            let Some(t292) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008006fc); };
            c.w32(11, t292);
            let Some(t293) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000800700); };
            c.cmp(t293, c.r32(11), 32);
            if c.cc_e() { blk = 215; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080070c); }
            blk = 28; continue;
        }
        215 => {
            let Some(t294) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000800712); };
            c.w32(11, t294);
            c.w64(12, c.r64(11));
            let r295 = c.r32(12) & 0xc0u64;
            c.w32(12, r295);
            c.logic(r295, 32);
            c.cmp(c.r32(12), 0xc0u64, 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t296) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700080072b); };
            c.w32(8, t296);
            let r297 = c.r32(8) & 0x1fu64;
            c.w32(8, r297);
            c.logic(r297, 32);
            let r298 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r298);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_be() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800740); }
            blk = 28; continue;
        }
        218 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 224; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t299) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700080074c); };
            c.w32(8, t299);
            let r300 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r300);
            let Some(t301) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000800753); };
            c.cmp(t301, c.r32(8), 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t302) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080075c); };
            c.w32(9, t302);
            let Some(t303) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000800760); };
            c.cmp(t303, c.r32(9), 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 221; continue;
        }
        221 => {
            c.w64(0, c.r64(8));
            blk = 87; continue;
        }
        222 => {
            let Some(t304) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000800771); };
            c.w32(0, t304);
            let r305 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r305);
            blk = 87; continue;
        }
        223 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800780); }
            blk = 28; continue;
        }
        224 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800789); }
            blk = 28; continue;
        }
        225 => {
            let Some(t306) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080078f); };
            c.w32(9, t306);
            let Some(t307) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000800793); };
            c.cmp(t307, c.r32(9), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080079f); }
            blk = 28; continue;
        }
        227 => {
            c.w64(0, c.r64(8));
            blk = 87; continue;
        }
        228 => {
            c.w64(0, c.r64(8));
            blk = 87; continue;
        }
        229 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008007b8); }
            blk = 28; continue;
        }
        230 => {
            let Some(t308) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008007be); };
            c.w64(12, t308);
            let Some(t309) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x70008007c5); };
            c.cmp(t309, 0x20u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 231; continue;
        }
        231 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t310) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008007d6); };
            c.w32(8, t310);
            let r311 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r311);
            blk = 234; continue;
        }
        233 => {
            let Some(t312) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008007df); };
            c.w32(8, t312);
            let r313 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r313);
            blk = 234; continue;
        }
        234 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            return RegfileExit::fault(0x70008007ec);
        }
        236 => {
            let r314 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r314);
            c.logic(r314, 64);
            let Some(t315) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008007f1); };
            c.w32(11, t315);
            let r316 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r316);
            let Some(t317) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008007f8); };
            c.w32(3, t317);
            let r318 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r318);
            let Some(t319) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008007ff); };
            c.w64(9, t319);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t320) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800809); };
            c.w32(11, t320);
            let r321 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r321);
            let Some(t322) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000800810); };
            c.w32(11, t322);
            let r323 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r323);
            blk = 239; continue;
        }
        238 => {
            c.w64(11, c.r64(2));
            blk = 239; continue;
        }
        239 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080081c); }
            c.w64(2, c.r64(9));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(8));
            c.w64(7, c.r64(11));
            return c.call(0x70007fe8d4, 0x700080082d, 0x7000800828);
        }
        240 => {
            blk = 87; continue;
        }
        241 => {
            let Some(t324) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000800832); };
            c.w64(8, t324);
            let Some(t325) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000800839); };
            c.cmp(t325, 0x20u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r326 = c.r64(11) & 0x3ffu64;
            c.w64(11, r326);
            c.logic(r326, 64);
            let r327 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r327);
            let Some(t328) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080084e); };
            c.w32(11, t328);
            let r329 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r329);
            let Some(t330) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000800855); };
            c.w32(11, t330);
            let r331 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r331);
            let Some(t332) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080085c); };
            c.w32(3, t332);
            let r333 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r333);
            let Some(t334) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000800864); };
            c.w64(5, t334);
            c.w64(1, c.r64(9));
            let r335 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r335);
            return RegfileExit::jump(0x70007fea14);
        }
        243 => {
            let Some(t336) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000800874); };
            c.w64(5, t336);
            let r337 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r337);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700080087c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080087d); }
            let Some(t338) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700080087e); };
            if !c.push64(t338) { return RegfileExit::fault(0x700080087e); }
            let Some(t339) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000800882); };
            c.w64(3, t339);
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x700080088e); }
            return RegfileExit::jump(0x700086ed74);
        }
        244 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r340 = c.r64(11) & 0x3ffu64;
            c.w64(11, r340);
            c.logic(r340, 64);
            let r341 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r341);
            let Some(t342) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008008a7); };
            c.w32(9, t342);
            let r343 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r343);
            let Some(t344) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008008ae); };
            c.w32(9, t344);
            let r345 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r345);
            let Some(t346) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008008b5); };
            c.w32(8, t346);
            let r347 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r347);
            let Some(t348) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008008bd); };
            c.w32(7, t348);
            let r349 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r349);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008008c4); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x70008008cc, 0x70008008c7);
        }
        245 => {
            blk = 87; continue;
        }
        246 => {
            let Some(t350) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008008d1); };
            c.w32(8, t350);
            let r351 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r351);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008008de); };
            c.w64(8, t352);
            blk = 248; continue;
        }
        248 => {
            let Some(t353) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008008e5); };
            c.w32(11, t353);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r354 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r354);
            let Some(t355) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008008f3); };
            c.w32(12, t355);
            let r356 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r356);
            let r357 = c.r64(12) & c.r64(11);
            c.w64(12, r357);
            c.logic(r357, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 249; continue;
        }
        249 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(0, c.r64(7).wrapping_mul(4));
            let Some(t358) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080090e); };
            c.w32(3, t358);
            let Some(t359) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000800913); };
            c.cmp(t359, c.r32(3), 32);
            if c.cc_e() { blk = 305; continue; }
            blk = 250; continue;
        }
        250 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 251; continue;
        }
        251 => {
            let r360 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r360);
            let r361 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r361);
            c.w64(0, c.r64(11));
            let r362 = c.r64(0) & c.r64(12);
            c.w64(0, r362);
            c.logic(r362, 64);
            c.w64(12, c.r64(0));
            blk = 249; continue;
        }
        252 => {
            let Some(t363) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800936); };
            c.w32(11, t363);
            let Some(t364) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080093a); };
            c.w32(12, t364);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 254; continue; }
            blk = 253; continue;
        }
        253 => {
            return RegfileExit::fault(0x700080094e);
        }
        254 => {
            let Some(t365) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080094f); };
            c.w32(0, t365);
            let r366 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r366);
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t367) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800961); };
            c.w32(8, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000800965); };
            c.cmp(t368, c.r32(8), 32);
            if c.cc_ne() { blk = 280; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t369) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800972); };
            c.w32(8, t369);
            let r370 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r370);
            let Some(t371) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800979); };
            c.w32(11, t371);
            let r372 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r372);
            let Some(t373) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000800980); };
            c.logic(t373 & 0x2u64, 8);
            if c.cc_ne() { blk = 278; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700080098b); };
            c.cmp(t374, c.r32(11), 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t375) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000800994); };
            c.w64(0, t375);
            blk = 87; continue;
        }
        259 => {
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008009a0); };
            c.w64(11, t376);
            let Some(t377) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008009a7); };
            c.cmp(t377, 0x20u64, 8);
            if c.cc_e() { blk = 276; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t378) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008009b1); };
            c.w32(11, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008009b5); };
            c.cmp(t379, c.r32(11), 32);
            if c.cc_e() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008009be); };
            c.w64(0, t380);
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008009c2); };
            c.w64(3, t381);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008009c9); }
            blk = 28; continue;
        }
        262 => {
            let Some(t382) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008009cf); };
            c.w32(11, t382);
            let r383 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r383);
            let Some(t384) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008009d6); };
            c.w32(12, t384);
            let Some(t385) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008009da); };
            c.cmp(t385, c.r32(12), 32);
            if c.cc_ne() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t386) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008009e3); };
            c.w32(11, t386);
            let r387 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r387);
            let Some(t388) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008009ea); };
            c.w32(12, t388);
            let Some(t389) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008009ee); };
            c.cmp(t389, c.r32(12), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 264; continue;
        }
        264 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 275; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t390) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800a01); };
            c.w32(12, t390);
            let r391 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r391);
            let Some(t392) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800a08); };
            c.w32(15, t392);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 270; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w32(11, c.r16(15));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t393) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000800a27); };
            c.cmp(t393, c.r32(12), 32);
            if c.cc_ne() { blk = 271; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800a30); };
            c.w64(0, t394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800a34); };
            c.w64(3, t395);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800a3b); }
            blk = 28; continue;
        }
        269 => {
            let Some(t396) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800a41); };
            c.w32(12, t396);
            let r397 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r397);
            c.w64(9, c.r64(12));
            blk = 271; continue;
        }
        270 => {
            c.w64(9, c.r64(11));
            blk = 271; continue;
        }
        271 => {
            let Some(t398) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000800a50); };
            c.w32(9, t398);
            let r399 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r399);
            let Some(t400) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800a57); };
            c.w32(9, t400);
            let r401 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r401);
            let Some(t402) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800a5e); };
            c.w32(11, t402);
            let r403 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r403);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 274; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t404) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000800a70); };
            c.w32(6, t404);
            let r405 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r405);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000800a77); }
            c.w64(2, c.r64(8));
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000800a82, 0x7000800a7d);
        }
        273 => {
            blk = 87; continue;
        }
        274 => {
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800a87); };
            c.w64(0, t406);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800a8b); };
            c.w64(3, t407);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800a92); }
            blk = 28; continue;
        }
        275 => {
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800a98); };
            c.w64(0, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800a9c); };
            c.w64(3, t409);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800aa3); }
            blk = 28; continue;
        }
        276 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000800aa9); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000800aab); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000800aac); }
            let Some(t410) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000800aad); };
            c.w64(3, t410);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000800abe, 0x7000800ab9);
        }
        277 => {
            blk = 87; continue;
        }
        278 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000800ac3); }
            c.w64(7, c.r64(8));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000800ace, 0x7000800ac9);
        }
        279 => {
            blk = 87; continue;
        }
        280 => {
            let Some(t411) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800ad3); };
            c.w32(8, t411);
            let r412 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r412);
            let Some(t413) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800ada); };
            c.w32(11, t413);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 302; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 291; continue; }
            blk = 282; continue;
        }
        282 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_e() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800b04); };
            c.w64(0, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800b08); };
            c.w64(3, t415);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800b0f); }
            blk = 28; continue;
        }
        284 => {
            let Some(t416) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800b15); };
            c.w32(8, t416);
            let Some(t417) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000800b19); };
            c.cmp(t417, c.r32(8), 32);
            if c.cc_e() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800b22); };
            c.w64(0, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800b26); };
            c.w64(3, t419);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800b2d); }
            blk = 28; continue;
        }
        286 => {
            let Some(t420) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800b33); };
            c.w32(8, t420);
            let r421 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r421);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 288; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800b40); };
            c.w64(0, t422);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800b44); };
            c.w64(3, t423);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800b4b); }
            blk = 28; continue;
        }
        288 => {
            let Some(t424) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800b51); };
            c.w32(9, t424);
            let r425 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r425);
            let Some(t426) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800b58); };
            c.cmp(t426, 0x40u64, 16);
            if c.cc_b() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800b60); };
            c.w64(0, t427);
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800b64); };
            c.w64(3, t428);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800b6b); }
            blk = 28; continue;
        }
        290 => {
            let Some(t429) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800b71); };
            c.w32(8, t429);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 87; continue;
        }
        291 => {
            let Some(t430) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800b7e); };
            c.w32(11, t430);
            let Some(t431) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000800b82); };
            c.cmp(t431, c.r32(11), 32);
            if c.cc_e() { blk = 293; continue; }
            blk = 292; continue;
        }
        292 => {
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800b8b); };
            c.w64(0, t432);
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800b8f); };
            c.w64(3, t433);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800b96); }
            blk = 28; continue;
        }
        293 => {
            let Some(t434) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000800b9c); };
            c.w32(8, t434);
            c.w64(11, c.r64(8));
            let r435 = c.r32(11) & 0xc0u64;
            c.w32(11, r435);
            c.logic(r435, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 296; continue; }
            blk = 294; continue;
        }
        294 => {
            let Some(t436) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800bb4); };
            c.w32(11, t436);
            let r437 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r437);
            let Some(t438) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000800bbb); };
            c.w32(11, t438);
            let r439 = c.r32(11) & 0x1fu64;
            c.w32(11, r439);
            c.logic(r439, 32);
            let r440 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r440);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800bcd); };
            c.w64(0, t441);
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800bd1); };
            c.w64(3, t442);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800bd8); }
            blk = 28; continue;
        }
        296 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 301; continue; }
            blk = 297; continue;
        }
        297 => {
            let Some(t443) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000800be4); };
            c.w32(0, t443);
            let r444 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r444);
            let Some(t445) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000800beb); };
            c.cmp(t445, c.r32(0), 32);
            if c.cc_e() { blk = 300; continue; }
            blk = 298; continue;
        }
        298 => {
            let Some(t446) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000800bf4); };
            c.w32(8, t446);
            let Some(t447) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000800bf8); };
            c.cmp(t447, c.r32(8), 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t448) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000800c05); };
            c.w32(0, t448);
            let r449 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r449);
            blk = 87; continue;
        }
        300 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800c10); };
            c.w64(0, t450);
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800c14); };
            c.w64(3, t451);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800c1b); }
            blk = 28; continue;
        }
        301 => {
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800c21); };
            c.w64(0, t452);
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800c25); };
            c.w64(3, t453);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800c2c); }
            blk = 28; continue;
        }
        302 => {
            let Some(t454) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000800c32); };
            c.w32(8, t454);
            let Some(t455) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000800c36); };
            c.cmp(t455, c.r32(8), 32);
            if c.cc_e() { blk = 304; continue; }
            blk = 303; continue;
        }
        303 => {
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800c3f); };
            c.w64(0, t456);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800c43); };
            c.w64(3, t457);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800c4a); }
            blk = 28; continue;
        }
        304 => {
            let Some(t458) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000800c50); };
            c.w32(0, t458);
            let r459 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r459);
            blk = 87; continue;
        }
        305 => {
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800c5c); };
            c.w64(0, t460);
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000800c60); };
            c.w64(3, t461);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800c67); }
            blk = 28; continue;
        }
        306 => {
            return RegfileExit::fault(0x7000800c6d);
        }
        307 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000800c6e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000800c72); }
            let r462 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r462);
            let Some(t463) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000800c78); };
            c.w64(9, t463);
            let Some(t464) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000800c7f); };
            c.w64(11, t464);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000800c86); }
            let Some(t465) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000800c8a); };
            c.w64(15, t465);
            let Some(t466) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000800c91); };
            c.cmp(t466, c.r64(12), 64);
            if c.cc_a() { blk = 310; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000800ca0, 0x7000800c9b);
        }
        309 => {
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800ca0); };
            c.w64(8, t467);
            blk = 311; continue;
        }
        310 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000800ca6); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 311; continue;
        }
        311 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t468) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000800cb9); };
            c.w64(8, t468);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000800cc0); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000800cc4); }
            blk = 87; continue;
        }
        312 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000800cce); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000800cd2); }
            let r469 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r469);
            let Some(t470) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000800cd8); };
            c.w64(9, t470);
            let Some(t471) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000800cdf); };
            c.w64(11, t471);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000800ce6); }
            let Some(t472) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000800cea); };
            c.w64(15, t472);
            let Some(t473) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000800cf1); };
            c.cmp(t473, c.r64(12), 64);
            if c.cc_a() { blk = 315; continue; }
            blk = 313; continue;
        }
        313 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000800d00, 0x7000800cfb);
        }
        314 => {
            let Some(t474) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800d00); };
            c.w64(8, t474);
            blk = 316; continue;
        }
        315 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000800d06); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 316; continue;
        }
        316 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t475) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000800d1c); };
            c.w64(8, t475);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000800d23); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000800d27); }
            blk = 87; continue;
        }
        317 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000800d31); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000800d35); }
            let r476 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r476);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000800d40); }
            return c.call(0x70007febb4, 0x7000800d49, 0x7000800d44);
        }
        318 => {
            c.w64(9, c.r64(0));
            let Some(t477) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000800d4c); };
            c.w64(8, t477);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800d53); }
            blk = 55; continue;
        }
        319 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000800d59); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000800d5d); }
            let r478 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r478);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000800d6d, 0x7000800d68);
        }
        320 => {
            c.w64(8, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800d73); }
            blk = 60; continue;
        }
        321 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000800d79); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000800d7d); }
            let r479 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r479);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000800d88); }
            return c.call(0x70007febf4, 0x7000800d92, 0x7000800d8d);
        }
        322 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000800d95); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000800d9d); }
            blk = 65; continue;
        }
        323 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000800da3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000800da7); }
            let r480 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r480);
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000800dad); };
            c.w64(8, t481);
            let Some(t482) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000800db4); };
            c.w32(8, t482);
            let r483 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r483);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000800dbb); }
            let Some(t484) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000800dbd); };
            c.w64(3, t484);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000800dd0, 0x7000800dcb);
        }
        324 => {
            return RegfileExit::fault(0x7000800dd0);
        }
        325 => {
            return RegfileExit::jump(0x7000800dd4);
        }
            _ => unreachable!(),
        }
    }
}
