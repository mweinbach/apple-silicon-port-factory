#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008e3674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e3674); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e3678); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e3679); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e367a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e367f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e3693); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e36a1); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e36a7); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e36b3); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e36b9); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e36bf); };
            c.w32(7, t9);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4au64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008e36ce);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_add(0x12f)) else { return RegfileExit::fault(0x70008e36cf); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e36d5); };
            c.w64(8, t11);
            let Some(t12) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e36dc); };
            c.w64(9, t12);
            c.w64(11, c.r64(9).wrapping_add(0x18));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e36e3); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e36ea); };
            c.cmp(t14, c.r64(11), 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008e36f4); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e36fb); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e36ff); };
            c.w64(7, t15);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008e3706); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008e370a); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008e370e); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008e3714); }
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008e3718); }
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            let r16 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r16);
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e3729); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e372a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e3730); }
            return c.ret(0x0, 0x70008e3732);
        }
        9 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3738); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3739); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3e3)) else { return RegfileExit::fault(0x70008e3740); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3749); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e374a); };
            c.w64(3, t20);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e375b, 0x70008e3756);
        }
        10 => {
            return RegfileExit::fault(0x70008e375b);
        }
        11 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3761); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3762); };
            c.w64(1, t21);
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0x3e3)) else { return RegfileExit::fault(0x70008e3769); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3772); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3773); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e3784, 0x70008e377f);
        }
        12 => {
            return RegfileExit::fault(0x70008e3784);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e3785); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e3789); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e378d); };
            c.w64(3, t25);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e379e, 0x70008e3799);
        }
        14 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e379e); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e37a2); };
            c.w64(0, t27);
            blk = 1; continue;
        }
        15 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e37b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008e37b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008e37b8); }
            return c.call(0x70007febb4, 0x70008e37c1, 0x70008e37bc);
        }
        16 => {
            c.w64(8, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e37c4); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e37c8); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e37cc); };
            c.w64(7, t30);
            blk = 8; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008e37d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e37f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e37f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e37f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e37f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e37fa); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008e37ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e3803); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e381b); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008e3820); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e382e); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e3835); };
            c.logic(t5 & 0x40u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008e3843); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 55; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(12, c.r64(0).wrapping_add(c.r64(0)));
            blk = 5; continue;
        }
        5 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008e3858); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e385a); };
            c.w64(15, t6);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008e3861); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008e3863); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(8));
            c.w64(2, c.r64(8));
            return c.call(0x70007f7214, 0x70008e3877, 0x70008e3872);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008e3877); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e3883); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e388a); };
            c.cmp(t9, 0x82eu64, 16);
            if c.cc_ne() { blk = 68; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e3897); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e389e); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 66; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008e38a9); };
            c.w32(9, t13);
            c.w64(11, c.r64(9));
            let r14 = c.r32(11) & 0xfffffffffffffffeu64;
            c.w32(11, r14);
            c.logic(r14, 32);
            let r15 = c.r32(9) & 0xfffffffffffffffcu64;
            c.w32(9, r15);
            c.logic(r15, 32);
            if !c.st32(c.r64(0).wrapping_add(0x33), c.r32(11)) { return RegfileExit::fault(0x70008e38b8); }
            if !c.st32(c.r64(0).wrapping_add(0x33), c.r32(9)) { return RegfileExit::fault(0x70008e38bc); }
            let Some(t16) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e38c0); };
            c.w64(9, t16);
            let Some(t17) = c.ld8(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e38c4); };
            c.logic(t17 & 0x1u64, 8);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e38ca); };
            c.w64(11, (t18 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 14; continue;
        }
        11 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e38d3); };
            c.w64(12, t19);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e38d7); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(10, 0x8000000000000000u64);
            let r20 = c.r64(11) | c.r64(10);
            c.w64(11, r20);
            c.logic(r20, 64);
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 64; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t21) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e391c); };
            c.w32(8, t21);
            c.w64(9, c.r64(8));
            let r22 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r22);
            let r23 = c.r32(8) & 0xfcu64;
            c.w32(8, r23);
            c.logic(r23, 32);
            let r24 = c.r32(9) & 0x3fu64;
            c.w32(9, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_g() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(8, 0x90au64);
            blk = 39; continue;
        }
        22 => {
            c.w32(8, 0x904u64);
            blk = 39; continue;
        }
        23 => {
            c.w32(8, 0x8feu64);
            blk = 39; continue;
        }
        24 => {
            c.w32(8, 0x8f8u64);
            blk = 39; continue;
        }
        25 => {
            c.w32(8, 0x8f2u64);
            blk = 39; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x70008e39af);
        }
        33 => {
            c.w32(8, 0x8ecu64);
            blk = 39; continue;
        }
        34 => {
            c.w32(8, 0x8e6u64);
            blk = 39; continue;
        }
        35 => {
            c.w32(8, 0x8e0u64);
            blk = 39; continue;
        }
        36 => {
            c.w32(8, 0x8dau64);
            blk = 39; continue;
        }
        37 => {
            c.w32(8, 0x8d4u64);
            blk = 39; continue;
        }
        38 => {
            c.w32(8, 0x8ceu64);
            blk = 39; continue;
        }
        39 => {
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008e39e1); }
            blk = 41; continue;
        }
        40 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e39e7); };
            c.w64(9, t25);
            let r26 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e39ef); };
            c.w64(8, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e39f3); };
            c.w64(0, t28);
            blk = 41; continue;
        }
        41 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e39f7); };
            c.w64(11, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e39fb); };
            c.w64(12, t30);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_ae() { blk = 48; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008e3a04); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t31) = c.ld64(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e3a0d); };
            c.w64(15, t31);
            c.w64(1, c.r64(15));
            blk = 45; continue;
        }
        44 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e3a16); };
            c.w64(1, t32);
            blk = 45; continue;
        }
        45 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3a1d); };
            c.w64(6, t33);
            c.w64(3, c.r64(9));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e3a24); };
            return c.call(t34, 0x70008e3a2c, 0x70008e3a24);
        }
        46 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 40; continue;
        }
        48 => {
            c.w64(8, c.r64(11));
            let r35 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r35);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e3a3d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e3a3e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e3a44); }
            return c.ret(0x0, 0x70008e3a46);
        }
        49 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3a4d); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3a4f); };
            c.w64(8, t36);
            let Some(t37) = c.ld32(c.r64(8).wrapping_add(0x3e7)) else { return RegfileExit::fault(0x70008e3a56); };
            c.w32(8, t37);
            let r38 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r38);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3a60); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3a62); };
            c.w64(3, t39);
            c.w32(0, 0x2u64);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3a6e); };
            c.w64(6, t40);
            return c.call(0x700086ed74, 0x70008e3a77, 0x70008e3a72);
        }
        50 => {
            return RegfileExit::fault(0x70008e3a77);
        }
        51 => {
            c.w32(9, 0xc2u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e3a7e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3a80); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3a82); };
            c.w64(3, t41);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e3a93, 0x70008e3a8e);
        }
        52 => {
            return RegfileExit::fault(0x70008e3a93);
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008e3a94); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e3a98); };
            c.w64(3, t42);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008e3aa9, 0x70008e3aa4);
        }
        54 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3aa9); };
            c.w64(6, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e3aad); };
            c.w64(0, t44);
            blk = 1; continue;
        }
        55 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e3ab6); };
            c.w64(12, t45);
            let Some(t46) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e3abd); };
            c.w64(15, t46);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008e3ac5); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e3ac9); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e3ad0); };
            c.cmp(t48, c.r64(0), 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e3adf, 0x70008e3ada);
        }
        57 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e3adf); };
            c.w64(9, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e3ae3); };
            c.w64(8, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3ae7); };
            c.w64(6, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e3aeb); };
            c.w64(11, t52);
            blk = 59; continue;
        }
        58 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008e3af1); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 59; continue;
        }
        59 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 63; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            let r53 = c.r64(10) | 0x1u64;
            c.w64(10, r53);
            c.logic(r53, 64);
            blk = 62; continue;
        }
        62 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 63; continue;
        }
        63 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e3b23); };
            c.w64(12, t54);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e3b2a); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e3b2e); }
            c.w64(12, c.r64(0));
            blk = 5; continue;
        }
        64 => {
            c.w32(8, 0x296u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3b41); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3b43); };
            c.w64(3, t55);
            c.w32(0, 0x1u64);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3b4f); };
            c.w64(6, t56);
            return c.call(0x700086ed74, 0x70008e3b58, 0x70008e3b53);
        }
        65 => {
            return RegfileExit::fault(0x70008e3b58);
        }
        66 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3b5f); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3b61); };
            c.w64(8, t57);
            let Some(t58) = c.ld32(c.r64(8).wrapping_add(0x3e7)) else { return RegfileExit::fault(0x70008e3b68); };
            c.w32(8, t58);
            let r59 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r59);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3b72); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3b74); };
            c.w64(3, t60);
            c.w32(0, 0x2u64);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3b80); };
            c.w64(6, t61);
            return c.call(0x700086ed74, 0x70008e3b89, 0x70008e3b84);
        }
        67 => {
            return RegfileExit::fault(0x70008e3b89);
        }
        68 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3b90); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3b92); };
            c.w64(8, t62);
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(0x3e7)) else { return RegfileExit::fault(0x70008e3b99); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3ba3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e3ba5); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3ba6); };
            c.w64(3, t65);
            c.w32(0, 0x3u64);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3bb2); };
            c.w64(6, t66);
            return c.call(0x700086ed74, 0x70008e3bbb, 0x70008e3bb6);
        }
        69 => {
            return RegfileExit::fault(0x70008e3bbb);
        }
        70 => {
            return RegfileExit::jump(0x70008e3bc0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e3bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e3bd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e3bd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e3bd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e3bda); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e3bdf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e3be3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e3bfb); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008e3c00); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e3c0e); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e3c15); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 59; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e3c22); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e3c29); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008e3c44); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e3c4e); };
            c.w64(0, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e3c54); };
            c.w64(0, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008e3c5b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e3c67); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e3c6d); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_ge() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e3c7d); };
            c.w64(1, t14);
            blk = 14; continue;
        }
        11 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t15) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e3c8c); };
            c.w64(1, t15);
            blk = 14; continue;
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e3c92); };
            c.w64(1, t16);
            blk = 14; continue;
        }
        14 => {
            let Some(t17) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e3c99); };
            c.w32(8, t17);
            c.w64(11, c.r64(8));
            let r18 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r18);
            let r19 = c.r32(8) & 0xfcu64;
            c.w32(8, r19);
            c.logic(r19, 32);
            let r20 = c.r32(11) & 0x3fu64;
            c.w32(11, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_g() { blk = 25; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(8, 0x906u64);
            blk = 38; continue;
        }
        21 => {
            c.w32(8, 0x900u64);
            blk = 38; continue;
        }
        22 => {
            c.w32(8, 0x8fau64);
            blk = 38; continue;
        }
        23 => {
            c.w32(8, 0x8f4u64);
            blk = 38; continue;
        }
        24 => {
            c.w32(8, 0x8eeu64);
            blk = 38; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x70008e3d2c);
        }
        32 => {
            c.w32(8, 0x8e8u64);
            blk = 38; continue;
        }
        33 => {
            c.w32(8, 0x8e2u64);
            blk = 38; continue;
        }
        34 => {
            c.w32(8, 0x8dcu64);
            blk = 38; continue;
        }
        35 => {
            c.w32(8, 0x8d6u64);
            blk = 38; continue;
        }
        36 => {
            c.w32(8, 0x8d0u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(8, 0x8cau64);
            blk = 38; continue;
        }
        38 => {
            let Some(t21) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e3d5b); };
            c.w64(11, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008e3d5f); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008e3d66); }
            blk = 40; continue;
        }
        39 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e3d6c); };
            c.w64(15, t22);
            let r23 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e3d74); };
            c.w64(8, t24);
            c.w64(1, c.r64(0));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e3d7b); };
            c.w64(12, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e3d7f); };
            c.w64(9, t26);
            blk = 40; continue;
        }
        40 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e3d83); };
            c.cmp(t27, c.r32(1), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008e3d90); }
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e3d94); };
            c.cmp(c.r64(15), t28, 64);
            if c.cc_ae() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e3d9a); };
            c.w32(7, t29);
            let r30 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r30);
            let Some(t31) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e3da1); };
            c.logic(t31 & 0x4u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008e3da7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008e3dab); }
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(15));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e3db5); };
            return c.call(t32, 0x70008e3dbd, 0x70008e3db5);
        }
        43 => {
            let Some(t33) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e3dbd); };
            c.cmp(t33, 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e3dc3); };
            c.cmp(t34, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e3dd1); };
            c.w64(15, t35);
            c.w32(1, c.r64(15).wrapping_add(c.r64(15)));
            blk = 46; continue;
        }
        46 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e3dd9); };
            if !c.push64(t36) { return RegfileExit::fault(0x70008e3dd9); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3ddc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e3ddd); }
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3dde); };
            if !c.push64(t37) { return RegfileExit::fault(0x70008e3dde); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e3de1); };
            c.w64(1, t38);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3de8); }
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e3de9); };
            c.w64(7, t39);
            c.w32(0, 0x4u64);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e3df2); };
            c.w64(6, t40);
            return c.call(0x70007f2134, 0x70008e3dfb, 0x70008e3df6);
        }
        47 => {
            blk = 39; continue;
        }
        48 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, c.r64(12));
            let r41 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r41);
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e3e12); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e3e13); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e3e19); }
            return c.ret(0x0, 0x70008e3e1b);
        }
        50 => {
            c.w32(1, 0x140u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3e21); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3e22); };
            c.w64(1, t42);
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x3eb)) else { return RegfileExit::fault(0x70008e3e29); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3e32); }
            c.w32(0, 0x2u64);
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3e38); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e3e3f); };
            c.w64(6, t46);
            return c.call(0x700086ed74, 0x70008e3e48, 0x70008e3e43);
        }
        51 => {
            return RegfileExit::fault(0x70008e3e48);
        }
        52 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t47) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e3e4f); };
            c.w64(1, t47);
            blk = 55; continue;
        }
        54 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e3e55); };
            c.w64(1, t48);
            blk = 55; continue;
        }
        55 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3e62); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3e64); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3e65); };
            c.w64(3, t49);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e3e76, 0x70008e3e71);
        }
        56 => {
            return RegfileExit::fault(0x70008e3e76);
        }
        57 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3e7c); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3e7d); };
            c.w64(1, t50);
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0x3eb)) else { return RegfileExit::fault(0x70008e3e84); };
            c.w32(1, t51);
            let r52 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r52);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3e8d); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3e8e); };
            c.w64(3, t53);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e3e9f, 0x70008e3e9a);
        }
        58 => {
            return RegfileExit::fault(0x70008e3e9f);
        }
        59 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3ea5); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3ea6); };
            c.w64(1, t54);
            let Some(t55) = c.ld32(c.r64(1).wrapping_add(0x3eb)) else { return RegfileExit::fault(0x70008e3ead); };
            c.w32(1, t55);
            let r56 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r56);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e3eb6); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3eb7); };
            c.w64(3, t57);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e3ec8, 0x70008e3ec3);
        }
        60 => {
            return RegfileExit::fault(0x70008e3ec8);
        }
        61 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e3ec9); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e3ecd); };
            c.w64(3, t58);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e3ede, 0x70008e3ed9);
        }
        62 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e3ede); };
            c.w64(6, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3ee2); };
            c.w64(0, t60);
            blk = 1; continue;
        }
        63 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e3eeb); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e3ef2); };
            c.w64(8, t62);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e3ef9); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e3efd); };
            c.w64(11, t63);
            let Some(t64) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e3f04); };
            c.cmp(t64, c.r64(9), 64);
            if c.cc_a() { blk = 66; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e3f13, 0x70008e3f0e);
        }
        65 => {
            c.w64(1, c.r64(0));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e3f16); };
            c.w64(0, t65);
            blk = 67; continue;
        }
        66 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008e3f1c); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 67; continue;
        }
        67 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e3f23); };
            c.w64(10, t66);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 71; continue; }
            blk = 68; continue;
        }
        68 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let r67 = c.r64(10) | 0x1u64;
            c.w64(10, r67);
            c.logic(r67, 64);
            blk = 70; continue;
        }
        70 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 71; continue;
        }
        71 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e3f4b); };
            c.w64(8, t68);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e3f52); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e3f56); }
            blk = 46; continue;
        }
        72 => {
            return RegfileExit::jump(0x70008e3f64);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e3f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e3f74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e3f78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e3f79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e3f7a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e3f7f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e3f83); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e3f9b); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008e3fa0); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e3fae); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e3fb5); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 59; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e3fc2); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e3fc9); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008e3fe4); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e3fee); };
            c.w64(0, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e3ff4); };
            c.w64(0, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e3ffb); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e4007); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e400d); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_ge() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e401d); };
            c.w64(1, t14);
            blk = 14; continue;
        }
        11 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t15) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e402c); };
            c.w64(1, t15);
            blk = 14; continue;
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4032); };
            c.w64(1, t16);
            blk = 14; continue;
        }
        14 => {
            let Some(t17) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e4039); };
            c.w32(8, t17);
            c.w64(11, c.r64(8));
            let r18 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r18);
            let r19 = c.r32(8) & 0xfcu64;
            c.w32(8, r19);
            c.logic(r19, 32);
            let r20 = c.r32(11) & 0x3fu64;
            c.w32(11, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_g() { blk = 25; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(8, 0x906u64);
            blk = 38; continue;
        }
        21 => {
            c.w32(8, 0x900u64);
            blk = 38; continue;
        }
        22 => {
            c.w32(8, 0x8fau64);
            blk = 38; continue;
        }
        23 => {
            c.w32(8, 0x8f4u64);
            blk = 38; continue;
        }
        24 => {
            c.w32(8, 0x8eeu64);
            blk = 38; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x70008e40cc);
        }
        32 => {
            c.w32(8, 0x8e8u64);
            blk = 38; continue;
        }
        33 => {
            c.w32(8, 0x8e2u64);
            blk = 38; continue;
        }
        34 => {
            c.w32(8, 0x8dcu64);
            blk = 38; continue;
        }
        35 => {
            c.w32(8, 0x8d6u64);
            blk = 38; continue;
        }
        36 => {
            c.w32(8, 0x8d0u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(8, 0x8cau64);
            blk = 38; continue;
        }
        38 => {
            let Some(t21) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e40fb); };
            c.w64(11, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008e40ff); }
            blk = 40; continue;
        }
        39 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e4105); };
            c.w64(11, t22);
            c.w64(1, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e410c); };
            c.w64(12, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e4110); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e4114); };
            c.w64(8, t25);
            blk = 40; continue;
        }
        40 => {
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e4118); };
            c.cmp(t26, c.r32(1), 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008e4125); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e412f); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            let Some(t29) = c.ld8(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e4136); };
            c.logic(t29 & 0x4u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 42; continue;
        }
        42 => {
            let r30 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r30);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008e4140); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008e4144); }
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(11));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e414e); };
            return c.call(t31, 0x70008e4156, 0x70008e414e);
        }
        43 => {
            let Some(t32) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e4156); };
            c.cmp(t32, 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e415c); };
            c.cmp(t33, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e416a); };
            c.w64(11, t34);
            c.w32(1, c.r64(11).wrapping_add(c.r64(11)));
            blk = 46; continue;
        }
        46 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e4172); };
            if !c.push64(t35) { return RegfileExit::fault(0x70008e4172); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e4175); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e4176); }
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4177); };
            if !c.push64(t36) { return RegfileExit::fault(0x70008e4177); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e417a); };
            c.w64(1, t37);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e4181); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e4182); };
            c.w64(7, t38);
            c.w32(0, 0x4u64);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e418b); };
            c.w64(6, t39);
            return c.call(0x70007f2134, 0x70008e4194, 0x70008e418f);
        }
        47 => {
            blk = 39; continue;
        }
        48 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, c.r64(12));
            let r40 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r40);
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e41ab); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e41ac); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e41b2); }
            return c.ret(0x0, 0x70008e41b4);
        }
        50 => {
            c.w32(1, 0x140u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e41ba); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e41bb); };
            c.w64(1, t41);
            let Some(t42) = c.ld32(c.r64(1).wrapping_add(0x3ef)) else { return RegfileExit::fault(0x70008e41c2); };
            c.w32(1, t42);
            let r43 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r43);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e41cb); }
            c.w32(0, 0x2u64);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e41d1); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e41d8); };
            c.w64(6, t45);
            return c.call(0x700086ed74, 0x70008e41e1, 0x70008e41dc);
        }
        51 => {
            return RegfileExit::fault(0x70008e41e1);
        }
        52 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t46) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e41e8); };
            c.w64(1, t46);
            blk = 55; continue;
        }
        54 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e41ee); };
            c.w64(1, t47);
            blk = 55; continue;
        }
        55 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e41fb); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e41fd); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e41fe); };
            c.w64(3, t48);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e420f, 0x70008e420a);
        }
        56 => {
            return RegfileExit::fault(0x70008e420f);
        }
        57 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e4215); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e4216); };
            c.w64(1, t49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x3ef)) else { return RegfileExit::fault(0x70008e421d); };
            c.w32(1, t50);
            let r51 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r51);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e4226); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e4227); };
            c.w64(3, t52);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e4238, 0x70008e4233);
        }
        58 => {
            return RegfileExit::fault(0x70008e4238);
        }
        59 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e423e); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e423f); };
            c.w64(1, t53);
            let Some(t54) = c.ld32(c.r64(1).wrapping_add(0x3ef)) else { return RegfileExit::fault(0x70008e4246); };
            c.w32(1, t54);
            let r55 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r55);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e424f); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e4250); };
            c.w64(3, t56);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e4261, 0x70008e425c);
        }
        60 => {
            return RegfileExit::fault(0x70008e4261);
        }
        61 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e4262); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e4266); };
            c.w64(3, t57);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e4277, 0x70008e4272);
        }
        62 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4277); };
            c.w64(6, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e427b); };
            c.w64(0, t59);
            blk = 1; continue;
        }
        63 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e4284); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e428b); };
            c.w64(8, t61);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008e4292); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e4296); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e429d); };
            c.cmp(t63, c.r64(9), 64);
            if c.cc_a() { blk = 66; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e42ac, 0x70008e42a7);
        }
        65 => {
            c.w64(1, c.r64(0));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e42af); };
            c.w64(0, t64);
            blk = 67; continue;
        }
        66 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008e42b5); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 67; continue;
        }
        67 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e42bc); };
            c.w64(10, t65);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 71; continue; }
            blk = 68; continue;
        }
        68 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let r66 = c.r64(10) | 0x1u64;
            c.w64(10, r66);
            c.logic(r66, 64);
            blk = 70; continue;
        }
        70 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 71; continue;
        }
        71 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e42e4); };
            c.w64(8, t67);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e42eb); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e42ef); }
            blk = 46; continue;
        }
        72 => {
            return RegfileExit::jump(0x70008e42fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e4314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e4314); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e4318); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4319); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e431a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e431f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e4323); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 166; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(11, c.r64(7).wrapping_add(0x18));
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(12).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e433b); };
            c.w64(12, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008e4340); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008e4344); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e4352); };
            c.w32(15, t3);
            let r4 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r4);
            let Some(t5) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e435a); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 164; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008e436a); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        5 => {
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e4384); };
            c.w64(11, t6);
            blk = 8; continue;
        }
        7 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e438a); };
            c.w64(11, t7);
            blk = 8; continue;
        }
        8 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4391); };
            c.cmp(t8, c.r32(11), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(11));
            blk = 13; continue;
        }
        11 => {
            c.w64(0, c.r64(11));
            return c.call(0x7000899a14, 0x70008e43b1, 0x70008e43ac);
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e43b1); };
            c.w64(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e43b5); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e43b9); };
            c.w64(15, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e43bd); };
            c.w64(12, t12);
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 192; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 24; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e43d6); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 192; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0x8000000000000000u64);
            let r13 = c.r64(11) | c.r64(10);
            c.w64(11, r13);
            c.logic(r13, 64);
            blk = 21; continue;
        }
        21 => {
            blk = 24; continue;
        }
        22 => {
            c.w32(3, 0x1u64);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        23 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            c.w64(2, c.r64(12));
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e4447); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e444e); };
            c.w32(12, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008e4455); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008e4459); }
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 194; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e446d); };
            c.w64(7, t17);
            blk = 29; continue;
        }
        28 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4473); };
            c.w64(7, t18);
            blk = 29; continue;
        }
        29 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t19) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e448d); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            let Some(t21) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e4494); };
            c.cmp(t21, 0x82eu64, 16);
            if c.cc_ne() { blk = 110; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e44a1); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            let Some(t24) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e44a7); };
            c.logic(t24 & 0x4u64, 8);
            if c.cc_ne() { blk = 194; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e44ba); };
            c.w64(3, t25);
            c.w64(1, c.r64(3).wrapping_add(c.r64(11)));
            let Some(t26) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e44c2); };
            c.cmp(t26, c.r64(1), 64);
            if c.cc_b() { blk = 192; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t27) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e44cc); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            let Some(t29) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e44d2); };
            c.w32(1, t29);
            c.w64(9, c.r64(1));
            let r30 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r30);
            let r31 = c.r32(9) & 0x3fu64;
            c.w32(9, r31);
            c.logic(r31, 32);
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_l() { blk = 53; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 168; continue;
        }
        41 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 168; continue;
        }
        45 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 50; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 168; continue;
        }
        50 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 168; continue;
        }
        53 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_l() { blk = 63; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 168; continue;
        }
        59 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 168; continue;
        }
        63 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_l() { blk = 68; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 67; continue;
        }
        67 => {
            blk = 168; continue;
        }
        68 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 168; continue;
        }
        71 => {
            blk = 88; continue;
        }
        72 => {
            c.w32(0, 0x1u64);
            blk = 88; continue;
        }
        73 => {
            c.w32(0, 0x1u64);
            blk = 88; continue;
        }
        74 => {
            c.w32(0, 0x2u64);
            blk = 88; continue;
        }
        75 => {
            c.w32(0, 0x2u64);
            blk = 88; continue;
        }
        76 => {
            c.w32(0, 0x2u64);
            blk = 88; continue;
        }
        77 => {
            c.w32(0, 0x3u64);
            blk = 88; continue;
        }
        78 => {
            c.w32(0, 0x3u64);
            blk = 88; continue;
        }
        79 => {
            c.w32(0, 0x3u64);
            blk = 88; continue;
        }
        80 => {
            c.w32(0, 0x1u64);
            blk = 88; continue;
        }
        81 => {
            c.w32(0, 0x1u64);
            blk = 88; continue;
        }
        82 => {
            c.w32(0, 0x2u64);
            blk = 88; continue;
        }
        83 => {
            c.w32(0, 0x2u64);
            blk = 88; continue;
        }
        84 => {
            c.w32(0, 0x2u64);
            blk = 88; continue;
        }
        85 => {
            c.w32(0, 0x3u64);
            blk = 88; continue;
        }
        86 => {
            c.w32(0, 0x3u64);
            blk = 88; continue;
        }
        87 => {
            c.w32(0, 0x3u64);
            blk = 88; continue;
        }
        88 => {
            let Some(t32) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e4680); };
            c.w32(8, t32);
            c.w64(6, c.r64(8));
            let r33 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r33);
            let r34 = c.r32(6) & 0x3fu64;
            c.w32(6, r34);
            c.logic(r34, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008e468e); }
            c.cmp(c.r32(6), c.r32(9), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 89; continue;
        }
        89 => {
            let r35 = c.r32(8) & 0xfcu64;
            c.w32(8, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(8), 0x64u64, 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.cmp(c.r32(8), 0x44u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r36 = c.r32(6) | c.r32(8);
            c.w32(6, r36);
            c.logic(r36, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 90; continue;
        }
        90 => {
            let r37 = c.r32(1) & 0xfcu64;
            c.w32(1, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(1), 0x64u64, 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.cmp(c.r32(1), 0x44u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let r38 = c.r32(6) | c.r32(1);
            c.w32(6, r38);
            c.logic(r38, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 109; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(6, 0x100000000u64);
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_b() { blk = 102; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(8, 0x1fffffffffffffu64);
            c.w64(1, c.r64(0));
            if let Some(sr) = c.shr(c.r64(8), c.r8(1), 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_b() { blk = 102; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r64(6), c.r64(11), 64);
            if c.cc_b() { blk = 101; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 101; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t39) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008e4729); };
            c.w64(6, t39);
            let Some(t40) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008e472d); };
            c.w32(2, t40);
            c.w64(1, c.r64(0));
            if let Some(sr) = c.shl(c.r64(11), c.r8(1), 64) { c.w64(11, sr); }
            let r41 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r41);
            c.w64(1, c.r64(0));
            if let Some(sr) = c.shl(c.r64(3), c.r8(1), 64) { c.w64(3, sr); }
            c.w64(1, c.r64(11).wrapping_add(c.r64(2)));
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t42) = c.ld64(c.r64(7).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008e4749); };
            c.w64(11, t42);
            let Some(t43) = c.ld32(c.r64(7).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008e474d); };
            c.w32(7, t43);
            c.w64(2, c.r64(11).wrapping_add(c.r64(7)));
            c.w64(10, c.r64(4));
            let r44 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r44);
            let r45 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r45);
            c.logic(r45, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e475f); }
            c.w64(8, c.r64(3));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x70008e4767); };
            c.w64(0, t46);
            c.w64(10, 0x70008e4775u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e4775); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e4779); }
            return c.call(c.r64(0), 0x70008e477f, 0x70008e477d);
        }
        98 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e477f); }
            let Some(t47) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e4787); };
            c.w64(4, t47);
            blk = 108; continue;
        }
        99 => {
            let Some(t48) = c.ld64(c.r64(7).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008e4791); };
            c.w64(11, t48);
            let Some(t49) = c.ld32(c.r64(7).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008e4795); };
            c.w32(7, t49);
            c.w64(2, c.r64(11).wrapping_add(c.r64(7)));
            c.w64(10, c.r64(4));
            let r50 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            let r51 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r51);
            c.logic(r51, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e47a7); }
            c.w64(8, c.r64(3));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x13e8)) else { return RegfileExit::fault(0x70008e47af); };
            c.w64(0, t52);
            c.w64(10, 0x70008e47bdu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e47bd); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e47c1); }
            return c.call(c.r64(0), 0x70008e47c7, 0x70008e47c5);
        }
        100 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e47c7); }
            let Some(t53) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e47cf); };
            c.w64(4, t53);
            blk = 108; continue;
        }
        101 => {
            return RegfileExit::fault(0x70008e47d6);
        }
        102 => {
            return RegfileExit::fault(0x70008e47d7);
        }
        103 => {
            blk = 109; continue;
        }
        104 => {
            c.w32(12, c.r64(9).wrapping_sub(0x1a));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e47de); };
            c.w64(0, t54);
            let r55 = c.sub(c.r32(0), 0x1au64, 32);
            c.w32(0, r55);
            c.cmp(c.r32(12), 0x1u64, 32);
            c.w8(12, (c.cc_be() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(0), 0x1u64, 32);
            c.w8(0, (c.cc_be() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(12), c.r32(0), 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 109; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(10, c.r64(4));
            let r56 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r56);
            let r57 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r57);
            c.logic(r57, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e4814); }
            c.w64(1, c.r64(7));
            c.w64(9, c.r64(11));
            c.w64(8, c.r64(3));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x1298)) else { return RegfileExit::fault(0x70008e4822); };
            c.w64(0, t58);
            c.w64(10, 0x70008e4830u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e4830); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e4834); }
            return c.call(c.r64(0), 0x70008e483a, 0x70008e4838);
        }
        107 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e483a); }
            let Some(t59) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e4842); };
            c.w64(4, t59);
            blk = 108; continue;
        }
        108 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4847); };
            c.w64(15, t60);
            blk = 109; continue;
        }
        109 => {
            c.w64(12, c.r64(15));
            c.w64(7, c.r64(12).wrapping_add(0x1));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4853); };
            c.w64(0, t61);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e485d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e485e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e4864); }
            return c.ret(0x0, 0x70008e4866);
        }
        110 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t62) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e486c); };
            c.w32(12, t62);
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e4873); };
            c.cmp(t64, c.r32(12), 32);
            if c.cc_ne() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(7, 0x84u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4881); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e4882); };
            c.w64(3, t65);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008e4893, 0x70008e488e);
        }
        113 => {
            return RegfileExit::fault(0x70008e4893);
        }
        114 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 172; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t66) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e489d); };
            c.cmp(t66, 0xbau64, 16);
            if c.cc_b() { blk = 172; continue; }
            blk = 116; continue;
        }
        116 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008e48b7); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t67) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e48c1); };
            c.w32(1, t67);
            let r68 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r68);
            let Some(t69) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e48c7); };
            c.cmp(t69, 0x834u64, 16);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e48cf); };
            c.w32(12, t70);
            let r71 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r71);
            blk = 140; continue;
        }
        119 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 121; continue;
        }
        120 => {
            c.w32(1, 0x1u64);
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t72) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e48e9); };
            c.w32(1, t72);
            let r73 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r73);
            let Some(t74) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e48ef); };
            c.w32(1, t74);
            let r75 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r75);
            let Some(t76) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e48f5); };
            c.w32(8, t76);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t77) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e48fc); };
            c.w32(9, t77);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xb0u64, 64);
            if c.cc_a() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            return RegfileExit::fault(0x70008e490c);
        }
        124 => {
            let Some(t78) = c.ld32(c.r64(1).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008e490d); };
            c.cmp(t78, c.r32(9), 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 176; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t79) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e4924); };
            c.w32(12, t79);
            let r80 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e492b); };
            c.w32(12, t81);
            let r82 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e4933); };
            c.w32(1, t83);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t84) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e493b); };
            c.w32(8, t84);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x44u64, 64);
            if c.cc_a() { blk = 128; continue; }
            blk = 127; continue;
        }
        127 => {
            return RegfileExit::fault(0x70008e4948);
        }
        128 => {
            let Some(t85) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x70008e4949); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r64(1), 0xaeu64, 64);
            if c.cc_a() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            return RegfileExit::fault(0x70008e495c);
        }
        131 => {
            let Some(t86) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008e495d); };
            c.cmp(t86, c.r32(8), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r64(1), 0xafu64, 64);
            if c.cc_a() { blk = 175; continue; }
            blk = 133; continue;
        }
        133 => {
            return RegfileExit::fault(0x70008e4974);
        }
        134 => {
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e4975); };
            c.w32(12, t87);
            let r88 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r88);
            blk = 135; continue;
        }
        135 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a17f4, 0x70008e498a, 0x70008e4985);
        }
        137 => {
            c.w64(12, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e498d); };
            c.w64(6, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4991); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e4995); };
            c.w64(2, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e4999); };
            c.w64(7, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e499d); };
            c.w64(11, t93);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e49a3); };
            c.w64(3, t94);
            blk = 140; continue;
        }
        138 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ge() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(12, c.r64(0));
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w64(8, c.r64(2));
            let Some(t95) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e49be); };
            c.w64(3, t95);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(2, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 146; continue;
        }
        143 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e49d0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 146; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t96) = c.ld64(0x70008e440au64) else { return RegfileExit::fault(0x70008e49e1); };
            c.w64(10, t96);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t97) = c.ld64(0x70008e4428u64) else { return RegfileExit::fault(0x70008e49fc); };
            c.w64(10, t97);
            let r98 = c.r64(2) | c.r64(10);
            c.w64(2, r98);
            c.logic(r98, 64);
            blk = 146; continue;
        }
        146 => {
            c.w64(1, c.r64(2).wrapping_add(c.r64(11)));
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_b() { blk = 192; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 163; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t99) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e4a1d); };
            c.w32(3, t99);
            let r100 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r100);
            let Some(t101) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e4a24); };
            c.w32(3, t101);
            let r102 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r102);
            let r103 = c.r32(3) & 0x3fu64;
            c.w32(3, r103);
            c.logic(r103, 32);
            let r104 = c.sub(c.r32(3), 0x1au64, 32);
            c.w32(3, r104);
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_be() { blk = 179; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t105) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e4a3a); };
            c.w32(3, t105);
            let r106 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r106);
            let Some(t107) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e4a40); };
            c.cmp(t107, 0x834u64, 16);
            if c.cc_ne() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            c.w32(1, 0x1u64);
            blk = 152; continue;
        }
        151 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t108) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e4a5a); };
            c.w32(1, t108);
            c.w64(9, c.r64(1));
            let r109 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r109);
            let r110 = c.r32(9) & 0x3fu64;
            c.w32(9, r110);
            c.logic(r110, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_a() { blk = 179; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t111) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e4a73); };
            c.w32(9, t111);
            let r112 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r112);
            let Some(t113) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e4a7a); };
            c.w32(9, t113);
            let r114 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r114);
            let Some(t115) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e4a81); };
            c.w32(6, t115);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x4cu64, 64);
            if c.cc_a() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            return RegfileExit::fault(0x70008e4a91);
        }
        156 => {
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008e4a92); };
            c.w32(6, t116);
            let Some(t117) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e4a99); };
            c.cmp(t117, c.r32(6), 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008e4aa2); };
            c.w64(3, t118);
            let Some(t119) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e4aa9); };
            c.cmp(t119, c.r32(0), 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w32(0, c.r8(1));
            let r120 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r120);
            let r121 = c.r32(0) & 0x3fu64;
            c.w32(0, r121);
            c.logic(r121, 32);
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_be() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            let r122 = c.sub(c.r32(0), 0x4u64, 32);
            c.w32(0, r122);
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_a() { blk = 179; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t123) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e4acc); };
            c.w32(12, t123);
            let r124 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r124);
            let Some(t125) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e4ad3); };
            c.logic(t125 & 0x4u64, 8);
            if c.cc_ne() { blk = 178; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w64(10, c.r64(4));
            let r126 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r126);
            let r127 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r127);
            c.logic(r127, 64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e4aea); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008e4aef); }
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4af4); };
            c.w64(1, t128);
            c.w64(9, c.r64(2));
            c.w64(2, c.r64(7));
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x1288)) else { return RegfileExit::fault(0x70008e4afe); };
            c.w64(0, t129);
            c.w64(10, 0x70008e4b0cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e4b0c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e4b10); }
            return c.call(c.r64(0), 0x70008e4b16, 0x70008e4b14);
        }
        162 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e4b16); }
            let Some(t130) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008e4b1e); };
            c.w64(4, t130);
            blk = 163; continue;
        }
        163 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4b23); };
            c.w64(7, t131);
            let r132 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r132);
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4b2b); };
            c.w64(0, t133);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e4b35); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e4b36); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e4b3c); }
            return c.ret(0x0, 0x70008e4b3e);
        }
        164 => {
            c.w32(7, 0xd6u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4b44); }
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e4b45); };
            c.w64(7, t134);
            let Some(t135) = c.ld32(c.r64(7).wrapping_add(0x3f3)) else { return RegfileExit::fault(0x70008e4b4c); };
            c.w32(7, t135);
            let r136 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r136);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4b55); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e4b56); };
            c.w64(3, t137);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e4b67, 0x70008e4b62);
        }
        165 => {
            return RegfileExit::fault(0x70008e4b67);
        }
        166 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e4b68); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e4b6e); };
            c.w64(3, t138);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008e4b7d, 0x70008e4b78);
        }
        167 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4b7d); };
            c.w64(6, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4b81); };
            c.w64(0, t140);
            blk = 1; continue;
        }
        168 => {
            return RegfileExit::fault(0x70008e4b8a);
        }
        169 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4b90); }
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e4b91); };
            c.w64(3, t141);
            c.w32(0, 0x1u64);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4b9d); };
            c.w64(6, t142);
            return c.call(0x700086ed74, 0x70008e4ba6, 0x70008e4ba1);
        }
        170 => {
            return RegfileExit::fault(0x70008e4ba6);
        }
        171 => {
            blk = 192; continue;
        }
        172 => {
            c.w64(0, c.r64(7));
            return c.call(0x70008a2054, 0x70008e4bb4, 0x70008e4baf);
        }
        173 => {
            c.w64(7, c.r64(0));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4bb7); };
            c.w64(6, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4bbb); };
            c.w64(15, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e4bbf); };
            c.w64(2, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e4bc3); };
            c.w64(11, t146);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e4bc9); };
            c.w64(3, t147);
            blk = 116; continue;
        }
        174 => {
            let Some(t148) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e4bd2); };
            c.w32(12, t148);
            let r149 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r149);
            blk = 135; continue;
        }
        175 => {
            let Some(t150) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x70008e4bde); };
            c.cmp(t150, c.r32(8), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008e4bec); };
            c.w64(3, t151);
            c.w64(0, c.r64(7));
            c.w64(12, c.r64(3));
            return c.call(0x7000870914, 0x70008e4bfe, 0x70008e4bf9);
        }
        177 => {
            c.w64(12, c.r64(0));
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4c01); };
            c.w64(6, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4c05); };
            c.w64(15, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e4c09); };
            c.w64(2, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e4c0d); };
            c.w64(7, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e4c11); };
            c.w64(11, t156);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e4c17); };
            c.w64(3, t157);
            blk = 135; continue;
        }
        178 => {
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4c20); };
            c.w64(6, t158);
            blk = 194; continue;
        }
        179 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008e4c29); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            let r159 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r159);
            blk = 190; continue;
        }
        181 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e4c3e); };
            c.w64(0, t160);
            let Some(t161) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008e4c45); };
            c.w64(3, t161);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e4c4c); };
            c.w64(1, t162);
            let Some(t163) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e4c53); };
            c.cmp(t163, c.r64(2), 64);
            if c.cc_a() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e4c62, 0x70008e4c5d);
        }
        183 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e4c62); };
            c.w64(11, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4c66); };
            c.w64(15, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e4c6a); };
            c.w64(8, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e4c6e); };
            c.w64(7, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e4c72); };
            c.w64(12, t168);
            blk = 185; continue;
        }
        184 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008e4c78); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 185; continue;
        }
        185 => {
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e4c7f); };
            c.w64(3, t169);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008e4c86); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 189; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            let r170 = c.r64(10) | 0x1u64;
            c.w64(10, r170);
            c.logic(r170, 64);
            blk = 188; continue;
        }
        188 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 189; continue;
        }
        189 => {
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e4cb0); }
            c.w64(11, c.r64(0));
            blk = 190; continue;
        }
        190 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e4cb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4cba); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008e4cbb); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e4cbd); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x2b30)) else { return RegfileExit::fault(0x70008e4cbf); };
            c.w64(3, t171);
            c.w32(0, 0x4u64);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4ccb); };
            c.w64(6, t172);
            return c.call(0x700086ed74, 0x70008e4cd4, 0x70008e4ccf);
        }
        191 => {
            blk = 163; continue;
        }
        192 => {
            c.w32(7, 0x1d0u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4cde); }
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008e4cdf); };
            c.w64(3, t173);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008e4cf0, 0x70008e4ceb);
        }
        193 => {
            return RegfileExit::fault(0x70008e4cf0);
        }
        194 => {
            c.w32(7, 0x66u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4cf6); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e4cf7); };
            c.w64(7, t174);
            let Some(t175) = c.ld32(c.r64(7).wrapping_add(0x3f3)) else { return RegfileExit::fault(0x70008e4cfe); };
            c.w32(7, t175);
            let r176 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r176);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4d07); }
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e4d08); };
            c.w64(3, t177);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e4d19, 0x70008e4d14);
        }
        195 => {
            return RegfileExit::fault(0x70008e4d19);
        }
        196 => {
            return RegfileExit::jump(0x70008e4d1c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e4d34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e4d34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e4d38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e4d39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e4d3a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008e4d3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e4d43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 220; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(11, c.r64(5));
            c.w64(12, c.r64(11).wrapping_add(0x18));
            c.w64(15, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e4d5b); };
            c.w64(15, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008e4d60); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008e4d64); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 265; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e4d72); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e4d79); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 265; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e4d85); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld8(c.r64(3).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e4d8c); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 263; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e4d96); };
            c.w64(3, t9);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008e4d9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008e4da1); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(11).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e4dab); };
            c.w64(11, t10);
            blk = 7; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4db1); };
            c.w64(11, t11);
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4db8); };
            c.w64(7, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4dbf); };
            c.cmp(t13, c.r32(11), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        9 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(11));
            return c.call(0x7000899a14, 0x70008e4dde, 0x70008e4dd9);
        }
        11 => {
            c.w64(11, c.r64(0));
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4de1); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4de5); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e4de9); };
            c.w64(0, t16);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4ded); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e4df4); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e4df8); };
            c.w64(12, t19);
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_b() { blk = 34; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, c.r64(3));
            blk = 34; continue;
        }
        16 => {
            let r20 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r20);
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_b() { blk = 34; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        18 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let r21 = c.r64(10) | 0x1u64;
            c.w64(10, r21);
            c.logic(r21, 64);
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e4e5a); }
            c.xorpd_rr(2, 2);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_a() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.mov128_rr(1, 0);
            blk = 25; continue;
        }
        25 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(1));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(10, 0x8000000000000000u64);
            let r22 = c.r64(11) | c.r64(10);
            c.w64(11, r22);
            c.logic(r22, 64);
            blk = 28; continue;
        }
        28 => {
            blk = 34; continue;
        }
        29 => {
            c.arith_sd(0, 1, c.xmm_u64(0));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(2));
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        31 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t23) = c.ld64(0x70008e4e7fu64) else { return RegfileExit::fault(0x70008e4ec3); };
            c.w64(10, t23);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(1));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t24) = c.ld64(0x70008e4e9du64) else { return RegfileExit::fault(0x70008e4ede); };
            c.w64(10, t24);
            let r25 = c.r64(11) | c.r64(10);
            c.w64(11, r25);
            c.logic(r25, 64);
            blk = 34; continue;
        }
        34 => {
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008e4eea); }
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t26) = c.ld64(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e4ef4); };
            c.w64(12, t26);
            blk = 37; continue;
        }
        36 => {
            c.w64(12, c.r64(7));
            blk = 37; continue;
        }
        37 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4efe); };
            c.cmp(t27, c.r32(12), 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(12, c.r64(3));
            blk = 65; continue;
        }
        39 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x70008e4f1d, 0x70008e4f18);
        }
        41 => {
            c.w64(12, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e4f20); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e4f24); };
            c.w64(6, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e4f28); };
            c.w64(0, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e4f2c); };
            c.w64(11, t31);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e4f32); };
            c.w64(7, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e4f39); };
            c.w64(3, t33);
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_l() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(12), c.r64(3), 64);
            if c.cc_b() { blk = 65; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(12, c.r64(3));
            blk = 65; continue;
        }
        46 => {
            let r34 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r34);
            c.cmp(c.r64(12), c.r64(3), 64);
            if c.cc_b() { blk = 65; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(12, c.r64(2));
            blk = 65; continue;
        }
        48 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e4f74); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let r35 = c.r64(10) | 0x1u64;
            c.w64(10, r35);
            c.logic(r35, 64);
            blk = 51; continue;
        }
        51 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 52; continue;
        }
        52 => {
            c.xorpd_rr(2, 2);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 59; continue; }
            blk = 53; continue;
        }
        53 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.mov128_rr(0, 1);
            blk = 55; continue;
        }
        55 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t36) = c.ld64(0x70008e4e7fu64) else { return RegfileExit::fault(0x70008e4fbf); };
            c.w64(10, t36);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_s() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t37) = c.ld64(0x70008e4e9du64) else { return RegfileExit::fault(0x70008e4fda); };
            c.w64(10, t37);
            let r38 = c.r64(3) | c.r64(10);
            c.w64(3, r38);
            c.logic(r38, 64);
            blk = 58; continue;
        }
        58 => {
            c.w64(12, c.r64(3));
            blk = 65; continue;
        }
        59 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(12, c.r64(2));
            blk = 65; continue;
        }
        61 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t39) = c.ld64(0x70008e4e7fu64) else { return RegfileExit::fault(0x70008e5002); };
            c.w64(10, t39);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_s() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t40) = c.ld64(0x70008e4e9du64) else { return RegfileExit::fault(0x70008e501d); };
            c.w64(10, t40);
            let r41 = c.r64(3) | c.r64(10);
            c.w64(3, r41);
            c.logic(r41, 64);
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(3));
            blk = 65; continue;
        }
        65 => {
            c.w64(3, c.r64(12));
            let r42 = c.sub(c.r64(3), c.r64(11), 64);
            c.w64(3, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008e5030); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_ge() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(3, c.r64(2));
            blk = 67; continue;
        }
        67 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008e503d); }
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 222; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w32(1, c.r64(3).wrapping_add(c.r64(3)));
            blk = 69; continue;
        }
        69 => {
            let Some(t43) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e5051); };
            c.w32(8, t43);
            let r44 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r44);
            let Some(t45) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e5058); };
            c.w32(9, t45);
            let r46 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r46);
            let r47 = c.r32(9) & 0x3fu64;
            c.w32(9, r47);
            c.logic(r47, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008e5065); }
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_l() { blk = 88; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_l() { blk = 80; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_l() { blk = 76; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 127; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 231; continue;
        }
        76 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 79; continue;
        }
        79 => {
            blk = 231; continue;
        }
        80 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 84; continue;
        }
        84 => {
            blk = 231; continue;
        }
        85 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 87; continue;
        }
        87 => {
            blk = 231; continue;
        }
        88 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_l() { blk = 98; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 94; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 115; continue; }
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 93; continue;
        }
        93 => {
            blk = 231; continue;
        }
        94 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 113; continue; }
            blk = 97; continue;
        }
        97 => {
            blk = 231; continue;
        }
        98 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_l() { blk = 103; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 231; continue;
        }
        103 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 231; continue;
        }
        106 => {
            c.w32(9, 0x10u64);
            blk = 128; continue;
        }
        107 => {
            c.w32(9, 0x11u64);
            blk = 128; continue;
        }
        108 => {
            c.w32(9, 0x12u64);
            blk = 128; continue;
        }
        109 => {
            c.w32(9, 0x13u64);
            blk = 128; continue;
        }
        110 => {
            c.w32(9, 0x14u64);
            blk = 128; continue;
        }
        111 => {
            c.w32(9, 0x15u64);
            blk = 128; continue;
        }
        112 => {
            c.w32(9, 0x16u64);
            blk = 128; continue;
        }
        113 => {
            c.w32(9, 0x17u64);
            blk = 128; continue;
        }
        114 => {
            c.w32(9, 0x18u64);
            blk = 128; continue;
        }
        115 => {
            c.w32(9, 0x19u64);
            blk = 128; continue;
        }
        116 => {
            c.w32(9, 0x1au64);
            blk = 128; continue;
        }
        117 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        118 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        119 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        120 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        121 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        122 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        123 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        124 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        125 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        126 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        127 => {
            c.w64(9, c.r64(2));
            blk = 128; continue;
        }
        128 => {
            let Some(t48) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e5226); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e522c); };
            c.w32(1, t50);
            let r51 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r51);
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e5232); };
            c.w32(11, t52);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            return RegfileExit::fault(0x70008e5241);
        }
        130 => {
            let Some(t53) = c.ld32(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e5242); };
            c.w32(9, t53);
            let r54 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r54);
            c.cmp(c.r64(11), 0xd4u64, 64);
            if c.cc_a() { blk = 132; continue; }
            blk = 131; continue;
        }
        131 => {
            return RegfileExit::fault(0x70008e5253);
        }
        132 => {
            let Some(t55) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e5254); };
            c.w32(11, t55);
            let r56 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r56);
            let Some(t57) = c.ld32(c.r64(1).wrapping_add(0x357)) else { return RegfileExit::fault(0x70008e525b); };
            c.w32(1, t57);
            let Some(t58) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e5261); };
            c.w32(11, t58);
            let r59 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r59);
            let Some(t60) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e5268); };
            c.cmp(t60, 0xbdu64, 16);
            if c.cc_ae() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008e5271); };
            c.w64(11, t61);
            blk = 135; continue;
        }
        134 => {
            let Some(t62) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e527a); };
            c.w32(11, t62);
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_ne() { blk = 232; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0xfc8)) else { return RegfileExit::fault(0x70008e5287); };
            c.w64(11, t63);
            let Some(t64) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e528e); };
            c.cmp(t64, c.r32(2), 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(9));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e529e); };
            c.w64(1, t65);
            c.w64(2, c.r64(7));
            return c.call(0x70008dd9b4, 0x70008e52aa, 0x70008e52a5);
        }
        138 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008e52aa); }
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e52ae); };
            c.w64(3, t66);
            let Some(t67) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e52b2); };
            c.cmp(t67, c.r64(3), 64);
            if c.cc_b() { blk = 257; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 217; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e52c6); };
            c.w64(11, t68);
            let Some(t69) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e52ca); };
            c.w32(12, t69);
            let r70 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e52d1); };
            c.w32(15, t71);
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 218; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t72) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e52e0); };
            c.w32(2, t72);
            let r73 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r73);
            let Some(t74) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e52e6); };
            c.w32(2, t74);
            let Some(t75) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e52ea); };
            c.w32(1, t75);
            let r76 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r76);
            c.w64(6, c.r64(2));
            let r77 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r77);
            let Some(t78) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e52f7); };
            c.w32(1, t78);
            let r79 = c.r32(6) & 0x3fu64;
            c.w32(6, r79);
            c.logic(r79, 32);
            c.cmp(c.r32(6), 0x1cu64, 32);
            if c.cc_l() { blk = 160; continue; }
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r32(6), 0x21u64, 32);
            if c.cc_l() { blk = 152; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(6), 0x24u64, 32);
            if c.cc_l() { blk = 148; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(6), 0x24u64, 32);
            if c.cc_e() { blk = 197; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(6), 0x25u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(6), 0x26u64, 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 147; continue;
        }
        147 => {
            blk = 254; continue;
        }
        148 => {
            c.cmp(c.r32(6), 0x21u64, 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(6), 0x22u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(6), 0x23u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 254; continue;
        }
        152 => {
            c.cmp(c.r32(6), 0x1eu64, 32);
            if c.cc_l() { blk = 157; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r32(6), 0x1eu64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r32(6), 0x1fu64, 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(6), 0x20u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 156; continue;
        }
        156 => {
            blk = 254; continue;
        }
        157 => {
            c.cmp(c.r32(6), 0x1cu64, 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(6), 0x1du64, 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 159; continue;
        }
        159 => {
            blk = 254; continue;
        }
        160 => {
            c.cmp(c.r32(6), 0x16u64, 32);
            if c.cc_l() { blk = 170; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(6), 0x19u64, 32);
            if c.cc_l() { blk = 166; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(6), 0x19u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(6), 0x1au64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(6), 0x1bu64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 165; continue;
        }
        165 => {
            blk = 254; continue;
        }
        166 => {
            c.cmp(c.r32(6), 0x16u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(6), 0x17u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 168; continue;
        }
        168 => {
            c.cmp(c.r32(6), 0x18u64, 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 169; continue;
        }
        169 => {
            blk = 254; continue;
        }
        170 => {
            c.cmp(c.r32(6), 0x13u64, 32);
            if c.cc_l() { blk = 175; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(6), 0x13u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(6), 0x14u64, 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r32(6), 0x15u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 174; continue;
        }
        174 => {
            blk = 254; continue;
        }
        175 => {
            c.cmp(c.r32(6), 0x11u64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(6), 0x12u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            blk = 254; continue;
        }
        178 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 200; continue;
        }
        179 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 200; continue;
        }
        180 => {
            c.w32(7, 0x1u64);
            blk = 200; continue;
        }
        181 => {
            c.w32(7, 0x1u64);
            blk = 200; continue;
        }
        182 => {
            c.w32(7, 0x2u64);
            blk = 200; continue;
        }
        183 => {
            c.w32(7, 0x2u64);
            blk = 200; continue;
        }
        184 => {
            c.w32(7, 0x2u64);
            blk = 200; continue;
        }
        185 => {
            c.w32(7, 0x3u64);
            blk = 200; continue;
        }
        186 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 200; continue;
        }
        187 => {
            c.w32(7, 0x3u64);
            blk = 200; continue;
        }
        188 => {
            c.w32(7, 0x3u64);
            blk = 200; continue;
        }
        189 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 200; continue;
        }
        190 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 200; continue;
        }
        191 => {
            c.w32(7, 0x1u64);
            blk = 200; continue;
        }
        192 => {
            c.w32(7, 0x1u64);
            blk = 200; continue;
        }
        193 => {
            c.w32(7, 0x2u64);
            blk = 200; continue;
        }
        194 => {
            c.w32(7, 0x2u64);
            blk = 200; continue;
        }
        195 => {
            c.w32(7, 0x2u64);
            blk = 200; continue;
        }
        196 => {
            c.w32(7, 0x3u64);
            blk = 200; continue;
        }
        197 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 200; continue;
        }
        198 => {
            c.w32(7, 0x3u64);
            blk = 200; continue;
        }
        199 => {
            c.w32(7, 0x3u64);
            blk = 200; continue;
        }
        200 => {
            c.w64(8, c.r64(1));
            let r80 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r80);
            let r81 = c.r32(8) & 0x3fu64;
            c.w32(8, r81);
            c.logic(r81, 32);
            c.cmp(c.r32(6), c.r32(8), 32);
            if c.cc_ne() { blk = 213; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e54ae); };
            c.cmp(t82, c.r32(12), 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 202; continue;
        }
        202 => {
            c.w64(12, 0x100000000u64);
            c.cmp(c.r64(12), c.r64(3), 64);
            if c.cc_b() { blk = 212; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w64(2, 0x1fffffffffffffu64);
            c.w64(1, c.r64(7));
            if let Some(sr) = c.shr(c.r64(2), c.r8(1), 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_b() { blk = 212; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e54e4); };
            c.cmp(c.r64(12), t83, 64);
            if c.cc_b() { blk = 211; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e54ee); };
            c.cmp(c.r64(2), t84, 64);
            if c.cc_b() { blk = 211; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t85) = c.ld64(c.r64(11).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008e54f8); };
            c.w64(12, t85);
            let Some(t86) = c.ld32(c.r64(11).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008e54fc); };
            c.w32(11, t86);
            c.w64(1, c.r64(7));
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e5503); };
            c.w64(2, t87);
            if let Some(sr) = c.shl(c.r64(2), c.r8(1), 64) { c.w64(2, sr); }
            let r88 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r88);
            c.w64(1, c.r64(7));
            if let Some(sr) = c.shl(c.r64(3), c.r8(1), 64) { c.w64(3, sr); }
            let r89 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r89);
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 209; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t90) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008e551c); };
            c.w64(11, t90);
            let Some(t91) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008e5520); };
            c.w32(12, t91);
            c.w64(1, c.r64(11).wrapping_add(c.r64(12)));
            c.w64(10, c.r64(4));
            let r92 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r92);
            let r93 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r93);
            c.logic(r93, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e5533); }
            c.w64(8, c.r64(3));
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x70008e553b); };
            c.w64(0, t94);
            c.w64(10, 0x70008e5549u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e5549); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e554d); }
            return c.call(c.r64(0), 0x70008e5553, 0x70008e5551);
        }
        208 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e5553); }
            let Some(t95) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e555b); };
            c.w64(4, t95);
            blk = 216; continue;
        }
        209 => {
            let Some(t96) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008e5565); };
            c.w64(11, t96);
            let Some(t97) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008e5569); };
            c.w32(12, t97);
            c.w64(1, c.r64(11).wrapping_add(c.r64(12)));
            c.w64(10, c.r64(4));
            let r98 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r98);
            let r99 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r99);
            c.logic(r99, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e557c); }
            c.w64(8, c.r64(3));
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x13e8)) else { return RegfileExit::fault(0x70008e5584); };
            c.w64(0, t100);
            c.w64(10, 0x70008e5592u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e5592); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e5596); }
            return c.call(c.r64(0), 0x70008e559c, 0x70008e559a);
        }
        210 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e559c); }
            let Some(t101) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e55a4); };
            c.w64(4, t101);
            blk = 216; continue;
        }
        211 => {
            return RegfileExit::fault(0x70008e55ae);
        }
        212 => {
            return RegfileExit::fault(0x70008e55af);
        }
        213 => {
            c.w32(12, c.r8(2));
            c.w32(15, c.r8(1));
            let r102 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r102);
            let r103 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r103);
            let r104 = c.r32(12) & 0x3fu64;
            c.w32(12, r104);
            c.logic(r104, 32);
            let r105 = c.r32(15) & 0x3fu64;
            c.w32(15, r105);
            c.logic(r105, 32);
            let r106 = c.sub(c.r32(12), 0x1au64, 32);
            c.w32(12, r106);
            let r107 = c.sub(c.r32(15), 0x1au64, 32);
            c.w32(15, r107);
            c.cmp(c.r32(12), 0x1u64, 32);
            c.w8(12, (c.cc_be() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(15), 0x1u64, 32);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_ne() { blk = 255; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(10, c.r64(4));
            let r108 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r108);
            let r109 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r109);
            c.logic(r109, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e55fc); }
            c.w64(1, c.r64(11));
            c.w64(2, c.r64(0));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e5607); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e560b); };
            c.w64(9, t111);
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x1290)) else { return RegfileExit::fault(0x70008e560f); };
            c.w64(0, t112);
            c.w64(10, 0x70008e561du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e561d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e5621); }
            return c.call(c.r64(0), 0x70008e5627, 0x70008e5625);
        }
        215 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e5627); }
            let Some(t113) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e562f); };
            c.w64(4, t113);
            blk = 216; continue;
        }
        216 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5634); };
            c.w64(0, t114);
            blk = 217; continue;
        }
        217 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e5638); };
            c.w64(11, t115);
            let r116 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r116);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e5643); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e5644); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(11).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e564a); }
            return c.ret(0x0, 0x70008e564c);
        }
        218 => {
            c.w32(11, 0x66u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e5653); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e5655); };
            c.w64(11, t117);
            let Some(t118) = c.ld32(c.r64(11).wrapping_add(0x3f7)) else { return RegfileExit::fault(0x70008e565c); };
            c.w32(11, t118);
            let r119 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r119);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e5666); }
            c.w32(0, 0x2u64);
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e566d); };
            c.w64(3, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5674); };
            c.w64(6, t121);
            return c.call(0x700086ed74, 0x70008e567d, 0x70008e5678);
        }
        219 => {
            return RegfileExit::fault(0x70008e567d);
        }
        220 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008e567e); }
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e5682); };
            c.w64(3, t122);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008e5693, 0x70008e568e);
        }
        221 => {
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5693); };
            c.w64(6, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e5697); };
            c.w64(0, t124);
            blk = 1; continue;
        }
        222 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e56a0); };
            c.w64(1, t125);
            let Some(t126) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e56a7); };
            c.w64(8, t126);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008e56ae); }
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e56b2); };
            c.w64(8, t127);
            let Some(t128) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e56b9); };
            c.cmp(t128, c.r64(9), 64);
            if c.cc_a() { blk = 225; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008e56cb, 0x70008e56c6);
        }
        224 => {
            c.w64(1, c.r64(0));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e56ce); };
            c.w64(15, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e56d2); };
            c.w64(6, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e56d6); };
            c.w64(0, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e56da); };
            c.w64(11, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e56de); };
            c.w64(12, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e56e2); };
            c.w64(3, t134);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e56e8); };
            c.w64(7, t135);
            blk = 226; continue;
        }
        225 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008e56f1); }
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e56f4); };
            c.w64(1, t136);
            let r137 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r137);
            blk = 226; continue;
        }
        226 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 230; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let r138 = c.r64(10) | 0x1u64;
            c.w64(10, r138);
            c.logic(r138, 64);
            blk = 229; continue;
        }
        229 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 230; continue;
        }
        230 => {
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e5723); };
            c.w64(8, t139);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e572a); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e572e); }
            blk = 69; continue;
        }
        231 => {
            return RegfileExit::fault(0x70008e5738);
        }
        232 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008e5739); }
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008e573d); };
            c.w64(3, t140);
            c.w64(0, c.r64(15));
            c.w64(11, c.r64(3));
            return c.call(0x7000870914, 0x70008e574f, 0x70008e574a);
        }
        233 => {
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e574f); };
            c.cmp(t141, c.r32(0), 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 234; continue;
        }
        234 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 244; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t142) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e5764); };
            c.w32(11, t142);
            let r143 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r143);
            let Some(t144) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e576b); };
            c.cmp(t144, 0xbau64, 16);
            if c.cc_b() { blk = 244; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008e5778); };
            c.w64(3, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e577f); };
            c.w64(6, t146);
            return c.call(0x7000870914, 0x70008e5788, 0x70008e5783);
        }
        237 => {
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008e5788); };
            c.cmp(t147, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t148) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e5797); };
            c.cmp(t148, c.r32(0), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r149 = c.r32(11) | c.r32(12);
            c.w32(11, r149);
            c.logic(r149, 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 238; continue;
        }
        238 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t150) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e57af); };
            c.w32(11, t150);
            let r151 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r151);
            let Some(t152) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e57b6); };
            c.logic(t152 & 0x40u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w64(9, c.r64(0));
            blk = 247; continue;
        }
        241 => {
            c.w32(11, 0x26eu64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e57c8); }
            c.w32(0, 0x1u64);
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e57cf); };
            c.w64(3, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e57d6); };
            c.w64(6, t154);
            return c.call(0x700086ed74, 0x70008e57df, 0x70008e57da);
        }
        242 => {
            return RegfileExit::fault(0x70008e57df);
        }
        243 => {
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e57e0); };
            c.w64(9, t155);
            blk = 247; continue;
        }
        244 => {
            c.w32(11, 0x54u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e57ec); }
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e57ee); };
            c.w64(11, t156);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e57f5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e57f7); }
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e57f8); };
            c.w64(3, t157);
            c.w32(0, 0x3u64);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5804); };
            c.w64(6, t158);
            return c.call(0x700086ed74, 0x70008e580d, 0x70008e5808);
        }
        245 => {
            return RegfileExit::fault(0x70008e580d);
        }
        246 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e580e); };
            c.w64(9, t159);
            blk = 247; continue;
        }
        247 => {
            let Some(t160) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e5812); };
            c.w32(11, t160);
            let r161 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r161);
            let Some(t162) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e5819); };
            c.logic(t162 & 0x40u64, 8);
            if c.cc_ne() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            return RegfileExit::fault(0x70008e5820);
        }
        249 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5821); };
            if !c.push64(t163) { return RegfileExit::fault(0x70008e5821); }
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e5824); };
            c.w64(11, t164);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e582b); }
            c.w64(7, c.r64(9));
            c.w64(2, c.r64(9));
            c.w32(0, 0x1u64);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5838); };
            c.w64(6, t165);
            return c.call(0x70007f7214, 0x70008e5841, 0x70008e583c);
        }
        250 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 261; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t166) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e5849); };
            c.w32(11, t166);
            let r167 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r167);
            let Some(t168) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e5850); };
            c.cmp(t168, 0x82eu64, 16);
            if c.cc_ne() { blk = 261; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t169) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e585d); };
            c.w32(11, t169);
            let r170 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r170);
            let Some(t171) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e5864); };
            c.logic(t171 & 0x4u64, 8);
            if c.cc_ne() { blk = 259; continue; }
            blk = 253; continue;
        }
        253 => {
            blk = 138; continue;
        }
        254 => {
            return RegfileExit::fault(0x70008e5870);
        }
        255 => {
            c.w32(11, 0x2au64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e5877); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e5879); };
            c.w64(3, t172);
            c.w32(0, 0x1u64);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5885); };
            c.w64(6, t173);
            return c.call(0x700086ed74, 0x70008e588e, 0x70008e5889);
        }
        256 => {
            return RegfileExit::fault(0x70008e588e);
        }
        257 => {
            c.w32(11, 0x296u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e5895); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e5897); };
            c.w64(3, t174);
            c.w32(0, 0x1u64);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e58a3); };
            c.w64(6, t175);
            return c.call(0x700086ed74, 0x70008e58ac, 0x70008e58a7);
        }
        258 => {
            return RegfileExit::fault(0x70008e58ac);
        }
        259 => {
            c.w32(11, 0x66u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e58b3); }
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e58b5); };
            c.w64(11, t176);
            let Some(t177) = c.ld32(c.r64(11).wrapping_add(0x3f7)) else { return RegfileExit::fault(0x70008e58bc); };
            c.w32(11, t177);
            let r178 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r178);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e58c6); }
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e58c8); };
            c.w64(3, t179);
            c.w32(0, 0x2u64);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e58d4); };
            c.w64(6, t180);
            return c.call(0x700086ed74, 0x70008e58dd, 0x70008e58d8);
        }
        260 => {
            return RegfileExit::fault(0x70008e58dd);
        }
        261 => {
            c.w32(11, 0x80u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e58e4); }
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e58e6); };
            c.w64(11, t181);
            let Some(t182) = c.ld32(c.r64(11).wrapping_add(0x3f7)) else { return RegfileExit::fault(0x70008e58ed); };
            c.w32(11, t182);
            let r183 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r183);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e58f7); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e58f9); }
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e58fa); };
            c.w64(3, t184);
            c.w32(0, 0x3u64);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5906); };
            c.w64(6, t185);
            return c.call(0x700086ed74, 0x70008e590f, 0x70008e590a);
        }
        262 => {
            return RegfileExit::fault(0x70008e590f);
        }
        263 => {
            c.w32(11, 0x66u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e5916); }
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e5918); };
            c.w64(11, t186);
            let Some(t187) = c.ld32(c.r64(11).wrapping_add(0x3f7)) else { return RegfileExit::fault(0x70008e591f); };
            c.w32(11, t187);
            let r188 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r188);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e5929); }
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e592b); };
            c.w64(3, t189);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e593c, 0x70008e5937);
        }
        264 => {
            return RegfileExit::fault(0x70008e593c);
        }
        265 => {
            c.w32(11, 0x80u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e5943); }
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e5945); };
            c.w64(11, t190);
            let Some(t191) = c.ld32(c.r64(11).wrapping_add(0x3f7)) else { return RegfileExit::fault(0x70008e594c); };
            c.w32(11, t191);
            let r192 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r192);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e5956); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008e5958); }
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e595a); };
            c.w64(3, t193);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008e596b, 0x70008e5966);
        }
        266 => {
            return RegfileExit::fault(0x70008e596b);
        }
        267 => {
            return RegfileExit::jump(0x70008e5970);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e5974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e5974); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e5978); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e5979); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e597a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e597f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e5983); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e599b); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008e59a0); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e59ae); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e59b5); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 61; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e59c2); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e59c9); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e59ea); };
            c.w64(0, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e59f0); };
            c.w64(0, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008e59f7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e5a03); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e5a09); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e5a19); };
            c.w64(1, t14);
            blk = 12; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e5a1f); };
            c.w64(1, t15);
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e5a26); };
            c.w32(8, t16);
            c.w64(11, c.r64(8));
            let r17 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            let r18 = c.r32(8) & 0xfcu64;
            c.w32(8, r18);
            c.logic(r18, 32);
            let r19 = c.r32(11) & 0x3fu64;
            c.w32(11, r19);
            c.logic(r19, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008e5a3d); }
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_g() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(8, 0x906u64);
            blk = 36; continue;
        }
        19 => {
            c.w32(8, 0x900u64);
            blk = 36; continue;
        }
        20 => {
            c.w32(8, 0x8fau64);
            blk = 36; continue;
        }
        21 => {
            c.w32(8, 0x8f4u64);
            blk = 36; continue;
        }
        22 => {
            c.w32(8, 0x8eeu64);
            blk = 36; continue;
        }
        23 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008e5abd);
        }
        30 => {
            c.w32(8, 0x8e8u64);
            blk = 36; continue;
        }
        31 => {
            c.w32(8, 0x8e2u64);
            blk = 36; continue;
        }
        32 => {
            c.w32(8, 0x8dcu64);
            blk = 36; continue;
        }
        33 => {
            c.w32(8, 0x8d6u64);
            blk = 36; continue;
        }
        34 => {
            c.w32(8, 0x8d0u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(8, 0x8cau64);
            blk = 36; continue;
        }
        36 => {
            let Some(t20) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e5aec); };
            c.w64(11, t20);
            let r21 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008e5af4); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008e5af8); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008e5aff); }
            blk = 38; continue;
        }
        37 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5b05); };
            c.w64(15, t22);
            let r23 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e5b0d); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5b11); };
            c.w64(12, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e5b15); };
            c.w64(9, t26);
            blk = 38; continue;
        }
        38 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e5b19); };
            c.cmp(c.r64(15), t27, 64);
            if c.cc_ae() { blk = 53; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e5b23); };
            c.w32(3, t28);
            let r29 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r29);
            let Some(t30) = c.ld8(c.r64(3).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e5b2a); };
            c.logic(t30 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e5b34); }
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(15));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e5b3e); };
            return c.call(t31, 0x70008e5b46, 0x70008e5b3e);
        }
        41 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5b46); };
            c.cmp(t32, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5b54); };
            c.w64(15, t33);
            c.w32(1, c.r64(15).wrapping_add(c.r64(15)));
            blk = 43; continue;
        }
        43 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e5b5c); };
            if !c.push64(t34) { return RegfileExit::fault(0x70008e5b5c); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e5b5f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e5b60); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e5b61); };
            if !c.push64(t35) { return RegfileExit::fault(0x70008e5b61); }
            c.w32(0, 0x3u64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e5b69); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e5b6d); };
            c.w64(6, t37);
            return c.call(0x70007f2274, 0x70008e5b76, 0x70008e5b71);
        }
        44 => {
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e5b76); };
            c.cmp(t38, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e5b85); };
            c.cmp(t39, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e5b92); };
            c.w32(1, t40);
            let r41 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r41);
            let Some(t42) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e5b98); };
            c.logic(t42 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e5ba2); };
            c.cmp(t43, c.r32(1), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t44) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e5baf); };
            c.cmp(t44, 0x41u64, 16);
            if c.cc_e() { blk = 74; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 52; continue;
        }
        51 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008e5bc5); };
            c.w64(0, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5bcc); };
            c.w64(1, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e5bd3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e5bd4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e5bda); }
            return c.ret(0x0, 0x70008e5bdc);
        }
        53 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e5bdd); };
            c.w64(0, t47);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e5be7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e5be8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e5bee); }
            return c.ret(0x0, 0x70008e5bf0);
        }
        54 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t48) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e5bf7); };
            c.w64(1, t48);
            blk = 57; continue;
        }
        56 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e5bfd); };
            c.w64(1, t49);
            blk = 57; continue;
        }
        57 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e5c0a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e5c0c); }
            c.w32(0, 0x2u64);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e5c12); };
            c.w64(3, t50);
            return c.call(0x700086ed74, 0x70008e5c1e, 0x70008e5c19);
        }
        58 => {
            return RegfileExit::fault(0x70008e5c1e);
        }
        59 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e5c24); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e5c25); };
            c.w64(1, t51);
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0x3fb)) else { return RegfileExit::fault(0x70008e5c2c); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e5c35); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e5c36); };
            c.w64(3, t54);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e5c47, 0x70008e5c42);
        }
        60 => {
            return RegfileExit::fault(0x70008e5c47);
        }
        61 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e5c4d); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e5c4e); };
            c.w64(1, t55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0x3fb)) else { return RegfileExit::fault(0x70008e5c55); };
            c.w32(1, t56);
            let r57 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r57);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e5c5e); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e5c5f); };
            c.w64(3, t58);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e5c70, 0x70008e5c6b);
        }
        62 => {
            return RegfileExit::fault(0x70008e5c70);
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e5c71); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e5c75); };
            c.w64(3, t59);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e5c86, 0x70008e5c81);
        }
        64 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e5c86); };
            c.w64(6, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5c8a); };
            c.w64(0, t61);
            blk = 1; continue;
        }
        65 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e5c93); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e5c9a); };
            c.w64(8, t63);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e5ca1); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e5ca5); };
            c.w64(11, t64);
            let Some(t65) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e5cac); };
            c.cmp(t65, c.r64(9), 64);
            if c.cc_a() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e5cbb, 0x70008e5cb6);
        }
        67 => {
            c.w64(1, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e5cbe); };
            c.w64(0, t66);
            blk = 69; continue;
        }
        68 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008e5cc4); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 69; continue;
        }
        69 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e5ccb); };
            c.w64(8, t67);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e5cd2); }
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5cd6); };
            c.w64(10, t68);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 73; continue; }
            blk = 70; continue;
        }
        70 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let r69 = c.r64(10) | 0x1u64;
            c.w64(10, r69);
            c.logic(r69, 64);
            blk = 72; continue;
        }
        72 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 73; continue;
        }
        73 => {
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e5cfe); }
            blk = 43; continue;
        }
        74 => {
            let Some(t70) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e5d08); };
            c.logic(t70 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 52; continue;
        }
        76 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e5d1a); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 52; continue; }
            blk = 77; continue;
        }
        77 => {
            blk = 37; continue;
        }
        78 => {
            return RegfileExit::jump(0x70008e5d34);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e5d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e5d54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e5d58); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            c.w64(8, c.r64(1));
            let r1 = c.sub(c.r64(8), c.r64(3), 64);
            c.w64(8, r1);
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r2 = c.add(c.r64(8), c.r64(3), 64);
            c.w64(8, r2);
            c.w64(9, c.r64(8));
            let r3 = c.sub(c.r64(9), c.r64(3), 64);
            c.w64(9, r3);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x70008e5d70); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008e5d74); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008e5d78); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008e5d7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008e5d80); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008e5d84); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008e5d88); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008e5d92); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008e5d92); }
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(2, c.r64(10));
            c.w64(1, c.r64(8));
            return c.call(0x70008e5d54, 0x70008e5da6, 0x70008e5da1);
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e5da6); };
            c.w64(6, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5daa); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5dae); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e5db2); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e5db6); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e5dba); };
            c.w64(8, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e5dbe); };
            c.w64(3, t11);
            blk = 3; continue;
        }
        3 => {
            c.w64(9, c.r64(1));
            let r12 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r12);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008e5dce); };
            if !c.push64(t13) { return RegfileExit::fault(0x70008e5dce); }
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(2, c.r64(10));
            c.w64(3, c.r64(8));
            return c.call(0x70008e5d54, 0x70008e5de2, 0x70008e5ddd);
        }
        5 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e5de2); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5de6); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5dea); };
            c.w64(0, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e5dee); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e5df2); };
            c.w64(1, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e5df6); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e5dfa); };
            c.w64(3, t20);
            blk = 6; continue;
        }
        6 => {
            c.w64(12, c.r64(1));
            c.w64(9, c.r64(8));
            c.w64(11, c.r64(3));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e5e07); };
            c.w64(1, t21);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(2, c.r64(10));
            blk = 8; continue;
        }
        7 => {
            let r22 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e5e1d); };
            c.w64(2, t23);
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(8));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e5e27); };
            c.w64(6, t24);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_ae() { blk = 47; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e5e34); };
            c.cmp(c.r64(11), t25, 64);
            if c.cc_b() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x0u64) { return RegfileExit::fault(0x70008e5e3a); }
            blk = 13; continue;
        }
        11 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x1u64) { return RegfileExit::fault(0x70008e5e4d); }
            blk = 13; continue;
        }
        13 => {
            let Some(t26) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e5e55); };
            c.cmp(t26, 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e5e5b); };
            c.w32(8, t27);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_ae() { blk = 48; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t28) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e5e6e); };
            c.w32(8, t28);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 49; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t29) = c.ld32(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e5e89); };
            c.w32(12, t29);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x70008e5e8e); }
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x70008e5e92); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r30 = c.r64(15) & c.r64(0);
            c.w64(15, r30);
            c.logic(r30, 64);
            let Some(t31) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e5ea6); };
            c.logic(t31 & 0x4u64, 8);
            if c.cc_ne() { blk = 70; continue; }
            blk = 18; continue;
        }
        18 => {
            let r32 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r32);
            c.w64(12, c.r64(11));
            c.w64(11, c.r64(9));
            blk = 41; continue;
        }
        19 => {
            let Some(t33) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e5ec0); };
            c.w32(8, t33);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 50; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t34) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e5ed3); };
            c.w32(15, t34);
            let r35 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r35);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008e5edb); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 51; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e5ee8); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e5ef0); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008e5ef2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e5ef4); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x70008e5ef5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008e5ef9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008e5efd); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008e5f01); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008e5f05); };
            c.w64(7, t38);
            c.w32(0, 0x2u64);
            return c.call(0x70007f2134, 0x70008e5f13, 0x70008e5f0e);
        }
        22 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e5f1b); };
            c.w64(8, t39);
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e5f1f); };
            c.w32(9, t40);
            let r41 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r41);
            let Some(t42) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e5f26); };
            c.logic(t42 & 0x4u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e5f35); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e5f48); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 36; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_le() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5f5e); };
            c.w64(9, t43);
            let Some(t44) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e5f62); };
            c.w32(11, t44);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e5f6c); };
            c.w64(3, t45);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 52; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(11, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e5f81); };
            c.w64(12, t46);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x70008e5f85); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r47 = c.r64(15) & c.r64(9);
            c.w64(15, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e5f99); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e5fa4); };
            c.w64(11, t49);
            let r50 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e5fac); };
            c.w64(12, t51);
            blk = 40; continue;
        }
        36 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e5fb2); };
            c.w64(9, t52);
            let Some(t53) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e5fb6); };
            c.w32(11, t53);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e5fc0); };
            c.w64(3, t54);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 53; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(11, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e5fd5); };
            c.w64(12, t55);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x70008e5fd9); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r56 = c.r64(15) & c.r64(9);
            c.w64(15, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e5fed); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e5ff8); };
            c.w64(11, t58);
            let r59 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r59);
            c.w64(12, c.r64(11));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e6003); };
            c.w64(11, t60);
            blk = 40; continue;
        }
        40 => {
            c.w64(7, c.r64(8));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e600a); };
            c.w64(2, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e600e); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e6012); };
            c.w64(1, t63);
            c.w64(0, c.r64(9));
            blk = 41; continue;
        }
        41 => {
            c.w64(8, c.r64(12));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e601f); };
            c.w64(12, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e6023); };
            c.w64(3, t65);
            blk = 7; continue;
        }
        42 => {
            let Some(t66) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e602c); };
            c.w32(8, t66);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_ae() { blk = 56; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(8, c.r64(2));
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e6042); };
            c.w32(15, t67);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ae() { blk = 57; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(15, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t68) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e6059); };
            c.w32(2, t68);
            if !c.st32(c.r64(0).wrapping_add(c.r64(15)), c.r32(2)) { return RegfileExit::fault(0x70008e605e); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r69 = c.r64(6) & c.r64(0);
            c.w64(6, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e6071); };
            c.logic(t70 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 46; continue;
        }
        46 => {
            let r71 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r71);
            c.w64(8, c.r64(11));
            c.w64(11, c.r64(9));
            blk = 7; continue;
        }
        47 => {
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e608c); }
            return c.ret(0x8, 0x70008e608d);
        }
        48 => {
            return RegfileExit::fault(0x70008e6090);
        }
        49 => {
            return RegfileExit::fault(0x70008e6091);
        }
        50 => {
            return RegfileExit::fault(0x70008e6092);
        }
        51 => {
            return RegfileExit::fault(0x70008e6093);
        }
        52 => {
            return RegfileExit::fault(0x70008e6094);
        }
        53 => {
            return RegfileExit::fault(0x70008e6095);
        }
        54 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e609c); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e609e); };
            c.w64(8, t72);
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(0x3ff)) else { return RegfileExit::fault(0x70008e60a5); };
            c.w32(8, t73);
            let r74 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r74);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e60af); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e60b1); };
            c.w64(3, t75);
            c.w32(0, 0x2u64);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e60bd); };
            c.w64(6, t76);
            return c.call(0x700086ed74, 0x70008e60c6, 0x70008e60c1);
        }
        55 => {
            return RegfileExit::fault(0x70008e60c6);
        }
        56 => {
            return RegfileExit::fault(0x70008e60c7);
        }
        57 => {
            return RegfileExit::fault(0x70008e60c8);
        }
        58 => {
            let Some(t77) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e60c9); };
            c.w32(8, t77);
            let Some(t78) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e60cd); };
            c.cmp(t78, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e60da); };
            c.w64(6, t79);
            return c.call(0x70008a1014, 0x70008e60e3, 0x70008e60de);
        }
        60 => {
            blk = 23; continue;
        }
        61 => {
            c.w32(2, c.r32(2));
            let r80 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r80);
            c.w64(6, 0xfffffffffffc0000u64);
            let r81 = c.r64(6) & c.r64(2);
            c.w64(6, r81);
            c.logic(r81, 64);
            let Some(t82) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e60f7); };
            c.logic(t82 & 0x2u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(8, c.r64(0).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e6105); }
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x70008e610e, 0x70008e6109);
        }
        63 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e610e); }
            blk = 46; continue;
        }
        64 => {
            c.w32(12, c.r32(12));
            let r83 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r83);
            c.w64(15, 0xfffffffffffc0000u64);
            let r84 = c.r64(15) & c.r64(12);
            c.w64(15, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e6124); };
            c.logic(t85 & 0x2u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(0, c.r64(9).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e6133); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e6134); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008e6141, 0x70008e613c);
        }
        66 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e6141); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e6143); }
            blk = 39; continue;
        }
        67 => {
            c.w32(12, c.r32(12));
            let r86 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r86);
            c.w64(15, 0xfffffffffffc0000u64);
            let r87 = c.r64(15) & c.r64(12);
            c.w64(15, r87);
            c.logic(r87, 64);
            let Some(t88) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e6159); };
            c.logic(t88 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(0, c.r64(9).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e6168); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e6169); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008e6176, 0x70008e6171);
        }
        69 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e6176); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e6178); }
            blk = 35; continue;
        }
        70 => {
            c.w32(12, c.r32(12));
            let r89 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r89);
            c.w64(15, 0xfffffffffffc0000u64);
            let r90 = c.r64(15) & c.r64(12);
            c.w64(15, r90);
            c.logic(r90, 64);
            let Some(t91) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e618e); };
            c.logic(t91 & 0x2u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(3, c.r64(0).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e619d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e619e); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008e61ab, 0x70008e61a6);
        }
        72 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e61ab); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e61ad); }
            blk = 18; continue;
        }
        73 => {
            return RegfileExit::jump(0x70008e61b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e61d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e61d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e61d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e61d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e61da); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008e61df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e61e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(5));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e61f7); };
            c.w64(11, t2);
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008e61ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008e6203); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e620d); };
            c.w64(9, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e6213); };
            c.w64(9, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e621a); };
            c.cmp(t5, c.r32(9), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008e6229); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(0, 0x1u64);
            blk = 11; continue;
        }
        6 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e6240); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e6247); };
            c.logic(t8 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(3, 0x1u64);
            blk = 10; continue;
        }
        9 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        10 => {
            c.w32(0, 0x1u64);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e626e); };
            c.w32(3, t9);
            let r10 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r10);
            let Some(t11) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e6275); };
            c.cmp(t11, 0x82eu64, 16);
            if c.cc_ne() { blk = 92; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e6281); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            let Some(t14) = c.ld8(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e6288); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 90; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t15) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e6292); };
            c.w64(1, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008e6296); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_b() { blk = 79; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t16) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e62c5); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e62cc); };
            c.logic(t18 & 0x2u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t19) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e62d7); };
            c.w32(15, t19);
            c.w64(0, c.r64(15));
            let r20 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            let r21 = c.r32(15) & 0xfcu64;
            c.w32(15, r21);
            c.logic(r21, 32);
            let r22 = c.r32(0) & 0x3fu64;
            c.w32(0, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(0), 0x15u64, 32);
            if c.cc_g() { blk = 30; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(15), 0x44u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(15), 0x48u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(15), 0x4cu64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(15), 0x50u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(15), 0x54u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(15, 0x908u64);
            c.w32(0, 0x906u64);
            blk = 43; continue;
        }
        26 => {
            c.w32(15, 0x902u64);
            c.w32(0, 0x900u64);
            blk = 43; continue;
        }
        27 => {
            c.w32(15, 0x8fcu64);
            c.w32(0, 0x8fau64);
            blk = 43; continue;
        }
        28 => {
            c.w32(15, 0x8f6u64);
            c.w32(0, 0x8f4u64);
            blk = 43; continue;
        }
        29 => {
            c.w32(15, 0x8f0u64);
            c.w32(0, 0x8eeu64);
            blk = 43; continue;
        }
        30 => {
            c.cmp(c.r32(15), 0x58u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(15), 0x60u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(15), 0x64u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(15), 0x68u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(15), 0x6cu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70008e638f);
        }
        37 => {
            c.w32(15, 0x8eau64);
            c.w32(0, 0x8e8u64);
            blk = 43; continue;
        }
        38 => {
            c.w32(15, 0x8e4u64);
            c.w32(0, 0x8e2u64);
            blk = 43; continue;
        }
        39 => {
            c.w32(15, 0x8deu64);
            c.w32(0, 0x8dcu64);
            blk = 43; continue;
        }
        40 => {
            c.w32(15, 0x8d8u64);
            c.w32(0, 0x8d6u64);
            blk = 43; continue;
        }
        41 => {
            c.w32(15, 0x8d2u64);
            c.w32(0, 0x8d0u64);
            blk = 43; continue;
        }
        42 => {
            c.w32(15, 0x8ccu64);
            c.w32(0, 0x8cau64);
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008e63dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70008e63e0); }
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 88; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, c.r64(1).wrapping_mul(4));
            c.w64(2, c.r64(8).wrapping_add(0x8));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e63fd); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e6404); };
            c.w64(7, t24);
            c.w64(9, c.r64(2).wrapping_add(c.r64(7)));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008e640b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008e640f); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e6413); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e641a); };
            c.cmp(t26, c.r64(9), 64);
            if c.cc_be() { blk = 84; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 84; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(3), c.r64(9)) { return RegfileExit::fault(0x70008e6430); }
            c.w64(9, c.r64(7).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e643a); };
            c.w64(11, t27);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008e6441); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008e6445); }
            c.w64(11, c.r64(9).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r28 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r28);
            let r29 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r29);
            c.logic(r29, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e6458); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008e645d); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x70008e6461); }
            c.w64(1, c.r64(11));
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008e646a); };
            c.w64(0, t30);
            c.w64(10, 0x70008e6478u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e6478); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e647c); }
            return c.call(c.r64(0), 0x70008e6482, 0x70008e6480);
        }
        48 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e6482); }
            let Some(t31) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e648a); };
            c.w64(4, t31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e648f); };
            c.w64(9, t32);
            let Some(t33) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008e6496); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e6499); };
            c.w64(2, t34);
            c.w64(12, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e64a1); };
            c.w64(15, t35);
            let Some(t36) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008e64a8); };
            c.cmp(t36, c.r64(12), 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 86; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008e64be); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 51; continue;
        }
        51 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e64c5); };
            c.w64(11, t37);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008e64cc); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e64d0); };
            c.w64(11, t38);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008e64d4); }
            c.w64(1, c.r64(9).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r39 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r39);
            let r40 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r40);
            c.logic(r40, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e64e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008e64ec); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e64f2); };
            c.w64(8, t41);
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008e64f6); };
            c.w64(0, t42);
            c.w64(10, 0x70008e6504u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e6504); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e6508); }
            return c.call(c.r64(0), 0x70008e650e, 0x70008e650c);
        }
        52 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e650e); }
            let Some(t43) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e6516); };
            c.w64(4, t43);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e651d); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e6521); };
            c.w64(9, t45);
            blk = 56; continue;
        }
        53 => {
            let r46 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r46);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x70008e652b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r47 = c.r64(12) & c.r64(11);
            c.w64(12, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e653d); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_ne() { blk = 97; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(9, c.r64(11));
            blk = 56; continue;
        }
        56 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e654c); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e6550); };
            c.cmp(c.r64(3), t50, 64);
            if c.cc_ae() { blk = 63; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x70008e6556); }
            c.w64(9, c.r64(0));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e655d); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e6561); };
            return c.call(t52, 0x70008e6569, 0x70008e6561);
        }
        58 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e6569); };
            c.w64(2, t53);
            let Some(t54) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e656d); };
            c.w32(9, t54);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e6577); };
            c.w64(3, t55);
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_ae() { blk = 72; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(9, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x70008e658c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r56 = c.r64(11) & c.r64(2);
            c.w64(11, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e659e); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 94; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e65a9); };
            c.w64(11, t58);
            let Some(t59) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e65ad); };
            c.w32(12, t59);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_b() { blk = 53; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 73; continue;
        }
        63 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e65c5); };
            if !c.push64(t60) { return RegfileExit::fault(0x70008e65c5); }
            c.w64(0, c.r64(9));
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e65cd); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e65d1); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e65d5); };
            c.w64(7, t63);
            return c.call(0x70008e5d54, 0x70008e65de, 0x70008e65d9);
        }
        64 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 67; continue;
        }
        65 => {
            c.w64(9, c.r64(3).wrapping_add(0x1));
            let Some(t64) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e65e6); };
            c.w32(1, t64);
            let r65 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r65);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008e65ed); }
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e65f1); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e65f5); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e65f9); };
            return c.call(t68, 0x70008e6601, 0x70008e65f9);
        }
        66 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e6601); };
            c.w64(3, t69);
            blk = 67; continue;
        }
        67 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e6605); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e6609); };
            c.w64(0, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e660d); };
            c.cmp(c.r64(3), t72, 64);
            if c.cc_ae() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t73) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e6613); };
            c.w32(9, t73);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_b() { blk = 65; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 71; continue;
        }
        70 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e6624); };
            c.w64(9, t74);
            let r75 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e662c); };
            c.w64(0, t76);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e6633); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e6634); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e663a); }
            return c.ret(0x0, 0x70008e663c);
        }
        71 => {
            return RegfileExit::fault(0x70008e663d);
        }
        72 => {
            return RegfileExit::fault(0x70008e663e);
        }
        73 => {
            return RegfileExit::fault(0x70008e663f);
        }
        74 => {
            return RegfileExit::fault(0x70008e6640);
        }
        75 => {
            c.w32(9, 0x1d4u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e6647); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e6649); };
            c.w64(3, t77);
            return c.call(0x700086ed74, 0x70008e6655, 0x70008e6650);
        }
        76 => {
            return RegfileExit::fault(0x70008e6655);
        }
        77 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e6656); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2b38)) else { return RegfileExit::fault(0x70008e6658); };
            c.w64(3, t78);
            return c.call(0x700086ed74, 0x70008e6664, 0x70008e665f);
        }
        78 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e6664); };
            c.w64(9, t79);
            let r80 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r80);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e666f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e6670); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e6676); }
            return c.ret(0x0, 0x70008e6678);
        }
        79 => {
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e6684); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e6685); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e668b); }
            return c.ret(0x0, 0x70008e668d);
        }
        80 => {
            c.w32(11, 0x24u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e6694); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e6696); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e6698); };
            c.w64(3, t81);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e66a9, 0x70008e66a4);
        }
        81 => {
            return RegfileExit::fault(0x70008e66a9);
        }
        82 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008e66aa); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e66b0); };
            c.w64(3, t82);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x70008e66bf, 0x70008e66ba);
        }
        83 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e66bf); };
            c.w64(6, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e66c3); };
            c.w64(0, t84);
            blk = 1; continue;
        }
        84 => {
            return c.call(0x70007febb4, 0x70008e66d1, 0x70008e66cc);
        }
        85 => {
            c.w64(9, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e66d4); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e66d8); };
            c.w64(8, t86);
            blk = 47; continue;
        }
        86 => {
            return c.call(0x70007febb4, 0x70008e66e6, 0x70008e66e1);
        }
        87 => {
            c.w64(9, c.r64(0));
            blk = 51; continue;
        }
        88 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e66ee); };
            c.w64(3, t87);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e66fe, 0x70008e66f9);
        }
        89 => {
            return RegfileExit::fault(0x70008e66fe);
        }
        90 => {
            c.w32(9, 0x66u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e6705); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e6707); };
            c.w64(9, t88);
            let Some(t89) = c.ld32(c.r64(9).wrapping_add(0x3ff)) else { return RegfileExit::fault(0x70008e670e); };
            c.w32(9, t89);
            let r90 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r90);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e6718); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e671a); };
            c.w64(3, t91);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e672b, 0x70008e6726);
        }
        91 => {
            return RegfileExit::fault(0x70008e672b);
        }
        92 => {
            c.w32(9, 0x80u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e6732); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e6734); };
            c.w64(9, t92);
            let Some(t93) = c.ld32(c.r64(9).wrapping_add(0x3ff)) else { return RegfileExit::fault(0x70008e673b); };
            c.w32(9, t93);
            let r94 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r94);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e6745); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e6747); }
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e6749); };
            c.w64(3, t95);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008e675a, 0x70008e6755);
        }
        93 => {
            return RegfileExit::fault(0x70008e675a);
        }
        94 => {
            c.w32(0, c.r32(0));
            let r96 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r96);
            c.w64(11, 0xfffffffffffc0000u64);
            let r97 = c.r64(11) & c.r64(0);
            c.w64(11, r97);
            c.logic(r97, 64);
            let Some(t98) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e676a); };
            c.logic(t98 & 0x2u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e6779); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e677a); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008e6787, 0x70008e6782);
        }
        96 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e6787); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e6789); }
            blk = 61; continue;
        }
        97 => {
            c.w32(0, c.r32(0));
            let r99 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r99);
            c.w64(12, 0xfffffffffffc0000u64);
            let r100 = c.r64(12) & c.r64(0);
            c.w64(12, r100);
            c.logic(r100, 64);
            let Some(t101) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e679e); };
            c.logic(t101 & 0x2u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e67ae); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e67af); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008e67bc, 0x70008e67b7);
        }
        99 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e67bc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e67be); }
            blk = 55; continue;
        }
        100 => {
            return RegfileExit::jump(0x70008e67c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e67d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e67d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e67d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e67d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e67da); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e67df); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e67e3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e67e7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 209; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e67ff); };
            c.w64(11, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e6804); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008e6808); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008e680c); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 207; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e681a); };
            c.w32(12, t3);
            let r4 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e6821); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 207; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e682f); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e6836); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008e683e); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_ne() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e6846); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x70008e684d); };
            c.w64(3, t10);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x70008e685c, 0x70008e6857);
        }
        6 => {
            c.w64(12, c.r64(0));
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e685f); };
            c.w64(6, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e6863); };
            c.w64(11, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e6867); };
            c.w64(9, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e686b); };
            c.w64(8, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e686f); };
            c.w64(0, t15);
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e6873); };
            c.w64(15, t16);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008e687a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008e687e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e6882); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t17) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e688c); };
            c.w64(8, t17);
            c.w64(3, c.r64(8));
            blk = 10; continue;
        }
        9 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e6895); };
            c.w64(3, t18);
            blk = 10; continue;
        }
        10 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e689c); };
            c.w64(8, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e68a3); };
            c.cmp(t20, c.r32(3), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        12 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(3));
            return c.call(0x7000899a14, 0x70008e68c0, 0x70008e68bb);
        }
        14 => {
            c.w64(3, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e68c3); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e68c7); };
            c.w64(0, t22);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e68cb); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e68d2); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e68d6); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e68da); };
            c.w64(15, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e68de); };
            c.w64(9, t27);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_l() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_b() { blk = 37; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(3, c.r64(15));
            blk = 37; continue;
        }
        19 => {
            let r28 = c.add(c.r64(3), c.r64(15), 64);
            c.w64(3, r28);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_b() { blk = 37; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        21 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let r29 = c.r64(10) | 0x1u64;
            c.w64(10, r29);
            c.logic(r29, 64);
            blk = 24; continue;
        }
        24 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 25; continue;
        }
        25 => {
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e693e); }
            c.xorpd_rr(2, 2);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_a() { blk = 32; continue; }
            blk = 26; continue;
        }
        26 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.mov128_rr(1, 0);
            blk = 28; continue;
        }
        28 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(1));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_s() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(10, 0x8000000000000000u64);
            let r30 = c.r64(3) | c.r64(10);
            c.w64(3, r30);
            c.logic(r30, 64);
            blk = 31; continue;
        }
        31 => {
            blk = 37; continue;
        }
        32 => {
            c.arith_sd(0, 1, c.xmm_u64(0));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(2));
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        34 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t31) = c.ld64(0x70008e6962u64) else { return RegfileExit::fault(0x70008e69a5); };
            c.w64(10, t31);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(1));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_s() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t32) = c.ld64(0x70008e6980u64) else { return RegfileExit::fault(0x70008e69c0); };
            c.w64(10, t32);
            let r33 = c.r64(3) | c.r64(10);
            c.w64(3, r33);
            c.logic(r33, 64);
            blk = 37; continue;
        }
        37 => {
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008e69cc); }
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t34) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e69d6); };
            c.w64(9, t34);
            blk = 40; continue;
        }
        39 => {
            c.w64(9, c.r64(8));
            blk = 40; continue;
        }
        40 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e69df); };
            c.cmp(t35, c.r32(9), 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(9, c.r64(15));
            blk = 68; continue;
        }
        42 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(0, c.r64(9));
            return c.call(0x7000899a14, 0x70008e69fe, 0x70008e69f9);
        }
        44 => {
            c.w64(9, c.r64(0));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e6a01); };
            c.w64(6, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e6a05); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e6a09); };
            c.w64(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e6a10); };
            c.w64(12, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e6a14); };
            c.w64(11, t40);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e6a1a); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e6a1e); };
            c.w64(15, t42);
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_l() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 68; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(9, c.r64(15));
            blk = 68; continue;
        }
        49 => {
            let r43 = c.add(c.r64(9), c.r64(15), 64);
            c.w64(9, r43);
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(9, c.r64(2));
            blk = 68; continue;
        }
        51 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e6a59); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let r44 = c.r64(10) | 0x1u64;
            c.w64(10, r44);
            c.logic(r44, 64);
            blk = 54; continue;
        }
        54 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 55; continue;
        }
        55 => {
            c.xorpd_rr(2, 2);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 62; continue; }
            blk = 56; continue;
        }
        56 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.mov128_rr(0, 1);
            blk = 58; continue;
        }
        58 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t45) = c.ld64(0x70008e6962u64) else { return RegfileExit::fault(0x70008e6aa3); };
            c.w64(10, t45);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_s() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t46) = c.ld64(0x70008e6980u64) else { return RegfileExit::fault(0x70008e6abe); };
            c.w64(10, t46);
            let r47 = c.r64(15) | c.r64(10);
            c.w64(15, r47);
            c.logic(r47, 64);
            blk = 61; continue;
        }
        61 => {
            c.w64(9, c.r64(15));
            blk = 68; continue;
        }
        62 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(9, c.r64(2));
            blk = 68; continue;
        }
        64 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t48) = c.ld64(0x70008e6962u64) else { return RegfileExit::fault(0x70008e6ae6); };
            c.w64(10, t48);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_s() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t49) = c.ld64(0x70008e6980u64) else { return RegfileExit::fault(0x70008e6b01); };
            c.w64(10, t49);
            let r50 = c.r64(15) | c.r64(10);
            c.w64(15, r50);
            c.logic(r50, 64);
            blk = 67; continue;
        }
        67 => {
            c.w64(9, c.r64(15));
            blk = 68; continue;
        }
        68 => {
            let r51 = c.sub(c.r64(9), c.r64(3), 64);
            c.w64(9, r51);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ge() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(9, c.r64(2));
            blk = 70; continue;
        }
        70 => {
            let Some(t52) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e6b1a); };
            c.w32(15, t52);
            let r53 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r53);
            let Some(t54) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e6b21); };
            c.w32(15, t54);
            let r55 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r55);
            let r56 = c.r32(15) & 0x3fu64;
            c.w32(15, r56);
            c.logic(r56, 32);
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_l() { blk = 89; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 81; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 77; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 127; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 211; continue;
        }
        77 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 211; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 86; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 85; continue;
        }
        85 => {
            blk = 211; continue;
        }
        86 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 88; continue;
        }
        88 => {
            blk = 211; continue;
        }
        89 => {
            c.cmp(c.r32(15), 0x16u64, 32);
            if c.cc_l() { blk = 99; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(15), 0x19u64, 32);
            if c.cc_l() { blk = 95; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(15), 0x19u64, 32);
            if c.cc_e() { blk = 115; continue; }
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(15), 0x1au64, 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 93; continue;
        }
        93 => {
            c.cmp(c.r32(15), 0x1bu64, 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 94; continue;
        }
        94 => {
            blk = 211; continue;
        }
        95 => {
            c.cmp(c.r32(15), 0x16u64, 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(15), 0x17u64, 32);
            if c.cc_e() { blk = 113; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(15), 0x18u64, 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 98; continue;
        }
        98 => {
            blk = 211; continue;
        }
        99 => {
            c.cmp(c.r32(15), 0x13u64, 32);
            if c.cc_l() { blk = 104; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(15), 0x13u64, 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(15), 0x14u64, 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(15), 0x15u64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 103; continue;
        }
        103 => {
            blk = 211; continue;
        }
        104 => {
            c.cmp(c.r32(15), 0x11u64, 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(15), 0x12u64, 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 106; continue;
        }
        106 => {
            blk = 211; continue;
        }
        107 => {
            c.w64(15, c.r64(2));
            blk = 129; continue;
        }
        108 => {
            c.w64(15, c.r64(2));
            blk = 129; continue;
        }
        109 => {
            c.w32(15, 0x1u64);
            blk = 129; continue;
        }
        110 => {
            c.w32(15, 0x1u64);
            blk = 129; continue;
        }
        111 => {
            c.w32(15, 0x2u64);
            blk = 129; continue;
        }
        112 => {
            c.w32(15, 0x2u64);
            blk = 129; continue;
        }
        113 => {
            c.w32(15, 0x2u64);
            blk = 129; continue;
        }
        114 => {
            c.w32(15, 0x3u64);
            blk = 129; continue;
        }
        115 => {
            c.w64(15, c.r64(2));
            blk = 129; continue;
        }
        116 => {
            c.w32(15, 0x3u64);
            blk = 129; continue;
        }
        117 => {
            c.w32(15, 0x3u64);
            blk = 129; continue;
        }
        118 => {
            c.w64(15, c.r64(2));
            blk = 129; continue;
        }
        119 => {
            c.w64(15, c.r64(2));
            blk = 129; continue;
        }
        120 => {
            c.w32(15, 0x1u64);
            blk = 129; continue;
        }
        121 => {
            c.w32(15, 0x1u64);
            blk = 129; continue;
        }
        122 => {
            c.w32(15, 0x2u64);
            blk = 129; continue;
        }
        123 => {
            c.w32(15, 0x2u64);
            blk = 129; continue;
        }
        124 => {
            c.w32(15, 0x2u64);
            blk = 129; continue;
        }
        125 => {
            c.w32(15, 0x3u64);
            blk = 129; continue;
        }
        126 => {
            c.w64(15, c.r64(2));
            blk = 129; continue;
        }
        127 => {
            c.w32(15, 0x3u64);
            blk = 129; continue;
        }
        128 => {
            c.w32(15, 0x3u64);
            blk = 129; continue;
        }
        129 => {
            let Some(t57) = c.ld64(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e6d03); };
            c.w64(7, t57);
            c.w64(1, 0x100000000u64);
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_b() { blk = 205; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(8, 0x1fffffffffffffu64);
            c.w64(1, c.r64(15));
            if let Some(sr) = c.shr(c.r64(8), c.r8(1), 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_b() { blk = 205; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w64(1, c.r64(15));
            if let Some(sr) = c.shl(c.r64(3), c.r8(1), 64) { c.w64(3, sr); }
            c.w64(8, c.r64(7).wrapping_add(c.r64(3)));
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 212; continue; }
            blk = 132; continue;
        }
        132 => {
            let r58 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r58);
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 221; continue; }
            blk = 134; continue;
        }
        134 => {
            let r59 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r59);
            blk = 135; continue;
        }
        135 => {
            let Some(t60) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e6d5d); };
            c.w32(15, t60);
            let r61 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r61);
            let Some(t62) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e6d64); };
            c.w32(3, t62);
            let r63 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r63);
            let r64 = c.r32(3) & 0x3fu64;
            c.w32(3, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_l() { blk = 154; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_l() { blk = 146; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_l() { blk = 142; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(3), 0x25u64, 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(3), 0x26u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 230; continue;
        }
        142 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(3), 0x22u64, 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(3), 0x23u64, 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 145; continue;
        }
        145 => {
            blk = 230; continue;
        }
        146 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_l() { blk = 151; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(3), 0x1fu64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 150; continue;
        }
        150 => {
            blk = 230; continue;
        }
        151 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(3), 0x1du64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 230; continue;
        }
        154 => {
            c.cmp(c.r32(3), 0x16u64, 32);
            if c.cc_l() { blk = 164; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(3), 0x19u64, 32);
            if c.cc_l() { blk = 160; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(3), 0x19u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(3), 0x1au64, 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(3), 0x1bu64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 159; continue;
        }
        159 => {
            blk = 230; continue;
        }
        160 => {
            c.cmp(c.r32(3), 0x16u64, 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(3), 0x17u64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(3), 0x18u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 163; continue;
        }
        163 => {
            blk = 230; continue;
        }
        164 => {
            c.cmp(c.r32(3), 0x13u64, 32);
            if c.cc_l() { blk = 169; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(3), 0x13u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(3), 0x14u64, 32);
            if c.cc_e() { blk = 175; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(3), 0x15u64, 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 168; continue;
        }
        168 => {
            blk = 230; continue;
        }
        169 => {
            c.cmp(c.r32(3), 0x11u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(3), 0x12u64, 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 171; continue;
        }
        171 => {
            blk = 230; continue;
        }
        172 => {
            c.w32(3, 0x10u64);
            blk = 194; continue;
        }
        173 => {
            c.w32(3, 0x11u64);
            blk = 194; continue;
        }
        174 => {
            c.w32(3, 0x12u64);
            blk = 194; continue;
        }
        175 => {
            c.w32(3, 0x13u64);
            blk = 194; continue;
        }
        176 => {
            c.w32(3, 0x14u64);
            blk = 194; continue;
        }
        177 => {
            c.w32(3, 0x15u64);
            blk = 194; continue;
        }
        178 => {
            c.w32(3, 0x16u64);
            blk = 194; continue;
        }
        179 => {
            c.w32(3, 0x17u64);
            blk = 194; continue;
        }
        180 => {
            c.w32(3, 0x18u64);
            blk = 194; continue;
        }
        181 => {
            c.w32(3, 0x19u64);
            blk = 194; continue;
        }
        182 => {
            c.w32(3, 0x1au64);
            blk = 194; continue;
        }
        183 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        184 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        185 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        186 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        187 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        188 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        189 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        190 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        191 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        192 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        193 => {
            c.w64(3, c.r64(2));
            blk = 194; continue;
        }
        194 => {
            let Some(t65) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e6f04); };
            c.w32(1, t65);
            let r66 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r66);
            let Some(t67) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e6f0a); };
            c.w32(1, t67);
            let r68 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r68);
            let Some(t69) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e6f10); };
            c.w32(7, t69);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(3), c.r64(7), 64);
            if c.cc_b() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            return RegfileExit::fault(0x70008e6f1e);
        }
        196 => {
            let Some(t70) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e6f1f); };
            c.w32(3, t70);
            let r71 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r71);
            c.cmp(c.r64(7), 0xd4u64, 64);
            if c.cc_a() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            return RegfileExit::fault(0x70008e6f2f);
        }
        198 => {
            let Some(t72) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e6f30); };
            c.w32(15, t72);
            let r73 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r73);
            let Some(t74) = c.ld32(c.r64(1).wrapping_add(0x357)) else { return RegfileExit::fault(0x70008e6f37); };
            c.w32(1, t74);
            let Some(t75) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e6f3d); };
            c.w32(15, t75);
            let r76 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r76);
            let Some(t77) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e6f44); };
            c.cmp(t77, 0xbdu64, 16);
            if c.cc_ae() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008e6f4d); };
            c.w64(15, t78);
            blk = 201; continue;
        }
        200 => {
            let Some(t79) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e6f56); };
            c.w32(15, t79);
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r32(1), c.r32(15), 32);
            if c.cc_ne() { blk = 231; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xfc8)) else { return RegfileExit::fault(0x70008e6f63); };
            c.w64(15, t80);
            let Some(t81) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e6f6a); };
            c.cmp(t81, c.r32(2), 32);
            if c.cc_e() { blk = 231; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w64(0, c.r64(3));
            c.w64(7, c.r64(9));
            c.w64(1, c.r64(12));
            c.w64(2, c.r64(8));
            return c.call(0x70008dd9b4, 0x70008e6f85, 0x70008e6f80);
        }
        204 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e6f85); };
            c.w64(15, t82);
            c.w64(8, c.r64(15).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e6f90); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e6f91); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e6f97); }
            return c.ret(0x0, 0x70008e6f99);
        }
        205 => {
            c.w32(8, 0x188u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e6fa0); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008e6fa2); };
            c.w64(3, t83);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008e6fb3, 0x70008e6fae);
        }
        206 => {
            return RegfileExit::fault(0x70008e6fb3);
        }
        207 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e6fba); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e6fbc); };
            c.w64(8, t84);
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0x403)) else { return RegfileExit::fault(0x70008e6fc3); };
            c.w32(8, t85);
            let r86 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r86);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e6fcd); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e6fcf); };
            c.w64(3, t87);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e6fe0, 0x70008e6fdb);
        }
        208 => {
            return RegfileExit::fault(0x70008e6fe0);
        }
        209 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e6fe1); };
            c.w64(3, t88);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008e6ff2, 0x70008e6fed);
        }
        210 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e6ff2); };
            c.w64(6, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e6ff6); };
            c.w64(0, t90);
            blk = 1; continue;
        }
        211 => {
            return RegfileExit::fault(0x70008e6fff);
        }
        212 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7000); };
            c.w64(15, t91);
            let Some(t92) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008e7007); };
            c.w64(3, t92);
            c.w64(1, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008e700e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e7012); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e7016); };
            c.w64(7, t93);
            let Some(t94) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e701d); };
            c.cmp(t94, c.r64(1), 64);
            if c.cc_a() { blk = 215; continue; }
            blk = 213; continue;
        }
        213 => {
            c.w32(2, 0xcu64);
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x70008e702f, 0x70008e702a);
        }
        214 => {
            c.w64(15, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7032); };
            c.w64(6, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7036); };
            c.w64(0, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e703a); };
            c.w64(12, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e703e); };
            c.w64(11, t98);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e7044); };
            c.w64(9, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e7048); };
            c.w64(8, t100);
            blk = 216; continue;
        }
        215 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x70008e704e); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 216; continue;
        }
        216 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(8) as i64));
            c.logic(c.r64(8) & c.r64(8), 64);
            if c.cc_ns() { blk = 220; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w64(10, c.r64(8));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            let r101 = c.r64(10) | 0x1u64;
            c.w64(10, r101);
            c.logic(r101, 64);
            blk = 219; continue;
        }
        219 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 220; continue;
        }
        220 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e707c); };
            c.w64(8, t102);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e7083); }
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e7087); }
            c.w64(8, c.r64(15));
            blk = 133; continue;
        }
        221 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7095); };
            c.w64(15, t103);
            let Some(t104) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008e709c); };
            c.w64(3, t104);
            c.w64(1, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e70a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008e70a7); }
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e70ab); };
            c.w64(7, t105);
            let Some(t106) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e70b2); };
            c.cmp(t106, c.r64(1), 64);
            if c.cc_a() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(2, 0xcu64);
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x70008e70c4, 0x70008e70bf);
        }
        223 => {
            c.w64(15, c.r64(0));
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e70c7); };
            c.w64(6, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e70cb); };
            c.w64(0, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e70cf); };
            c.w64(12, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e70d3); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e70d7); };
            c.w64(11, t111);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e70dd); };
            c.w64(9, t112);
            blk = 225; continue;
        }
        224 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x70008e70e3); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 225; continue;
        }
        225 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e70ea); };
            c.w64(3, t113);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008e70f1); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 229; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            let r114 = c.r64(10) | 0x1u64;
            c.w64(10, r114);
            c.logic(r114, 64);
            blk = 228; continue;
        }
        228 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 229; continue;
        }
        229 => {
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e711c); }
            c.w64(9, c.r64(15));
            blk = 135; continue;
        }
        230 => {
            return RegfileExit::fault(0x70008e712a);
        }
        231 => {
            c.w64(15, c.r64(0));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e712e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008e7132); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008e7136); }
            c.w64(0, c.r64(11));
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008e713d); };
            c.w64(3, t115);
            return c.call(0x7000870914, 0x70008e7149, 0x70008e7144);
        }
        232 => {
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7149); };
            c.cmp(t116, c.r32(0), 32);
            if c.cc_e() { blk = 245; continue; }
            blk = 233; continue;
        }
        233 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t117) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e715e); };
            c.w32(8, t117);
            let r118 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r118);
            let Some(t119) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e7165); };
            c.cmp(t119, 0xbau64, 16);
            if c.cc_b() { blk = 243; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008e7172); };
            c.w64(3, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7179); };
            c.w64(6, t121);
            return c.call(0x7000870914, 0x70008e7182, 0x70008e717d);
        }
        236 => {
            let Some(t122) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008e7182); };
            c.cmp(t122, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7191); };
            c.cmp(t123, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r124 = c.r32(8) | c.r32(9);
            c.w32(8, r124);
            c.logic(r124, 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 237; continue;
        }
        237 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t125) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e71a9); };
            c.w32(8, t125);
            let r126 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r126);
            let Some(t127) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e71b0); };
            c.logic(t127 & 0x40u64, 8);
            if c.cc_e() { blk = 240; continue; }
            blk = 239; continue;
        }
        239 => {
            c.w64(3, c.r64(0));
            blk = 246; continue;
        }
        240 => {
            c.w32(8, 0x26eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e71c2); }
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e71c4); };
            c.w64(3, t128);
            c.w32(0, 0x1u64);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e71d0); };
            c.w64(6, t129);
            return c.call(0x700086ed74, 0x70008e71d9, 0x70008e71d4);
        }
        241 => {
            return RegfileExit::fault(0x70008e71d9);
        }
        242 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e71da); };
            c.w64(3, t130);
            blk = 246; continue;
        }
        243 => {
            c.w32(8, 0x54u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e71e6); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e71e8); };
            c.w64(8, t131);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e71ef); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e71f1); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e71f2); };
            c.w64(3, t132);
            c.w32(0, 0x3u64);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e71fe); };
            c.w64(6, t133);
            return c.call(0x700086ed74, 0x70008e7207, 0x70008e7202);
        }
        244 => {
            return RegfileExit::fault(0x70008e7207);
        }
        245 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e7208); };
            c.w64(3, t134);
            blk = 246; continue;
        }
        246 => {
            let Some(t135) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e720c); };
            c.w32(8, t135);
            let r136 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r136);
            let Some(t137) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e7213); };
            c.logic(t137 & 0x40u64, 8);
            if c.cc_ne() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            return RegfileExit::fault(0x70008e721a);
        }
        248 => {
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e721b); };
            if !c.push64(t138) { return RegfileExit::fault(0x70008e721b); }
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e721e); };
            if !c.push64(t139) { return RegfileExit::fault(0x70008e721e); }
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e7221); };
            if !c.push64(t140) { return RegfileExit::fault(0x70008e7221); }
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7224); };
            c.w64(8, t141);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e722b); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(3));
            c.w64(2, c.r64(3));
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7238); };
            c.w64(6, t142);
            return c.call(0x70007f7214, 0x70008e7241, 0x70008e723c);
        }
        249 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t143) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e7245); };
            c.w32(8, t143);
            let r144 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r144);
            let Some(t145) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e724c); };
            c.cmp(t145, 0x82eu64, 16);
            if c.cc_ne() { blk = 255; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t146) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e7255); };
            c.w32(8, t146);
            let r147 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r147);
            let Some(t148) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e725c); };
            c.logic(t148 & 0x4u64, 8);
            if c.cc_ne() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7263); };
            c.w64(15, t149);
            c.w64(8, c.r64(15).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e726e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e726f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e7275); }
            return c.ret(0x0, 0x70008e7277);
        }
        253 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e727e); }
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e7280); };
            c.w64(8, t150);
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0x403)) else { return RegfileExit::fault(0x70008e7287); };
            c.w32(8, t151);
            let r152 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r152);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e7291); }
            c.w32(0, 0x2u64);
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e7298); };
            c.w64(3, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e729f); };
            c.w64(6, t154);
            return c.call(0x700086ed74, 0x70008e72a8, 0x70008e72a3);
        }
        254 => {
            return RegfileExit::fault(0x70008e72a8);
        }
        255 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e72af); }
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e72b1); };
            c.w64(8, t155);
            let Some(t156) = c.ld32(c.r64(8).wrapping_add(0x403)) else { return RegfileExit::fault(0x70008e72b8); };
            c.w32(8, t156);
            let r157 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r157);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e72c2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e72c4); }
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e72c5); };
            c.w64(3, t158);
            c.w32(0, 0x3u64);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e72d1); };
            c.w64(6, t159);
            return c.call(0x700086ed74, 0x70008e72da, 0x70008e72d5);
        }
        256 => {
            return RegfileExit::fault(0x70008e72da);
        }
        257 => {
            return RegfileExit::jump(0x70008e72dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e72f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e72f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e72f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e72f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e72fa); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e72ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e7313); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e7321); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e7327); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e7333); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e7339); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e733f); };
            c.w32(7, t9);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4au64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008e734e);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_add(0x12f)) else { return RegfileExit::fault(0x70008e734f); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7355); };
            c.w64(8, t11);
            let Some(t12) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e735c); };
            c.w64(9, t12);
            c.w64(11, c.r64(9).wrapping_add(0x18));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e7363); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e736a); };
            c.cmp(t14, c.r64(11), 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008e7374); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e737b); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e737f); };
            c.w64(7, t15);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008e7386); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008e738a); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008e738e); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008e7394); }
            c.w32(1, 0x2u64);
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008e739d); }
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            let r16 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r16);
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e73ae); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e73af); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e73b5); }
            return c.ret(0x0, 0x70008e73b7);
        }
        9 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e73bd); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e73be); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x407)) else { return RegfileExit::fault(0x70008e73c5); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e73ce); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e73cf); };
            c.w64(3, t20);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e73e0, 0x70008e73db);
        }
        10 => {
            return RegfileExit::fault(0x70008e73e0);
        }
        11 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e73e6); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e73e7); };
            c.w64(1, t21);
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0x407)) else { return RegfileExit::fault(0x70008e73ee); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e73f7); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e73f8); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e7409, 0x70008e7404);
        }
        12 => {
            return RegfileExit::fault(0x70008e7409);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e740a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e740e); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e7412); };
            c.w64(3, t25);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e7423, 0x70008e741e);
        }
        14 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7423); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7427); };
            c.w64(0, t27);
            blk = 1; continue;
        }
        15 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e7435); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008e7439); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008e743d); }
            return c.call(0x70007febb4, 0x70008e7446, 0x70008e7441);
        }
        16 => {
            c.w64(8, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7449); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e744d); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7451); };
            c.w64(7, t30);
            blk = 8; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008e745c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e7474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7474); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e7478); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e7479); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e747a); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e747f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e7483); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e748d); };
            c.cmp(t2, c.r32(2), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e749a); };
            c.w64(1, t3);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e74a7); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e74ae); };
            c.w32(8, t6);
            let r7 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r7);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 42; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e74c7); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e74ce); };
            c.logic(t10 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008e74d5); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e74dc); };
            c.w32(9, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008e74e0); };
            c.cmp(t14, c.r32(9), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e74e9); };
            c.cmp(t15, c.r32(7), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, c.r64(8));
            blk = 10; continue;
        }
        8 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e74f4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008e74f5); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e74f6); };
            c.w64(7, t16);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e74fd); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2380)) else { return RegfileExit::fault(0x70008e74fe); };
            c.w64(3, t17);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008e750f, 0x70008e750a);
        }
        9 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e750f); };
            c.w64(1, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7513); };
            c.w64(6, t19);
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e7517); };
            c.logic(t20 & 0x200000u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e7520); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e7527); };
            c.w64(8, t22);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7531); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7538); };
            c.w64(7, t24);
            blk = 24; continue;
        }
        12 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7544); };
            c.w64(7, t25);
            let Some(t26) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e754b); };
            c.w64(8, t26);
            c.w64(9, c.r64(8).wrapping_add(0x4c));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e7552); };
            c.w64(11, t27);
            let Some(t28) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e7559); };
            c.cmp(t28, c.r64(9), 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008e7562); }
            let r29 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r29);
            blk = 14; continue;
        }
        14 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008e7569); };
            c.w64(9, t30);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008e7570); }
            c.w32(9, 0x22u64);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008e757a); }
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e757e); };
            c.w32(9, t31);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 50; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008e7595); }
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e7599); };
            c.w32(12, t32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 51; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008e75ad); }
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e75b1); };
            c.w32(12, t33);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(12, 0x8u64);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x70008e75cb); }
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e75cf); };
            c.w32(12, t34);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(12, 0x2u64);
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008e75e9); }
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e75ed); };
            c.w32(12, t35);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4u64, 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x70008e7601); }
            c.w64(12, c.r64(8).wrapping_add(0x4b));
            c.w64(15, c.r64(8).wrapping_add(0x1b));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7612); };
            c.w64(3, t36);
            blk = 21; continue;
        }
        21 => {
            if !c.st32(c.r64(15), c.r32(3)) { return RegfileExit::fault(0x70008e7619); }
            let r37 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r37);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_ne() { blk = 21; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7627); };
            c.w64(3, t38);
            blk = 24; continue;
        }
        24 => {
            let Some(t39) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e762e); };
            c.w32(12, t39);
            c.w64(2, c.r64(12).wrapping_mul(4));
            let Some(t40) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e763b); };
            c.w64(12, t40);
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t41) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e7642); };
            c.cmp(t41, c.r64(15), 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x70008e764b); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008e7657); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008e765a); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e765e); };
            c.w64(8, t42);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008e7665); }
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e7669); };
            c.w32(8, t43);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008e766d); }
            c.w64(12, c.r64(8));
            let r44 = c.r32(12) & 0xe0000000u64;
            c.w32(12, r44);
            c.logic(r44, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            let r45 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r45);
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(3)) { return RegfileExit::fault(0x70008e7687); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 28; continue; }
            blk = 29; continue;
        }
        29 => {
            blk = 36; continue;
        }
        30 => {
            let r46 = c.sub(c.r32(8), 0x20000000u64, 32);
            c.w32(8, r46);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008e769a); }
            let Some(t47) = c.ld8(c.r64(0).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008e769e); };
            c.w32(8, t47);
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 33; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008e76b0); };
            c.w64(15, t48);
            blk = 32; continue;
        }
        32 => {
            let r49 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r49);
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(15)) { return RegfileExit::fault(0x70008e76bb); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ne() { blk = 32; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let r50 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r50);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(3)) { return RegfileExit::fault(0x70008e76ce); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 34; continue; }
            blk = 35; continue;
        }
        35 => {
            let r51 = c.add(c.r32(12), 0xe0000000u64, 32);
            c.w32(12, r51);
            if c.cc_e() { blk = 57; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e76e5); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2cb8)) else { return RegfileExit::fault(0x70008e76e6); };
            c.w64(3, t52);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008e76f7, 0x70008e76f2);
        }
        37 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e76f7); };
            c.w64(0, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e76fb); };
            c.w64(1, t54);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008e76ff); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r55 = c.r64(7) & c.r64(0);
            c.w64(7, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e7711); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e771b); };
            c.w64(1, t57);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e7722); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return c.ret(0x10, 0x70008e7729);
        }
        41 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e772c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e7733); }
            return c.ret(0x0, 0x70008e7735);
        }
        42 => {
            c.w32(1, 0x31eu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e773b); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e773c); };
            c.w64(3, t58);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008e774d, 0x70008e7748);
        }
        43 => {
            return RegfileExit::fault(0x70008e774d);
        }
        44 => {
            c.w32(1, 0x52u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e7753); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xb68)) else { return RegfileExit::fault(0x70008e7754); };
            c.w64(1, t59);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e775b); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e775c); };
            c.w64(3, t60);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e776d, 0x70008e7768);
        }
        45 => {
            return RegfileExit::fault(0x70008e776d);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008e776e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008e7772); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e7776); };
            c.w64(3, t61);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e7784, 0x70008e777f);
        }
        47 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7784); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7788); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e778c); };
            c.w64(2, t64);
            blk = 1; continue;
        }
        48 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008e779a); }
            return c.call(0x70007febf4, 0x70008e77a3, 0x70008e779e);
        }
        49 => {
            c.w64(8, c.r64(0));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e77a6); };
            c.w64(6, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e77aa); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e77ae); };
            c.w64(0, t67);
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e77b2); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e77b9); };
            c.w64(11, t69);
            blk = 14; continue;
        }
        50 => {
            return RegfileExit::fault(0x70008e77c5);
        }
        51 => {
            return RegfileExit::fault(0x70008e77c6);
        }
        52 => {
            return RegfileExit::fault(0x70008e77c7);
        }
        53 => {
            return RegfileExit::fault(0x70008e77c8);
        }
        54 => {
            return RegfileExit::fault(0x70008e77c9);
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008e77ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e77ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008e77d2); }
            return c.call(0x70007febf4, 0x70008e77db, 0x70008e77d6);
        }
        56 => {
            c.w64(7, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e77de); };
            c.w64(6, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e77e2); };
            c.w64(1, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e77e6); };
            c.w64(0, t72);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e77ed); };
            c.w64(3, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e77f4); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e77f8); };
            c.w64(2, t75);
            blk = 26; continue;
        }
        57 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e7801); }
            c.w32(0, 0x1u64);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008e7807); };
            c.w64(3, t76);
            c.w64(6, c.r64(9));
            return c.call(0x700086ed74, 0x70008e7816, 0x70008e7811);
        }
        58 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7816); };
            c.w64(6, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e781a); };
            c.w64(1, t78);
            blk = 36; continue;
        }
        59 => {
            c.w32(1, c.r32(1));
            let r79 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r79);
            c.w64(7, 0xfffffffffffc0000u64);
            let r80 = c.r64(7) & c.r64(1);
            c.w64(7, r80);
            c.logic(r80, 64);
            let Some(t81) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e7832); };
            c.logic(t81 & 0x2u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(8, c.r64(0).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e7840); }
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x70008e7849, 0x70008e7844);
        }
        61 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e7849); }
            blk = 39; continue;
        }
        62 => {
            return RegfileExit::jump(0x70008e7850);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e7854(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7854); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e7858); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e7859); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e785a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e785f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008e7869); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e7872); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e7878); };
            c.cmp(t5, 0x841u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e7880); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e7886); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e788a); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e7893); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2cb8)) else { return RegfileExit::fault(0x70008e7894); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008e78a8, 0x70008e78a3);
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e78a8); };
            c.w64(0, t10);
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e78ac); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e78b3); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x70008e78ba);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e78bd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e78c4); }
            return c.ret(0x0, 0x70008e78c6);
        }
        9 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e78cc); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e78cd); };
            c.w64(1, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0x40b)) else { return RegfileExit::fault(0x70008e78d4); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e78dd); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008e78de); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e78df); };
            c.w64(3, t15);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008e78f0, 0x70008e78eb);
        }
        10 => {
            return RegfileExit::fault(0x70008e78f0);
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e78f1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e78f5); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e7903, 0x70008e78fe);
        }
        12 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7903); };
            c.w64(6, t17);
            blk = 1; continue;
        }
        13 => {
            return RegfileExit::jump(0x70008e7910);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e7914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x0u64, 64);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_le() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(1));
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 48; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w32(7, c.r32(7));
            let r0 = c.imul2(c.r32(7), 0x4u64, 32);
            c.w32(7, r0);
            if c.cc_o() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r1 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r1);
            if c.cc_o() { blk = 22; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(9, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e797b); };
            c.w64(12, t2);
            let Some(t3) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e7982); };
            c.w64(12, t3);
            c.w64(15, c.r64(12).wrapping_add(c.r64(7)));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e798a); };
            c.w64(2, t4);
            let Some(t5) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008e7991); };
            c.cmp(t5, c.r64(15), 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 45; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e79a7); };
            c.w64(7, t6);
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x70008e79ae); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e79be); };
            c.w64(12, t7);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e79c5); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008e79c9); }
            c.w64(9, c.r64(3));
            blk = 12; continue;
        }
        11 => {
            c.w64(15, c.r64(8).wrapping_add(0x4));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e79d6); }
            c.w64(8, c.r64(15));
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ne() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(12, c.r64(0).wrapping_add(0x18));
            let Some(t8) = c.ld64(c.r64(12).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e79f7); };
            c.w64(12, t8);
            blk = 18; continue;
        }
        17 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e79fd); };
            c.w64(12, t9);
            blk = 18; continue;
        }
        18 => {
            let r10 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r10);
            blk = 11; continue;
        }
        19 => {
            let r11 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e7a0e); };
            c.w64(12, t12);
            blk = 11; continue;
        }
        20 => {
            return RegfileExit::fault(0x70008e7a17);
        }
        21 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70008e7a1b);
        }
        22 => {
            return RegfileExit::fault(0x70008e7a1c);
        }
        23 => {
            return RegfileExit::fault(0x70008e7a1d);
        }
        24 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e7a1e); };
            c.w64(0, t13);
            return c.ret(0x0, 0x70008e7a25);
        }
        25 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w32(7, c.r32(7));
            let r14 = c.imul2(c.r32(7), 0x4u64, 32);
            c.w32(7, r14);
            if c.cc_o() { blk = 42; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r15 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r15);
            if c.cc_o() { blk = 41; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(9, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7a69); };
            c.w64(12, t16);
            let Some(t17) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e7a70); };
            c.w64(12, t17);
            c.w64(15, c.r64(12).wrapping_add(c.r64(7)));
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e7a80); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e7a87); };
            c.cmp(t19, c.r64(15), 64);
            if c.cc_be() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 51; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7a9d); };
            c.w64(7, t20);
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x70008e7aa4); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 32; continue;
        }
        32 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e7aac); };
            c.w64(12, t21);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e7ab3); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008e7ab7); }
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        33 => {
            c.w64(12, c.r64(8).wrapping_add(0x4));
            let r22 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r22);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008e7ac7); }
            c.w64(8, c.r64(12));
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_e() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(9, c.r64(0).wrapping_add(0x18));
            let Some(t23) = c.ld64(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e7ae2); };
            c.w64(9, t23);
            blk = 33; continue;
        }
        38 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7ae8); };
            c.w64(9, t24);
            blk = 33; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008e7af1);
        }
        40 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70008e7af5);
        }
        41 => {
            return RegfileExit::fault(0x70008e7af6);
        }
        42 => {
            return RegfileExit::fault(0x70008e7af7);
        }
        43 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e7af8); };
            c.w64(0, t25);
            return c.ret(0x0, 0x70008e7aff);
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7b00); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7b04); }
            let r26 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r26);
            blk = 46; continue;
        }
        45 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7b0c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7b10); }
            let r27 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008e7b16); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008e7b1a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008e7b1e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008e7b22); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008e7b26); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008e7b32, 0x70008e7b2d);
        }
        47 => {
            c.w64(7, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e7b35); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e7b39); };
            c.w64(1, t29);
            c.w32(8, 0x8u64);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7b43); };
            c.w64(3, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7b47); };
            c.w64(11, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e7b4b); };
            c.w64(9, t32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e7b52); }
            blk = 10; continue;
        }
        48 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7b58); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7b5c); }
            let r33 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r33);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e7b64); };
            c.w64(3, t34);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e7b72, 0x70008e7b6d);
        }
        49 => {
            return RegfileExit::fault(0x70008e7b72);
        }
        50 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7b73); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7b77); }
            let r35 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r35);
            blk = 52; continue;
        }
        51 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7b7f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7b83); }
            let r36 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r36);
            blk = 52; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008e7b89); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008e7b8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008e7b91); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008e7b95); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008e7ba1, 0x70008e7b9c);
        }
        53 => {
            c.w64(7, c.r64(0));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e7ba4); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e7ba8); };
            c.w64(1, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7bac); };
            c.w64(11, t39);
            c.w32(8, 0x8u64);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7bb6); };
            c.w64(9, t40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e7bbd); }
            blk = 32; continue;
        }
        54 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7bc3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7bc7); }
            let r41 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r41);
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e7bcd); };
            c.w64(3, t42);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e7bdd, 0x70008e7bd8);
        }
        55 => {
            return RegfileExit::fault(0x70008e7bdd);
        }
        56 => {
            return RegfileExit::jump(0x70008e7be0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e7bf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 18; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 23; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w32(9, c.r32(9));
            let r0 = c.imul2(c.r32(9), 0x4u64, 32);
            c.w32(9, r0);
            if c.cc_o() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(11, 0x8u64);
            let r1 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r1);
            if c.cc_o() { blk = 16; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7c3c); };
            c.w64(12, t2);
            let Some(t3) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e7c43); };
            c.w64(15, t3);
            c.w64(3, c.r64(15).wrapping_add(c.r64(9)));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e7c4b); };
            c.w64(2, t4);
            let Some(t5) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008e7c52); };
            c.cmp(t5, c.r64(3), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008e7c68); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            c.w64(8, c.r64(8).wrapping_mul(2).wrapping_add(0x8));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e7c78); };
            c.w64(12, t6);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e7c7f); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008e7c83); }
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w64(12, c.r64(11).wrapping_add(0x4));
            let r7 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r7);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e7c93); }
            c.w64(11, c.r64(12));
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(7, c.r64(0).wrapping_add(0x18));
            let Some(t8) = c.ld64(c.r64(7).wrapping_add(c.r64(6).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e7cae); };
            c.w64(7, t8);
            blk = 8; continue;
        }
        13 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7cb4); };
            c.w64(7, t9);
            blk = 8; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008e7cbd);
        }
        15 => {
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x70008e7cc1);
        }
        16 => {
            return RegfileExit::fault(0x70008e7cc2);
        }
        17 => {
            return RegfileExit::fault(0x70008e7cc3);
        }
        18 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e7cc4); };
            c.w64(0, t10);
            return c.ret(0x0, 0x70008e7ccb);
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7ccc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7cd0); }
            let r11 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r11);
            blk = 21; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7cd8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7cdc); }
            let r12 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r12);
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008e7ce2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008e7ce6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008e7cea); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008e7cee); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008e7cfa, 0x70008e7cf5);
        }
        22 => {
            c.w64(9, c.r64(0));
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e7cfd); };
            c.w64(0, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e7d01); };
            c.w64(1, t14);
            c.w32(11, 0x8u64);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7d0b); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7d0f); };
            c.w64(7, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e7d16); }
            blk = 7; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7d1c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7d20); }
            let r17 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r17);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e7d26); };
            c.w64(3, t18);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e7d36, 0x70008e7d31);
        }
        24 => {
            return RegfileExit::fault(0x70008e7d36);
        }
        25 => {
            return RegfileExit::jump(0x70008e7d38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e7d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_le() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            let r0 = c.sub(c.r64(7), c.r64(3), 64);
            c.w64(7, r0);
            blk = 3; continue;
        }
        2 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 26; continue; }
            blk = 5; continue;
        }
        5 => {
            let r1 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r1);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w32(9, c.r32(9));
            let r2 = c.imul2(c.r32(9), 0x4u64, 32);
            c.w32(9, r2);
            if c.cc_o() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(11, 0x8u64);
            let r3 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r3);
            if c.cc_o() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7dac); };
            c.w64(12, t4);
            let Some(t5) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e7db3); };
            c.w64(15, t5);
            c.w64(2, c.r64(15).wrapping_add(c.r64(9)));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e7dbb); };
            c.w64(6, t6);
            let Some(t7) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008e7dc2); };
            c.cmp(t7, c.r64(2), 64);
            if c.cc_be() { blk = 22; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x70008e7dd4); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            c.w64(8, c.r64(8).wrapping_mul(2).wrapping_add(0x8));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e7de4); };
            c.w64(12, t8);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e7deb); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008e7def); }
            blk = 12; continue;
        }
        11 => {
            c.w64(12, c.r64(11).wrapping_add(0x4));
            let r9 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r9);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e7dfd); }
            c.w64(11, c.r64(12));
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_e() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_be() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(7, c.r64(0).wrapping_add(0x18));
            let Some(t10) = c.ld64(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e7e18); };
            c.w64(7, t10);
            blk = 11; continue;
        }
        16 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7e1e); };
            c.w64(7, t11);
            blk = 11; continue;
        }
        17 => {
            return RegfileExit::fault(0x70008e7e27);
        }
        18 => {
            c.w64(0, c.r64(9));
            return c.ret(0x0, 0x70008e7e2b);
        }
        19 => {
            return RegfileExit::fault(0x70008e7e2c);
        }
        20 => {
            return RegfileExit::fault(0x70008e7e2d);
        }
        21 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e7e2e); };
            c.w64(0, t12);
            return c.ret(0x0, 0x70008e7e35);
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7e36); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7e3a); }
            let r13 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            blk = 24; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7e42); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7e46); }
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008e7e4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008e7e50); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008e7e54); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008e7e58); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008e7e5c); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008e7e68, 0x70008e7e63);
        }
        25 => {
            c.w64(9, c.r64(0));
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e7e6b); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e7e6f); };
            c.w64(1, t16);
            c.w32(11, 0x8u64);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e7e79); };
            c.w64(3, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e7e7d); };
            c.w64(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e7e81); };
            c.w64(7, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e7e88); }
            blk = 10; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7e8e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7e92); }
            let r20 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e7e98); };
            c.w64(3, t21);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e7ea6, 0x70008e7ea1);
        }
        27 => {
            return RegfileExit::fault(0x70008e7ea6);
        }
        28 => {
            return RegfileExit::jump(0x70008e7ea8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e7eb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e7eb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e7eb8); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008e7ebe); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008e7ec2); };
            c.w64(7, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008e7ec6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008e7eca); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e7ed8); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e7ede); };
            c.w32(7, t5);
            let r6 = c.sub(c.r32(7), 0x89u64, 32);
            c.w32(7, r6);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_a() { blk = 81; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e7ef1); };
            c.w64(7, t7);
            let Some(t8) = c.ld64(c.r64(1).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e7ef5); };
            c.w64(8, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e7ef9); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            c.cmp(c.r64(8), 0x0u64, 64);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t11) = c.ld16(c.r64(7).wrapping_add(0x15)) else { return RegfileExit::fault(0x70008e7f0b); };
            c.w32(7, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008e7f0f); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(11, c.r64(8));
            blk = 6; continue;
        }
        5 => {
            c.w64(11, c.r64(7));
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 84; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(9, c.r64(8).wrapping_add(c.r64(8)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w32(9, c.r32(9));
            let r12 = c.imul2(c.r32(9), 0x4u64, 32);
            c.w32(9, r12);
            if c.cc_o() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(12, 0x8u64);
            let r13 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r13);
            if c.cc_o() { blk = 25; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(15, c.r64(8).wrapping_add(c.r64(8)));
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7f71); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008e7f78); };
            c.w64(2, t15);
            c.w64(0, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e7f7f); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008e7f86); };
            c.cmp(t17, c.r64(0), 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 82; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e7f9c); };
            c.w64(9, t18);
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x70008e7fa3); }
            c.w64(9, c.r64(2).wrapping_add(0x1));
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e7faa); };
            c.w64(0, t19);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008e7fb1); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008e7fb5); }
            c.w64(15, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(11));
            blk = 15; continue;
        }
        14 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008e7fc6); }
            let r20 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r20);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_e() { blk = 28; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ne() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(3, c.r64(1).wrapping_add(0x18));
            let Some(t21) = c.ld64(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e7fe8); };
            c.w64(3, t21);
            blk = 21; continue;
        }
        20 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e7fee); };
            c.w64(3, t22);
            blk = 21; continue;
        }
        21 => {
            let r23 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r23);
            blk = 14; continue;
        }
        22 => {
            let r24 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r24);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e7fff); };
            c.w64(3, t25);
            blk = 14; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008e8008);
        }
        24 => {
            blk = 28; continue;
        }
        25 => {
            return RegfileExit::fault(0x70008e800b);
        }
        26 => {
            return RegfileExit::fault(0x70008e800c);
        }
        27 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e800d); };
            c.w64(1, t26);
            c.w64(9, c.r64(1));
            blk = 28; continue;
        }
        28 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 49; continue;
        }
        29 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 88; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(9, c.r64(8).wrapping_add(c.r64(8)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w32(9, c.r32(9));
            let r27 = c.imul2(c.r32(9), 0x4u64, 32);
            c.w32(9, r27);
            if c.cc_o() { blk = 46; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(11, 0x8u64);
            let r28 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r28);
            if c.cc_o() { blk = 45; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(12, c.r64(8).wrapping_add(c.r64(8)));
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e8065); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e806c); };
            c.w64(3, t30);
            c.w64(2, c.r64(3).wrapping_add(c.r64(9)));
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e807b); };
            c.w64(0, t31);
            let Some(t32) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008e8082); };
            c.cmp(t32, c.r64(2), 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 86; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e8098); };
            c.w64(9, t33);
            if !c.st64(c.r64(9), c.r64(2)) { return RegfileExit::fault(0x70008e809f); }
            c.w64(9, c.r64(3).wrapping_add(0x1));
            blk = 36; continue;
        }
        36 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e80a6); };
            c.w64(0, t34);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008e80ad); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008e80b1); }
            c.w64(12, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        37 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008e80bd); }
            let r35 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r35);
            let r36 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r36);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 43; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(0, c.r64(1).wrapping_add(0x18));
            let Some(t37) = c.ld64(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e80dd); };
            c.w64(0, t37);
            blk = 37; continue;
        }
        42 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e80e3); };
            c.w64(0, t38);
            blk = 37; continue;
        }
        43 => {
            return RegfileExit::fault(0x70008e80ec);
        }
        44 => {
            blk = 48; continue;
        }
        45 => {
            return RegfileExit::fault(0x70008e80ef);
        }
        46 => {
            return RegfileExit::fault(0x70008e80f0);
        }
        47 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e80f1); };
            c.w64(9, t39);
            blk = 48; continue;
        }
        48 => {
            c.w32(1, 0x1u64);
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_le() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(1, c.r64(8));
            blk = 53; continue;
        }
        52 => {
            c.w64(1, c.r64(7));
            blk = 53; continue;
        }
        53 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8113); };
            c.w64(11, t40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e8117); };
            c.w32(12, t41);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008e8121); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            return RegfileExit::fault(0x70008e812b);
        }
        55 => {
            let Some(t42) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e812c); };
            c.w32(12, t42);
            let r43 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r43);
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e8133); };
            c.w32(12, t44);
            c.logic(c.r32(12) & 0x8000000u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(12, 0x2u64);
            blk = 58; continue;
        }
        57 => {
            c.w32(12, 0x3u64);
            blk = 58; continue;
        }
        58 => {
            c.w32(15, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(0, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(3, c.r32(3));
            let r45 = c.imul2(c.r32(3), 0x4u64, 32);
            c.w32(3, r45);
            if c.cc_o() { blk = 75; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0x10u64);
            let r46 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r46);
            if c.cc_o() { blk = 74; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e8178); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008e817f); };
            c.w64(6, t48);
            c.w64(8, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e8186); };
            c.w64(1, t49);
            let Some(t50) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e818d); };
            c.cmp(t50, c.r64(8), 64);
            if c.cc_be() { blk = 90; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 90; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e81a3); };
            c.w64(1, t51);
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008e81aa); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 63; continue;
        }
        63 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xe50)) else { return RegfileExit::fault(0x70008e81b1); };
            c.w64(8, t52);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e81b8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008e81bc); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008e81c0); }
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008e81c4); }
            let r53 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r53);
            c.w64(8, c.r64(0).wrapping_mul(2).wrapping_add(0x10));
            c.w64(9, c.r64(7));
            blk = 65; continue;
        }
        64 => {
            let r54 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r54);
            c.w32(12, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(1).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e81e0); }
            let r55 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r55);
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(12, c.r64(7));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e81f1); };
            let r57 = c.sub(c.r64(12), t56, 64);
            c.w64(12, r57);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ne() { blk = 64; continue; }
            blk = 67; continue;
        }
        67 => {
            return RegfileExit::fault(0x70008e81fa);
        }
        68 => {
            let Some(t58) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e81fb); };
            c.w32(7, t58);
            let r59 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e8202); };
            c.w32(7, t60);
            let r61 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r61);
            let Some(t62) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e8208); };
            c.w32(8, t62);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x44u64, 64);
            if c.cc_a() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            return RegfileExit::fault(0x70008e8218);
        }
        70 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e8219); };
            c.w64(8, t63);
            let r64 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r64);
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(0x117)) else { return RegfileExit::fault(0x70008e8220); };
            c.w32(7, t65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e8226); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008e822d); };
            c.w64(11, t67);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e8234); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008e823b); };
            c.cmp(t69, c.r64(12), 64);
            if c.cc_be() { blk = 92; continue; }
            blk = 71; continue;
        }
        71 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008e8244); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 72; continue;
        }
        72 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e824b); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e824f); };
            c.w64(7, t70);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008e8256); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008e825a); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008e825e); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8262); };
            c.w64(1, t71);
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008e8266); }
            c.w64(0, c.r64(9));
            blk = 73; continue;
        }
        73 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e8270); }
            return c.ret(0x0, 0x70008e8271);
        }
        74 => {
            return RegfileExit::fault(0x70008e8272);
        }
        75 => {
            return RegfileExit::fault(0x70008e8273);
        }
        76 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8274); };
            c.w64(1, t72);
            let Some(t73) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e8278); };
            c.w32(1, t73);
            let r74 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r74);
            let Some(t75) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e827e); };
            c.w32(1, t75);
            let r76 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r76);
            let Some(t77) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e8284); };
            c.w32(7, t77);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xaeu64, 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            return RegfileExit::fault(0x70008e8296);
        }
        78 => {
            let Some(t78) = c.ld32(c.r64(1).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008e8297); };
            c.w32(1, t78);
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e829d); };
            c.w32(7, t79);
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e82a1); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e82a8); };
            c.w64(11, t81);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e82af); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008e82b6); };
            c.cmp(t83, c.r64(12), 64);
            if c.cc_be() { blk = 94; continue; }
            blk = 79; continue;
        }
        79 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008e82bf); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 80; continue;
        }
        80 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008e82c6); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e82ca); };
            c.w64(1, t84);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008e82d1); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008e82d5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008e82d9); }
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e82dd); };
            c.w64(1, t85);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008e82e1); }
            c.w64(0, c.r64(8));
            blk = 73; continue;
        }
        81 => {
            return RegfileExit::fault(0x70008e82ea);
        }
        82 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008e82eb); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008e82ef); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008e82f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008e82f7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008e82fb); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008e8307, 0x70008e8302);
        }
        83 => {
            c.w64(9, c.r64(0));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e830a); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e830e); };
            c.w64(7, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8312); };
            c.w64(1, t88);
            c.w32(12, 0x8u64);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e831c); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e8320); };
            c.w64(3, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e8324); };
            c.w64(15, t91);
            blk = 13; continue;
        }
        84 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e8331); };
            c.w64(3, t92);
            return c.call(0x700086ed74, 0x70008e833d, 0x70008e8338);
        }
        85 => {
            return RegfileExit::fault(0x70008e833d);
        }
        86 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008e833e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008e8342); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008e8346); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008e834a); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008e8356, 0x70008e8351);
        }
        87 => {
            c.w64(9, c.r64(0));
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8359); };
            c.w64(6, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e835d); };
            c.w64(8, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e8361); };
            c.w64(7, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8365); };
            c.w64(1, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e8369); };
            c.w64(15, t97);
            c.w32(11, 0x8u64);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e8373); };
            c.w64(12, t98);
            blk = 36; continue;
        }
        88 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e837c); };
            c.w64(3, t99);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e838c, 0x70008e8387);
        }
        89 => {
            return RegfileExit::fault(0x70008e838c);
        }
        90 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008e838d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008e8391); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008e8395); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008e8399); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008e839d); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008e83a9, 0x70008e83a4);
        }
        91 => {
            c.w64(1, c.r64(0));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e83ac); };
            c.w64(9, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e83b0); };
            c.w64(11, t101);
            c.w32(2, 0x10u64);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e83b9); };
            c.w64(0, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e83bd); };
            c.w64(12, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e83c1); };
            c.w64(7, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e83c5); };
            c.w64(15, t105);
            blk = 63; continue;
        }
        92 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008e83d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008e83d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008e83db); }
            return c.call(0x70007febb4, 0x70008e83e4, 0x70008e83df);
        }
        93 => {
            c.w64(9, c.r64(0));
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e83e7); };
            c.w64(1, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e83eb); };
            c.w64(7, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e83ef); };
            c.w64(8, t108);
            blk = 72; continue;
        }
        94 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008e83fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008e8401); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008e8405); }
            return c.call(0x70007febb4, 0x70008e840e, 0x70008e8409);
        }
        95 => {
            c.w64(8, c.r64(0));
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8411); };
            c.w64(7, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8415); };
            c.w64(9, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e8419); };
            c.w64(1, t111);
            blk = 80; continue;
        }
        96 => {
            return RegfileExit::jump(0x70008e8424);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e8434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            let Some(t0) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008e8437); };
            c.w64(7, t0);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e8445); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e844b); };
            c.w32(7, t3);
            let r4 = c.sub(c.r32(7), 0x89u64, 32);
            c.w32(7, r4);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_a() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e845e); };
            c.w64(7, t5);
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 31; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(8, c.r64(7).wrapping_add(c.r64(7)));
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w32(11, c.r32(11));
            let r6 = c.imul2(c.r32(11), 0x4u64, 32);
            c.w32(11, r6);
            if c.cc_o() { blk = 19; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(12, 0x8u64);
            let r7 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r7);
            if c.cc_o() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e84a5); };
            c.w64(15, t8);
            let Some(t9) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008e84ac); };
            c.w64(15, t9);
            c.w64(0, c.r64(15).wrapping_add(c.r64(11)));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e84b3); };
            c.w64(3, t10);
            let Some(t11) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e84ba); };
            c.cmp(t11, c.r64(0), 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 28; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e84d0); };
            c.w64(11, t12);
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008e84d7); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 9; continue;
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e84de); };
            c.w64(15, t13);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008e84e5); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008e84e9); }
            c.w64(8, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        10 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008e84fa); }
            let r14 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r14);
            let r15 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r15);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(15, c.r64(1).wrapping_add(0x18));
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e851a); };
            c.w64(15, t16);
            blk = 10; continue;
        }
        15 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e8520); };
            c.w64(15, t17);
            blk = 10; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008e8529);
        }
        17 => {
            blk = 21; continue;
        }
        18 => {
            return RegfileExit::fault(0x70008e852c);
        }
        19 => {
            return RegfileExit::fault(0x70008e852d);
        }
        20 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e852e); };
            c.w64(0, t18);
            c.w64(11, c.r64(0));
            blk = 21; continue;
        }
        21 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e8538); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e853e); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e8544); };
            c.w32(7, t23);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xb0u64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70008e8556);
        }
        23 => {
            let Some(t24) = c.ld32(c.r64(1).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008e8557); };
            c.w32(1, t24);
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e855d); };
            c.w32(7, t25);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e8561); };
            c.w64(8, t26);
            let Some(t27) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e8568); };
            c.w64(9, t27);
            c.w64(12, c.r64(9).wrapping_add(0x10));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e856f); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008e8576); };
            c.cmp(t29, c.r64(12), 64);
            if c.cc_be() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008e857f); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008e8586); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e858a); };
            c.w64(1, t30);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008e8591); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008e8595); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008e8599); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70008e85a0);
        }
        26 => {
            return RegfileExit::fault(0x70008e85a1);
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e85a2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e85a6); }
            let r31 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r31);
            blk = 29; continue;
        }
        28 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e85ae); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e85b2); }
            let r32 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r32);
            blk = 29; continue;
        }
        29 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008e85b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008e85bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008e85c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008e85c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008e85c8); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008e85d4, 0x70008e85cf);
        }
        30 => {
            c.w64(11, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e85d7); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e85db); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e85df); };
            c.w64(7, t35);
            c.w32(12, 0x8u64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e85e9); };
            c.w64(9, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e85ed); };
            c.w64(8, t37);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e85f4); }
            blk = 9; continue;
        }
        31 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e85fa); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e85fe); }
            let r38 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e8604); };
            c.w64(3, t39);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e8614, 0x70008e860f);
        }
        32 => {
            return RegfileExit::fault(0x70008e8614);
        }
        33 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e8615); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e8619); }
            let r40 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r40);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008e8624); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008e8628); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008e862c); }
            return c.call(0x70007febb4, 0x70008e8635, 0x70008e8630);
        }
        34 => {
            c.w64(8, c.r64(0));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8638); };
            c.w64(7, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e863c); };
            c.w64(11, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8640); };
            c.w64(1, t43);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e8647); }
            blk = 25; continue;
        }
        35 => {
            return RegfileExit::jump(0x70008e8650);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e8654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            let Some(t0) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008e8657); };
            c.w64(7, t0);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e8665); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e866b); };
            c.w32(7, t3);
            let r4 = c.sub(c.r32(7), 0x89u64, 32);
            c.w32(7, r4);
            c.cmp(c.r32(7), 0x9u64, 32);
            if c.cc_a() { blk = 29; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e867e); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8684); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e8688); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e868e); };
            c.w32(9, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8692); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t13) = c.ld64(c.r64(1).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e869c); };
            c.w64(11, t13);
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x15)) else { return RegfileExit::fault(0x70008e86a0); };
            c.w32(8, t14);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x70008e86ae);
        }
        4 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_le() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(11));
            let r15 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r15);
            blk = 7; continue;
        }
        6 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008e86c2); };
            c.w32(7, t16);
            c.w32(6, 0);
            c.logic(0, 32);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(9), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 34; continue; }
            blk = 9; continue;
        }
        9 => {
            let r17 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r17);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w32(15, c.r32(15));
            let r18 = c.imul2(c.r32(15), 0x4u64, 32);
            c.w32(15, r18);
            if c.cc_o() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(0, 0x8u64);
            let r19 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r19);
            if c.cc_o() { blk = 23; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e870b); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e8712); };
            c.w64(3, t21);
            c.w64(2, c.r64(3).wrapping_add(c.r64(15)));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e8719); };
            c.w64(6, t22);
            let Some(t23) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008e8720); };
            c.cmp(t23, c.r64(2), 64);
            if c.cc_be() { blk = 30; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 31; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e8736); };
            c.w64(15, t24);
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008e873d); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e8744); };
            c.w64(3, t25);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008e874b); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008e874f); }
            c.w64(9, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            blk = 16; continue;
        }
        15 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(0)).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e875d); }
            let r26 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r26);
            let r27 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r27);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(9), c.r64(0), 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(12, c.r64(1).wrapping_add(0x18));
            let Some(t28) = c.ld64(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008e877d); };
            c.w64(12, t28);
            blk = 15; continue;
        }
        20 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e8783); };
            c.w64(12, t29);
            blk = 15; continue;
        }
        21 => {
            return RegfileExit::fault(0x70008e878c);
        }
        22 => {
            blk = 26; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008e878f);
        }
        24 => {
            return RegfileExit::fault(0x70008e8790);
        }
        25 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e8791); };
            c.w64(0, t30);
            c.w64(15, c.r64(0));
            blk = 26; continue;
        }
        26 => {
            let Some(t31) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e879b); };
            c.w32(1, t31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e879f); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e87a6); };
            c.w64(9, t33);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e87ad); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e87b4); };
            c.cmp(t35, c.r64(11), 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008e87be); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e87c5); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e87c9); };
            c.w64(7, t36);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008e87d0); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008e87d4); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008e87d8); }
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70008e87df);
        }
        29 => {
            return RegfileExit::fault(0x70008e87e0);
        }
        30 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e87e1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e87e5); }
            let r37 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r37);
            blk = 32; continue;
        }
        31 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e87ed); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e87f1); }
            let r38 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r38);
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008e87f7); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008e87fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008e87ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008e8803); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008e8807); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008e880b); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008e8817, 0x70008e8812);
        }
        33 => {
            c.w64(15, c.r64(0));
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e881a); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e881e); };
            c.w64(1, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8822); };
            c.w64(11, t41);
            c.w32(0, 0x8u64);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e882b); };
            c.w64(8, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e882f); };
            c.w64(12, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e8833); };
            c.w64(9, t44);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e883a); }
            blk = 14; continue;
        }
        34 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e8840); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e8844); }
            let r45 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r45);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e884a); };
            c.w64(3, t46);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e8858, 0x70008e8853);
        }
        35 => {
            return RegfileExit::fault(0x70008e8858);
        }
        36 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e8859); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e885d); }
            let r47 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r47);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008e8868); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008e886c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008e8870); }
            return c.call(0x70007febb4, 0x70008e8879, 0x70008e8874);
        }
        37 => {
            c.w64(8, c.r64(0));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e887c); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8880); };
            c.w64(15, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8884); };
            c.w64(7, t50);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e888b); }
            blk = 28; continue;
        }
        38 => {
            return RegfileExit::jump(0x70008e8894);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e88b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e88b4); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e88ba); };
            c.cmp(t2, 0x0u64, 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e88c0); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e88c6); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e88cc); };
            c.w32(3, t7);
            let r8 = c.r32(3) & 0x7u64;
            c.w32(3, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e88dc); };
            c.w32(6, t9);
            let r10 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r10);
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e88e2); };
            c.cmp(t11, 0x0u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 0; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e88ee); };
            c.w32(3, t12);
            let r13 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r13);
            let Some(t14) = c.ld8(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e88f4); };
            c.logic(t14 & 0x8u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e88fe); };
            c.w32(3, t15);
            c.w64(6, c.r64(3));
            let r16 = c.add(c.r32(6), c.r32(3), 32);
            c.w32(6, r16);
            if c.cc_o() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r17 = c.add(c.r64(6), c.r64(7), 64);
            c.w64(6, r17);
            if c.cc_o() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e891d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e8921); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e8927); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e892e); };
            c.w64(7, t20);
            let r21 = c.add(c.r64(6), 0x3u64, 64);
            c.w64(6, r21);
            let r22 = c.r64(6) & 0xfffffffffffffffcu64;
            c.w64(6, r22);
            c.logic(r22, 64);
            c.w64(11, c.r64(7).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008e893d); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e8941); };
            c.w64(12, t23);
            let Some(t24) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e8948); };
            c.cmp(t24, c.r64(11), 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_ae() { blk = 35; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e895f); };
            c.w64(6, t25);
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x70008e8966); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008e896d); };
            c.w64(7, t26);
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e8974); }
            if !c.st32(c.r64(6).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008e8977); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008e897e); }
            let Some(t27) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8981); };
            c.w32(9, t27);
            c.w64(2, c.r64(6).wrapping_add(0xb));
            c.w64(10, c.r64(4));
            let r28 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r28);
            let r29 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r29);
            c.logic(r29, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e8994); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008e8999); }
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x1458)) else { return RegfileExit::fault(0x70008e89a3); };
            c.w64(0, t30);
            c.w64(10, 0x70008e89b1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e89b1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e89b5); }
            return c.call(c.r64(0), 0x70008e89bb, 0x70008e89b9);
        }
        11 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e89bb); }
            let Some(t31) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e89c3); };
            c.w64(4, t31);
            blk = 20; continue;
        }
        12 => {
            return RegfileExit::fault(0x70008e89cd);
        }
        13 => {
            return RegfileExit::fault(0x70008e89ce);
        }
        14 => {
            c.w64(1, c.r64(0));
            let Some(t32) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e89d2); };
            c.w32(3, t32);
            c.w64(6, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r33 = c.add(c.r64(6), c.r64(7), 64);
            c.w64(6, r33);
            if c.cc_o() { blk = 24; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e89e6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e89ea); }
            let r34 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e89f0); };
            c.w64(7, t35);
            let Some(t36) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e89f7); };
            c.w64(7, t36);
            let r37 = c.add(c.r64(6), 0x3u64, 64);
            c.w64(6, r37);
            let r38 = c.r64(6) & 0xfffffffffffffffcu64;
            c.w64(6, r38);
            c.logic(r38, 64);
            c.w64(11, c.r64(7).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008e8a06); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e8a0a); };
            c.w64(12, t39);
            let Some(t40) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e8a11); };
            c.cmp(t40, c.r64(11), 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_ae() { blk = 37; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e8a28); };
            c.w64(6, t41);
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x70008e8a2f); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 18; continue;
        }
        18 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008e8a36); };
            c.w64(7, t42);
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e8a3d); }
            if !c.st32(c.r64(6).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008e8a40); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008e8a47); }
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8a4a); };
            c.w32(9, t43);
            c.w64(2, c.r64(6).wrapping_add(0xb));
            c.w64(10, c.r64(4));
            let r44 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r44);
            let r45 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r45);
            c.logic(r45, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e8a5d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008e8a62); }
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1450)) else { return RegfileExit::fault(0x70008e8a69); };
            c.w64(0, t46);
            c.w64(10, 0x70008e8a77u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e8a77); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e8a7b); }
            return c.call(c.r64(0), 0x70008e8a81, 0x70008e8a7f);
        }
        19 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e8a81); }
            let Some(t47) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e8a89); };
            c.w64(4, t47);
            blk = 20; continue;
        }
        20 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8a8e); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8a92); };
            c.w64(3, t49);
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008e8a96); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r50 = c.r64(6) & c.r64(3);
            c.w64(6, r50);
            c.logic(r50, 64);
            let Some(t51) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e8aa7); };
            c.logic(t51 & 0x4u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e8ab1); };
            c.w64(6, t52);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x70008e8ab8); }
            blk = 23; continue;
        }
        23 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e8abe); }
            return c.ret(0x0, 0x70008e8abf);
        }
        24 => {
            return RegfileExit::fault(0x70008e8ac0);
        }
        25 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        26 => {
            c.w32(6, 0x1u64);
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e8acf); };
            c.w32(3, t53);
            let r54 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8ad5); };
            c.cmp(t55, 0x0u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e8adb); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e8adf); }
            let r56 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r56);
            return c.call(0x70008e88b4, 0x70008e8aea, 0x70008e8ae5);
        }
        30 => {
            blk = 23; continue;
        }
        31 => {
            let Some(t57) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8aec); };
            c.w32(0, t57);
            let r58 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r58);
            return c.ret(0x0, 0x70008e8af2);
        }
        32 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t59) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8af8); };
            c.w32(0, t59);
            let r60 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r60);
            return c.ret(0x0, 0x70008e8afe);
        }
        34 => {
            return c.ret(0x0, 0x70008e8aff);
        }
        35 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008e8b00); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x70008e8b0c, 0x70008e8b07);
        }
        36 => {
            c.w64(6, c.r64(0));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8b0f); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8b13); };
            c.w64(3, t62);
            blk = 10; continue;
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008e8b1c); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x70008e8b28, 0x70008e8b23);
        }
        38 => {
            c.w64(6, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8b2b); };
            c.w64(1, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8b2f); };
            c.w64(3, t64);
            blk = 18; continue;
        }
        39 => {
            c.w32(0, c.r32(0));
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            c.w64(6, 0xfffffffffffc0000u64);
            let r66 = c.r64(6) & c.r64(0);
            c.w64(6, r66);
            c.logic(r66, 64);
            let Some(t67) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e8b47); };
            c.logic(t67 & 0x2u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(7, c.r64(3).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e8b55); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e8b56); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x70008e8b63, 0x70008e8b5e);
        }
        41 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e8b63); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e8b65); }
            blk = 22; continue;
        }
        42 => {
            return RegfileExit::jump(0x70008e8b6c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e8b74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e8b74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e8b78); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008e8b7e); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(1));
            blk = 3; continue;
        }
        2 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8b91); };
            c.w32(12, t1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8b95); };
            c.w32(15, t2);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008e8b9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008e8ba2); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_le() { blk = 139; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(7, c.r64(11).wrapping_add(c.r64(12)));
            c.cmp(c.r64(15), c.r64(7), 64);
            if c.cc_l() { blk = 138; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(8, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008e8bc5); }
            c.w64(7, c.r64(6));
            blk = 7; continue;
        }
        7 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e8bcc); };
            c.w32(12, t3);
            let r4 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8bd3); };
            c.w32(12, t5);
            c.w64(2, c.r64(12));
            let r6 = c.r32(2) & 0xfu64;
            c.w32(2, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(1, c.r64(12));
            let r7 = c.r32(1) & 0x7u64;
            c.w32(1, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8bf8); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            blk = 7; continue;
        }
        11 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e8c05); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8c0c); };
            c.cmp(t12, 0x0u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008e8c14); }
            return c.call(0x70008e88b4, 0x70008e8c1d, 0x70008e8c18);
        }
        14 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8c1d); };
            c.w64(7, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8c21); };
            c.w64(11, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e8c25); };
            c.w64(3, t15);
            blk = 7; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8c2b); };
            c.w32(0, t16);
            let r17 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r17);
            blk = 7; continue;
        }
        16 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e8c38); };
            c.w32(6, t18);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r19 = c.add(c.r64(7), c.r64(6), 64);
            c.w64(7, r19);
            let Some(t20) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8c44); };
            c.w32(0, t20);
            let r21 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r21);
            blk = 7; continue;
        }
        18 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8c4c); };
            c.w32(6, t22);
            let Some(t23) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e8c4f); };
            c.cmp(c.r32(6), t23, 32);
            if c.cc_b() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8c57); };
            let r25 = c.sub(c.r64(6), t24, 64);
            c.w64(6, r25);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_b() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(6, c.r64(7).wrapping_add(0xc));
            blk = 34; continue;
        }
        21 => {
            return RegfileExit::fault(0x70008e8c69);
        }
        22 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8c6a); };
            c.w32(6, t26);
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e8c6d); };
            c.cmp(c.r32(6), t27, 32);
            if c.cc_b() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8c75); };
            let r29 = c.sub(c.r64(6), t28, 64);
            c.w64(6, r29);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_b() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(6, c.r64(7).wrapping_mul(2).wrapping_add(0xc));
            blk = 93; continue;
        }
        25 => {
            return RegfileExit::fault(0x70008e8c8b);
        }
        26 => {
            c.cmp(c.r32(2), 0xau64, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t30) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e8c9b); };
            c.w64(6, t30);
            blk = 31; continue;
        }
        29 => {
            c.w64(10, c.r64(4));
            let r31 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r31);
            let r32 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r32);
            c.logic(r32, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e8cac); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e8cb1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008e8cb5); }
            c.w64(1, c.r64(0));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008e8cbc); };
            c.w64(0, t33);
            c.w64(10, 0x70008e8ccau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e8cca); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e8cce); }
            return c.call(c.r64(0), 0x70008e8cd4, 0x70008e8cd2);
        }
        30 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e8cd4); }
            let Some(t34) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e8cdc); };
            c.w64(4, t34);
            c.w64(6, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8ce4); };
            c.w64(11, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e8ce8); };
            c.w64(3, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e8cec); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8cf0); };
            c.w64(7, t38);
            blk = 31; continue;
        }
        31 => {
            c.w64(1, c.r64(0));
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8cf7); };
            c.w32(12, t39);
            let Some(t40) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e8cfb); };
            c.cmp(c.r32(12), t40, 32);
            if c.cc_b() { blk = 84; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8d08); };
            let r42 = c.sub(c.r64(12), t41, 64);
            c.w64(12, r42);
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_b() { blk = 84; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(6, c.r64(7).wrapping_add(c.r64(6)).wrapping_add(0x1));
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        34 => {
            let Some(t43) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8d1c); };
            c.w32(7, t43);
            c.w64(12, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008e8d22); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e8d26); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008e8d2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x70008e8d2e); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t44) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e8d35); };
            c.w32(2, t44);
            let r45 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r45);
            let Some(t46) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8d3b); };
            c.w32(2, t46);
            c.w64(1, c.r64(2));
            let r47 = c.r32(1) & 0xfu64;
            c.w32(1, r47);
            c.logic(r47, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(8, c.r64(2));
            let r48 = c.r32(8) & 0x7u64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t49) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8d64); };
            c.w32(3, t49);
            let r50 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r50);
            blk = 35; continue;
        }
        39 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t51) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e8d72); };
            c.w32(2, t51);
            let r52 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8d78); };
            c.cmp(t53, 0x0u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e8d7e); }
            c.w64(0, c.r64(3));
            return c.call(0x70008e88b4, 0x70008e8d8a, 0x70008e8d85);
        }
        42 => {
            c.w64(3, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8d8d); };
            c.w64(12, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e8d91); };
            c.w64(6, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e8d95); };
            c.w64(0, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e8d99); };
            c.w64(15, t57);
            blk = 35; continue;
        }
        43 => {
            let Some(t58) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8d9f); };
            c.w32(3, t58);
            let r59 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r59);
            blk = 35; continue;
        }
        44 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t60) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e8dad); };
            c.w32(7, t60);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r61 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r61);
            let Some(t62) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e8db9); };
            c.w32(3, t62);
            let r63 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r63);
            blk = 35; continue;
        }
        46 => {
            let Some(t64) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8dc4); };
            c.w32(7, t64);
            let Some(t65) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e8dc7); };
            c.cmp(c.r32(7), t65, 32);
            if c.cc_b() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r66 = c.sub(c.r64(7), c.r64(12), 64);
            c.w64(7, r66);
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_b() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(7, c.r64(15).wrapping_add(0xc));
            blk = 62; continue;
        }
        49 => {
            return RegfileExit::fault(0x70008e8de0);
        }
        50 => {
            let Some(t67) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8de1); };
            c.w32(7, t67);
            let Some(t68) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e8de4); };
            c.cmp(c.r32(7), t68, 32);
            if c.cc_b() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r69 = c.sub(c.r64(7), c.r64(12), 64);
            c.w64(7, r69);
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_b() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(7, c.r64(15).wrapping_mul(2).wrapping_add(0xc));
            blk = 80; continue;
        }
        53 => {
            return RegfileExit::fault(0x70008e8e01);
        }
        54 => {
            c.cmp(c.r32(1), 0xau64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t70) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e8e10); };
            c.w64(7, t70);
            blk = 59; continue;
        }
        57 => {
            c.w64(10, c.r64(4));
            let r71 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r71);
            let r72 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r72);
            c.logic(r72, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e8e21); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008e8e26); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e8e2a); }
            c.w64(1, c.r64(3));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008e8e31); };
            c.w64(0, t73);
            c.w64(10, 0x70008e8e3fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e8e3f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e8e43); }
            return c.call(c.r64(0), 0x70008e8e49, 0x70008e8e47);
        }
        58 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e8e49); }
            let Some(t74) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e8e51); };
            c.w64(4, t74);
            c.w64(7, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8e59); };
            c.w64(12, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e8e5d); };
            c.w64(6, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e8e61); };
            c.w64(0, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e8e65); };
            c.w64(3, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e8e69); };
            c.w64(15, t79);
            blk = 59; continue;
        }
        59 => {
            let Some(t80) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8e6d); };
            c.w32(3, t80);
            let Some(t81) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e8e70); };
            c.cmp(c.r32(3), t81, 32);
            if c.cc_b() { blk = 71; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r82 = c.sub(c.r64(3), c.r64(12), 64);
            c.w64(3, r82);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_b() { blk = 71; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(7)).wrapping_add(0x1));
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 62; continue;
        }
        62 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(7)).wrapping_sub(0x1));
            c.w64(2, c.r64(0).wrapping_add(c.r64(6)).wrapping_sub(0x1));
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(10, c.r64(4));
            let r83 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r83);
            let r84 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r84);
            c.logic(r84, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008e8eaa); }
            c.w64(7, c.r64(12));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(7)) { return RegfileExit::fault(0x70008e8eb2); }
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8eb7); };
            c.w64(3, t85);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(3)) { return RegfileExit::fault(0x70008e8ebb); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8ec0); };
            c.w64(8, t86);
            c.w64(1, c.r64(13));
            c.w64(11, c.r64(1));
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x1430)) else { return RegfileExit::fault(0x70008e8eca); };
            c.w64(0, t87);
            c.w64(10, 0x70008e8ed8u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e8ed8); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e8edc); }
            return c.call(c.r64(0), 0x70008e8ee2, 0x70008e8ee0);
        }
        64 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e8ee2); }
            let Some(t88) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008e8eea); };
            c.w64(4, t88);
            blk = 69; continue;
        }
        65 => {
            let Some(t89) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x70008e8ef1); };
            c.w32(6, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8ef5); };
            c.w64(3, t90);
            c.w64(1, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e8efd); };
            c.w64(8, t91);
            let r92 = c.sub(c.r64(8), c.r64(3), 64);
            c.w64(8, r92);
            c.w64(10, c.r64(4));
            let r93 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r93);
            let r94 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r94);
            c.logic(r94, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e8f0f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008e8f14); }
            c.w64(2, c.r64(6));
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x13c0)) else { return RegfileExit::fault(0x70008e8f1b); };
            c.w64(0, t95);
            c.w64(10, 0x70008e8f29u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e8f29); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e8f2d); }
            return c.call(c.r64(0), 0x70008e8f33, 0x70008e8f31);
        }
        66 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e8f33); }
            let Some(t96) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e8f3b); };
            c.w64(4, t96);
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8f46); };
            let r98 = c.sub(c.r64(0), t97, 64);
            c.w64(0, r98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e8f4a); };
            c.w64(12, t99);
            let r100 = c.add(c.r64(0), c.r64(12), 64);
            c.w64(0, r100);
            blk = 69; continue;
        }
        68 => {
            c.w64(0, 0xffffffffffffffffu64);
            blk = 69; continue;
        }
        69 => {
            let r101 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r101);
            blk = 70; continue;
        }
        70 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e8f5f); }
            return c.ret(0x0, 0x70008e8f60);
        }
        71 => {
            return RegfileExit::fault(0x70008e8f61);
        }
        72 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 73; continue;
        }
        73 => {
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t102) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e8f70); };
            c.w64(7, t102);
            blk = 77; continue;
        }
        75 => {
            c.w64(10, c.r64(4));
            let r103 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r103);
            let r104 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r104);
            c.logic(r104, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e8f81); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008e8f86); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e8f8a); }
            c.w64(1, c.r64(3));
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008e8f91); };
            c.w64(0, t105);
            c.w64(10, 0x70008e8f9fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e8f9f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e8fa3); }
            return c.call(c.r64(0), 0x70008e8fa9, 0x70008e8fa7);
        }
        76 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e8fa9); }
            let Some(t106) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e8fb1); };
            c.w64(4, t106);
            c.w64(7, c.r64(0));
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e8fb9); };
            c.w64(12, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e8fbd); };
            c.w64(6, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e8fc1); };
            c.w64(0, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e8fc5); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e8fc9); };
            c.w64(15, t111);
            blk = 77; continue;
        }
        77 => {
            let Some(t112) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e8fcd); };
            c.w32(3, t112);
            let Some(t113) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e8fd0); };
            c.cmp(c.r32(3), t113, 32);
            if c.cc_b() { blk = 82; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r114 = c.sub(c.r64(3), c.r64(12), 64);
            c.w64(3, r114);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_b() { blk = 82; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(7, c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 80; continue;
        }
        80 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(7)).wrapping_sub(0x1));
            c.w64(2, c.r64(0).wrapping_add(c.r64(6)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r115 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r115);
            let r116 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r116);
            c.logic(r116, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008e8ffc); }
            c.w64(3, c.r64(12));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(3)) { return RegfileExit::fault(0x70008e9004); }
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e9009); };
            c.w64(6, t117);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(6)) { return RegfileExit::fault(0x70008e900d); }
            c.w64(1, c.r64(13));
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9015); };
            c.w64(8, t118);
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x1438)) else { return RegfileExit::fault(0x70008e9019); };
            c.w64(0, t119);
            c.w64(10, 0x70008e9027u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e9027); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e902b); }
            return c.call(c.r64(0), 0x70008e9031, 0x70008e902f);
        }
        81 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e9031); }
            let Some(t120) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008e9039); };
            c.w64(4, t120);
            let r121 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r121);
            blk = 70; continue;
        }
        82 => {
            return RegfileExit::fault(0x70008e9045);
        }
        83 => {
            return RegfileExit::fault(0x70008e9046);
        }
        84 => {
            return RegfileExit::fault(0x70008e9047);
        }
        85 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 86; continue;
        }
        86 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t122) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e9057); };
            c.w64(6, t122);
            blk = 90; continue;
        }
        88 => {
            c.w64(10, c.r64(4));
            let r123 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r123);
            let r124 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r124);
            c.logic(r124, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e9068); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e906d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008e9071); }
            c.w64(1, c.r64(0));
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008e9078); };
            c.w64(0, t125);
            c.w64(10, 0x70008e9086u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e9086); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e908a); }
            return c.call(c.r64(0), 0x70008e9090, 0x70008e908e);
        }
        89 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e9090); }
            let Some(t126) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e9098); };
            c.w64(4, t126);
            c.w64(6, c.r64(0));
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e90a0); };
            c.w64(11, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e90a4); };
            c.w64(3, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e90a8); };
            c.w64(0, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e90ac); };
            c.w64(7, t130);
            blk = 90; continue;
        }
        90 => {
            let Some(t131) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e90b0); };
            c.w32(12, t131);
            let Some(t132) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e90b4); };
            c.cmp(c.r32(12), t132, 32);
            if c.cc_b() { blk = 136; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e90c1); };
            let r134 = c.sub(c.r64(12), t133, 64);
            c.w64(12, r134);
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_b() { blk = 136; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(6, c.r64(6).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 93; continue;
        }
        93 => {
            let Some(t135) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e90d5); };
            c.w32(7, t135);
            c.w64(12, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008e90db); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e90df); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008e90e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x70008e90e7); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 94; continue;
        }
        94 => {
            let Some(t136) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e90ee); };
            c.w32(2, t136);
            let r137 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r137);
            let Some(t138) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e90f4); };
            c.w32(2, t138);
            c.w64(1, c.r64(2));
            let r139 = c.r32(1) & 0xfu64;
            c.w32(1, r139);
            c.logic(r139, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(8, c.r64(2));
            let r140 = c.r32(8) & 0x7u64;
            c.w32(8, r140);
            c.logic(r140, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t141) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e911d); };
            c.w32(3, t141);
            let r142 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r142);
            blk = 94; continue;
        }
        98 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t143) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e912b); };
            c.w32(2, t143);
            let r144 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r144);
            let Some(t145) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9131); };
            c.cmp(t145, 0x0u64, 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e9137); }
            c.w64(0, c.r64(3));
            return c.call(0x70008e88b4, 0x70008e9143, 0x70008e913e);
        }
        101 => {
            c.w64(3, c.r64(0));
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e9146); };
            c.w64(12, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e914a); };
            c.w64(6, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e914e); };
            c.w64(0, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e9152); };
            c.w64(15, t149);
            blk = 94; continue;
        }
        102 => {
            let Some(t150) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e9158); };
            c.w32(3, t150);
            let r151 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r151);
            blk = 94; continue;
        }
        103 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 113; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t152) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e9166); };
            c.w32(7, t152);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r153 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r153);
            let Some(t154) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e9172); };
            c.w32(3, t154);
            let r155 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r155);
            blk = 94; continue;
        }
        105 => {
            let Some(t156) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e917d); };
            c.w32(7, t156);
            let Some(t157) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e9180); };
            c.cmp(c.r32(7), t157, 32);
            if c.cc_b() { blk = 108; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r158 = c.sub(c.r64(7), c.r64(12), 64);
            c.w64(7, r158);
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_b() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(7, c.r64(15).wrapping_add(0xc));
            blk = 121; continue;
        }
        108 => {
            return RegfileExit::fault(0x70008e9199);
        }
        109 => {
            let Some(t159) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e919a); };
            c.w32(7, t159);
            let Some(t160) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e919d); };
            c.cmp(c.r32(7), t160, 32);
            if c.cc_b() { blk = 112; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r161 = c.sub(c.r64(7), c.r64(12), 64);
            c.w64(7, r161);
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_b() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(7, c.r64(15).wrapping_mul(2).wrapping_add(0xc));
            blk = 132; continue;
        }
        112 => {
            return RegfileExit::fault(0x70008e91ba);
        }
        113 => {
            c.cmp(c.r32(1), 0xau64, 32);
            if c.cc_ne() { blk = 124; continue; }
            blk = 114; continue;
        }
        114 => {
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t162) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e91c9); };
            c.w64(7, t162);
            blk = 118; continue;
        }
        116 => {
            c.w64(10, c.r64(4));
            let r163 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r163);
            let r164 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r164);
            c.logic(r164, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e91da); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008e91df); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e91e3); }
            c.w64(1, c.r64(3));
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008e91ea); };
            c.w64(0, t165);
            c.w64(10, 0x70008e91f8u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e91f8); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e91fc); }
            return c.call(c.r64(0), 0x70008e9202, 0x70008e9200);
        }
        117 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e9202); }
            let Some(t166) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e920a); };
            c.w64(4, t166);
            c.w64(7, c.r64(0));
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e9212); };
            c.w64(12, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e9216); };
            c.w64(6, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e921a); };
            c.w64(0, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e921e); };
            c.w64(3, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e9222); };
            c.w64(15, t171);
            blk = 118; continue;
        }
        118 => {
            let Some(t172) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9226); };
            c.w32(3, t172);
            let Some(t173) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e9229); };
            c.cmp(c.r32(3), t173, 32);
            if c.cc_b() { blk = 123; continue; }
            blk = 119; continue;
        }
        119 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r174 = c.sub(c.r64(3), c.r64(12), 64);
            c.w64(3, r174);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_b() { blk = 123; continue; }
            blk = 120; continue;
        }
        120 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(7)).wrapping_add(0x1));
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 121; continue;
        }
        121 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(7)).wrapping_sub(0x1));
            c.w64(2, c.r64(0).wrapping_add(c.r64(6)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r175 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r175);
            let r176 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r176);
            c.logic(r176, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008e9255); }
            c.w64(7, c.r64(12));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(7)) { return RegfileExit::fault(0x70008e925d); }
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e9262); };
            c.w64(11, t177);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(11)) { return RegfileExit::fault(0x70008e9266); }
            c.w64(1, c.r64(13));
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e926e); };
            c.w64(8, t178);
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0x1440)) else { return RegfileExit::fault(0x70008e9272); };
            c.w64(0, t179);
            c.w64(10, 0x70008e9280u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e9280); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e9284); }
            return c.call(c.r64(0), 0x70008e928a, 0x70008e9288);
        }
        122 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e928a); }
            let Some(t180) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008e9292); };
            c.w64(4, t180);
            let r181 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r181);
            blk = 70; continue;
        }
        123 => {
            return RegfileExit::fault(0x70008e929e);
        }
        124 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 125; continue;
        }
        125 => {
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t182) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e92ad); };
            c.w64(7, t182);
            blk = 129; continue;
        }
        127 => {
            c.w64(10, c.r64(4));
            let r183 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r183);
            let r184 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r184);
            c.logic(r184, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e92be); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008e92c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e92c7); }
            c.w64(1, c.r64(3));
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008e92ce); };
            c.w64(0, t185);
            c.w64(10, 0x70008e92dcu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e92dc); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e92e0); }
            return c.call(c.r64(0), 0x70008e92e6, 0x70008e92e4);
        }
        128 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e92e6); }
            let Some(t186) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e92ee); };
            c.w64(4, t186);
            c.w64(7, c.r64(0));
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e92f6); };
            c.w64(12, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e92fa); };
            c.w64(6, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e92fe); };
            c.w64(0, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e9302); };
            c.w64(3, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e9306); };
            c.w64(15, t191);
            blk = 129; continue;
        }
        129 => {
            let Some(t192) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e930a); };
            c.w32(3, t192);
            let Some(t193) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e930d); };
            c.cmp(c.r32(3), t193, 32);
            if c.cc_b() { blk = 134; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r194 = c.sub(c.r64(3), c.r64(12), 64);
            c.w64(3, r194);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_b() { blk = 134; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w64(7, c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 132; continue;
        }
        132 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(7)).wrapping_sub(0x1));
            c.w64(2, c.r64(0).wrapping_add(c.r64(6)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r195 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r195);
            let r196 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r196);
            c.logic(r196, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008e9339); }
            c.w64(3, c.r64(12));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(3)) { return RegfileExit::fault(0x70008e9341); }
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e9346); };
            c.w64(3, t197);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(3)) { return RegfileExit::fault(0x70008e934a); }
            c.w64(1, c.r64(13));
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9352); };
            c.w64(8, t198);
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x1448)) else { return RegfileExit::fault(0x70008e9356); };
            c.w64(0, t199);
            c.w64(10, 0x70008e9364u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e9364); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e9368); }
            return c.call(c.r64(0), 0x70008e936e, 0x70008e936c);
        }
        133 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e936e); }
            let Some(t200) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008e9376); };
            c.w64(4, t200);
            let r201 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r201);
            blk = 70; continue;
        }
        134 => {
            return RegfileExit::fault(0x70008e9382);
        }
        135 => {
            return RegfileExit::fault(0x70008e9383);
        }
        136 => {
            return RegfileExit::fault(0x70008e9384);
        }
        137 => {
            return RegfileExit::fault(0x70008e9385);
        }
        138 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 70; continue;
        }
        139 => {
            c.w64(0, c.r64(7));
            blk = 70; continue;
        }
        140 => {
            return RegfileExit::jump(0x70008e939c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e93b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            return c.ret(0x0, 0x70008e93b4);
        }
        1 => {
            return RegfileExit::jump(0x70008e93b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e93d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(3, c.r64(0).wrapping_add(0x2));
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008e93da);
        }
        1 => {
            return RegfileExit::jump(0x70008e93dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e93f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(3, c.r64(0).wrapping_add(0x4));
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008e93fa);
        }
        1 => {
            return RegfileExit::jump(0x70008e93fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9414(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e9414); };
            c.w64(3, t0);
            let Some(t1) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e941b); };
            c.w64(1, t1);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e9422); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e9429); };
            c.cmp(t3, c.r64(6), 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008e942e); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xe90)) else { return RegfileExit::fault(0x70008e9435); };
            c.w64(1, t4);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008e943c); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008e943f); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008e9444); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008e944a);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e944b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e944f); }
            let r5 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r5);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008e945a); }
            return c.call(0x70007febb4, 0x70008e9463, 0x70008e945e);
        }
        4 => {
            c.w64(3, c.r64(0));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9466); };
            c.w64(0, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e946d); }
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008e9474);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e9494); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            c.w32(3, c.r64(0).wrapping_add(0x2));
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008e94a0);
        }
        1 => {
            return RegfileExit::jump(0x70008e94a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e94b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e94b4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e94ba); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e94c0); };
            c.w32(1, t4);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008e94c7);
        }
        2 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e94cb); };
            c.w32(0, t5);
            let r6 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r6);
            return c.ret(0x0, 0x70008e94d2);
        }
        3 => {
            return RegfileExit::jump(0x70008e94d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e94f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e94f4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e94fa); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9500); };
            c.w32(1, t4);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008e9507);
        }
        2 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e950b); };
            c.w32(0, t5);
            let r6 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r6);
            return c.ret(0x0, 0x70008e9512);
        }
        3 => {
            return RegfileExit::jump(0x70008e9514);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9534); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e953a); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9540); };
            c.w32(6, t4);
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008e9547);
        }
        2 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x70008e954b); }
            c.w64(6, 0xfff7fffffff7ffffu64);
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e955b); };
            c.cmp(t5, c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e9562); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e9569); };
            c.w64(1, t7);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e9570); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e9577); };
            c.cmp(t9, c.r64(6), 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008e957c); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e9583); };
            c.w64(1, t10);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008e958a); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e958d); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008e9595);
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e9596); };
            c.w64(0, t11);
            return c.ret(0x0, 0x70008e959d);
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e959e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e95a2); }
            let r12 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r12);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008e95ad); }
            return c.call(0x70007febf4, 0x70008e95b7, 0x70008e95b2);
        }
        8 => {
            c.w64(3, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008e95ba); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e95c2); }
            blk = 5; continue;
        }
        9 => {
            return RegfileExit::jump(0x70008e95c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e95d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e95d4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e95da); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e95e0); };
            c.w32(6, t4);
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_ae() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008e95ea); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008e95f0);
        }
        2 => {
            return RegfileExit::fault(0x70008e95f1);
        }
        3 => {
            return RegfileExit::jump(0x70008e95f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9614); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e961a); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9620); };
            c.w32(6, t4);
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008e9627);
        }
        2 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(2).wrapping_add(c.r64(3)), c.r32(1)) { return RegfileExit::fault(0x70008e9633); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r5 = c.r64(6) & c.r64(2);
            c.w64(6, r5);
            c.logic(r5, 64);
            let Some(t6) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9645); };
            c.logic(t6 & 0x4u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008e964d);
        }
        5 => {
            c.w32(1, c.r32(1));
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            c.w64(6, 0xfffffffffffc0000u64);
            let r8 = c.r64(6) & c.r64(1);
            c.w64(6, r8);
            c.logic(r8, 64);
            let Some(t9) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e965d); };
            c.logic(t9 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, c.r64(2).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e9667); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e9668); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f1594, 0x70008e9675, 0x70008e9670);
        }
        7 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e9675); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e9677); }
            blk = 4; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008e967c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9694); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e969a); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e96a0); }
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e96a5); };
            c.w32(1, t4);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_ae() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x70008e96b9); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008e96c1);
        }
        2 => {
            return RegfileExit::fault(0x70008e96c2);
        }
        3 => {
            return RegfileExit::jump(0x70008e96c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e96d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e96d4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e96da); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e96e0); };
            c.w32(1, t4);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008e96e7);
        }
        2 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e96eb); };
            c.w64(1, t5);
            if !c.st32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008e96f2); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008e96f8);
        }
        3 => {
            return RegfileExit::jump(0x70008e96fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9714); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e971a); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9720); };
            c.w32(1, t4);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008e9727);
        }
        2 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e972b); };
            c.w64(1, t5);
            if !c.st32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008e9732); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008e9738);
        }
        3 => {
            return RegfileExit::jump(0x70008e973c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9754); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e975a); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9760); };
            c.w32(1, t4);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008e9767);
        }
        2 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(1, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7), c.r64(1)) { return RegfileExit::fault(0x70008e9775); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008e977c);
        }
        3 => {
            return RegfileExit::jump(0x70008e9780);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e9794); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e9798); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e97a9); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 4; continue;
        }
        2 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(9, 0x1u64);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e97d0); };
            c.w32(9, t1);
            let r2 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r2);
            let Some(t3) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e97d7); };
            c.cmp(t3, 0x40u64, 16);
            if c.cc_ae() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008e97e3); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e97f0); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e97f6); };
            c.cmp(t6, 0x40u64, 16);
            if c.cc_ae() { blk = 20; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008e9801); }
            c.w64(2, c.r64(3));
            c.w64(0, c.r64(1));
            return c.call(0x70007fb9f4, 0x70008e9810, 0x70008e980b);
        }
        9 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008e9810); };
            c.cmp(t7, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9819); };
            c.w64(2, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e981d); };
            c.w64(0, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e9821); };
            c.w64(6, t10);
            return c.call(0x70007fb9f4, 0x70008e982a, 0x70008e9825);
        }
        11 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008e982a); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e9838); }
            return c.ret(0x0, 0x70008e9839);
        }
        14 => {
            c.w32(0, 0x2u64);
            blk = 13; continue;
        }
        15 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 13; continue;
        }
        16 => {
            c.w64(10, c.r64(4));
            let r12 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r12);
            let r13 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r13);
            c.logic(r13, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e9855); }
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(3));
            c.w64(7, c.r64(1));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1470)) else { return RegfileExit::fault(0x70008e9866); };
            c.w64(0, t14);
            c.w64(10, 0x70008e9874u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e9874); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e9878); }
            return c.call(c.r64(0), 0x70008e987e, 0x70008e987c);
        }
        17 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e987e); }
            let Some(t15) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e9886); };
            c.w64(4, t15);
            blk = 13; continue;
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008e988d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008e9891); }
            c.w64(0, c.r64(3));
            return c.call(0x700089c794, 0x70008e989d, 0x70008e9898);
        }
        19 => {
            c.w64(3, c.r64(0));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e98a0); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e98a4); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e98a8); };
            c.w64(7, t18);
            blk = 6; continue;
        }
        20 => {
            c.w64(0, c.r64(1));
            return c.call(0x700089c794, 0x70008e98b9, 0x70008e98b4);
        }
        21 => {
            c.w64(1, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e98bc); };
            c.w64(3, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e98c0); };
            c.w64(6, t20);
            blk = 8; continue;
        }
        22 => {
            return RegfileExit::jump(0x70008e98cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e98d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e98d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e98d8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e98de); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008e98df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e98e0); };
            c.w64(3, t1);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008e98e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008e98e8); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2134, 0x70008e98fc, 0x70008e98f7);
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e9904); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e9914); }
            return c.ret(0x0, 0x70008e9915);
        }
        6 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        7 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e991a); };
            c.w32(3, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e991d); };
            c.cmp(t3, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9926); };
            c.w64(6, t4);
            return c.call(0x70008a1014, 0x70008e992f, 0x70008e992a);
        }
        9 => {
            blk = 2; continue;
        }
        10 => {
            return RegfileExit::jump(0x70008e9934);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008e9954); };
            c.w64(0, t0);
            return c.ret(0x0, 0x70008e995b);
        }
        1 => {
            return RegfileExit::jump(0x70008e9960);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(6, c.r32(7));
            c.w32(8, c.r32(2));
            let r0 = c.add(c.r32(7), c.r32(3), 32);
            c.w32(7, r0);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            blk = 5; continue;
        }
        2 => {
            c.w32(6, c.r64(2).wrapping_add(0x2));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w32(8, c.r64(3).wrapping_add(0x2));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9994); };
            c.w32(3, t1);
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(2)), c.r32(3)) { return RegfileExit::fault(0x70008e99a0); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r2 = c.r64(9) & c.r64(1);
            c.w64(9, r2);
            c.logic(r2, 64);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e99b2); };
            c.logic(t3 & 0x4u64, 8);
            if c.cc_ne() { blk = 25; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(2, c.r64(6));
            c.w64(3, c.r64(8));
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(3), c.r32(7), 32);
            if c.cc_ge() { blk = 19; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e99cb); };
            c.w32(6, t4);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_ae() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e99d2); };
            c.w32(6, t5);
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 10; continue;
        }
        9 => {
            return RegfileExit::fault(0x70008e99db);
        }
        10 => {
            return RegfileExit::fault(0x70008e99dc);
        }
        11 => {
            c.w32(2, c.r64(6).wrapping_add(c.r64(8)).wrapping_sub(0x2));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(6, c.r64(7).wrapping_sub(0x2));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        12 => {
            c.w32(7, c.r64(2).wrapping_sub(0x2));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(8, c.r64(6).wrapping_sub(0x2));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9a00); };
            c.w32(6, t6);
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(2)), c.r32(6)) { return RegfileExit::fault(0x70008e9a0c); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r7 = c.r64(9) & c.r64(1);
            c.w64(9, r7);
            c.logic(r7, 64);
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9a1f); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(2, c.r64(7));
            c.w64(6, c.r64(8));
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_g() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9a30); };
            c.w32(7, t9);
            c.cmp(c.r32(2), c.r32(7), 32);
            if c.cc_ae() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9a37); };
            c.w32(7, t10);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_b() { blk = 12; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 21; continue;
        }
        19 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008e9a42);
        }
        20 => {
            return RegfileExit::fault(0x70008e9a43);
        }
        21 => {
            return RegfileExit::fault(0x70008e9a44);
        }
        22 => {
            c.w32(6, c.r32(6));
            let r11 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r11);
            c.w64(9, 0xfffffffffffc0000u64);
            let r12 = c.r64(9) & c.r64(6);
            c.w64(9, r12);
            c.logic(r12, 64);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9a54); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(11, c.r64(1).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e9a5f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e9a60); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70008e9a6d, 0x70008e9a68);
        }
        24 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e9a6d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e9a6f); }
            blk = 14; continue;
        }
        25 => {
            c.w32(3, c.r32(3));
            let r14 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r14);
            c.w64(9, 0xfffffffffffc0000u64);
            let r15 = c.r64(9) & c.r64(3);
            c.w64(9, r15);
            c.logic(r15, 64);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9a81); };
            c.logic(t16 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, c.r64(1).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e9a90); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e9a91); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70008e9a9e, 0x70008e9a99);
        }
        27 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e9a9e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008e9aa0); }
            blk = 4; continue;
        }
        28 => {
            return RegfileExit::jump(0x70008e9aa8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e9ab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e9ab4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e9ab8); }
            let r0 = c.sub(c.r64(4), 0x88u64, 64);
            c.w64(4, r0);
            c.w64(9, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008e9ac5); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e9acc); };
            c.w32(12, t3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008e9ad0); };
            c.w32(15, t4);
            let r5 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r5);
            let Some(t6) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9ad7); };
            c.w32(2, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008e9adb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008e9adf); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008e9ae3); }
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008e9aec);
        }
        2 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            let r7 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r7);
            let Some(t8) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9af4); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008e9afc); }
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_b() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x70008e9b05);
        }
        4 => {
            c.w32(7, c.r64(3).wrapping_add(0x2));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9b12); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008e9b1a); }
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_b() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008e9b22);
        }
        6 => {
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9b2e); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            c.w32(15, c.r64(7).wrapping_add(0x2));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(1)) { return RegfileExit::fault(0x70008e9b3a); }
            c.cmp(c.r32(15), c.r32(2), 32);
            if c.cc_b() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008e9b43);
        }
        8 => {
            c.w64(2, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(0, c.r32(8));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008e9b4a); }
            c.w64(0, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9b56); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r15);
            c.w32(1, c.r64(8).wrapping_add(c.r64(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008e9b65); }
            c.w32(9, c.r64(12).wrapping_sub(0x6));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008e9b6f); }
            c.cmp(c.r32(9), c.r32(3), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(9, c.r64(3).wrapping_add(0x4));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9b86); };
            c.w32(3, t16);
            c.cmp(c.r32(9), c.r32(3), 32);
            if c.cc_b() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008e9b8f);
        }
        11 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            let r17 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9b97); };
            c.w32(1, t18);
            c.cmp(c.r32(9), c.r32(3), 32);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x70008e9ba1);
        }
        13 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9ba5); };
            c.w32(9, t19);
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_b() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008e9bae);
        }
        15 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(6)), c.r32(1)) { return RegfileExit::fault(0x70008e9baf); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r20 = c.r64(3) & c.r64(11);
            c.w64(3, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9bc2); };
            c.logic(t21 & 0x4u64, 8);
            if c.cc_ne() { blk = 313; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9bcc); };
            c.w32(3, t22);
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_b() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x70008e9bd5);
        }
        19 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x70008e9bd6); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r23 = c.r64(15) & c.r64(11);
            c.w64(15, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9bea); };
            c.logic(t24 & 0x4u64, 8);
            if c.cc_ne() { blk = 310; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(9, c.r64(12).wrapping_sub(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t25) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e9bfd); };
            c.w32(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e9c01); };
            c.w64(0, t26);
            if !c.st32(c.r64(0).wrapping_add(0x2b), c.r32(9)) { return RegfileExit::fault(0x70008e9c05); }
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9c09); };
            c.w64(3, t27);
            let Some(t28) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9c0d); };
            c.w32(9, t28);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008e9c11); }
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e9c15); };
            c.w64(12, t29);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_b() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70008e9c1e);
        }
        23 => {
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t30) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9c22); };
            c.w32(1, t30);
            let r31 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r31);
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008e9c2d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008e9c2f); }
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e9c33); };
            c.w64(2, t32);
            c.w64(7, c.r64(8));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e9c3a); };
            c.w64(6, t33);
            return c.call(0x70008eb4d4, 0x70008e9c43, 0x70008e9c3e);
        }
        24 => {
            c.w32(9, c.r32(0));
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e9c46); };
            c.w64(11, t34);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e9c53); };
            c.w64(11, t35);
            let r36 = c.sub(c.r32(11), c.r32(9), 32);
            c.w32(11, r36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e9c5a); };
            c.w64(9, t37);
            let r38 = c.add(c.r32(9), c.r32(0), 32);
            c.w32(9, r38);
            c.w32(12, c.r64(11).wrapping_add(c.r64(9)).wrapping_sub(0x2));
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9c66); };
            c.w64(3, t39);
            let Some(t40) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9c6a); };
            c.w32(15, t40);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008e9c6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008e9c72); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008e9c76); }
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_b() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            return RegfileExit::fault(0x70008e9c7f);
        }
        27 => {
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e9c83); };
            c.w64(7, t41);
            c.w32(0, c.r64(7).wrapping_sub(0x2));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t42) = c.ld32(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9c8d); };
            c.w32(1, t42);
            let r43 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r43);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e9c95); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008e9c96); }
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e9c9a); };
            c.w64(0, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e9c9e); };
            c.w64(2, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e9ca2); };
            c.w64(6, t46);
            return c.call(0x70008eb174, 0x70008e9cab, 0x70008e9ca6);
        }
        28 => {
            c.w32(9, c.r32(0));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e9cae); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008e9cb2); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 258; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e9cc0); };
            c.w64(11, t47);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_le() { blk = 143; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(0), 0x40u64, 32);
            if c.cc_g() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(12, 0x40u64);
            blk = 33; continue;
        }
        32 => {
            c.w64(12, c.r64(0));
            blk = 33; continue;
        }
        33 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e9cdd); };
            c.w64(15, t48);
            let Some(t49) = c.ld32(c.r64(15).wrapping_add(0x37)) else { return RegfileExit::fault(0x70008e9ce1); };
            c.w32(3, t49);
            let r50 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r50);
            let Some(t51) = c.ld32(c.r64(15).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008e9ce8); };
            c.w32(2, t51);
            let r52 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r52);
            let Some(t53) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9cef); };
            c.w32(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008e9cf2); }
            c.cmp(c.r32(12), c.r32(1), 32);
            if c.cc_le() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(12));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 262; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e9d15); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e9d1c); };
            c.w64(1, t55);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(12, c.r64(8).wrapping_add(0x8));
            c.w64(6, c.r64(12).wrapping_add(c.r64(1)));
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e9d2b); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008e9d32); };
            c.cmp(t57, c.r64(6), 64);
            if c.cc_be() { blk = 260; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 260; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e9d48); };
            c.w64(12, t58);
            if !c.st64(c.r64(12), c.r64(6)) { return RegfileExit::fault(0x70008e9d4f); }
            c.w64(12, c.r64(1).wrapping_add(0x1));
            blk = 38; continue;
        }
        38 => {
            let r59 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r59);
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e9d59); };
            c.w64(1, t60);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008e9d60); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008e9d65); }
            c.w64(1, c.r64(12).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r61 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r61);
            let r62 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r62);
            c.logic(r62, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008e9d7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(12)) { return RegfileExit::fault(0x70008e9d7f); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008e9d88); };
            c.w64(0, t63);
            c.w64(10, 0x70008e9d96u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008e9d96); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008e9d9a); }
            return c.call(c.r64(0), 0x70008e9da0, 0x70008e9d9e);
        }
        39 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008e9da0); }
            let Some(t64) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008e9da8); };
            c.w64(4, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e9dad); };
            c.w64(0, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008e9db1); };
            c.w64(3, t66);
            if !c.st32(c.r64(0).wrapping_add(0x37), c.r32(3)) { return RegfileExit::fault(0x70008e9db8); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r67 = c.r64(9) & c.r64(0);
            c.w64(9, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9dca); };
            c.logic(t68 & 0x4u64, 8);
            if c.cc_ne() { blk = 307; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(15, c.r64(0));
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e9dd8); };
            c.w64(0, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9ddc); };
            c.w64(2, t70);
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(3)) { return RegfileExit::fault(0x70008e9de0); }
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(1, c.r64(3));
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e9df0); };
            c.w64(3, t71);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e9df6); };
            c.w64(6, t72);
            return c.call(0x70008e9974, 0x70008e9dff, 0x70008e9dfa);
        }
        43 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9dff); };
            c.w64(3, t73);
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9e03); };
            c.w32(9, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e9e07); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e9e0b); };
            c.w64(12, t76);
            c.w32(11, c.r64(11).wrapping_add(c.r64(12)).wrapping_sub(0x2));
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_b() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            return RegfileExit::fault(0x70008e9e19);
        }
        45 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e9e1a); };
            c.w64(12, t77);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_b() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            return RegfileExit::fault(0x70008e9e23);
        }
        47 => {
            c.w32(9, c.r64(11).wrapping_sub(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e9e2e); };
            c.w64(15, t78);
            let Some(t79) = c.ld32(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9e32); };
            c.w32(15, t79);
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(15)) { return RegfileExit::fault(0x70008e9e3f); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r80 = c.r64(0) & c.r64(3);
            c.w64(0, r80);
            c.logic(r80, 64);
            let Some(t81) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9e53); };
            c.logic(t81 & 0x4u64, 8);
            if c.cc_ne() { blk = 304; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e9e5d); };
            c.w64(11, t82);
            let r83 = c.add(c.r32(11), 0xfffffffffffffffeu64, 32);
            c.w32(11, r83);
            if c.cc_e() { blk = 139; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e9e6b); };
            c.w64(0, t84);
            c.w32(15, c.r64(0).wrapping_sub(0x2));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r85 = c.sub(c.r32(12), 0x2u64, 32);
            c.w32(12, r85);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e9e89); };
            c.w64(2, t86);
            let Some(t87) = c.ld32(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008e9e8d); };
            c.w32(1, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008e9e90); };
            c.w64(6, t88);
            blk = 53; continue;
        }
        52 => {
            c.w32(12, c.r64(1).wrapping_add(0x2));
            c.w64(1, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(3));
            if !c.st32(c.r64(0).wrapping_add(0x27), c.r32(1)) { return RegfileExit::fault(0x70008e9ea3); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(6));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9eac); };
            c.w64(3, t89);
            c.w64(6, c.r64(9));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e9eb3); };
            c.w64(12, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e9eb7); };
            c.w64(9, t91);
            blk = 53; continue;
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008e9ebb); }
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008e9ec1); }
            c.w64(8, c.r64(9));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008e9ec8); }
            blk = 55; continue;
        }
        54 => {
            let r92 = c.sub(c.r32(2), 0x2u64, 32);
            c.w32(2, r92);
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008e9ed4); }
            c.w64(6, c.r64(9));
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e9edb); };
            c.w64(2, t93);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008e9edf); }
            blk = 55; continue;
        }
        55 => {
            let Some(t94) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9ee3); };
            c.w32(9, t94);
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_ae() { blk = 112; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(9, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t95) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9ef3); };
            c.w32(11, t95);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(15)) { return RegfileExit::fault(0x70008e9ef7); }
            let Some(t96) = c.ld32(c.r64(6).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9efb); };
            c.w32(15, t96);
            let r97 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r97);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008e9f03); }
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ae() { blk = 113; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t98) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e9f13); };
            c.w32(9, t98);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x70008e9f17); }
            let Some(t99) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e9f1b); };
            c.w32(9, t99);
            let r100 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r100);
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(9)) { return RegfileExit::fault(0x70008e9f22); }
            let Some(t101) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9f29); };
            c.w32(9, t101);
            let r102 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r102);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e9f31); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008e9f35); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008e9f39); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x70008e9f3d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008e9f41); }
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008e9f45); };
            c.w64(0, t103);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(9));
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e9f52); };
            c.w64(6, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008e9f56); };
            c.w64(9, t105);
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e9f5a); };
            return c.call(t106, 0x70008e9f62, 0x70008e9f5a);
        }
        58 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e9f66); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 69; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 62; continue;
        }
        61 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 69; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e9f84); };
            c.w64(3, t107);
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9f88); };
            c.w32(9, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e9f8c); };
            c.w64(11, t109);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_ae() { blk = 114; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008e9f99); };
            c.w64(9, t110);
            let Some(t111) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e9fa0); };
            c.w32(12, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008e9fa4); };
            c.w64(15, t112);
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_ae() { blk = 115; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(12, c.r64(15).wrapping_sub(0x2));
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e9fc3); };
            c.w64(0, t113);
            let r114 = c.sub(c.r32(0), 0x2u64, 32);
            c.w32(0, r114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e9fca); };
            c.w64(2, t115);
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e9fce); };
            c.w32(2, t116);
            if !c.st32(c.r64(3).wrapping_add(c.r64(12)), c.r32(2)) { return RegfileExit::fault(0x70008e9fd3); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r117 = c.r64(1) & c.r64(3);
            c.w64(1, r117);
            c.logic(r117, 64);
            let Some(t118) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e9fe6); };
            c.logic(t118 & 0x4u64, 8);
            if c.cc_ne() { blk = 301; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e9ff9); };
            c.w64(7, t119);
            c.w32(12, c.r64(7).wrapping_add(0x2));
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea007); };
            c.w64(1, t120);
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_le() { blk = 76; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(2, c.r64(11));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea017); };
            c.w64(6, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea01b); };
            c.w64(12, t122);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 54; continue;
        }
        69 => {
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea027); };
            c.w64(3, t123);
            let Some(t124) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea02b); };
            c.w32(11, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea02f); };
            c.w64(9, t125);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ae() { blk = 117; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea03c); };
            c.w64(12, t126);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ae() { blk = 118; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t127) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea049); };
            c.w32(11, t127);
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ea058); };
            c.w64(0, t128);
            let Some(t129) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea05c); };
            c.w32(0, t129);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x70008ea060); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r130 = c.r64(2) & c.r64(3);
            c.w64(2, r130);
            c.logic(r130, 64);
            let Some(t131) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ea072); };
            c.logic(t131 & 0x4u64, 8);
            if c.cc_ne() { blk = 298; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(15, c.r64(9).wrapping_sub(0x2));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r132 = c.add(c.r32(11), 0xfffffffffffffffeu64, 32);
            c.w32(11, r132);
            if c.cc_e() { blk = 130; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ea08d); };
            c.w64(15, t133);
            let r134 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r134);
            c.w64(0, (c.r32(15) as u32 as i32 as i64 as u64));
            let r135 = c.sub(c.r32(12), 0x2u64, 32);
            c.w32(12, r135);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea0a2); };
            c.w64(1, t136);
            c.cmp(c.r32(1), c.r32(15), 32);
            if c.cc_le() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(6, c.r64(11));
            c.w64(11, c.r64(0));
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea0b1); };
            c.w64(0, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ea0b5); };
            c.w64(15, t138);
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ea0be); };
            c.w64(9, t139);
            blk = 54; continue;
        }
        76 => {
            c.w64(2, c.r64(11));
            c.w64(6, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea0d2); };
            c.w64(12, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea0d6); };
            c.w64(11, t141);
            blk = 78; continue;
        }
        77 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ea0dc); };
            c.w64(15, t142);
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ea0e5); };
            c.w64(9, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea0ec); };
            c.w64(6, t144);
            blk = 78; continue;
        }
        78 => {
            let r145 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r145);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let r146 = c.sub(c.r32(2), 0x2u64, 32);
            c.w32(2, r146);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(8, 0x1u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008ea102); }
            blk = 80; continue;
        }
        79 => {
            c.w32(9, c.r64(0).wrapping_sub(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t147) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ea112); };
            c.w64(0, (t147 as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008ea11c); };
            c.w64(1, t148);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008ea120); }
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ea124); };
            c.w64(9, t149);
            blk = 80; continue;
        }
        80 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008ea12b); }
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ea12f); };
            c.w64(3, t150);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_ge() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_ge() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w32(0, c.r64(1).wrapping_sub(0x2));
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_g() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(1, 0x2u64);
            blk = 86; continue;
        }
        85 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 86; continue;
        }
        86 => {
            if !c.st32(c.r64(3).wrapping_add(0x27), c.r32(1)) { return RegfileExit::fault(0x70008ea159); }
            let Some(t151) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea15c); };
            c.w32(0, t151);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(1)) { return RegfileExit::fault(0x70008ea160); }
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_ae() { blk = 119; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t152) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ea16d); };
            c.w64(0, (t152 as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(8, c.r32(11));
            c.w32(12, c.r64(11).wrapping_sub(0x2));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008ea17e); }
            let Some(t153) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea182); };
            c.w32(11, t153);
            let r154 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r154);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008ea18a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008ea18c); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(15)) { return RegfileExit::fault(0x70008ea190); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008ea194); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008ea198); }
            c.w64(12, c.r64(3));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea1a5); };
            c.w64(3, t155);
            c.w64(1, c.r64(11));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea1ac); };
            c.w64(7, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ea1b0); };
            c.w64(6, t157);
            return c.call(0x70008eb4d4, 0x70008ea1b9, 0x70008ea1b4);
        }
        88 => {
            c.w32(9, c.r32(0));
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ea1bc); };
            c.w64(11, t158);
            let r159 = c.sub(c.r32(11), c.r32(9), 32);
            c.w32(11, r159);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008ea1c3); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_g() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea1cd); };
            c.w64(12, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea1d1); };
            c.w64(9, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea1d5); };
            c.w64(15, t162);
            blk = 93; continue;
        }
        90 => {
            let Some(t163) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea1db); };
            c.w32(9, t163);
            let Some(t164) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea1df); };
            c.w32(12, t164);
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            let r165 = c.sub(c.r32(9), c.r32(11), 32);
            c.w32(9, r165);
            let r166 = c.sub(c.r32(12), c.r32(11), 32);
            c.w32(12, r166);
            c.w32(15, c.r64(9).wrapping_add(0x2));
            c.w32(0, c.r64(12).wrapping_add(0x2));
            c.w64(2, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008ea1fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008ea1ff); }
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea203); };
            c.w64(0, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ea207); };
            c.w64(6, t168);
            c.w64(1, c.r64(0));
            return c.call(0x70008e9974, 0x70008ea213, 0x70008ea20e);
        }
        91 => {
            let Some(t169) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ea213); };
            c.w64(9, (t169 as u32 as i32 as i64 as u64));
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea217); };
            c.w64(11, t170);
            let Some(t171) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ea21b); };
            c.cmp(c.r32(11), t171, 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ea225); };
            c.w64(11, t172);
            let Some(t173) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ea229); };
            let r174 = c.sub(c.r32(11), t173, 32);
            c.w32(11, r174);
            let Some(t175) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea22d); };
            c.w64(12, (t175 as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(9));
            c.w64(9, c.r64(11));
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ea23a); };
            c.w64(11, t176);
            blk = 93; continue;
        }
        93 => {
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea23e); };
            c.w64(1, t177);
            let Some(t178) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea242); };
            c.w32(0, t178);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008ea245); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008ea249); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008ea24d); }
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_ae() { blk = 120; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ea25a); };
            c.w64(3, t179);
            let Some(t180) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea261); };
            c.w32(0, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ea264); };
            c.w64(2, t181);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ae() { blk = 121; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea270); };
            c.w64(0, t182);
            let r183 = c.sub(c.r32(0), 0x2u64, 32);
            c.w32(0, r183);
            c.w64(6, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea27a); };
            c.w64(7, t184);
            let Some(t185) = c.ld32(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea27e); };
            c.w32(7, t185);
            c.w64(6, c.r64(6).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(7)) { return RegfileExit::fault(0x70008ea28a); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r186 = c.r64(8) & c.r64(1);
            c.w64(8, r186);
            c.logic(r186, 64);
            let Some(t187) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ea29d); };
            c.logic(t187 & 0x4u64, 8);
            if c.cc_ne() { blk = 295; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70008ea2a8); }
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t188) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea2b5); };
            c.w32(9, t188);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_ae() { blk = 122; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(6, c.r64(0).wrapping_sub(0x2));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t189) = c.ld32(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea2ce); };
            c.w32(8, t189);
            let r190 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r190);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ea2d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008ea2d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008ea2db); }
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ea2df); };
            c.w64(0, t191);
            c.w64(1, c.r64(8));
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ea2e8); };
            c.w64(6, t192);
            return c.call(0x70008eb174, 0x70008ea2f1, 0x70008ea2ec);
        }
        100 => {
            c.w32(9, c.r32(0));
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ea2f4); };
            c.w64(15, t193);
            c.w32(11, c.r64(15).wrapping_sub(0x2));
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ea2fc); };
            c.w64(12, t194);
            let r195 = c.sub(c.r32(12), c.r32(9), 32);
            c.w32(12, r195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea303); };
            c.w64(9, t196);
            let r197 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r197);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008ea30b); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_g() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea31b); };
            c.w64(6, t198);
            blk = 106; continue;
        }
        102 => {
            let r199 = c.sub(c.r32(9), c.r32(12), 32);
            c.w32(9, r199);
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            let r200 = c.sub(c.r32(11), c.r32(12), 32);
            c.w32(11, r200);
            let r201 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r201);
            c.w32(0, c.r64(11).wrapping_add(0x2));
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ea335); };
            c.w64(3, t202);
            let r203 = c.sub(c.r32(3), c.r32(12), 32);
            c.w32(3, r203);
            c.w64(2, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008ea342); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008ea346); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008ea34a); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ea34e); };
            c.w64(0, t204);
            c.w64(3, c.r64(9));
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea358); };
            c.w64(1, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ea35c); };
            c.w64(6, t206);
            return c.call(0x70008e9974, 0x70008ea365, 0x70008ea360);
        }
        103 => {
            let Some(t207) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea365); };
            c.cmp(t207, 0x2u64, 32);
            if c.cc_e() { blk = 127; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t208) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea36f); };
            c.w64(0, (t208 as u32 as i32 as i64 as u64));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ea373); };
            c.w64(9, t209);
            let Some(t210) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ea377); };
            c.cmp(c.r32(9), t210, 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t211) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea381); };
            c.w64(15, (t211 as u32 as i32 as i64 as u64));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ea385); };
            c.w64(12, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea389); };
            c.w64(9, t213);
            c.w64(6, c.r64(0));
            blk = 106; continue;
        }
        106 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea390); };
            c.w64(3, t214);
            let Some(t215) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea394); };
            c.w32(11, t215);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ae() { blk = 123; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea39d); };
            c.w64(0, t216);
            c.cmp(c.r32(0), c.r32(11), 32);
            if c.cc_ae() { blk = 124; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ea3a9); };
            c.w64(2, t217);
            let Some(t218) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea3ad); };
            c.w32(2, t218);
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(2)) { return RegfileExit::fault(0x70008ea3b9); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r219 = c.r64(1) & c.r64(3);
            c.w64(1, r219);
            c.logic(r219, 64);
            let Some(t220) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ea3cc); };
            c.logic(t220 & 0x4u64, 8);
            if c.cc_ne() { blk = 292; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t221) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea3d6); };
            c.w32(11, t221);
            let r222 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r222);
            c.w64(2, (c.r32(9) as u32 as i32 as i64 as u64));
            let r223 = c.add(c.r32(11), 0xfffffffffffffffeu64, 32);
            c.w32(11, r223);
            if c.cc_e() { blk = 125; continue; }
            blk = 111; continue;
        }
        111 => {
            blk = 79; continue;
        }
        112 => {
            return RegfileExit::fault(0x70008ea3ec);
        }
        113 => {
            return RegfileExit::fault(0x70008ea3ed);
        }
        114 => {
            return RegfileExit::fault(0x70008ea3ee);
        }
        115 => {
            return RegfileExit::fault(0x70008ea3ef);
        }
        116 => {
            let r224 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r224);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(11));
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea3fa); };
            c.w64(11, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea3fe); };
            c.w64(12, t226);
            blk = 131; continue;
        }
        117 => {
            return RegfileExit::fault(0x70008ea404);
        }
        118 => {
            return RegfileExit::fault(0x70008ea405);
        }
        119 => {
            return RegfileExit::fault(0x70008ea406);
        }
        120 => {
            return RegfileExit::fault(0x70008ea407);
        }
        121 => {
            return RegfileExit::fault(0x70008ea408);
        }
        122 => {
            return RegfileExit::fault(0x70008ea409);
        }
        123 => {
            return RegfileExit::fault(0x70008ea40a);
        }
        124 => {
            return RegfileExit::fault(0x70008ea40b);
        }
        125 => {
            c.w64(9, c.r64(2));
            c.w64(0, c.r64(6));
            blk = 140; continue;
        }
        126 => {
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea417); };
            c.w64(9, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea41b); };
            c.w64(3, t228);
            blk = 140; continue;
        }
        127 => {
            let Some(t229) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea424); };
            c.w64(15, (t229 as u32 as i32 as i64 as u64));
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea428); };
            c.w64(3, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea42c); };
            c.w64(12, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea430); };
            c.w64(9, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea434); };
            c.w64(11, t233);
            blk = 131; continue;
        }
        128 => {
            c.w32(9, c.r64(2).wrapping_sub(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(11, c.r64(15).wrapping_sub(0x2));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(9));
            c.w64(3, c.r64(1));
            c.w64(9, c.r64(11));
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea451); };
            c.w64(11, t234);
            blk = 131; continue;
        }
        129 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea457); };
            c.w64(3, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea45b); };
            c.w64(0, t236);
            blk = 140; continue;
        }
        130 => {
            c.w64(9, c.r64(15));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea467); };
            c.w64(0, t237);
            blk = 140; continue;
        }
        131 => {
            c.w32(0, c.r32(11));
            c.w32(9, c.r32(9));
            c.w32(12, c.r32(12));
            let r238 = c.sub(c.r32(9), c.r32(0), 32);
            c.w32(9, r238);
            let r239 = c.sub(c.r32(12), c.r32(0), 32);
            c.w32(12, r239);
            c.w32(0, c.r64(9).wrapping_add(0x2));
            let r240 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r240);
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(15)) { return RegfileExit::fault(0x70008ea48d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008ea491); }
            c.w64(0, c.r64(3));
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(12));
            c.w64(7, c.r64(11));
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ea4a1); };
            c.w64(6, t241);
            return c.call(0x70008e9974, 0x70008ea4aa, 0x70008ea4a5);
        }
        132 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea4aa); };
            c.w64(1, t242);
            let Some(t243) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea4ae); };
            c.w32(9, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea4b2); };
            c.w64(11, t244);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_b() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            return RegfileExit::fault(0x70008ea4bb);
        }
        134 => {
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ea4bc); };
            c.w64(12, t245);
            let Some(t246) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea4c3); };
            c.w32(9, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ea4c8); };
            c.w64(15, t247);
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_b() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            return RegfileExit::fault(0x70008ea4d1);
        }
        136 => {
            c.w64(9, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t248) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea4d8); };
            c.w32(9, t248);
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(11)), c.r32(9)) { return RegfileExit::fault(0x70008ea4e5); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r249 = c.r64(12) & c.r64(1);
            c.w64(12, r249);
            c.logic(r249, 64);
            let Some(t250) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ea4f9); };
            c.logic(t250 & 0x4u64, 8);
            if c.cc_ne() { blk = 289; continue; }
            blk = 138; continue;
        }
        138 => {
            blk = 256; continue;
        }
        139 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea50a); };
            c.w64(0, t251);
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_le() { blk = 256; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w32(11, c.r64(0).wrapping_sub(0x2));
            c.w32(9, c.r32(9));
            let r252 = c.sub(c.r32(9), c.r32(11), 32);
            c.w32(9, r252);
            c.w64(2, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(0));
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ea527); };
            c.w64(0, t253);
            c.w64(1, c.r64(3));
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ea533); };
            c.w64(6, t254);
            return c.call(0x70008e9974, 0x70008ea53c, 0x70008ea537);
        }
        142 => {
            blk = 256; continue;
        }
        143 => {
            c.cmp(c.r32(11), 0x40u64, 32);
            if c.cc_g() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w32(12, 0x40u64);
            blk = 146; continue;
        }
        145 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 146; continue;
        }
        146 => {
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ea552); };
            c.w64(15, t255);
            let Some(t256) = c.ld32(c.r64(15).wrapping_add(0x37)) else { return RegfileExit::fault(0x70008ea556); };
            c.w32(0, t256);
            let r257 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r257);
            let Some(t258) = c.ld32(c.r64(15).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008ea55d); };
            c.w32(3, t258);
            let r259 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r259);
            let Some(t260) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea564); };
            c.w32(2, t260);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ea567); }
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_le() { blk = 155; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(12));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 266; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ea589); };
            c.w64(2, t261);
            let Some(t262) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008ea590); };
            c.w64(2, t262);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(12, c.r64(8).wrapping_add(0x8));
            c.w64(1, c.r64(12).wrapping_add(c.r64(2)));
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ea59f); };
            c.w64(6, t263);
            let Some(t264) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008ea5a6); };
            c.cmp(t264, c.r64(1), 64);
            if c.cc_be() { blk = 264; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 264; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ea5bc); };
            c.w64(12, t265);
            if !c.st64(c.r64(12), c.r64(1)) { return RegfileExit::fault(0x70008ea5c3); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            blk = 151; continue;
        }
        151 => {
            let r266 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r266);
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008ea5cd); };
            c.w64(2, t267);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008ea5d4); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008ea5d9); }
            c.w64(1, c.r64(12).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r268 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r268);
            let r269 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r269);
            c.logic(r269, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ea5ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x70008ea5f3); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t270) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008ea5fc); };
            c.w64(0, t270);
            c.w64(10, 0x70008ea60au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ea60a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ea60e); }
            return c.call(c.r64(0), 0x70008ea614, 0x70008ea612);
        }
        152 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ea614); }
            let Some(t271) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ea61c); };
            c.w64(4, t271);
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ea621); };
            c.w64(9, t272);
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ea625); };
            c.w64(0, t273);
            if !c.st32(c.r64(9).wrapping_add(0x37), c.r32(0)) { return RegfileExit::fault(0x70008ea62c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r274 = c.r64(11) & c.r64(9);
            c.w64(11, r274);
            c.logic(r274, 64);
            let Some(t275) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ea63e); };
            c.logic(t275 & 0x4u64, 8);
            if c.cc_ne() { blk = 286; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w64(15, c.r64(9));
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea64c); };
            c.w64(3, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea650); };
            c.w64(11, t277);
            blk = 155; continue;
        }
        155 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t278) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ea657); };
            c.w64(12, (t278 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(0)) { return RegfileExit::fault(0x70008ea65b); }
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(12));
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ea66d); };
            c.w64(6, t279);
            return c.call(0x70008e9974, 0x70008ea676, 0x70008ea671);
        }
        156 => {
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea676); };
            c.w64(1, t280);
            let Some(t281) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea67a); };
            c.w32(9, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ea67e); };
            c.w64(11, t282);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_b() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            return RegfileExit::fault(0x70008ea687);
        }
        158 => {
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ea688); };
            c.w64(12, t283);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_b() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            return RegfileExit::fault(0x70008ea691);
        }
        160 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea695); };
            c.w64(15, t284);
            let Some(t285) = c.ld32(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea699); };
            c.w32(15, t285);
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x70008ea6a6); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r286 = c.r64(0) & c.r64(1);
            c.w64(0, r286);
            c.logic(r286, 64);
            let Some(t287) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ea6ba); };
            c.logic(t287 & 0x4u64, 8);
            if c.cc_ne() { blk = 283; continue; }
            blk = 162; continue;
        }
        162 => {
            c.w32(9, c.r64(11).wrapping_add(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea6cb); };
            c.w64(11, t288);
            let r289 = c.add(c.r32(11), 0xfffffffffffffffeu64, 32);
            c.w32(11, r289);
            if c.cc_e() { blk = 253; continue; }
            blk = 163; continue;
        }
        163 => {
            let r290 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r290);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t291) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea6e3); };
            c.cmp(t291, 0x2u64, 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t292) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea6ed); };
            c.w64(15, (t292 as u32 as i32 as i64 as u64));
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ea6f1); };
            c.w64(0, t293);
            let Some(t294) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ea6f5); };
            c.w32(3, t294);
            c.w64(2, c.r64(9));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 166; continue;
        }
        165 => {
            c.w32(9, c.r64(12).wrapping_add(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(3));
            if !c.st32(c.r64(12).wrapping_add(0x27), c.r32(9)) { return RegfileExit::fault(0x70008ea70b); }
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(15));
            c.w64(15, c.r64(11));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(6));
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ea71f); };
            c.w64(11, t295);
            blk = 166; continue;
        }
        166 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ea723); };
            c.w64(7, t296);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008ea72a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008ea72e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008ea732); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(2));
            blk = 168; continue;
        }
        167 => {
            let r297 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r297);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(3));
            c.w64(3, c.r64(12));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008ea749); }
            c.w64(2, c.r64(0));
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ea750); };
            c.w64(0, t298);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008ea754); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(6));
            blk = 168; continue;
        }
        168 => {
            let Some(t299) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea75e); };
            c.w32(11, t299);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008ea764); }
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ae() { blk = 225; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008ea774); }
            let Some(t300) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea778); };
            c.w32(12, t300);
            let Some(t301) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea77c); };
            c.w32(15, t301);
            let r302 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r302);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008ea784); }
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_ae() { blk = 226; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008ea794); }
            let Some(t303) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ea798); };
            c.w32(9, t303);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008ea79c); }
            let Some(t304) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ea7a0); };
            c.w32(9, t304);
            let r305 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r305);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008ea7a7); }
            let Some(t306) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea7ab); };
            c.w32(9, t306);
            let r307 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r307);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x70008ea7b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008ea7b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008ea7bb); }
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ea7bf); };
            c.w64(0, t308);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(9));
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ea7c9); };
            c.w64(6, t309);
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ea7cd); };
            c.w64(9, t310);
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ea7d1); };
            return c.call(t311, 0x70008ea7d9, 0x70008ea7d1);
        }
        171 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 174; continue; }
            blk = 172; continue;
        }
        172 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ea7dd); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 182; continue; }
            blk = 173; continue;
        }
        173 => {
            blk = 175; continue;
        }
        174 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 182; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea7fb); };
            c.w64(1, t312);
            let Some(t313) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea7ff); };
            c.w32(9, t313);
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea803); };
            c.w64(2, t314);
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_ae() { blk = 227; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ea810); };
            c.w64(3, t315);
            let Some(t316) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea817); };
            c.w32(9, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea81b); };
            c.w64(11, t317);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_ae() { blk = 228; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea828); };
            c.w64(15, t318);
            c.w32(9, c.r64(15).wrapping_sub(0x2));
            let r319 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r319);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ea842); };
            c.w64(15, t320);
            let Some(t321) = c.ld32(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea846); };
            c.w32(15, t321);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x70008ea84b); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r322 = c.r64(0) & c.r64(1);
            c.w64(0, r322);
            c.logic(r322, 64);
            let Some(t323) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ea85f); };
            c.logic(t323 & 0x4u64, 8);
            if c.cc_ne() { blk = 280; continue; }
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ea873); };
            c.w64(8, t324);
            c.w32(12, c.r64(8).wrapping_add(0x2));
            c.w64(8, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea881); };
            c.w64(0, t325);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_le() { blk = 189; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(12, c.r64(0));
            c.w64(15, c.r64(9));
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea894); };
            c.w64(6, t326);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ea89a); };
            c.w64(9, t327);
            blk = 167; continue;
        }
        182 => {
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ea8a3); };
            c.w64(12, t328);
            let Some(t329) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea8a7); };
            c.w32(9, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea8ac); };
            c.w64(2, t330);
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_ae() { blk = 230; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea8b9); };
            c.w64(15, t331);
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_ae() { blk = 231; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(9, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ea8d1); };
            c.w64(11, t332);
            let Some(t333) = c.ld32(c.r64(12).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea8d5); };
            c.w32(11, t333);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x70008ea8da); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r334 = c.r64(0) & c.r64(12);
            c.w64(0, r334);
            c.logic(r334, 64);
            let Some(t335) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ea8ee); };
            c.logic(t335 & 0x4u64, 8);
            if c.cc_ne() { blk = 277; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t336) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ea8f8); };
            c.w32(9, t336);
            c.w32(11, c.r64(2).wrapping_add(0x2));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r337 = c.add(c.r32(9), 0xfffffffffffffffeu64, 32);
            c.w32(9, r337);
            if c.cc_e() { blk = 243; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008ea90d); };
            c.w64(11, t338);
            let r339 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r339);
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r340 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r340);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ea922); };
            c.w64(3, t341);
            c.cmp(c.r32(3), c.r32(11), 32);
            if c.cc_le() { blk = 190; continue; }
            blk = 188; continue;
        }
        188 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(15));
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea931); };
            c.w64(15, t342);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea935); };
            c.w64(11, t343);
            c.w64(1, c.r64(12));
            c.w64(12, c.r64(3));
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ea93f); };
            c.w64(3, t344);
            blk = 167; continue;
        }
        189 => {
            c.w64(12, c.r64(0));
            c.w64(6, c.r64(11));
            c.w64(11, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ea956); };
            c.w64(9, t345);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ea95a); };
            c.w64(15, t346);
            blk = 191; continue;
        }
        190 => {
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ea960); };
            c.w64(6, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ea964); };
            c.w64(11, t348);
            c.w64(1, c.r64(12));
            c.w64(12, c.r64(3));
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ea971); };
            c.w64(3, t349);
            blk = 191; continue;
        }
        191 => {
            let r350 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r350);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r351 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r351);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(7, 0x1u64);
            blk = 193; continue;
        }
        192 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ea991); };
            c.w64(8, t352);
            c.w64(6, c.r64(12));
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ea998); };
            c.w64(12, t353);
            blk = 193; continue;
        }
        193 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008ea99c); }
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ea9a0); };
            c.w64(3, t354);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_ge() { blk = 196; continue; }
            blk = 194; continue;
        }
        194 => {
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_ge() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 196; continue;
        }
        196 => {
            let r355 = c.sub(c.r32(12), 0x2u64, 32);
            c.w32(12, r355);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_g() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            c.w32(12, 0x2u64);
            blk = 199; continue;
        }
        198 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 199; continue;
        }
        199 => {
            if !c.st32(c.r64(3).wrapping_add(0x27), c.r32(12)) { return RegfileExit::fault(0x70008ea9cd); }
            let Some(t356) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ea9d1); };
            c.w32(0, t356);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(12)) { return RegfileExit::fault(0x70008ea9d4); }
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_ae() { blk = 232; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w64(0, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t357) = c.ld32(c.r64(1).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ea9e4); };
            c.w32(7, t357);
            let r358 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r358);
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ea9ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008ea9f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008ea9f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(2)) { return RegfileExit::fault(0x70008ea9f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70008ea9fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008eaa00); }
            c.w64(0, c.r64(3));
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008eaa07); };
            c.w64(3, t359);
            c.w64(1, c.r64(7));
            c.w64(2, c.r64(6));
            c.w64(7, c.r64(11));
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eaa17); };
            c.w64(6, t360);
            return c.call(0x70008eb4d4, 0x70008eaa20, 0x70008eaa1b);
        }
        201 => {
            c.w32(9, c.r32(0));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(0)) { return RegfileExit::fault(0x70008eaa23); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_g() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eaa2c); };
            c.w64(15, t361);
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008eaa30); };
            c.w64(11, t362);
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eaa34); };
            c.w64(9, t363);
            c.w64(8, c.r64(0));
            blk = 207; continue;
        }
        203 => {
            let Some(t364) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eaa3d); };
            c.w32(11, t364);
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eaa41); };
            c.w64(3, t365);
            c.w32(12, c.r64(3).wrapping_add(c.r64(0)));
            let r366 = c.sub(c.r32(11), c.r32(9), 32);
            c.w32(11, r366);
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008eaa4f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008eaa53); }
            c.w64(7, c.r64(0));
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008eaa5a); };
            c.w64(0, t367);
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eaa61); };
            c.w64(1, t368);
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008eaa65); };
            c.w64(2, t369);
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eaa69); };
            c.w64(6, t370);
            return c.call(0x70008e9974, 0x70008eaa72, 0x70008eaa6d);
        }
        204 => {
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008eaa72); };
            c.w64(2, t371);
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008eaa76); };
            c.w64(8, t372);
            c.w32(9, c.r64(2).wrapping_add(c.r64(8)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t373) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eaa81); };
            c.cmp(t373, 0x2u64, 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t374) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eaa8b); };
            c.w64(15, (t374 as u32 as i32 as i64 as u64));
            let Some(t375) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eaa8f); };
            c.cmp(c.r32(8), t375, 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(11, c.r64(9));
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eaa9c); };
            c.w64(9, t376);
            blk = 207; continue;
        }
        207 => {
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eaaa0); };
            c.w64(3, t377);
            let Some(t378) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eaaa4); };
            c.w32(12, t378);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008eaaa8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008eaaac); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008eaab0); }
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ae() { blk = 233; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eaabd); };
            c.w64(0, t379);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_ae() { blk = 234; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008eaacd); };
            c.w64(2, t380);
            let Some(t381) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eaad1); };
            c.w32(2, t381);
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            let Some(t382) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eaadd); };
            c.w32(1, t382);
            if !c.st32(c.r64(3).wrapping_add(c.r64(12)), c.r32(2)) { return RegfileExit::fault(0x70008eaae0); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r383 = c.r64(6) & c.r64(3);
            c.w64(6, r383);
            c.logic(r383, 64);
            let Some(t384) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eaaf3); };
            c.logic(t384 & 0x4u64, 8);
            if c.cc_ne() { blk = 274; continue; }
            blk = 211; continue;
        }
        211 => {
            let r385 = c.add(c.r32(1), 0xfffffffffffffffeu64, 32);
            c.w32(1, r385);
            if c.cc_e() { blk = 240; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008eab06); };
            c.w64(12, t386);
            let Some(t387) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eab0d); };
            c.w32(2, t387);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008eab12); }
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_ae() { blk = 235; continue; }
            blk = 213; continue;
        }
        213 => {
            c.w64(2, (c.r32(9) as u32 as i32 as i64 as u64));
            let r388 = c.add(c.r32(0), 0x2u64, 32);
            c.w32(0, r388);
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(6, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t389) = c.ld32(c.r64(12).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eab2b); };
            c.w32(9, t389);
            let r390 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r390);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008eab33); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008eab3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008eab3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008eab40); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008eab44); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008eab48); }
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eab4c); };
            c.w64(0, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008eab50); };
            c.w64(1, t392);
            c.w64(2, c.r64(6));
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eab57); };
            c.w64(6, t393);
            return c.call(0x70008eb174, 0x70008eab60, 0x70008eab5b);
        }
        214 => {
            c.w32(9, c.r32(0));
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008eab63); };
            c.w64(11, t394);
            let r395 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r395);
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(0)) { return RegfileExit::fault(0x70008eab6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008eab72); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_g() { blk = 216; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eab7b); };
            c.w64(15, t396);
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eab7f); };
            c.w64(9, t397);
            blk = 219; continue;
        }
        216 => {
            let r398 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r398);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008eab8b); }
            c.w64(7, c.r64(0));
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eab92); };
            c.w64(0, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eab96); };
            c.w64(3, t400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eab9a); };
            c.w64(6, t401);
            c.w64(1, c.r64(0));
            return c.call(0x70008e9974, 0x70008eaba6, 0x70008eaba1);
        }
        217 => {
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eaba6); };
            c.w64(9, t402);
            let Some(t403) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008eabaa); };
            c.cmp(c.r32(9), t403, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t404) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008eabb4); };
            let r405 = c.sub(c.r32(9), t404, 32);
            c.w32(9, r405);
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eabbb); };
            c.w64(9, t406);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008eabbf); };
            c.w64(0, t407);
            let r408 = c.add(c.r32(9), c.r32(0), 32);
            c.w32(9, r408);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008eabc9); };
            c.w64(2, t409);
            c.w64(9, c.r64(11));
            c.w64(15, c.r64(12));
            blk = 219; continue;
        }
        219 => {
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eabd3); };
            c.w64(1, t410);
            let Some(t411) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eabd7); };
            c.w32(11, t411);
            c.cmp(c.r32(2), c.r32(11), 32);
            if c.cc_ae() { blk = 236; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008eabe4); };
            c.w64(6, t412);
            let Some(t413) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eabeb); };
            c.w32(11, t413);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eabef); };
            c.w64(12, t414);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ae() { blk = 237; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eabfc); };
            c.w64(3, t415);
            c.w32(11, c.r64(3).wrapping_sub(0x2));
            let r416 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r416);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008eac0e); };
            c.w64(7, t417);
            let Some(t418) = c.ld32(c.r64(6).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eac12); };
            c.w32(7, t418);
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3)), c.r32(7)) { return RegfileExit::fault(0x70008eac1e); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r419 = c.r64(8) & c.r64(1);
            c.w64(8, r419);
            c.logic(r419, 64);
            let Some(t420) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eac31); };
            c.logic(t420 & 0x4u64, 8);
            if c.cc_ne() { blk = 271; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w32(3, c.r64(2).wrapping_add(0x2));
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 224; continue;
        }
        224 => {
            blk = 192; continue;
        }
        225 => {
            return RegfileExit::fault(0x70008eac4d);
        }
        226 => {
            return RegfileExit::fault(0x70008eac4e);
        }
        227 => {
            return RegfileExit::fault(0x70008eac4f);
        }
        228 => {
            return RegfileExit::fault(0x70008eac50);
        }
        229 => {
            c.w32(9, c.r64(2).wrapping_add(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eac61); };
            c.w64(15, t421);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eac65); };
            c.w64(12, t422);
            blk = 245; continue;
        }
        230 => {
            return RegfileExit::fault(0x70008eac6e);
        }
        231 => {
            return RegfileExit::fault(0x70008eac6f);
        }
        232 => {
            return RegfileExit::fault(0x70008eac70);
        }
        233 => {
            return RegfileExit::fault(0x70008eac71);
        }
        234 => {
            return RegfileExit::fault(0x70008eac72);
        }
        235 => {
            return RegfileExit::fault(0x70008eac73);
        }
        236 => {
            return RegfileExit::fault(0x70008eac74);
        }
        237 => {
            return RegfileExit::fault(0x70008eac75);
        }
        238 => {
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(15));
            c.w64(15, c.r64(10));
            c.w64(11, c.r64(2));
            blk = 245; continue;
        }
        239 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008eac8d); };
            c.w64(0, t423);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eac94); };
            c.w64(15, t424);
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008eac98); };
            c.w64(11, t425);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eac9c); };
            c.w64(1, t426);
            blk = 254; continue;
        }
        240 => {
            let r427 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r427);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008eacac); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008eacb0); }
            c.w64(1, c.r64(3));
            blk = 254; continue;
        }
        241 => {
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eacbc); };
            c.w64(1, t428);
            c.w64(11, c.r64(9));
            blk = 254; continue;
        }
        242 => {
            c.w64(11, c.r64(9));
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eaccb); };
            c.w64(9, t429);
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eaccf); };
            c.w64(15, t430);
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eacd3); };
            c.w64(12, t431);
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eacd7); };
            c.w64(1, t432);
            blk = 245; continue;
        }
        243 => {
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eacdd); };
            c.w64(15, t433);
            c.w64(1, c.r64(12));
            blk = 254; continue;
        }
        244 => {
            c.w64(15, c.r64(11));
            c.w64(11, c.r64(9));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 245; continue;
        }
        245 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008eacf2); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008eacf6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008eacfa); }
            c.w64(0, c.r64(1));
            c.w64(3, c.r64(12));
            c.w64(2, c.r64(11));
            c.w64(7, c.r64(15));
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ead0a); };
            c.w64(6, t434);
            return c.call(0x70008e9974, 0x70008ead13, 0x70008ead0e);
        }
        246 => {
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ead13); };
            c.w64(9, t435);
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ead17); };
            c.w64(11, t436);
            let r437 = c.add(c.r32(9), c.r32(11), 32);
            c.w32(9, r437);
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ead1e); };
            c.w64(1, t438);
            let Some(t439) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ead22); };
            c.w32(11, t439);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_b() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            return RegfileExit::fault(0x70008ead2b);
        }
        248 => {
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ead2c); };
            c.w64(0, t440);
            let Some(t441) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ead33); };
            c.w32(11, t441);
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ead37); };
            c.w64(12, t442);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_b() { blk = 250; continue; }
            blk = 249; continue;
        }
        249 => {
            return RegfileExit::fault(0x70008ead40);
        }
        250 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t443) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ead47); };
            c.w32(11, t443);
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x70008ead54); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 252; continue; }
            blk = 251; continue;
        }
        251 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r444 = c.r64(12) & c.r64(1);
            c.w64(12, r444);
            c.logic(r444, 64);
            let Some(t445) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ead68); };
            c.logic(t445 & 0x4u64, 8);
            if c.cc_ne() { blk = 268; continue; }
            blk = 252; continue;
        }
        252 => {
            blk = 256; continue;
        }
        253 => {
            let Some(t446) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ead76); };
            c.w64(15, (t446 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), 0x0u64) { return RegfileExit::fault(0x70008ead7a); }
            c.w64(11, c.r64(9));
            blk = 254; continue;
        }
        254 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_le() { blk = 256; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ead8b); };
            c.w64(0, t447);
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ead92); };
            c.w64(3, t448);
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ead96); };
            c.w64(6, t449);
            c.w64(2, c.r64(11));
            c.w64(7, c.r64(15));
            return c.call(0x70008e9974, 0x70008eada5, 0x70008eada0);
        }
        256 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 257; continue;
        }
        257 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008eadaa); }
            return c.ret(0x0, 0x70008eadab);
        }
        258 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 257; continue;
        }
        259 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 257; continue;
        }
        260 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008eadb4); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(3)) { return RegfileExit::fault(0x70008eadb8); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008eadc4, 0x70008eadbf);
        }
        261 => {
            c.w64(12, c.r64(0));
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eadc7); };
            c.w64(8, t450);
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008eadcb); };
            c.w64(3, t451);
            blk = 38; continue;
        }
        262 => {
            let Some(t452) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008eadd4); };
            c.w64(3, t452);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008eade4, 0x70008eaddf);
        }
        263 => {
            return RegfileExit::fault(0x70008eade4);
        }
        264 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008eade5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008eade9); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008eadf5, 0x70008eadf0);
        }
        265 => {
            c.w64(12, c.r64(0));
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008eadf8); };
            c.w64(8, t453);
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eadfc); };
            c.w64(0, t454);
            blk = 151; continue;
        }
        266 => {
            let Some(t455) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008eae05); };
            c.w64(3, t455);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008eae15, 0x70008eae10);
        }
        267 => {
            return RegfileExit::fault(0x70008eae15);
        }
        268 => {
            c.w32(11, c.r32(11));
            let r456 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r456);
            c.w64(12, 0xfffffffffffc0000u64);
            let r457 = c.r64(12) & c.r64(11);
            c.w64(12, r457);
            c.logic(r457, 64);
            let Some(t458) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eae26); };
            c.logic(t458 & 0x2u64, 8);
            if c.cc_e() { blk = 252; continue; }
            blk = 269; continue;
        }
        269 => {
            c.w64(15, c.r64(1).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eae36); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eae37); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008eae44, 0x70008eae3f);
        }
        270 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eae44); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eae46); }
            blk = 252; continue;
        }
        271 => {
            c.w32(7, c.r32(7));
            let r459 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r459);
            c.w64(8, 0xfffffffffffc0000u64);
            let r460 = c.r64(8) & c.r64(7);
            c.w64(8, r460);
            c.logic(r460, 64);
            let Some(t461) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eae5b); };
            c.logic(t461 & 0x2u64, 8);
            if c.cc_e() { blk = 223; continue; }
            blk = 272; continue;
        }
        272 => {
            c.w64(6, c.r64(1).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eae6a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eae6b); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008eae78, 0x70008eae73);
        }
        273 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eae78); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eae7a); }
            blk = 223; continue;
        }
        274 => {
            c.w32(2, c.r32(2));
            let r462 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r462);
            c.w64(6, 0xfffffffffffc0000u64);
            let r463 = c.r64(6) & c.r64(2);
            c.w64(6, r463);
            c.logic(r463, 64);
            let Some(t464) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eae8f); };
            c.logic(t464 & 0x2u64, 8);
            if c.cc_e() { blk = 211; continue; }
            blk = 275; continue;
        }
        275 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eae9d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eae9e); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x70008eaeab, 0x70008eaea6);
        }
        276 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eaeab); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eaead); }
            blk = 211; continue;
        }
        277 => {
            c.w32(11, c.r32(11));
            let r465 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r465);
            c.w64(0, 0xfffffffffffc0000u64);
            let r466 = c.r64(0) & c.r64(11);
            c.w64(0, r466);
            c.logic(r466, 64);
            let Some(t467) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eaec3); };
            c.logic(t467 & 0x2u64, 8);
            if c.cc_e() { blk = 186; continue; }
            blk = 278; continue;
        }
        278 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eaed1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eaed2); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008eaedf, 0x70008eaeda);
        }
        279 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eaedf); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eaee1); }
            blk = 186; continue;
        }
        280 => {
            c.w32(15, c.r32(15));
            let r468 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r468);
            c.w64(0, 0xfffffffffffc0000u64);
            let r469 = c.r64(0) & c.r64(15);
            c.w64(0, r469);
            c.logic(r469, 64);
            let Some(t470) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eaef7); };
            c.logic(t470 & 0x2u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w64(6, c.r64(1).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eaf05); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eaf06); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008eaf13, 0x70008eaf0e);
        }
        282 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eaf13); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eaf15); }
            blk = 179; continue;
        }
        283 => {
            c.w32(15, c.r32(15));
            let r471 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r471);
            c.w64(0, 0xfffffffffffc0000u64);
            let r472 = c.r64(0) & c.r64(15);
            c.w64(0, r472);
            c.logic(r472, 64);
            let Some(t473) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eaf2b); };
            c.logic(t473 & 0x2u64, 8);
            if c.cc_e() { blk = 162; continue; }
            blk = 284; continue;
        }
        284 => {
            c.w64(3, c.r64(1).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eaf39); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eaf3a); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008eaf47, 0x70008eaf42);
        }
        285 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eaf47); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eaf49); }
            blk = 162; continue;
        }
        286 => {
            c.w32(0, c.r32(0));
            let r474 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r474);
            c.w64(11, 0xfffffffffffc0000u64);
            let r475 = c.r64(11) & c.r64(0);
            c.w64(11, r475);
            c.logic(r475, 64);
            let Some(t476) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eaf5e); };
            c.logic(t476 & 0x2u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 287; continue;
        }
        287 => {
            c.w64(12, c.r64(9).wrapping_add(0x37));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eaf6d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eaf6e); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008eaf7b, 0x70008eaf76);
        }
        288 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eaf7b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eaf7d); }
            blk = 154; continue;
        }
        289 => {
            c.w32(9, c.r32(9));
            let r477 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r477);
            c.w64(12, 0xfffffffffffc0000u64);
            let r478 = c.r64(12) & c.r64(9);
            c.w64(12, r478);
            c.logic(r478, 64);
            let Some(t479) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eaf93); };
            c.logic(t479 & 0x2u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 290; continue;
        }
        290 => {
            c.w64(15, c.r64(1).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eafa3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eafa4); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008eafb1, 0x70008eafac);
        }
        291 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eafb1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eafb3); }
            blk = 138; continue;
        }
        292 => {
            c.w32(2, c.r32(2));
            let r480 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r480);
            c.w64(1, 0xfffffffffffc0000u64);
            let r481 = c.r64(1) & c.r64(2);
            c.w64(1, r481);
            c.logic(r481, 64);
            let Some(t482) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eafc8); };
            c.logic(t482 & 0x2u64, 8);
            if c.cc_e() { blk = 110; continue; }
            blk = 293; continue;
        }
        293 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eafd6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eafd7); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x70008eafe4, 0x70008eafdf);
        }
        294 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eafe4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eafe6); }
            blk = 110; continue;
        }
        295 => {
            c.w32(7, c.r32(7));
            let r483 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r483);
            c.w64(8, 0xfffffffffffc0000u64);
            let r484 = c.r64(8) & c.r64(7);
            c.w64(8, r484);
            c.logic(r484, 64);
            let Some(t485) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eaffb); };
            c.logic(t485 & 0x2u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w64(9, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eb00a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eb00b); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008eb018, 0x70008eb013);
        }
        297 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eb018); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eb01a); }
            blk = 97; continue;
        }
        298 => {
            c.w32(0, c.r32(0));
            let r486 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r486);
            c.w64(2, 0xfffffffffffc0000u64);
            let r487 = c.r64(2) & c.r64(0);
            c.w64(2, r487);
            c.logic(r487, 64);
            let Some(t488) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eb02f); };
            c.logic(t488 & 0x2u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 299; continue;
        }
        299 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eb03d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eb03e); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008eb04b, 0x70008eb046);
        }
        300 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eb04b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eb04d); }
            blk = 73; continue;
        }
        301 => {
            c.w32(2, c.r32(2));
            let r489 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r489);
            c.w64(1, 0xfffffffffffc0000u64);
            let r490 = c.r64(1) & c.r64(2);
            c.w64(1, r490);
            c.logic(r490, 64);
            let Some(t491) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eb062); };
            c.logic(t491 & 0x2u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eb070); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eb071); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008eb07e, 0x70008eb079);
        }
        303 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eb07e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eb080); }
            blk = 66; continue;
        }
        304 => {
            c.w32(15, c.r32(15));
            let r492 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r492);
            c.w64(0, 0xfffffffffffc0000u64);
            let r493 = c.r64(0) & c.r64(15);
            c.w64(0, r493);
            c.logic(r493, 64);
            let Some(t494) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eb096); };
            c.logic(t494 & 0x2u64, 8);
            if c.cc_e() { blk = 49; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w64(2, c.r64(3).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eb0a4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eb0a5); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008eb0b2, 0x70008eb0ad);
        }
        306 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eb0b2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eb0b4); }
            blk = 49; continue;
        }
        307 => {
            c.w32(3, c.r32(3));
            let r495 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r495);
            c.w64(9, 0xfffffffffffc0000u64);
            let r496 = c.r64(9) & c.r64(3);
            c.w64(9, r496);
            c.logic(r496, 64);
            let Some(t497) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eb0c9); };
            c.logic(t497 & 0x2u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w64(11, c.r64(0).wrapping_add(0x37));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eb0d8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eb0d9); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008eb0e6, 0x70008eb0e1);
        }
        309 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eb0e6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eb0e8); }
            blk = 41; continue;
        }
        310 => {
            c.w32(9, c.r32(9));
            let r498 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r498);
            c.w64(15, 0xfffffffffffc0000u64);
            let r499 = c.r64(15) & c.r64(9);
            c.w64(15, r499);
            c.logic(r499, 64);
            let Some(t500) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eb0fe); };
            c.logic(t500 & 0x2u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(3, c.r64(11).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eb10d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eb10e); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008eb11b, 0x70008eb116);
        }
        312 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eb11b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eb11d); }
            blk = 21; continue;
        }
        313 => {
            c.w32(1, c.r32(1));
            let r501 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r501);
            c.w64(3, 0xfffffffffffc0000u64);
            let r502 = c.r64(3) & c.r64(1);
            c.w64(3, r502);
            c.logic(r502, 64);
            let Some(t503) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eb132); };
            c.logic(t503 & 0x2u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 314; continue;
        }
        314 => {
            c.w64(7, c.r64(11).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eb140); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eb141); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(11));
            return c.call(0x70007f0e54, 0x70008eb14e, 0x70008eb149);
        }
        315 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008eb14e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008eb150); }
            blk = 17; continue;
        }
        316 => {
            return RegfileExit::jump(0x70008eb158);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eb174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eb174); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eb178); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008eb17e); };
            c.w64(8, t1);
            c.w32(9, c.r32(8));
            c.w32(11, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb189); };
            c.w32(12, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008eb18d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008eb191); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008eb195); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008eb199); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008eb19d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x70008eb1a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008eb1a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008eb1a9); }
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008eb1b2);
        }
        2 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eb1b6); };
            c.w32(15, t3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eb1ba); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eb1c0); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(12));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb1ce); };
            return c.call(t8, 0x70008eb1d6, 0x70008eb1ce);
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb1da); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 24; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 7; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008eb1f8); };
            c.w64(2, t9);
            let r10 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r10);
            c.w64(7, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008eb202); }
            c.w32(8, 0x2u64);
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008eb20f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008eb213); }
            blk = 9; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008eb219); }
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb21d); };
            c.w64(2, t11);
            blk = 9; continue;
        }
        9 => {
            c.w32(15, c.r32(8));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb224); };
            c.w64(12, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb228); };
            c.w64(11, t13);
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_ge() { blk = 21; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb235); };
            c.w64(0, t14);
            let r15 = c.sub(c.r32(0), c.r32(15), 32);
            c.w32(0, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb23c); };
            c.w32(15, t16);
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_ae() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eb24c); };
            c.w32(0, t17);
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eb251); };
            c.w32(3, t18);
            let r19 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r19);
            let Some(t20) = c.ld32(c.r64(11).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eb259); };
            c.w32(15, t20);
            let r21 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008eb261); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(15));
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eb26e); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eb272); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb276); };
            return c.call(t24, 0x70008eb27e, 0x70008eb276);
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb282); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 16; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eb298); };
            c.w64(9, t25);
            c.w64(2, c.r64(9).wrapping_add(c.r64(9)));
            let r26 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r26);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_le() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(2));
            blk = 8; continue;
        }
        18 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eb2b3); };
            c.w64(8, t27);
            blk = 8; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008eb2bc);
        }
        20 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb2bd); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb2c1); };
            c.w64(11, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eb2c5); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb2c9); };
            c.w64(2, t31);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(2), c.r32(8), 32);
            if c.cc_ge() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eb2d2); };
            c.w64(8, t32);
            blk = 23; continue;
        }
        23 => {
            let Some(t33) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008eb2d6); };
            c.w32(2, t33);
            c.w32(7, c.r32(8));
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb2dc); };
            c.w64(8, t34);
            let r35 = c.sub(c.r32(8), c.r32(2), 32);
            c.w32(8, r35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb2e3); };
            c.w64(2, t36);
            let r37 = c.sub(c.r32(2), c.r32(7), 32);
            c.w32(2, r37);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        24 => {
            let Some(t38) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb2f4); };
            c.w32(2, t38);
            let Some(t39) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb2f7); };
            let r40 = c.sub(c.r32(2), t39, 32);
            c.w32(2, r40);
            c.w64(7, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008eb2fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008eb301); }
            c.w32(8, 0x2u64);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        25 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb310); };
            c.w64(2, t41);
            blk = 26; continue;
        }
        26 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb314); };
            c.w64(15, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb318); };
            c.w64(12, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb31c); };
            c.w64(11, t44);
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_ge() { blk = 38; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(8)));
            let Some(t45) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb32d); };
            c.w32(3, t45);
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ae() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t46) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eb33d); };
            c.w32(3, t46);
            let Some(t47) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eb341); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eb348); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008eb350); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008eb354); }
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eb361); };
            c.w64(1, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eb365); };
            c.w64(6, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb369); };
            return c.call(t53, 0x70008eb371, 0x70008eb369);
        }
        29 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb375); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eb38b); };
            c.w64(9, t54);
            c.w64(2, c.r64(9).wrapping_add(c.r64(9)));
            let r55 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r55);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_le() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(2));
            blk = 25; continue;
        }
        35 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb3a6); };
            c.w64(8, t56);
            blk = 25; continue;
        }
        36 => {
            return RegfileExit::fault(0x70008eb3af);
        }
        37 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb3b0); };
            c.w64(15, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb3b4); };
            c.w64(12, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008eb3b8); };
            c.w64(9, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eb3bc); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb3c0); };
            c.w64(2, t61);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(2), c.r32(8), 32);
            if c.cc_ge() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb3c9); };
            c.w64(8, t62);
            blk = 40; continue;
        }
        40 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008eb3cd); };
            c.w64(2, t63);
            c.w32(7, c.r64(8).wrapping_add(c.r64(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r64 = c.add(c.r32(2), c.r32(9), 32);
            c.w32(2, r64);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            c.w64(12, c.r64(15));
            blk = 41; continue;
        }
        41 => {
            let r65 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r65);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(12));
            blk = 43; continue;
        }
        42 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb3f2); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb3f6); };
            c.w64(11, t67);
            blk = 43; continue;
        }
        43 => {
            c.w32(12, c.r32(2));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008eb3fd); }
            let Some(t68) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb401); };
            c.w32(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008eb405); };
            c.w64(8, t69);
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb409); };
            c.cmp(c.r32(2), t70, 32);
            if c.cc_ge() { blk = 52; continue; }
            blk = 44; continue;
        }
        44 => {
            let r71 = c.sub(c.r32(15), c.r32(12), 32);
            c.w32(15, r71);
            let r72 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r72);
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            let r73 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r73);
            c.logic(r73, 64);
            let r74 = c.add(c.r32(12), c.r32(2), 32);
            c.w32(12, r74);
            c.w32(15, c.r64(12).wrapping_add(c.r64(8)));
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb426); };
            c.w32(0, t75);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008eb42a); }
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_ae() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eb43a); };
            c.w32(0, t76);
            let Some(t77) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eb43e); };
            c.w32(3, t77);
            let r78 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r78);
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eb445); };
            c.w32(15, t79);
            let r80 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r80);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008eb44d); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(15));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eb45a); };
            c.w64(1, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eb45e); };
            c.w64(6, t82);
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb462); };
            return c.call(t83, 0x70008eb46a, 0x70008eb462);
        }
        46 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb46e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 50; continue;
        }
        49 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t84) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb484); };
            c.w64(7, (t84 as u32 as i32 as i64 as u64));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb488); };
            c.w64(2, t85);
            blk = 42; continue;
        }
        51 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb491); };
            c.w64(2, t86);
            let r87 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r87);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb49b); };
            c.w64(7, t88);
            blk = 42; continue;
        }
        52 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb4a4); };
            c.w64(0, t89);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008eb4ab); }
            return c.ret(0x8, 0x70008eb4ac);
        }
        53 => {
            return RegfileExit::fault(0x70008eb4af);
        }
        54 => {
            return RegfileExit::jump(0x70008eb4b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eb4d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eb4d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eb4d8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008eb4de); };
            c.w64(8, t1);
            c.w32(9, c.r32(8));
            c.w32(11, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb4e9); };
            c.w32(12, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008eb4ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008eb4f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008eb4f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008eb4f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008eb4fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x70008eb501); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008eb505); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008eb509); }
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008eb512);
        }
        2 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eb516); };
            c.w32(15, t3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eb51a); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eb520); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(12));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb531); };
            return c.call(t8, 0x70008eb539, 0x70008eb531);
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb53d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 24; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 7; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb55b); };
            c.w32(2, t9);
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb55e); };
            let r11 = c.sub(c.r32(2), t10, 32);
            c.w32(2, r11);
            c.w64(7, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008eb564); }
            c.w32(8, 0);
            c.logic(0, 32);
            c.w32(9, 0x2u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008eb571); }
            blk = 9; continue;
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb577); };
            c.w64(2, t12);
            blk = 9; continue;
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb57b); };
            c.w64(15, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb57f); };
            c.w64(12, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb583); };
            c.w64(11, t15);
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_ge() { blk = 21; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(9)));
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb594); };
            c.w32(3, t16);
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ae() { blk = 19; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eb5a4); };
            c.w32(3, t17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eb5a8); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eb5af); };
            c.w32(0, t20);
            let r21 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008eb5b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008eb5bb); }
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(2, c.r64(3));
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eb5cb); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eb5cf); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb5d3); };
            return c.call(t24, 0x70008eb5db, 0x70008eb5d3);
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb5df); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 16; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eb5f5); };
            c.w64(8, t25);
            c.w64(2, c.r64(8).wrapping_add(c.r64(8)));
            let r26 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r26);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_le() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(2));
            blk = 8; continue;
        }
        18 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb610); };
            c.w64(9, t27);
            blk = 8; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008eb619);
        }
        20 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb61a); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb61e); };
            c.w64(12, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eb622); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eb626); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb62a); };
            c.w64(2, t32);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_ge() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb633); };
            c.w64(9, t33);
            blk = 23; continue;
        }
        23 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008eb637); };
            c.w64(2, t34);
            c.w32(7, c.r64(9).wrapping_add(c.r64(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r35 = c.add(c.r32(2), c.r32(8), 32);
            c.w32(2, r35);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        24 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008eb64d); };
            c.w64(2, t36);
            let r37 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r37);
            c.w64(7, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008eb657); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008eb65b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), 0x0u64) { return RegfileExit::fault(0x70008eb65f); }
            c.w32(9, 0x2u64);
            blk = 26; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008eb66f); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb673); };
            c.w64(2, t38);
            blk = 26; continue;
        }
        26 => {
            c.w32(15, c.r32(9));
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb67a); };
            c.w64(12, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb67e); };
            c.w64(11, t40);
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_ge() { blk = 38; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb68b); };
            c.w64(0, t41);
            let r42 = c.sub(c.r32(0), c.r32(15), 32);
            c.w32(0, r42);
            let Some(t43) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb692); };
            c.w32(15, t43);
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_ae() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eb6a2); };
            c.w32(0, t44);
            let Some(t45) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eb6a7); };
            c.w32(3, t45);
            let r46 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r46);
            let Some(t47) = c.ld32(c.r64(11).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eb6af); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008eb6b7); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eb6c1); };
            c.w64(3, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eb6c5); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb6c9); };
            return c.call(t51, 0x70008eb6d1, 0x70008eb6c9);
        }
        29 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb6d5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eb6eb); };
            c.w64(8, t52);
            c.w64(2, c.r64(8).wrapping_add(c.r64(8)));
            let r53 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r53);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_le() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(2));
            blk = 25; continue;
        }
        35 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008eb706); };
            c.w64(9, t54);
            blk = 25; continue;
        }
        36 => {
            return RegfileExit::fault(0x70008eb70f);
        }
        37 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb710); };
            c.w64(12, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb714); };
            c.w64(11, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eb718); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb71c); };
            c.w64(2, t58);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_ge() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008eb725); };
            c.w64(9, t59);
            blk = 40; continue;
        }
        40 => {
            let Some(t60) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008eb729); };
            c.w32(2, t60);
            c.w32(7, c.r32(9));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb72f); };
            c.w64(8, t61);
            let r62 = c.sub(c.r32(8), c.r32(2), 32);
            c.w32(8, r62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb736); };
            c.w64(2, t63);
            let r64 = c.sub(c.r32(2), c.r32(7), 32);
            c.w32(2, r64);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(11));
            blk = 41; continue;
        }
        41 => {
            let r65 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r65);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(12));
            c.w64(11, c.r64(15));
            blk = 43; continue;
        }
        42 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb756); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb75a); };
            c.w64(11, t67);
            blk = 43; continue;
        }
        43 => {
            c.w32(12, c.r32(2));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008eb761); }
            let Some(t68) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb765); };
            c.w32(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008eb769); };
            c.w64(8, t69);
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb76d); };
            c.cmp(c.r32(2), t70, 32);
            if c.cc_ge() { blk = 52; continue; }
            blk = 44; continue;
        }
        44 => {
            let r71 = c.sub(c.r32(15), c.r32(12), 32);
            c.w32(15, r71);
            let r72 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r72);
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            let r73 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r73);
            c.logic(r73, 64);
            let r74 = c.add(c.r32(12), c.r32(2), 32);
            c.w32(12, r74);
            c.w32(15, c.r64(12).wrapping_add(c.r64(8)));
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb78a); };
            c.w32(0, t75);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008eb78e); }
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_ae() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eb79e); };
            c.w32(0, t76);
            let Some(t77) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eb7a2); };
            c.w32(3, t77);
            let r78 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r78);
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eb7a9); };
            c.w32(1, t79);
            let r80 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r80);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008eb7b1); }
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eb7bb); };
            c.w64(3, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eb7bf); };
            c.w64(6, t82);
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb7c3); };
            return c.call(t83, 0x70008eb7cb, 0x70008eb7c3);
        }
        46 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb7cf); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 50; continue;
        }
        49 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb7e5); };
            c.w64(2, t84);
            let r85 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r85);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb7ef); };
            c.w64(7, t86);
            blk = 42; continue;
        }
        51 => {
            let Some(t87) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb7f8); };
            c.w64(7, (t87 as u32 as i32 as i64 as u64));
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb7fc); };
            c.w64(2, t88);
            blk = 42; continue;
        }
        52 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eb805); };
            c.w64(0, t89);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008eb80c); }
            return c.ret(0x8, 0x70008eb80d);
        }
        53 => {
            return RegfileExit::fault(0x70008eb810);
        }
        54 => {
            return RegfileExit::jump(0x70008eb814);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eb834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eb834); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eb838); }
            let r0 = c.sub(c.r64(4), 0x90u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008eb841); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008eb847); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008eb84e); };
            c.w32(9, t5);
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb852); };
            c.w32(11, t6);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008eb862); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008eb866); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x70008eb86a); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008eb86e); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(12)) { return RegfileExit::fault(0x70008eb872); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(15, 0x7ffffffeu64);
            blk = 3; continue;
        }
        2 => {
            c.w64(15, c.r64(8));
            blk = 3; continue;
        }
        3 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008eb88d); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(11));
            c.w64(11, c.r64(7));
            c.w64(9, c.r64(2));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008eb89c); }
            blk = 5; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008eb8a2); };
            c.w64(12, t7);
            let r8 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r8);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008eb8ad); }
            c.w64(1, c.r64(11));
            c.w64(11, c.r64(8));
            c.w64(9, c.r64(12));
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008eb8ba); };
            c.w64(8, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008eb8be); };
            c.w64(0, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eb8c2); };
            c.w64(6, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008eb8c6); };
            c.w64(15, t12);
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb8d2); }
            let Some(t13) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008eb8d8); };
            c.cmp(t13, 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb8de); }
            blk = 11; continue;
        }
        8 => {
            c.w64(7, c.r64(8));
            let r14 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r14);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 11; continue;
        }
        9 => {
            let Some(t15) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008eb8f5); };
            c.cmp(t15, 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(7, c.r64(9));
            let r16 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r16);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eb900); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 11; continue;
        }
        11 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 42; continue;
        }
        13 => {
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_ge() { blk = 42; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008eb922); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008eb926); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008eb92a); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008eb92e); }
            c.w64(3, c.r64(9));
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008eb935); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008eb939); };
            return c.call(t18, 0x70008eb941, 0x70008eb939);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008eb941); }
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008eb945); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008eb952); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eb95f); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb963); };
            c.cmp(c.r64(11), t22, 64);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb969); };
            c.w64(2, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eb96d); };
            c.w64(7, t24);
            blk = 36; continue;
        }
        19 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eb976); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eb97a); };
            c.w32(8, t26);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eb984); };
            c.w64(2, t27);
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_a() { blk = 228; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_b() { blk = 228; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, c.r64(11));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r28 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r28);
            let r29 = c.add(c.r64(8), 0x10u64, 64);
            c.w64(8, r29);
            if c.cc_e() { blk = 34; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 236; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(9, c.r64(8).wrapping_add(c.r64(8)));
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w32(12, c.r32(12));
            let r30 = c.imul2(c.r32(12), 0x4u64, 32);
            c.w32(12, r30);
            if c.cc_o() { blk = 232; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0x8u64);
            let r31 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r31);
            if c.cc_o() { blk = 231; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008eb9e7); };
            c.w64(3, t32);
            let Some(t33) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008eb9ee); };
            c.w64(1, t33);
            c.w64(6, c.r64(1).wrapping_add(c.r64(12)));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008eb9f5); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008eb9fc); };
            c.cmp(t35, c.r64(6), 64);
            if c.cc_be() { blk = 234; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 234; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008eba12); }
            c.w64(12, c.r64(1).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008eba19); };
            c.w64(0, t36);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008eba20); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008eba25); }
            c.w64(9, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008eba32); };
            c.w64(0, t37);
            c.w64(11, c.r64(15));
            blk = 30; continue;
        }
        29 => {
            c.w64(1, c.r64(15).wrapping_add(0x4));
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008eba42); }
            c.w64(15, c.r64(1));
            c.w64(11, c.r64(3));
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(3, c.r64(2).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008eba5f); };
            c.w32(3, t38);
            let r39 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r39);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 29; continue;
        }
        33 => {
            c.w64(3, c.r64(11));
            c.w64(11, c.r64(0));
            blk = 29; continue;
        }
        34 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008eba7b); };
            c.w64(7, t40);
            c.w64(12, c.r64(7));
            blk = 35; continue;
        }
        35 => {
            c.w64(11, c.r64(8));
            c.w64(7, c.r64(12));
            blk = 36; continue;
        }
        36 => {
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eba8b); };
            c.w32(8, t41);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_ae() { blk = 233; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(8, c.r64(2).wrapping_add(0x1));
            c.w64(9, 0xffffffffffffffffu64);
            c.w64(12, c.r64(9).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x8));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ebaae); };
            c.w64(15, t42);
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x70008ebab2); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r43 = c.r64(0) & c.r64(7);
            c.w64(0, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ebac6); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 259; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(2, c.r64(8));
            c.w64(8, c.r64(7));
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ebad6); };
            c.w64(7, t45);
            blk = 4; continue;
        }
        40 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ebadf); };
            c.w64(7, t46);
            let r47 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r47);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(9, 0xffffffffffffffffu64);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebaf0); };
            c.w64(2, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ebaf4); };
            c.w64(11, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ebaf8); };
            c.w64(8, t50);
            blk = 4; continue;
        }
        41 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ebb01); };
            c.w64(7, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ebb05); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ebb09); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebb0d); };
            c.w64(2, t54);
            c.w64(9, 0xffffffffffffffffu64);
            blk = 4; continue;
        }
        42 => {
            c.w32(7, c.r64(2).wrapping_add(c.r64(2)));
            if !c.st32(c.r64(0).wrapping_add(0x33), c.r32(11)) { return RegfileExit::fault(0x70008ebb20); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r55 = c.r64(8) & c.r64(0);
            c.w64(8, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ebb34); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 256; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.st32(c.r64(0).wrapping_add(0x3b), c.r32(15)) { return RegfileExit::fault(0x70008ebb3f); }
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ebb43); };
            c.w64(8, t57);
            if !c.st32(c.r64(0).wrapping_add(0x3f), c.r32(8)) { return RegfileExit::fault(0x70008ebb47); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(7)) { return RegfileExit::fault(0x70008ebb4b); }
            c.cmp(c.r32(7), 0x4u64, 32);
            if c.cc_l() { blk = 194; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(9, c.r64(7));
            c.w64(8, c.r64(3));
            blk = 47; continue;
        }
        46 => {
            c.w32(8, c.r32(8));
            let r58 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r58);
            let r59 = c.r32(11) & 0x2u64;
            c.w32(11, r59);
            c.logic(r59, 32);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r60 = c.r32(11) | c.r32(8);
            c.w32(11, r60);
            c.logic(r60, 32);
            let r61 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r61);
            c.logic(r61, 64);
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 47; continue;
        }
        47 => {
            c.w32(11, c.r32(9));
            c.cmp(c.r32(9), 0x80u64, 32);
            if c.cc_ge() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(8)) { return RegfileExit::fault(0x70008ebb86); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x70008ebb8d); }
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(7));
            blk = 50; continue;
        }
        49 => {
            let Some(t62) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebb9c); };
            c.w32(7, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebb9f); };
            c.w64(8, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ebba3); };
            c.w64(11, t64);
            let r65 = c.add(c.r32(8), c.r32(11), 32);
            c.w32(8, r65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ebbaa); };
            c.w64(9, t66);
            let r67 = c.sub(c.r32(9), c.r32(7), 32);
            c.w32(9, r67);
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 50; continue;
        }
        50 => {
            c.w32(7, c.r32(12));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008ebbba); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(7, c.r32(11));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008ebbca); }
            c.w32(7, c.r64(11).wrapping_add(0x2));
            let Some(t68) = c.ld32(c.r64(0).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008ebbd2); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            c.w32(9, c.r64(11).wrapping_add(c.r64(12)));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008ebbdd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008ebbe1); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008ebbe5); }
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ebbf2); };
            c.w32(9, t70);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 171; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008ebbff); }
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(12)) { return RegfileExit::fault(0x70008ebc06); }
            c.w32(12, c.r64(7).wrapping_sub(0x2));
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ebc0e); };
            c.w32(11, t71);
            let r72 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r72);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008ebc16); }
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_ae() { blk = 172; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t73) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ebc26); };
            c.w32(12, t73);
            let Some(t74) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ebc2a); };
            c.w32(7, t74);
            let r75 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r75);
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ebc30); };
            c.w32(9, t76);
            let r77 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r77);
            c.w64(0, c.r64(7));
            c.w64(3, c.r64(11));
            c.w64(1, c.r64(9));
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ebc41); };
            return c.call(t78, 0x70008ebc49, 0x70008ebc41);
        }
        55 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ebc4d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 59; continue;
        }
        58 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 61; continue;
        }
        60 => {
            c.w32(7, 0x1u64);
            blk = 61; continue;
        }
        61 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ebc6c); };
            c.w64(8, t79);
            let r80 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r80);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008ebc77); }
            c.w64(9, c.r64(8));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebc7e); };
            c.w64(1, t81);
            c.w32(8, 0x4u64);
            blk = 63; continue;
        }
        62 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ebc8a); };
            c.w64(8, t82);
            c.w32(7, c.r64(8).wrapping_add(0x2));
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebc95); };
            c.w64(8, t83);
            c.w32(7, c.r64(8).wrapping_add(0x2));
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ebca0); };
            c.w64(1, t84);
            blk = 63; continue;
        }
        63 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ebca4); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ebca8); };
            c.w64(11, t86);
            let Some(t87) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ebcac); };
            c.cmp(c.r32(9), t87, 32);
            if c.cc_ge() { blk = 77; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t88) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ebcb6); };
            c.w32(12, t88);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_ae() { blk = 173; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t89) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ebcc6); };
            c.w32(15, t89);
            let Some(t90) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ebcca); };
            c.w32(3, t90);
            let r91 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r91);
            let Some(t92) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ebcd0); };
            c.w32(12, t92);
            let r93 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r93);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008ebcd8); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008ebcdc); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008ebce0); }
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(12));
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ebcea); };
            c.w64(6, t94);
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ebcee); };
            return c.call(t95, 0x70008ebcf6, 0x70008ebcee);
        }
        66 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t96) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ebd00); };
            c.cmp(t96, 0x0u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ebd0b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 78; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 62; continue;
        }
        70 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 78; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 62; continue;
        }
        72 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ebd2e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 78; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 62; continue;
        }
        75 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 78; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 62; continue;
        }
        77 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008ebd4c); }
            blk = 78; continue;
        }
        78 => {
            let Some(t97) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ebd50); };
            c.cmp(t97, 0x0u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t98) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebd5a); };
            c.w32(7, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ebd5d); };
            c.w64(8, t99);
            c.w32(7, c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x2));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ebd69); };
            c.w64(8, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ebd6d); };
            c.w64(9, t101);
            blk = 82; continue;
        }
        80 => {
            let r102 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r102);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let r103 = c.sub(c.r32(7), 0x2u64, 32);
            c.w32(7, r103);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(9).wrapping_add(c.r64(0)), c.r32(12)) { return RegfileExit::fault(0x70008ebd80); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r104 = c.r64(11) & c.r64(9);
            c.w64(11, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ebd94); };
            c.logic(t105 & 0x4u64, 8);
            if c.cc_ne() { blk = 253; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_ge() { blk = 89; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t106) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ebda4); };
            c.w32(11, t106);
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_ae() { blk = 174; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            let Some(t107) = c.ld32(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ebdbc); };
            c.w32(12, t107);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_ae() { blk = 175; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            let Some(t108) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ebdd5); };
            c.w32(11, t108);
            if !c.st32(c.r64(9).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x70008ebdda); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r109 = c.r64(3) & c.r64(9);
            c.w64(3, r109);
            c.logic(r109, 64);
            let Some(t110) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ebdee); };
            c.logic(t110 & 0x4u64, 8);
            if c.cc_ne() { blk = 250; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t111) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ebdf8); };
            c.w32(11, t111);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_b() { blk = 80; continue; }
            blk = 88; continue;
        }
        88 => {
            blk = 176; continue;
        }
        89 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebe0a); };
            c.w64(8, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ebe0e); };
            c.w64(0, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ebe12); };
            c.w64(6, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ebe16); };
            c.w64(11, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ebe1a); };
            c.w64(12, t116);
            blk = 91; continue;
        }
        90 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008ebe20); }
            c.w32(8, 0x2u64);
            blk = 91; continue;
        }
        91 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ebe2a); };
            c.w64(7, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ebe31); };
            c.w64(9, t118);
            c.w32(15, c.r64(9).wrapping_add(c.r64(7)));
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_ge() { blk = 125; continue; }
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_ge() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            blk = 94; continue;
        }
        94 => {
            let r119 = c.add(c.r32(8), c.r32(11), 32);
            c.w32(8, r119);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(12)) { return RegfileExit::fault(0x70008ebe50); }
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 97; continue;
        }
        96 => {
            let r120 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r120);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 97; continue;
        }
        97 => {
            let Some(t121) = c.ld32(c.r64(0).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008ebe65); };
            c.w32(15, t121);
            let r122 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r122);
            c.w32(3, c.r64(11).wrapping_add(c.r64(12)));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008ebe70); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x70008ebe74); }
            blk = 101; continue;
        }
        98 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.w64(9, c.r64(2));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(9)) { return RegfileExit::fault(0x70008ebe88); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r123 = c.r64(12) & c.r64(8);
            c.w64(12, r123);
            c.logic(r123, 64);
            let Some(t124) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ebe9c); };
            c.logic(t124 & 0x4u64, 8);
            if c.cc_ne() { blk = 247; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ebea8); };
            c.w64(7, t125);
            let r126 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r126);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(8));
            c.w64(8, c.r64(7));
            blk = 101; continue;
        }
        101 => {
            let Some(t127) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008ebeb8); };
            c.cmp(c.r32(8), t127, 32);
            if c.cc_ge() { blk = 124; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t128) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ebec2); };
            c.w32(2, t128);
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_ae() { blk = 189; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t129) = c.ld32(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ebed2); };
            c.w32(2, t129);
            let r130 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r130);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008ebeda); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008ebede); }
            c.w64(1, c.r64(11));
            blk = 105; continue;
        }
        104 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ebee7); };
            c.w64(2, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ebeeb); };
            c.w64(15, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ebeef); };
            c.w64(0, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ebef3); };
            c.w64(6, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ebef7); };
            c.w64(11, t135);
            blk = 105; continue;
        }
        105 => {
            c.w32(7, c.r32(1));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008ebefd); }
            let Some(t136) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ebf01); };
            c.w32(8, t136);
            let Some(t137) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ebf05); };
            c.cmp(c.r32(1), t137, 32);
            if c.cc_ge() { blk = 114; continue; }
            blk = 106; continue;
        }
        106 => {
            let r138 = c.sub(c.r32(8), c.r32(7), 32);
            c.w32(8, r138);
            let r139 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r139);
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            let r140 = c.r64(7) & 0xfffffffffffffffeu64;
            c.w64(7, r140);
            c.logic(r140, 64);
            let r141 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r141);
            let Some(t142) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ebf1d); };
            c.w32(8, t142);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008ebf21); }
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ae() { blk = 193; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t143) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ebf31); };
            c.w32(7, t143);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x70008ebf34); }
            let Some(t144) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ebf38); };
            c.w32(7, t144);
            let r145 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r145);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x70008ebf3e); }
            let Some(t146) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ebf42); };
            c.w32(7, t146);
            let r147 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r147);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008ebf4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008ebf4e); }
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ebf52); };
            c.w64(0, t148);
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(7));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ebf5c); };
            c.w64(7, t149);
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ebf60); };
            return c.call(t150, 0x70008ebf68, 0x70008ebf60);
        }
        108 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 109; continue;
        }
        109 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ebf6c); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 113; continue; }
            blk = 110; continue;
        }
        110 => {
            blk = 112; continue;
        }
        111 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ebf82); };
            c.w64(7, t151);
            let r152 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r152);
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ebf8c); };
            c.w64(8, t153);
            blk = 104; continue;
        }
        113 => {
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ebf95); };
            c.w64(1, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ebf99); };
            c.w64(8, t155);
            blk = 104; continue;
        }
        114 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ebfa2); };
            c.w64(7, t156);
            c.w64(8, c.r64(15));
            blk = 118; continue;
        }
        115 => {
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ebfb1); };
            c.w32(12, t157);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(12)) { return RegfileExit::fault(0x70008ebfbe); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r158 = c.r64(15) & c.r64(8);
            c.w64(15, r158);
            c.logic(r158, 64);
            let Some(t159) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ebfd2); };
            c.logic(t159 & 0x4u64, 8);
            if c.cc_ne() { blk = 244; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(7, c.r64(9));
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_ge() { blk = 122; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t160) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ebfe4); };
            c.w32(9, t160);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 191; continue; }
            blk = 120; continue;
        }
        120 => {
            c.w32(12, c.r64(7).wrapping_sub(0x2));
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_b() { blk = 115; continue; }
            blk = 121; continue;
        }
        121 => {
            blk = 192; continue;
        }
        122 => {
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ebfff); };
            c.w32(7, t161);
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_b() { blk = 98; continue; }
            blk = 123; continue;
        }
        123 => {
            blk = 190; continue;
        }
        124 => {
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ec010); };
            c.w64(8, t162);
            blk = 125; continue;
        }
        125 => {
            let Some(t163) = c.ld32(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008ec014); };
            c.w32(7, t163);
            c.w64(9, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t164) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ec01b); };
            c.w32(12, t164);
            let r165 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r165);
            let Some(t166) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec022); };
            c.w32(15, t166);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008ec027); }
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_ae() { blk = 177; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            let r167 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r167);
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x70008ec043); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 127; continue;
        }
        127 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r168 = c.r64(3) & c.r64(12);
            c.w64(3, r168);
            c.logic(r168, 64);
            let Some(t169) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec057); };
            c.logic(t169 & 0x4u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t170) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec061); };
            c.w32(15, t170);
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_ae() { blk = 178; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r171 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r171);
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70008ec080); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r172 = c.r64(15) & c.r64(12);
            c.w64(15, r172);
            c.logic(r172, 64);
            let Some(t173) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec094); };
            c.logic(t173 & 0x4u64, 8);
            if c.cc_ne() { blk = 238; continue; }
            blk = 131; continue;
        }
        131 => {
            if !c.st32(c.r64(0).wrapping_add(0x2b), c.r32(7)) { return RegfileExit::fault(0x70008ec09f); }
            let Some(t174) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ec0a2); };
            c.w32(7, t174);
            let r175 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r175);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ec0a8); }
            blk = 133; continue;
        }
        132 => {
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ec0ae); };
            c.w64(0, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec0b2); };
            c.w64(6, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ec0b6); };
            c.w64(7, t178);
            blk = 133; continue;
        }
        133 => {
            let Some(t179) = c.ld32(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008ec0ba); };
            c.w32(9, t179);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_le() { blk = 49; continue; }
            blk = 134; continue;
        }
        134 => {
            let r180 = c.sub(c.r32(9), 0x4u64, 32);
            c.w32(9, r180);
            c.w32(12, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_l() { blk = 139; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r181 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r181);
            let Some(t182) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec0e0); };
            c.w32(3, t182);
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ae() { blk = 179; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w32(2, c.r64(12).wrapping_sub(0x2));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r183 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r183);
            let Some(t184) = c.ld32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec0fd); };
            c.w32(15, t184);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_ae() { blk = 180; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w32(1, c.r64(12).wrapping_sub(0x4));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r185 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r185);
            let Some(t186) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec11b); };
            c.w32(2, t186);
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_ae() { blk = 181; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t187) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec12a); };
            c.w32(3, t187);
            let r188 = c.add(c.r32(15), c.r32(2), 32);
            c.w32(15, r188);
            c.cmp(c.r32(15), c.r32(3), 32);
            c.w8(15, (c.cc_l() as u64));
            c.w32(15, c.r8(15));
            blk = 140; continue;
        }
        139 => {
            c.w32(15, 0x1u64);
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 146; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r189 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r189);
            let Some(t190) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec15e); };
            c.w32(3, t190);
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ae() { blk = 182; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w32(2, c.r64(9).wrapping_sub(0x2));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r191 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r191);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t192) = c.ld32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec17a); };
            c.w32(15, t192);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_ae() { blk = 183; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w32(1, c.r64(9).wrapping_sub(0x4));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r193 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r193);
            let Some(t194) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec197); };
            c.w32(2, t194);
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_ae() { blk = 184; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t195) = c.ld32(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec1a6); };
            c.w32(3, t195);
            let r196 = c.add(c.r32(15), c.r32(2), 32);
            c.w32(15, r196);
            c.cmp(c.r32(15), c.r32(3), 32);
            c.w8(15, (c.cc_l() as u64));
            c.w32(15, c.r8(15));
            blk = 147; continue;
        }
        146 => {
            c.w32(15, 0x1u64);
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r197 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r197);
            let Some(t198) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec1d0); };
            c.w32(3, t198);
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ae() { blk = 185; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r199 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r199);
            let Some(t200) = c.ld32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec1e9); };
            c.w32(15, t200);
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_ae() { blk = 186; continue; }
            blk = 150; continue;
        }
        150 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t201) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec1fa); };
            c.w32(12, t201);
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_g() { blk = 49; continue; }
            blk = 151; continue;
        }
        151 => {
            c.w64(3, (c.r32(9) as u32 as i32 as i64 as u64));
            return c.call(0x70008e9ab4, 0x70008ec210, 0x70008ec20b);
        }
        152 => {
            blk = 132; continue;
        }
        153 => {
            c.w32(15, c.r64(9).wrapping_sub(0x2));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r202 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r202);
            let Some(t203) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec223); };
            c.w32(3, t203);
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ae() { blk = 187; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r204 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r204);
            let Some(t205) = c.ld32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec23c); };
            c.w32(15, t205);
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_ae() { blk = 188; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t206) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec24d); };
            c.w32(12, t206);
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_l() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 158; continue;
        }
        157 => {
            let r207 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r207);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 158; continue;
        }
        158 => {
            c.w64(3, c.r64(9));
            return c.call(0x70008e9ab4, 0x70008ec26b, 0x70008ec266);
        }
        159 => {
            blk = 132; continue;
        }
        160 => {
            let Some(t208) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ec270); };
            c.w32(7, t208);
            let r209 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r209);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008ec276); }
            blk = 163; continue;
        }
        161 => {
            c.w64(3, c.r64(9));
            return c.call(0x70008e9ab4, 0x70008ec284, 0x70008ec27f);
        }
        162 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ec284); };
            c.w64(0, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec288); };
            c.w64(6, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ec28c); };
            c.w64(7, t212);
            blk = 163; continue;
        }
        163 => {
            let Some(t213) = c.ld32(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008ec290); };
            c.w32(8, t213);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_le() { blk = 194; continue; }
            blk = 164; continue;
        }
        164 => {
            let r214 = c.sub(c.r32(8), 0x4u64, 32);
            c.w32(8, r214);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_le() { blk = 168; continue; }
            blk = 165; continue;
        }
        165 => {
            c.w32(9, c.r64(8).wrapping_sub(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t215) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec2ab); };
            c.w32(11, t215);
            c.w64(12, c.r64(9).wrapping_add(c.r64(9)));
            let r216 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r216);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ae() { blk = 169; continue; }
            blk = 166; continue;
        }
        166 => {
            c.w32(15, c.r64(8).wrapping_add(0x2));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r217 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r217);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t218) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec2cd); };
            c.w32(12, t218);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_ae() { blk = 170; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t219) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec2da); };
            c.w32(11, t219);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_l() { blk = 161; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 161; continue;
        }
        169 => {
            return RegfileExit::fault(0x70008ec2e9);
        }
        170 => {
            return RegfileExit::fault(0x70008ec2ea);
        }
        171 => {
            return RegfileExit::fault(0x70008ec2eb);
        }
        172 => {
            return RegfileExit::fault(0x70008ec2ec);
        }
        173 => {
            return RegfileExit::fault(0x70008ec2ed);
        }
        174 => {
            return RegfileExit::fault(0x70008ec2ee);
        }
        175 => {
            return RegfileExit::fault(0x70008ec2ef);
        }
        176 => {
            return RegfileExit::fault(0x70008ec2f0);
        }
        177 => {
            return RegfileExit::fault(0x70008ec2f1);
        }
        178 => {
            return RegfileExit::fault(0x70008ec2f2);
        }
        179 => {
            return RegfileExit::fault(0x70008ec2f3);
        }
        180 => {
            return RegfileExit::fault(0x70008ec2f4);
        }
        181 => {
            return RegfileExit::fault(0x70008ec2f5);
        }
        182 => {
            return RegfileExit::fault(0x70008ec2f6);
        }
        183 => {
            return RegfileExit::fault(0x70008ec2f7);
        }
        184 => {
            return RegfileExit::fault(0x70008ec2f8);
        }
        185 => {
            return RegfileExit::fault(0x70008ec2f9);
        }
        186 => {
            return RegfileExit::fault(0x70008ec2fa);
        }
        187 => {
            return RegfileExit::fault(0x70008ec2fb);
        }
        188 => {
            return RegfileExit::fault(0x70008ec2fc);
        }
        189 => {
            return RegfileExit::fault(0x70008ec2fd);
        }
        190 => {
            return RegfileExit::fault(0x70008ec2fe);
        }
        191 => {
            return RegfileExit::fault(0x70008ec2ff);
        }
        192 => {
            return RegfileExit::fault(0x70008ec300);
        }
        193 => {
            return RegfileExit::fault(0x70008ec301);
        }
        194 => {
            let Some(t220) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec302); };
            c.w32(7, t220);
            let r221 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r221);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 205; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t222) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec312); };
            c.w32(8, t222);
            let r223 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r223);
            let Some(t224) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec319); };
            c.cmp(t224, 0x834u64, 16);
            if c.cc_ne() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w32(9, 0x1u64);
            blk = 198; continue;
        }
        197 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t225) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008ec333); };
            c.w32(9, t225);
            let r226 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r226);
            let r227 = c.r32(9) & 0x3fu64;
            c.w32(9, r227);
            c.logic(r227, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_a() { blk = 205; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t228) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec346); };
            c.w32(9, t228);
            let r229 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r229);
            let Some(t230) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ec34d); };
            c.w32(9, t230);
            let r231 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r231);
            let Some(t232) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec354); };
            c.w32(11, t232);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            return RegfileExit::fault(0x70008ec364);
        }
        202 => {
            let Some(t233) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008ec365); };
            c.w32(9, t233);
            let Some(t234) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ec36c); };
            c.cmp(t234, c.r32(9), 32);
            if c.cc_ne() { blk = 205; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008ec372); };
            c.w64(8, t235);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t236) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ec37c); };
            c.cmp(t236, c.r32(9), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            c.w32(15, 0x1u64);
            blk = 206; continue;
        }
        205 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t237) = c.ld32(c.r64(0).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ec396); };
            c.w32(8, t237);
            let Some(t238) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec39a); };
            c.w32(3, t238);
            let r239 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r239);
            let Some(t240) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ec3a0); };
            c.w32(1, t240);
            let r241 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r241);
            c.w64(0, c.r64(7));
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ec3a9); };
            return c.call(t242, 0x70008ec3b1, 0x70008ec3a9);
        }
        208 => {
            let Some(t243) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ec3b1); };
            c.cmp(t243, c.r32(0), 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ec3ba); };
            c.w64(0, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec3be); };
            c.w64(6, t245);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 212; continue;
        }
        210 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ec3c7); };
            c.w64(0, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec3cb); };
            c.w64(6, t247);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 211; continue;
        }
        211 => {
            c.w32(7, 0x8c4u64);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(7)) { return RegfileExit::fault(0x70008ec3d7); }
            c.w32(7, 0x8c6u64);
            if !c.st32(c.r64(0).wrapping_add(0x1b), c.r32(7)) { return RegfileExit::fault(0x70008ec3df); }
            c.w32(7, 0x8c8u64);
            if !c.st32(c.r64(0).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70008ec3e7); }
            blk = 212; continue;
        }
        212 => {
            let Some(t248) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008ec3ea); };
            c.w32(7, t248);
            let Some(t249) = c.ld32(c.r64(0).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008ec3ed); };
            c.w32(8, t249);
            let r250 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r250);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008ec3f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008ec3f8); }
            c.w64(11, c.r64(9));
            blk = 215; continue;
        }
        213 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t251) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec404); };
            c.w32(1, t251);
            let r252 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r252);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008ec40c); }
            c.w64(3, c.r64(11));
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ec413); };
            return c.call(t253, 0x70008ec41b, 0x70008ec413);
        }
        214 => {
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ec41b); };
            c.w64(11, t254);
            c.w32(7, c.r64(11).wrapping_add(0x2));
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ec426); };
            c.w64(8, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ec42a); };
            c.w64(0, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec42e); };
            c.w64(6, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ec432); };
            c.w64(7, t258);
            blk = 215; continue;
        }
        215 => {
            let Some(t259) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008ec436); };
            c.cmp(c.r32(11), t259, 32);
            if c.cc_ge() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t260) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec43f); };
            c.w32(12, t260);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_b() { blk = 213; continue; }
            blk = 217; continue;
        }
        217 => {
            blk = 227; continue;
        }
        218 => {
            let Some(t261) = c.ld32(c.r64(0).wrapping_add(0x3f)) else { return RegfileExit::fault(0x70008ec44d); };
            c.w32(8, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008ec451); };
            c.w64(9, t262);
            let r263 = c.add(c.r32(8), c.r32(9), 32);
            c.w32(8, r263);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008ec45b); }
            blk = 221; continue;
        }
        219 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008ec461); }
            c.w64(3, c.r64(11));
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ec468); };
            c.w64(1, t264);
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ec46f); };
            return c.call(t265, 0x70008ec477, 0x70008ec46f);
        }
        220 => {
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ec477); };
            c.w64(11, t266);
            c.w32(7, c.r64(11).wrapping_add(0x2));
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ec482); };
            c.w64(7, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ec486); };
            c.w64(0, t268);
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec48a); };
            c.w64(6, t269);
            blk = 221; continue;
        }
        221 => {
            let Some(t270) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ec48e); };
            c.cmp(c.r32(11), t270, 32);
            if c.cc_l() { blk = 219; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t271) = c.ld32(c.r64(0).wrapping_add(0x3b)) else { return RegfileExit::fault(0x70008ec494); };
            c.w32(7, t271);
            let Some(t272) = c.ld32(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ec497); };
            c.w32(8, t272);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008ec49b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008ec49f); }
            blk = 225; continue;
        }
        223 => {
            c.w32(9, c.r64(11).wrapping_add(0x2));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008ec4ac); }
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(8));
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ec4b6); };
            return c.call(t273, 0x70008ec4be, 0x70008ec4b6);
        }
        224 => {
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ec4be); };
            c.w64(11, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ec4c2); };
            c.w64(8, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ec4c6); };
            c.w64(0, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec4ca); };
            c.w64(6, t277);
            blk = 225; continue;
        }
        225 => {
            let Some(t278) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ec4ce); };
            c.cmp(c.r32(11), t278, 32);
            if c.cc_l() { blk = 223; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ec4d9); }
            return c.ret(0x0, 0x70008ec4da);
        }
        227 => {
            return RegfileExit::fault(0x70008ec4db);
        }
        228 => {
            let Some(t279) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ec4dc); };
            c.w64(2, t279);
            let Some(t280) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008ec4e3); };
            c.w32(2, t280);
            let r281 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r281);
            return c.call(0x7000824234, 0x70008ec4f1, 0x70008ec4ec);
        }
        229 => {
            return RegfileExit::fault(0x70008ec4f1);
        }
        230 => {
            return RegfileExit::fault(0x70008ec4f2);
        }
        231 => {
            return RegfileExit::fault(0x70008ec4f3);
        }
        232 => {
            return RegfileExit::fault(0x70008ec4f4);
        }
        233 => {
            return RegfileExit::fault(0x70008ec4f5);
        }
        234 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008ec4f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008ec4fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008ec4fe); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008ec50a, 0x70008ec505);
        }
        235 => {
            c.w64(12, c.r64(0));
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ec50d); };
            c.w64(7, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ec511); };
            c.w64(2, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ec515); };
            c.w64(8, t284);
            c.w32(15, 0x8u64);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ec51f); };
            c.w64(11, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ec523); };
            c.w64(9, t286);
            blk = 28; continue;
        }
        236 => {
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008ec52c); };
            c.w64(3, t287);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ec53c, 0x70008ec537);
        }
        237 => {
            return RegfileExit::fault(0x70008ec53c);
        }
        238 => {
            c.w32(8, c.r32(8));
            let r288 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r288);
            c.w64(15, 0xfffffffffffc0000u64);
            let r289 = c.r64(15) & c.r64(8);
            c.w64(15, r289);
            c.logic(r289, 64);
            let Some(t290) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec54d); };
            c.logic(t290 & 0x2u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 239; continue;
        }
        239 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ec55c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ec55d); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008ec56a, 0x70008ec565);
        }
        240 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ec56a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ec56c); }
            blk = 131; continue;
        }
        241 => {
            c.w32(11, c.r32(11));
            let r291 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r291);
            c.w64(3, 0xfffffffffffc0000u64);
            let r292 = c.r64(3) & c.r64(11);
            c.w64(3, r292);
            c.logic(r292, 64);
            let Some(t293) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec582); };
            c.logic(t293 & 0x2u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 242; continue;
        }
        242 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ec590); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ec591); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008ec59e, 0x70008ec599);
        }
        243 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ec59e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ec5a0); }
            blk = 128; continue;
        }
        244 => {
            c.w32(12, c.r32(12));
            let r294 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r294);
            c.w64(15, 0xfffffffffffc0000u64);
            let r295 = c.r64(15) & c.r64(12);
            c.w64(15, r295);
            c.logic(r295, 64);
            let Some(t296) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec5b6); };
            c.logic(t296 & 0x2u64, 8);
            if c.cc_e() { blk = 117; continue; }
            blk = 245; continue;
        }
        245 => {
            c.w64(3, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ec5c5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ec5c6); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008ec5d3, 0x70008ec5ce);
        }
        246 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ec5d3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ec5d5); }
            blk = 117; continue;
        }
        247 => {
            c.w32(9, c.r32(9));
            let r297 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r297);
            c.w64(12, 0xfffffffffffc0000u64);
            let r298 = c.r64(12) & c.r64(9);
            c.w64(12, r298);
            c.logic(r298, 64);
            let Some(t299) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec5eb); };
            c.logic(t299 & 0x2u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ec5fb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ec5fc); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008ec609, 0x70008ec604);
        }
        249 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ec609); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ec60b); }
            blk = 100; continue;
        }
        250 => {
            c.w32(11, c.r32(11));
            let r300 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r300);
            c.w64(3, 0xfffffffffffc0000u64);
            let r301 = c.r64(3) & c.r64(11);
            c.w64(3, r301);
            c.logic(r301, 64);
            let Some(t302) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec621); };
            c.logic(t302 & 0x2u64, 8);
            if c.cc_e() { blk = 87; continue; }
            blk = 251; continue;
        }
        251 => {
            c.w64(2, c.r64(9).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ec62f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ec630); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008ec63d, 0x70008ec638);
        }
        252 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ec63d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ec63f); }
            blk = 87; continue;
        }
        253 => {
            c.w32(12, c.r32(12));
            let r303 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r303);
            c.w64(11, 0xfffffffffffc0000u64);
            let r304 = c.r64(11) & c.r64(12);
            c.w64(11, r304);
            c.logic(r304, 64);
            let Some(t305) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec655); };
            c.logic(t305 & 0x2u64, 8);
            if c.cc_e() { blk = 82; continue; }
            blk = 254; continue;
        }
        254 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ec664); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ec665); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008ec672, 0x70008ec66d);
        }
        255 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ec672); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ec674); }
            blk = 82; continue;
        }
        256 => {
            c.w32(11, c.r32(11));
            let r306 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r306);
            c.w64(8, 0xfffffffffffc0000u64);
            let r307 = c.r64(8) & c.r64(11);
            c.w64(8, r307);
            c.logic(r307, 64);
            let Some(t308) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec68a); };
            c.logic(t308 & 0x2u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 257; continue;
        }
        257 => {
            c.w64(9, c.r64(0).wrapping_add(0x33));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ec699); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ec69a); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008ec6a7, 0x70008ec6a2);
        }
        258 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ec6a7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ec6a9); }
            blk = 44; continue;
        }
        259 => {
            c.w32(15, c.r32(15));
            let r309 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r309);
            c.w64(0, 0xfffffffffffc0000u64);
            let r310 = c.r64(0) & c.r64(15);
            c.w64(0, r310);
            c.logic(r310, 64);
            let Some(t311) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ec6bf); };
            c.logic(t311 & 0x2u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 260; continue;
        }
        260 => {
            c.w64(3, c.r64(7).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ec6cd); }
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008ec6d7, 0x70008ec6d2);
        }
        261 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ec6d7); }
            blk = 39; continue;
        }
        262 => {
            return RegfileExit::jump(0x70008ec6e0);
        }
            _ => unreachable!(),
        }
    }
}
