#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000863494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000863494); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000863498); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863499); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086349a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086349f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 64; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008634a9); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008634b7); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008634be); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 84; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008634cb); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008634d2); };
            c.w32(11, t8);
            c.w64(12, c.r64(11));
            let r9 = c.r32(12) & 0x1u64;
            c.w32(12, r9);
            c.logic(r9, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008634e5); };
            c.logic(t10 & 0x4u64, 8);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008634ec); };
            c.w64(7, t11);
            c.w64(11, c.r64(7));
            blk = 8; continue;
        }
        7 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 66; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000863509); };
            c.w64(1, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000863510); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x8, 0x7000863517);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086351a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000863521); }
            return c.ret(0x0, 0x7000863523);
        }
        13 => {
            let Some(t13) = c.ld32(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000863524); };
            c.w32(11, t13);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t14) = c.ld64(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086353c); };
            c.w64(9, t14);
            let Some(t15) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000863540); };
            c.w64(11, t15);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t16) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x700086354a); };
            c.w64(8, t16);
            let r17 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r17);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t18) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086355a); };
            c.w64(8, t18);
            c.w64(0, c.r64(8));
            blk = 42; continue;
        }
        18 => {
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t19) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700086356f); };
            c.w32(8, t19);
            let r20 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r20);
            let r21 = c.r32(8) & 0x3fu64;
            c.w32(8, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 27; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x70008635d4);
        }
        34 => {
            c.w32(8, 0x1u64);
            blk = 38; continue;
        }
        35 => {
            c.w32(8, 0x2u64);
            blk = 38; continue;
        }
        36 => {
            c.w32(8, 0x4u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(8, 0x8u64);
            blk = 38; continue;
        }
        38 => {
            let r22 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r22);
            c.w64(0, c.r64(9));
            c.cqo();
            c.idiv(c.r64(8), 64);
            blk = 42; continue;
        }
        39 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 62; continue;
        }
        40 => {
            c.w64(10, c.r64(4));
            let r23 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r23);
            let r24 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r24);
            c.logic(r24, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000863613); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700086361e); };
            c.w64(0, t25);
            c.w64(10, 0x700086362cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086362c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000863630); }
            return c.call(c.r64(0), 0x7000863636, 0x7000863634);
        }
        41 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000863636); }
            let Some(t26) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700086363e); };
            c.w64(4, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000863643); };
            c.w64(7, t27);
            blk = 42; continue;
        }
        42 => {
            let Some(t28) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000863647); };
            c.w32(7, t28);
            let r29 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r29);
            let Some(t30) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700086364d); };
            c.w32(7, t30);
            let r31 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r31);
            let r32 = c.r32(7) & 0x3fu64;
            c.w32(7, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_l() { blk = 50; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_l() { blk = 47; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x25u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(7), 0x26u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(7), 0x22u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(7), 0x23u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_l() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(7), 0x1fu64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(7), 0x1cu64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(7), 0x1du64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::fault(0x700086369d);
        }
        57 => {
            c.w32(7, 0x1u64);
            blk = 61; continue;
        }
        58 => {
            c.w32(7, 0x2u64);
            blk = 61; continue;
        }
        59 => {
            c.w32(7, 0x4u64);
            blk = 61; continue;
        }
        60 => {
            c.w32(7, 0x8u64);
            blk = 61; continue;
        }
        61 => {
            let r33 = c.imul2(c.r64(7), c.r64(0), 64);
            c.w64(7, r33);
            c.w64(11, c.r64(7));
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 10; continue;
        }
        64 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008636d5); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008636d9); };
            c.w64(3, t34);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008636e7, 0x70008636e2);
        }
        65 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008636e7); };
            c.w64(6, t35);
            blk = 1; continue;
        }
        66 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008636f0); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008636f7); };
            c.w64(8, t37);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008636fe); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000863702); };
            c.w64(12, t38);
            let Some(t39) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000863709); };
            c.cmp(t39, c.r64(9), 64);
            if c.cc_a() { blk = 69; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000863719, 0x7000863714);
        }
        68 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000863719); };
            c.w64(11, t40);
            blk = 70; continue;
        }
        69 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700086371f); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 70; continue;
        }
        70 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 74; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let r41 = c.r64(10) | 0x1u64;
            c.w64(10, r41);
            c.logic(r41, 64);
            blk = 73; continue;
        }
        73 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 74; continue;
        }
        74 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000863750); };
            c.w64(7, t42);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000863757); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700086375a); }
            blk = 10; continue;
        }
        75 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000863764); };
            c.w64(7, t43);
            let Some(t44) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086376b); };
            c.w64(8, t44);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x7000863772); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000863776); };
            c.w64(12, t45);
            let Some(t46) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086377d); };
            c.cmp(t46, c.r64(9), 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700086378d, 0x7000863788);
        }
        77 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086378d); };
            c.w64(11, t47);
            blk = 79; continue;
        }
        78 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000863793); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 79; continue;
        }
        79 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086379d); };
            c.w64(7, t48);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008637a4); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 83; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            let r49 = c.r64(10) | 0x1u64;
            c.w64(10, r49);
            c.logic(r49, 64);
            blk = 82; continue;
        }
        82 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 83; continue;
        }
        83 => {
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008637ce); }
            blk = 10; continue;
        }
        84 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008637de); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008637e0); };
            c.w64(8, t50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008637e7); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008637f1); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008637f3); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008637f4); };
            c.w64(3, t53);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000863805, 0x7000863800);
        }
        85 => {
            return RegfileExit::fault(0x7000863805);
        }
        86 => {
            return RegfileExit::jump(0x7000863808);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000863814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000863814); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000863818); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863819); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086381a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086381f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000863829); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000863836); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086383c); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000863848); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700086384e); };
            c.w32(8, t8);
            c.logic(c.r8(8) & 0x4u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0x33)) else { return RegfileExit::fault(0x700086385c); };
            c.w32(9, t9);
            c.w64(11, c.r64(9));
            let r10 = c.r32(11) & 0x1u64;
            c.w32(11, r10);
            c.logic(r10, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000863875); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000863879); };
            c.w64(8, t12);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000863883); };
            c.w64(9, t13);
            let r14 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r14);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 16; continue;
        }
        10 => {
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000863896); };
            c.w64(1, t15);
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 13; continue;
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008638a6); };
            c.w64(1, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008638ad); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return c.ret(0x8, 0x70008638b4);
        }
        15 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008638b7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008638be); }
            return c.ret(0x0, 0x70008638c0);
        }
        16 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008638c5); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008638c9); };
            c.w64(3, t17);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008638d7, 0x70008638d2);
        }
        18 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008638d7); };
            c.w64(6, t18);
            blk = 1; continue;
        }
        19 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008638e0); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008638e7); };
            c.w64(8, t20);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008638ee); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008638f2); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008638f9); };
            c.cmp(t22, c.r64(9), 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000863908, 0x7000863903);
        }
        21 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000863908); };
            c.w64(1, t23);
            blk = 23; continue;
        }
        22 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700086390e); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 23; continue;
        }
        23 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let r24 = c.r64(10) | 0x1u64;
            c.w64(10, r24);
            c.logic(r24, 64);
            blk = 26; continue;
        }
        26 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 27; continue;
        }
        27 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086393f); };
            c.w64(1, t25);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000863946); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000863949); }
            blk = 13; continue;
        }
        28 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863958); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000863959); };
            c.w64(7, t26);
            let Some(t27) = c.ld32(c.r64(7).wrapping_add(0xcb)) else { return RegfileExit::fault(0x7000863960); };
            c.w32(7, t27);
            let r28 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r28);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863969); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086396a); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086396b); };
            c.w64(3, t29);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700086397c, 0x7000863977);
        }
        29 => {
            return RegfileExit::fault(0x700086397c);
        }
        30 => {
            return RegfileExit::jump(0x7000863980);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000863994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000863994); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000863998); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863999); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086399a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086399f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008639a9); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008639b6); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008639bc); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008639c8); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008639cf); };
            c.w32(9, t8);
            c.w64(11, c.r64(9));
            let r9 = c.r32(11) & 0x1u64;
            c.w32(11, r9);
            c.logic(r9, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_ne() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008639e2); };
            c.logic(t10 & 0x4u64, 8);
            if c.cc_ne() { blk = 33; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008639ed); };
            c.w64(7, t11);
            c.w64(0, c.r64(7));
            blk = 39; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008639f9); };
            c.w32(9, t12);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_ne() { blk = 33; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000863a11); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000863a15); };
            c.w64(9, t14);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000863a1f); };
            c.w64(7, t15);
            let r16 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r16);
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t17) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000863a2f); };
            c.w64(7, t17);
            c.w64(0, c.r64(7));
            blk = 39; continue;
        }
        12 => {
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000863a44); };
            c.w32(7, t18);
            let r19 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r19);
            let r20 = c.r32(7) & 0x3fu64;
            c.w32(7, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_l() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_l() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(7), 0x25u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(7), 0x26u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(7), 0x22u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(7), 0x23u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_l() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(7), 0x1fu64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(7), 0x1cu64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(7), 0x1du64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x7000863a94);
        }
        28 => {
            c.w32(7, 0x1u64);
            blk = 32; continue;
        }
        29 => {
            c.w32(7, 0x2u64);
            blk = 32; continue;
        }
        30 => {
            c.w32(7, 0x4u64);
            blk = 32; continue;
        }
        31 => {
            c.w32(7, 0x8u64);
            blk = 32; continue;
        }
        32 => {
            let r21 = c.sub(c.r64(8), c.r64(9), 64);
            c.w64(8, r21);
            c.w64(0, c.r64(8));
            c.cqo();
            c.idiv(c.r64(7), 64);
            blk = 39; continue;
        }
        33 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        34 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000863abe); };
            c.w64(1, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000863ac5); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            return c.ret(0x8, 0x7000863acc);
        }
        36 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000863acf); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000863ad6); }
            return c.ret(0x0, 0x7000863ad8);
        }
        37 => {
            c.w64(10, c.r64(4));
            let r23 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r23);
            let r24 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r24);
            c.logic(r24, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000863ae4); }
            c.w64(1, c.r64(13));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000863aec); };
            c.w64(0, t25);
            c.w64(10, 0x7000863afau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000863afa); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000863afe); }
            return c.call(c.r64(0), 0x7000863b04, 0x7000863b02);
        }
        38 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000863b04); }
            let Some(t26) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000863b0c); };
            c.w64(4, t26);
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(0), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 40; continue;
        }
        40 => {
            let r27 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            blk = 34; continue;
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000863b1d); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000863b21); };
            c.w64(3, t28);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000863b2f, 0x7000863b2a);
        }
        42 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000863b2f); };
            c.w64(6, t29);
            blk = 1; continue;
        }
        43 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000863b38); };
            c.w64(7, t30);
            let Some(t31) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000863b3f); };
            c.w64(8, t31);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000863b46); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000863b4a); };
            c.w64(11, t32);
            let Some(t33) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000863b51); };
            c.cmp(t33, c.r64(9), 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000863b60, 0x7000863b5b);
        }
        45 => {
            c.w64(7, c.r64(0));
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000863b63); };
            c.w64(0, t34);
            blk = 47; continue;
        }
        46 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000863b69); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(0) as i64));
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ns() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(10, c.r64(0));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let r35 = c.r64(10) | 0x1u64;
            c.w64(10, r35);
            c.logic(r35, 64);
            blk = 50; continue;
        }
        50 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 51; continue;
        }
        51 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000863b97); };
            c.w64(8, t36);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000863b9e); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000863ba2); }
            c.w64(0, c.r64(7));
            blk = 34; continue;
        }
        52 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863bb4); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000863bb5); };
            c.w64(7, t37);
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(0xcf)) else { return RegfileExit::fault(0x7000863bbc); };
            c.w32(7, t38);
            let r39 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r39);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863bc5); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000863bc6); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000863bc7); };
            c.w64(3, t40);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000863bd8, 0x7000863bd3);
        }
        53 => {
            return RegfileExit::fault(0x7000863bd8);
        }
        54 => {
            return RegfileExit::jump(0x7000863bdc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000863bf4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1bd0)) else { return RegfileExit::fault(0x7000863bf4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000863c14(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1bd8)) else { return RegfileExit::fault(0x7000863c14); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000863c34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1be0)) else { return RegfileExit::fault(0x7000863c34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000863c54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1be8)) else { return RegfileExit::fault(0x7000863c54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000863c74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1bf0)) else { return RegfileExit::fault(0x7000863c74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000863c94(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1bf8)) else { return RegfileExit::fault(0x7000863c94); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000863cb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000863cb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000863cb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863cb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000863cba); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000863cbb); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000863cc5); };
            c.w64(2, t1);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000863cd2); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000863cd8); };
            c.w32(2, t4);
            let r5 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r5);
            let r6 = c.r32(2) & 0x3fu64;
            c.w32(2, r6);
            c.logic(r6, 32);
            let r7 = c.sub(c.r32(2), 0x11u64, 32);
            c.w32(2, r7);
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_l() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(2), 0x9u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(2), 0xau64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 35; continue;
        }
        8 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(2), 0x6u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(2), 0x7u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 35; continue;
        }
        12 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x4u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 35; continue;
        }
        17 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 35; continue;
        }
        20 => {
            blk = 35; continue;
        }
        21 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xb20)) else { return RegfileExit::fault(0x7000863d59); };
            c.w64(0, t8);
            blk = 22; continue;
        }
        22 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000863d60); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000863d67); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return c.ret(0x8, 0x7000863d6e);
        }
        24 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000863d71); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000863d78); }
            return c.ret(0x0, 0x7000863d7a);
        }
        25 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x7e8)) else { return RegfileExit::fault(0x7000863d7b); };
            c.w64(0, t10);
            blk = 22; continue;
        }
        26 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xb10)) else { return RegfileExit::fault(0x7000863d84); };
            c.w64(0, t11);
            blk = 22; continue;
        }
        27 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x7d8)) else { return RegfileExit::fault(0x7000863d8d); };
            c.w64(0, t12);
            blk = 22; continue;
        }
        28 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb18)) else { return RegfileExit::fault(0x7000863d96); };
            c.w64(0, t13);
            blk = 22; continue;
        }
        29 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x7e0)) else { return RegfileExit::fault(0x7000863d9f); };
            c.w64(0, t14);
            blk = 22; continue;
        }
        30 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x708)) else { return RegfileExit::fault(0x7000863da8); };
            c.w64(0, t15);
            blk = 22; continue;
        }
        31 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x710)) else { return RegfileExit::fault(0x7000863db1); };
            c.w64(0, t16);
            blk = 22; continue;
        }
        32 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xb28)) else { return RegfileExit::fault(0x7000863dba); };
            c.w64(0, t17);
            blk = 22; continue;
        }
        33 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x560)) else { return RegfileExit::fault(0x7000863dc3); };
            c.w64(0, t18);
            blk = 22; continue;
        }
        34 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x558)) else { return RegfileExit::fault(0x7000863dcc); };
            c.w64(0, t19);
            blk = 22; continue;
        }
        35 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000863dd5); };
            c.w64(0, t20);
            blk = 22; continue;
        }
        36 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000863dde); };
            c.w64(3, t21);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000863dec, 0x7000863de7);
        }
        37 => {
            blk = 1; continue;
        }
        38 => {
            return RegfileExit::jump(0x7000863df4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000863e14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000863e14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000863e18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863e19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000863e1a); }
            let r0 = c.sub(c.r64(4), 0x78u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000863e1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000863e23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 516; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000863e3b); };
            c.w64(11, t2);
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000863e43); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(11)) { return RegfileExit::fault(0x7000863e47); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000863e51); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000863e57); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000863e5e); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000863e68); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000863e6e); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x7000863e75); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 730; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000863e83); };
            c.w32(15, t7);
            let r8 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r8);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000863e8a); }
            let Some(t9) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000863e8e); };
            c.cmp(t9, 0x82eu64, 16);
            if c.cc_ne() { blk = 730; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000863e9b); };
            c.w32(0, t10);
            let r11 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r11);
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(0)) { return RegfileExit::fault(0x7000863ea2); }
            let Some(t12) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000863ea9); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 728; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000863eb3); };
            c.w64(3, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000863eb7); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 726; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000863ec5); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r15);
            let Some(t16) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000863ecc); };
            c.logic(t16 & 0x2u64, 8);
            if c.cc_e() { blk = 726; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 518; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r64(3).wrapping_add(c.r64(3)));
            blk = 14; continue;
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000863ee6); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000863eed); };
            c.w32(7, t19);
            let r20 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r20);
            let r21 = c.r32(7) & 0x3fu64;
            c.w32(7, r21);
            c.logic(r21, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000863ef7); }
            c.cmp(c.r32(7), 0x1cu64, 32);
            if c.cc_l() { blk = 33; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_l() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_l() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(7), 0x25u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(7), 0x26u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 527; continue;
        }
        21 => {
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(7), 0x22u64, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(7), 0x23u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 527; continue;
        }
        25 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_l() { blk = 30; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(7), 0x1fu64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 29; continue;
        }
        29 => {
            blk = 527; continue;
        }
        30 => {
            c.cmp(c.r32(7), 0x1cu64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(7), 0x1du64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 527; continue;
        }
        33 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_l() { blk = 43; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(7), 0x19u64, 32);
            if c.cc_l() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(7), 0x19u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(7), 0x1au64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(7), 0x1bu64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 38; continue;
        }
        38 => {
            blk = 527; continue;
        }
        39 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(7), 0x17u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(7), 0x18u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 527; continue;
        }
        43 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 527; continue;
        }
        48 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 527; continue;
        }
        51 => {
            c.w32(7, 0x10u64);
            blk = 73; continue;
        }
        52 => {
            c.w32(7, 0x11u64);
            blk = 73; continue;
        }
        53 => {
            c.w32(7, 0x12u64);
            blk = 73; continue;
        }
        54 => {
            c.w32(7, 0x13u64);
            blk = 73; continue;
        }
        55 => {
            c.w32(7, 0x14u64);
            blk = 73; continue;
        }
        56 => {
            c.w32(7, 0x15u64);
            blk = 73; continue;
        }
        57 => {
            c.w32(7, 0x16u64);
            blk = 73; continue;
        }
        58 => {
            c.w32(7, 0x17u64);
            blk = 73; continue;
        }
        59 => {
            c.w32(7, 0x18u64);
            blk = 73; continue;
        }
        60 => {
            c.w32(7, 0x19u64);
            blk = 73; continue;
        }
        61 => {
            c.w32(7, 0x1au64);
            blk = 73; continue;
        }
        62 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        63 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        64 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        65 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        66 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        67 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        68 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        69 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        70 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        71 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        72 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 73; continue;
        }
        73 => {
            let Some(t22) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864082); };
            c.w32(2, t22);
            let r23 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r23);
            let Some(t24) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000864088); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086408e); };
            c.w32(8, t26);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_b() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            return RegfileExit::fault(0x700086409d);
        }
        75 => {
            let Some(t27) = c.ld32(c.r64(2).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086409e); };
            c.w32(7, t27);
            let r28 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r28);
            c.cmp(c.r64(8), 0xd4u64, 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x70008640ae);
        }
        77 => {
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008640af); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x357)) else { return RegfileExit::fault(0x70008640b6); };
            c.w32(2, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008640bc); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008640c3); };
            c.cmp(t34, 0xbdu64, 16);
            if c.cc_ae() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008640cc); };
            c.w64(8, t35);
            blk = 80; continue;
        }
        79 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008640d5); };
            c.w32(8, t36);
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(2), c.r32(8), 32);
            if c.cc_ne() { blk = 528; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xfc8)) else { return RegfileExit::fault(0x70008640e5); };
            c.w64(2, t37);
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008640ec); };
            c.cmp(t38, c.r32(8), 32);
            if c.cc_e() { blk = 528; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008640f6); };
            c.w64(2, t39);
            c.w64(0, c.r64(7));
            c.w64(3, c.r64(7));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864103); };
            c.w64(1, t40);
            c.w64(7, c.r64(2));
            return c.call(0x70008dd9b4, 0x700086410f, 0x700086410a);
        }
        83 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700086410f); }
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000864113); };
            c.w64(8, t41);
            let Some(t42) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000864117); };
            c.cmp(t42, c.r64(8), 64);
            if c.cc_b() { blk = 720; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864121); };
            c.w64(9, t43);
            let Some(t44) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000864125); };
            c.w32(9, t44);
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086412a); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000864131); };
            c.w64(12, t47);
            let Some(t48) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864135); };
            c.w32(15, t48);
            let r49 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r49);
            let r50 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r50);
            let Some(t51) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000864141); };
            c.w32(11, t51);
            let Some(t52) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000864146); };
            c.w32(15, t52);
            let r53 = c.r32(9) & 0x3fu64;
            c.w32(9, r53);
            c.logic(r53, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(11)) { return RegfileExit::fault(0x700086414f); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(15)) { return RegfileExit::fault(0x7000864156); }
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_l() { blk = 92; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 89; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 361; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 420; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 467; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 261; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 295; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 328; continue; }
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_l() { blk = 96; continue; }
            blk = 93; continue;
        }
        93 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 98; continue;
        }
        98 => {
            return RegfileExit::fault(0x70008641d6);
        }
        99 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 130; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 102; continue;
        }
        101 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008641e5); };
            c.w64(11, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008641ec); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008641f0); };
            c.w64(12, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008641f4); };
            c.w64(15, t57);
            blk = 102; continue;
        }
        102 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008641f8); };
            c.w64(9, t58);
            let Some(t59) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008641ff); };
            c.logic(t59 & 0x4u64, 8);
            if c.cc_ne() { blk = 130; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t60) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086420a); };
            c.w64(3, t60);
            let Some(t61) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700086420f); };
            c.w32(2, t61);
            c.w64(1, c.r64(11));
            let r62 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r62);
            c.w64(6, c.r64(15));
            let r63 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r63);
            let r64 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r64);
            let r65 = c.r32(1) & 0x3fu64;
            c.w32(1, r65);
            c.logic(r65, 32);
            let r66 = c.r32(6) & 0x3fu64;
            c.w32(6, r66);
            c.logic(r66, 32);
            let Some(t67) = c.ld8(c.r64(7).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000864229); };
            c.w32(3, t67);
            c.cmp(c.r32(1), c.r32(6), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000864235); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000864239); }
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 550; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w32(1, c.r64(7).wrapping_add(c.r64(7)));
            blk = 105; continue;
        }
        105 => {
            let r68 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r68);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700086424f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000864251); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000864252); }
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000864253); };
            if !c.push64(t69) { return RegfileExit::fault(0x7000864253); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000864256); }
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086425a); };
            c.w64(6, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086425e); };
            c.w64(7, t71);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x700086426f, 0x700086426a);
        }
        106 => {
            let Some(t72) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086426f); };
            c.cmp(t72, 0x0u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(1, c.r64(0));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864278); };
            c.w64(0, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086427c); };
            c.w64(3, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864280); };
            c.w64(6, t75);
            return c.call(0x7000871cb4, 0x7000864289, 0x7000864284);
        }
        108 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864289); };
            c.w64(0, t76);
            blk = 129; continue;
        }
        109 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 559; continue; }
            blk = 110; continue;
        }
        110 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x700086429a); }
            let Some(t77) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086429e); };
            c.logic(t77 & 0x1u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008642a4); };
            c.w64(1, t78);
            let Some(t79) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008642a8); };
            c.w32(8, t79);
            let r80 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r80);
            let Some(t81) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008642af); };
            c.cmp(t81, 0x42u64, 16);
            if c.cc_ne() { blk = 562; continue; }
            blk = 112; continue;
        }
        112 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008642bb); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 562; continue; }
            blk = 113; continue;
        }
        113 => {
            if c.cc_ne() { blk = 562; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w64(9, 0x1fffffffffffffu64);
            let r82 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r82);
            c.w64(11, 0x3ffffffffffffeu64);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 117; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 562; continue;
        }
        116 => {
            let Some(t83) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008642ff); };
            c.w64(8, (t83 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 117; continue;
        }
        117 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000864306); }
            c.w64(9, c.r64(0));
            blk = 120; continue;
        }
        118 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086430f); };
            c.w64(6, t84);
            return c.call(0x70008a1014, 0x7000864318, 0x7000864313);
        }
        119 => {
            c.w64(9, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086431b); };
            c.w64(8, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086431f); };
            c.w64(0, t86);
            blk = 120; continue;
        }
        120 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t87) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864329); };
            c.w32(11, t87);
            let r88 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r88);
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000864330); };
            c.cmp(t89, c.r32(11), 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t90) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864339); };
            c.cmp(t90, 0x43u64, 16);
            if c.cc_ne() { blk = 118; continue; }
            blk = 123; continue;
        }
        123 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864341); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 744; continue; }
            blk = 124; continue;
        }
        124 => {
            c.w32(9, c.r32(9));
            blk = 126; continue;
        }
        125 => {
            let r91 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r91);
            blk = 126; continue;
        }
        126 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086435e); };
            c.w64(0, t92);
            let Some(t93) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000864362); };
            c.w32(11, t93);
            let r94 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r94);
            let Some(t95) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864369); };
            c.logic(t95 & 0x4u64, 8);
            if c.cc_ne() { blk = 562; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t96) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000864374); };
            c.cmp(t96, c.r64(8), 64);
            if c.cc_be() { blk = 562; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t97) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086437e); };
            c.w64(11, t97);
            let Some(t98) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864382); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r99);
            if !c.st8(c.r64(8).wrapping_add(c.r64(11)), c.r8(9)) { return RegfileExit::fault(0x7000864389); }
            blk = 129; continue;
        }
        129 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086438d); };
            c.w64(7, t100);
            let r101 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000864395); };
            c.cmp(c.r64(7), t102, 64);
            if c.cc_ne() { blk = 101; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086439f); };
            c.w64(9, t103);
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008643aa); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008643ab); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008643b1); }
            return c.ret(0x0, 0x70008643b3);
        }
        131 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 162; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 134; continue;
        }
        133 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008643c2); };
            c.w64(12, t104);
            blk = 134; continue;
        }
        134 => {
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008643c6); };
            c.w64(9, t105);
            let Some(t106) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008643cd); };
            c.logic(t106 & 0x4u64, 8);
            if c.cc_ne() { blk = 162; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t107) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008643d8); };
            c.w64(3, t107);
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008643dd); };
            c.w32(2, t108);
            let r109 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r109);
            let Some(t110) = c.ld8(c.r64(7).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70008643e5); };
            c.w32(3, (t110 as u8 as i8 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008643e9); }
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 564; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            blk = 137; continue;
        }
        137 => {
            let r111 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r111);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008643ff); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000864401); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000864402); }
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000864403); };
            if !c.push64(t112) { return RegfileExit::fault(0x7000864403); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000864406); }
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086440a); };
            c.w64(6, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086440e); };
            c.w64(7, t114);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x700086441f, 0x700086441a);
        }
        138 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700086441f); };
            c.w64(8, t115);
            let r116 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700086442a); };
            c.w64(9, t117);
            let r118 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r118);
            let r119 = c.r32(8) & 0x3fu64;
            c.w32(8, r119);
            c.logic(r119, 32);
            let r120 = c.r32(9) & 0x3fu64;
            c.w32(9, r120);
            c.logic(r120, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(1, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864442); };
            c.w64(0, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864446); };
            c.w64(3, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086444a); };
            c.w64(6, t123);
            return c.call(0x7000871cb4, 0x7000864453, 0x700086444e);
        }
        140 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864453); };
            c.w64(0, t124);
            blk = 161; continue;
        }
        141 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 573; continue; }
            blk = 142; continue;
        }
        142 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000864464); }
            let Some(t125) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864468); };
            c.logic(t125 & 0x1u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086446e); };
            c.w64(2, t126);
            let Some(t127) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864472); };
            c.w32(8, t127);
            let r128 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r128);
            let Some(t129) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864479); };
            c.cmp(t129, 0x42u64, 16);
            if c.cc_ne() { blk = 576; continue; }
            blk = 144; continue;
        }
        144 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864485); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 576; continue; }
            blk = 145; continue;
        }
        145 => {
            if c.cc_ne() { blk = 576; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t130) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x70008644a8); };
            c.w64(9, t130);
            let r131 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r131);
            let Some(t132) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x70008644b2); };
            c.w64(11, t132);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 149; continue; }
            blk = 147; continue;
        }
        147 => {
            blk = 576; continue;
        }
        148 => {
            let Some(t133) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008644c3); };
            c.w64(8, (t133 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 149; continue;
        }
        149 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008644ca); }
            c.w64(9, c.r64(0));
            blk = 152; continue;
        }
        150 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008644d3); };
            c.w64(6, t134);
            return c.call(0x70008a1014, 0x70008644dc, 0x70008644d7);
        }
        151 => {
            c.w64(9, c.r64(0));
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008644df); };
            c.w64(8, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008644e3); };
            c.w64(0, t136);
            blk = 152; continue;
        }
        152 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 157; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t137) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008644ed); };
            c.w32(11, t137);
            let r138 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r138);
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008644f4); };
            c.cmp(t139, c.r32(11), 32);
            if c.cc_e() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t140) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008644fd); };
            c.cmp(t140, 0x43u64, 16);
            if c.cc_ne() { blk = 150; continue; }
            blk = 155; continue;
        }
        155 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864505); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 742; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w32(9, c.r32(9));
            blk = 158; continue;
        }
        157 => {
            let r141 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r141);
            blk = 158; continue;
        }
        158 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864522); };
            c.w64(0, t142);
            let Some(t143) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000864526); };
            c.w32(11, t143);
            let r144 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r144);
            let Some(t145) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700086452d); };
            c.logic(t145 & 0x4u64, 8);
            if c.cc_ne() { blk = 576; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t146) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000864538); };
            c.cmp(t146, c.r64(8), 64);
            if c.cc_be() { blk = 576; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t147) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864542); };
            c.w64(11, t147);
            let Some(t148) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864546); };
            c.w32(12, t148);
            let r149 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r149);
            if !c.st8(c.r64(8).wrapping_add(c.r64(11)), c.r8(9)) { return RegfileExit::fault(0x700086454d); }
            blk = 161; continue;
        }
        161 => {
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000864551); };
            c.w64(7, t150);
            let r151 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000864559); };
            c.cmp(c.r64(7), t152, 64);
            if c.cc_ne() { blk = 133; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000864563); };
            c.w64(8, t153);
            let r154 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r154);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086456e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086456f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000864575); }
            return c.ret(0x0, 0x7000864577);
        }
        163 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 194; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 166; continue;
        }
        165 => {
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000864586); };
            c.w64(12, t155);
            blk = 166; continue;
        }
        166 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700086458a); };
            c.w64(9, t156);
            let Some(t157) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864591); };
            c.logic(t157 & 0x4u64, 8);
            if c.cc_ne() { blk = 194; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t158) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086459c); };
            c.w64(3, t158);
            let Some(t159) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008645a1); };
            c.w32(2, t159);
            let r160 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r160);
            let Some(t161) = c.ld16(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x70008645a9); };
            c.w32(3, t161);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008645ad); }
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 578; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            blk = 169; continue;
        }
        169 => {
            let r162 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r162);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008645c3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008645c5); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008645c6); }
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008645c7); };
            if !c.push64(t163) { return RegfileExit::fault(0x70008645c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008645ca); }
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008645ce); };
            c.w64(6, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008645d2); };
            c.w64(7, t165);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x70008645e3, 0x70008645de);
        }
        170 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008645e3); };
            c.w64(8, t166);
            let r167 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008645ee); };
            c.w64(9, t168);
            let r169 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r169);
            let r170 = c.r32(8) & 0x3fu64;
            c.w32(8, r170);
            c.logic(r170, 32);
            let r171 = c.r32(9) & 0x3fu64;
            c.w32(9, r171);
            c.logic(r171, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(1, c.r64(0));
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864606); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086460a); };
            c.w64(3, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086460e); };
            c.w64(6, t174);
            return c.call(0x7000871cb4, 0x7000864617, 0x7000864612);
        }
        172 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864617); };
            c.w64(0, t175);
            blk = 193; continue;
        }
        173 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 587; continue; }
            blk = 174; continue;
        }
        174 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000864628); }
            let Some(t176) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086462c); };
            c.logic(t176 & 0x1u64, 8);
            if c.cc_e() { blk = 180; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864632); };
            c.w64(2, t177);
            let Some(t178) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864636); };
            c.w32(8, t178);
            let r179 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r179);
            let Some(t180) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086463d); };
            c.cmp(t180, 0x42u64, 16);
            if c.cc_ne() { blk = 590; continue; }
            blk = 176; continue;
        }
        176 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864649); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 590; continue; }
            blk = 177; continue;
        }
        177 => {
            if c.cc_ne() { blk = 590; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t181) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x700086466c); };
            c.w64(9, t181);
            let r182 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x7000864676); };
            c.w64(11, t183);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 181; continue; }
            blk = 179; continue;
        }
        179 => {
            blk = 590; continue;
        }
        180 => {
            let Some(t184) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864687); };
            c.w64(8, (t184 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 181; continue;
        }
        181 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086468e); }
            c.w64(9, c.r64(0));
            blk = 184; continue;
        }
        182 => {
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864697); };
            c.w64(6, t185);
            return c.call(0x70008a1014, 0x70008646a0, 0x700086469b);
        }
        183 => {
            c.w64(9, c.r64(0));
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008646a3); };
            c.w64(8, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008646a7); };
            c.w64(0, t187);
            blk = 184; continue;
        }
        184 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 189; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t188) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008646b1); };
            c.w32(11, t188);
            let r189 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008646b8); };
            c.cmp(t190, c.r32(11), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t191) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008646c1); };
            c.cmp(t191, 0x43u64, 16);
            if c.cc_ne() { blk = 182; continue; }
            blk = 187; continue;
        }
        187 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008646c9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 740; continue; }
            blk = 188; continue;
        }
        188 => {
            c.w32(9, c.r32(9));
            blk = 190; continue;
        }
        189 => {
            let r192 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r192);
            blk = 190; continue;
        }
        190 => {
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008646e6); };
            c.w64(0, t193);
            let Some(t194) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008646ea); };
            c.w32(11, t194);
            let r195 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r195);
            let Some(t196) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008646f1); };
            c.logic(t196 & 0x4u64, 8);
            if c.cc_ne() { blk = 590; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t197) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008646fc); };
            c.cmp(t197, c.r64(8), 64);
            if c.cc_be() { blk = 590; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t198) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864706); };
            c.w64(11, t198);
            let Some(t199) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700086470a); };
            c.w32(12, t199);
            let r200 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r200);
            if !c.st16(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(2)), c.r16(9)) { return RegfileExit::fault(0x7000864711); }
            blk = 193; continue;
        }
        193 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000864716); };
            c.w64(7, t201);
            let r202 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086471e); };
            c.cmp(c.r64(7), t203, 64);
            if c.cc_ne() { blk = 165; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000864728); };
            c.w64(8, t204);
            let r205 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r205);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000864733); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000864734); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086473a); }
            return c.ret(0x0, 0x700086473c);
        }
        195 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 226; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 198; continue;
        }
        197 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700086474b); };
            c.w64(12, t206);
            blk = 198; continue;
        }
        198 => {
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700086474f); };
            c.w64(9, t207);
            let Some(t208) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864756); };
            c.logic(t208 & 0x4u64, 8);
            if c.cc_ne() { blk = 226; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t209) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864761); };
            c.w64(3, t209);
            let Some(t210) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864766); };
            c.w32(2, t210);
            let r211 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r211);
            let Some(t212) = c.ld16(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x700086476e); };
            c.w32(3, (t212 as u16 as i16 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000864772); }
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 592; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            blk = 201; continue;
        }
        201 => {
            let r213 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r213);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000864788); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086478a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086478b); }
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700086478c); };
            if !c.push64(t214) { return RegfileExit::fault(0x700086478c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700086478f); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864793); };
            c.w64(6, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000864797); };
            c.w64(7, t216);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x70008647a8, 0x70008647a3);
        }
        202 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008647a8); };
            c.w64(8, t217);
            let r218 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008647b3); };
            c.w64(9, t219);
            let r220 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r220);
            let r221 = c.r32(8) & 0x3fu64;
            c.w32(8, r221);
            c.logic(r221, 32);
            let r222 = c.r32(9) & 0x3fu64;
            c.w32(9, r222);
            c.logic(r222, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w64(1, c.r64(0));
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008647cb); };
            c.w64(0, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008647cf); };
            c.w64(3, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008647d3); };
            c.w64(6, t225);
            return c.call(0x7000871cb4, 0x70008647dc, 0x70008647d7);
        }
        204 => {
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008647dc); };
            c.w64(0, t226);
            blk = 225; continue;
        }
        205 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 601; continue; }
            blk = 206; continue;
        }
        206 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008647ed); }
            let Some(t227) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008647f1); };
            c.logic(t227 & 0x1u64, 8);
            if c.cc_e() { blk = 212; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008647f7); };
            c.w64(2, t228);
            let Some(t229) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008647fb); };
            c.w32(8, t229);
            let r230 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r230);
            let Some(t231) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864802); };
            c.cmp(t231, 0x42u64, 16);
            if c.cc_ne() { blk = 604; continue; }
            blk = 208; continue;
        }
        208 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700086480e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 604; continue; }
            blk = 209; continue;
        }
        209 => {
            if c.cc_ne() { blk = 604; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t232) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x7000864831); };
            c.w64(9, t232);
            let r233 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r233);
            let Some(t234) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x700086483b); };
            c.w64(11, t234);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 213; continue; }
            blk = 211; continue;
        }
        211 => {
            blk = 604; continue;
        }
        212 => {
            let Some(t235) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086484c); };
            c.w64(8, (t235 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 213; continue;
        }
        213 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000864853); }
            c.w64(9, c.r64(0));
            blk = 216; continue;
        }
        214 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086485c); };
            c.w64(6, t236);
            return c.call(0x70008a1014, 0x7000864865, 0x7000864860);
        }
        215 => {
            c.w64(9, c.r64(0));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864868); };
            c.w64(8, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086486c); };
            c.w64(0, t238);
            blk = 216; continue;
        }
        216 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 221; continue; }
            blk = 217; continue;
        }
        217 => {
            let Some(t239) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864876); };
            c.w32(11, t239);
            let r240 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r240);
            let Some(t241) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086487d); };
            c.cmp(t241, c.r32(11), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t242) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864886); };
            c.cmp(t242, 0x43u64, 16);
            if c.cc_ne() { blk = 214; continue; }
            blk = 219; continue;
        }
        219 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700086488e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 738; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w32(9, c.r32(9));
            blk = 222; continue;
        }
        221 => {
            let r243 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r243);
            blk = 222; continue;
        }
        222 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008648ab); };
            c.w64(0, t244);
            let Some(t245) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008648af); };
            c.w32(11, t245);
            let r246 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r246);
            let Some(t247) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008648b6); };
            c.logic(t247 & 0x4u64, 8);
            if c.cc_ne() { blk = 604; continue; }
            blk = 223; continue;
        }
        223 => {
            let Some(t248) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008648c1); };
            c.cmp(t248, c.r64(8), 64);
            if c.cc_be() { blk = 604; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t249) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008648cb); };
            c.w64(11, t249);
            let Some(t250) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008648cf); };
            c.w32(12, t250);
            let r251 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r251);
            if !c.st16(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(2)), c.r16(9)) { return RegfileExit::fault(0x70008648d6); }
            blk = 225; continue;
        }
        225 => {
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008648db); };
            c.w64(7, t252);
            let r253 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008648e3); };
            c.cmp(c.r64(7), t254, 64);
            if c.cc_ne() { blk = 197; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008648ed); };
            c.w64(8, t255);
            let r256 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r256);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008648f8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008648f9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008648ff); }
            return c.ret(0x0, 0x7000864901);
        }
        227 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 260; continue; }
            blk = 228; continue;
        }
        228 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 230; continue;
        }
        229 => {
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000864910); };
            c.w64(12, t257);
            blk = 230; continue;
        }
        230 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000864914); };
            c.w64(9, t258);
            let Some(t259) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700086491b); };
            c.logic(t259 & 0x4u64, 8);
            if c.cc_ne() { blk = 260; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t260) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864926); };
            c.w64(3, t260);
            let Some(t261) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700086492b); };
            c.w32(2, t261);
            let r262 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r262);
            let Some(t263) = c.ld32(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(4))) else { return RegfileExit::fault(0x7000864933); };
            c.w32(3, t263);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000864936); }
            c.cmp(c.r32(3), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 606; continue; }
            blk = 232; continue;
        }
        232 => {
            let r264 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r264);
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 611; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            blk = 235; continue;
        }
        235 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000864958); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086495a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086495b); }
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700086495c); };
            if !c.push64(t265) { return RegfileExit::fault(0x700086495c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700086495f); }
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864963); };
            c.w64(6, t266);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000864967); };
            c.w64(7, t267);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x7000864978, 0x7000864973);
        }
        236 => {
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000864978); };
            c.w64(8, t268);
            let r269 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000864983); };
            c.w64(9, t270);
            let r271 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r271);
            let r272 = c.r32(8) & 0x3fu64;
            c.w32(8, r272);
            c.logic(r272, 32);
            let r273 = c.r32(9) & 0x3fu64;
            c.w32(9, r273);
            c.logic(r273, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w64(1, c.r64(0));
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086499b); };
            c.w64(0, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086499f); };
            c.w64(3, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008649a3); };
            c.w64(6, t276);
            return c.call(0x7000871cb4, 0x70008649ac, 0x70008649a7);
        }
        238 => {
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008649ac); };
            c.w64(0, t277);
            blk = 259; continue;
        }
        239 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 620; continue; }
            blk = 240; continue;
        }
        240 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008649bd); }
            let Some(t278) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008649c1); };
            c.logic(t278 & 0x1u64, 8);
            if c.cc_e() { blk = 246; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008649c7); };
            c.w64(2, t279);
            let Some(t280) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008649cb); };
            c.w32(8, t280);
            let r281 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r281);
            let Some(t282) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008649d2); };
            c.cmp(t282, 0x42u64, 16);
            if c.cc_ne() { blk = 623; continue; }
            blk = 242; continue;
        }
        242 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008649de); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 623; continue; }
            blk = 243; continue;
        }
        243 => {
            if c.cc_ne() { blk = 623; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t283) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x7000864a01); };
            c.w64(9, t283);
            let r284 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r284);
            let Some(t285) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x7000864a0b); };
            c.w64(11, t285);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 247; continue; }
            blk = 245; continue;
        }
        245 => {
            blk = 623; continue;
        }
        246 => {
            let Some(t286) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864a1c); };
            c.w64(8, (t286 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 247; continue;
        }
        247 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000864a23); }
            c.w64(9, c.r64(0));
            blk = 250; continue;
        }
        248 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864a2c); };
            c.w64(6, t287);
            return c.call(0x70008a1014, 0x7000864a35, 0x7000864a30);
        }
        249 => {
            c.w64(9, c.r64(0));
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864a38); };
            c.w64(8, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000864a3c); };
            c.w64(0, t289);
            blk = 250; continue;
        }
        250 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t290) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864a46); };
            c.w32(11, t290);
            let r291 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r291);
            let Some(t292) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000864a4d); };
            c.cmp(t292, c.r32(11), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t293) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864a56); };
            c.cmp(t293, 0x43u64, 16);
            if c.cc_ne() { blk = 248; continue; }
            blk = 253; continue;
        }
        253 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864a5e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 736; continue; }
            blk = 254; continue;
        }
        254 => {
            c.w32(9, c.r32(9));
            blk = 256; continue;
        }
        255 => {
            let r294 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r294);
            blk = 256; continue;
        }
        256 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864a7b); };
            c.w64(0, t295);
            let Some(t296) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000864a7f); };
            c.w32(11, t296);
            let r297 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r297);
            let Some(t298) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864a86); };
            c.logic(t298 & 0x4u64, 8);
            if c.cc_ne() { blk = 623; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t299) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000864a91); };
            c.cmp(t299, c.r64(8), 64);
            if c.cc_be() { blk = 623; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t300) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864a9b); };
            c.w64(11, t300);
            let Some(t301) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864a9f); };
            c.w32(12, t301);
            let r302 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r302);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)), c.r32(9)) { return RegfileExit::fault(0x7000864aa6); }
            blk = 259; continue;
        }
        259 => {
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000864aaa); };
            c.w64(7, t303);
            let r304 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000864ab2); };
            c.cmp(c.r64(7), t305, 64);
            if c.cc_ne() { blk = 229; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000864abc); };
            c.w64(8, t306);
            let r307 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r307);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000864ac7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000864ac8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000864ace); }
            return c.ret(0x0, 0x7000864ad0);
        }
        261 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 294; continue; }
            blk = 262; continue;
        }
        262 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 264; continue;
        }
        263 => {
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000864adf); };
            c.w64(12, t308);
            blk = 264; continue;
        }
        264 => {
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000864ae3); };
            c.w64(9, t309);
            let Some(t310) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864aea); };
            c.logic(t310 & 0x4u64, 8);
            if c.cc_ne() { blk = 294; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t311) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864af5); };
            c.w64(3, t311);
            let Some(t312) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864afa); };
            c.w32(2, t312);
            let r313 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r313);
            let Some(t314) = c.ld32(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(4))) else { return RegfileExit::fault(0x7000864b02); };
            c.w32(3, t314);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000864b05); }
            c.w64(2, c.r64(3));
            let r315 = c.add(c.r32(2), c.r32(3), 32);
            c.w32(2, r315);
            if c.cc_o() { blk = 625; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 267; continue;
        }
        267 => {
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 630; continue; }
            blk = 268; continue;
        }
        268 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            blk = 269; continue;
        }
        269 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000864b27); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000864b29); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000864b2a); }
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000864b2b); };
            if !c.push64(t316) { return RegfileExit::fault(0x7000864b2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000864b2e); }
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864b32); };
            c.w64(6, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000864b36); };
            c.w64(7, t318);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x7000864b47, 0x7000864b42);
        }
        270 => {
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000864b47); };
            c.w64(8, t319);
            let r320 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r320);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000864b52); };
            c.w64(9, t321);
            let r322 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r322);
            let r323 = c.r32(8) & 0x3fu64;
            c.w32(8, r323);
            c.logic(r323, 32);
            let r324 = c.r32(9) & 0x3fu64;
            c.w32(9, r324);
            c.logic(r324, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 273; continue; }
            blk = 271; continue;
        }
        271 => {
            c.w64(1, c.r64(0));
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864b6a); };
            c.w64(0, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864b6e); };
            c.w64(3, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864b72); };
            c.w64(6, t327);
            return c.call(0x7000871cb4, 0x7000864b7b, 0x7000864b76);
        }
        272 => {
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864b7b); };
            c.w64(0, t328);
            blk = 293; continue;
        }
        273 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 639; continue; }
            blk = 274; continue;
        }
        274 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000864b8c); }
            let Some(t329) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864b90); };
            c.logic(t329 & 0x1u64, 8);
            if c.cc_e() { blk = 280; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864b96); };
            c.w64(2, t330);
            let Some(t331) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864b9a); };
            c.w32(8, t331);
            let r332 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r332);
            let Some(t333) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864ba1); };
            c.cmp(t333, 0x42u64, 16);
            if c.cc_ne() { blk = 642; continue; }
            blk = 276; continue;
        }
        276 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864bad); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 642; continue; }
            blk = 277; continue;
        }
        277 => {
            if c.cc_ne() { blk = 642; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t334) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x7000864bd0); };
            c.w64(9, t334);
            let r335 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r335);
            let Some(t336) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x7000864bda); };
            c.w64(11, t336);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 281; continue; }
            blk = 279; continue;
        }
        279 => {
            blk = 642; continue;
        }
        280 => {
            let Some(t337) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864beb); };
            c.w64(8, (t337 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 281; continue;
        }
        281 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000864bf2); }
            c.w64(9, c.r64(0));
            blk = 284; continue;
        }
        282 => {
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864bfb); };
            c.w64(6, t338);
            return c.call(0x70008a1014, 0x7000864c04, 0x7000864bff);
        }
        283 => {
            c.w64(9, c.r64(0));
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864c07); };
            c.w64(8, t339);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000864c0b); };
            c.w64(0, t340);
            blk = 284; continue;
        }
        284 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 289; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t341) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864c15); };
            c.w32(11, t341);
            let r342 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r342);
            let Some(t343) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000864c1c); };
            c.cmp(t343, c.r32(11), 32);
            if c.cc_e() { blk = 287; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t344) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864c25); };
            c.cmp(t344, 0x43u64, 16);
            if c.cc_ne() { blk = 282; continue; }
            blk = 287; continue;
        }
        287 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864c2d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 734; continue; }
            blk = 288; continue;
        }
        288 => {
            c.w32(9, c.r32(9));
            blk = 290; continue;
        }
        289 => {
            let r345 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r345);
            blk = 290; continue;
        }
        290 => {
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864c4a); };
            c.w64(0, t346);
            let Some(t347) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000864c4e); };
            c.w32(11, t347);
            let r348 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r348);
            let Some(t349) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864c55); };
            c.logic(t349 & 0x4u64, 8);
            if c.cc_ne() { blk = 642; continue; }
            blk = 291; continue;
        }
        291 => {
            let Some(t350) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000864c60); };
            c.cmp(t350, c.r64(8), 64);
            if c.cc_be() { blk = 642; continue; }
            blk = 292; continue;
        }
        292 => {
            let Some(t351) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864c6a); };
            c.w64(11, t351);
            let Some(t352) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864c6e); };
            c.w32(12, t352);
            let r353 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r353);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)), c.r32(9)) { return RegfileExit::fault(0x7000864c75); }
            blk = 293; continue;
        }
        293 => {
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000864c79); };
            c.w64(7, t354);
            let r355 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000864c81); };
            c.cmp(c.r64(7), t356, 64);
            if c.cc_ne() { blk = 263; continue; }
            blk = 294; continue;
        }
        294 => {
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000864c8b); };
            c.w64(8, t357);
            let r358 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r358);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000864c96); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000864c97); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000864c9d); }
            return c.ret(0x0, 0x7000864c9f);
        }
        295 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 327; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 298; continue;
        }
        297 => {
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000864cae); };
            c.w64(12, t359);
            blk = 298; continue;
        }
        298 => {
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000864cb2); };
            c.w64(9, t360);
            let Some(t361) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864cb9); };
            c.logic(t361 & 0x4u64, 8);
            if c.cc_ne() { blk = 327; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t362) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864cc4); };
            c.w64(3, t362);
            let Some(t363) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864cc9); };
            c.w32(2, t363);
            let r364 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r364);
            if !c.movss_rm(0, c.r64(3).wrapping_add(c.r64(7).wrapping_mul(4))) { return RegfileExit::fault(0x7000864cd1); }
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000864cd6); };
            c.w64(3, t365);
            let Some(t366) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000864cdd); };
            c.w64(2, t366);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000864ce4); }
            let Some(t367) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000864ce8); };
            c.w64(6, t367);
            let Some(t368) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000864cef); };
            c.cmp(t368, c.r64(1), 64);
            if c.cc_be() { blk = 644; continue; }
            blk = 300; continue;
        }
        300 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x7000864cf8); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 301; continue;
        }
        301 => {
            c.cvtss2sd(0, c.xmm_u32(0));
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000864d03); };
            c.w64(2, t369);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000864d0a); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000864d0d); }
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 646; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            blk = 303; continue;
        }
        303 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000864d22); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000864d24); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000864d25); }
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000864d26); };
            if !c.push64(t370) { return RegfileExit::fault(0x7000864d26); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000864d29); }
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864d2d); };
            c.w64(6, t371);
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000864d31); };
            c.w64(7, t372);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x7000864d42, 0x7000864d3d);
        }
        304 => {
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000864d42); };
            c.w64(8, t373);
            let r374 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000864d4d); };
            c.w64(9, t375);
            let r376 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r376);
            let r377 = c.r32(8) & 0x3fu64;
            c.w32(8, r377);
            c.logic(r377, 32);
            let r378 = c.r32(9) & 0x3fu64;
            c.w32(9, r378);
            c.logic(r378, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 307; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w64(1, c.r64(0));
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864d65); };
            c.w64(0, t379);
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864d69); };
            c.w64(3, t380);
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864d6d); };
            c.w64(6, t381);
            return c.call(0x7000871cb4, 0x7000864d76, 0x7000864d71);
        }
        306 => {
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864d76); };
            c.w64(0, t382);
            blk = 326; continue;
        }
        307 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 655; continue; }
            blk = 308; continue;
        }
        308 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000864d87); }
            let Some(t383) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864d8b); };
            c.logic(t383 & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864d91); };
            c.w64(2, t384);
            let Some(t385) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864d95); };
            c.w32(8, t385);
            let r386 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r386);
            let Some(t387) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864d9c); };
            c.cmp(t387, 0x42u64, 16);
            if c.cc_ne() { blk = 658; continue; }
            blk = 310; continue;
        }
        310 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864da8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 658; continue; }
            blk = 311; continue;
        }
        311 => {
            if c.cc_ne() { blk = 658; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t388) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x7000864dcb); };
            c.w64(9, t388);
            let r389 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r389);
            let Some(t390) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x7000864dd5); };
            c.w64(11, t390);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 315; continue; }
            blk = 313; continue;
        }
        313 => {
            blk = 658; continue;
        }
        314 => {
            let Some(t391) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864de6); };
            c.w64(8, (t391 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 315; continue;
        }
        315 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000864ded); }
            c.w64(9, c.r64(0));
            blk = 318; continue;
        }
        316 => {
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864df6); };
            c.w64(6, t392);
            return c.call(0x70008a1014, 0x7000864dff, 0x7000864dfa);
        }
        317 => {
            c.w64(9, c.r64(0));
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864e02); };
            c.w64(8, t393);
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000864e06); };
            c.w64(0, t394);
            blk = 318; continue;
        }
        318 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 322; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t395) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864e10); };
            c.w32(11, t395);
            let r396 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r396);
            let Some(t397) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000864e17); };
            c.cmp(t397, c.r32(11), 32);
            if c.cc_e() { blk = 321; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t398) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864e20); };
            c.cmp(t398, 0x43u64, 16);
            if c.cc_ne() { blk = 316; continue; }
            blk = 321; continue;
        }
        321 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864e28); }
            c.cvtsd2ss(0, c.xmm_u64(0));
            blk = 323; continue;
        }
        322 => {
            let r399 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r399);
            c.xorpd_rr(0, 0);
            c.cvtsi2ss(0, (c.r32(9) as u32 as i32 as i64));
            blk = 323; continue;
        }
        323 => {
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864e3f); };
            c.w64(0, t400);
            let Some(t401) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000864e43); };
            c.w32(9, t401);
            let r402 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r402);
            let Some(t403) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864e4a); };
            c.logic(t403 & 0x4u64, 8);
            if c.cc_ne() { blk = 658; continue; }
            blk = 324; continue;
        }
        324 => {
            let Some(t404) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000864e55); };
            c.cmp(t404, c.r64(8), 64);
            if c.cc_be() { blk = 658; continue; }
            blk = 325; continue;
        }
        325 => {
            let Some(t405) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864e5f); };
            c.w64(9, t405);
            let Some(t406) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864e63); };
            c.w32(11, t406);
            let r407 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r407);
            if !c.movss_mr(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)), 0) { return RegfileExit::fault(0x7000864e6a); }
            blk = 326; continue;
        }
        326 => {
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000864e70); };
            c.w64(7, t408);
            let r409 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000864e78); };
            c.cmp(c.r64(7), t410, 64);
            if c.cc_ne() { blk = 297; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000864e82); };
            c.w64(8, t411);
            let r412 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r412);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000864e8d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000864e8e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000864e94); }
            return c.ret(0x0, 0x7000864e96);
        }
        328 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 360; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 331; continue;
        }
        330 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000864ea5); };
            c.w64(12, t413);
            blk = 331; continue;
        }
        331 => {
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000864ea9); };
            c.w64(9, t414);
            let Some(t415) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000864eb0); };
            c.logic(t415 & 0x4u64, 8);
            if c.cc_ne() { blk = 360; continue; }
            blk = 332; continue;
        }
        332 => {
            let Some(t416) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000864ebb); };
            c.w64(3, t416);
            let Some(t417) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000864ec0); };
            c.w32(2, t417);
            let r418 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r418);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(7).wrapping_mul(8))) { return RegfileExit::fault(0x7000864ec8); }
            let Some(t419) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000864ecd); };
            c.w64(3, t419);
            let Some(t420) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000864ed4); };
            c.w64(2, t420);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000864edb); }
            let Some(t421) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000864edf); };
            c.w64(6, t421);
            let Some(t422) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000864ee6); };
            c.cmp(t422, c.r64(1), 64);
            if c.cc_be() { blk = 660; continue; }
            blk = 333; continue;
        }
        333 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x7000864eef); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 334; continue;
        }
        334 => {
            let Some(t423) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000864ef6); };
            c.w64(2, t423);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000864efd); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000864f00); }
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 662; continue; }
            blk = 335; continue;
        }
        335 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            blk = 336; continue;
        }
        336 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000864f15); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000864f17); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000864f18); }
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000864f19); };
            if !c.push64(t424) { return RegfileExit::fault(0x7000864f19); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000864f1c); }
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864f20); };
            c.w64(6, t425);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000864f24); };
            c.w64(7, t426);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x7000864f35, 0x7000864f30);
        }
        337 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000864f35); };
            c.w64(8, t427);
            let r428 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r428);
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000864f40); };
            c.w64(9, t429);
            let r430 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r430);
            let r431 = c.r32(8) & 0x3fu64;
            c.w32(8, r431);
            c.logic(r431, 32);
            let r432 = c.r32(9) & 0x3fu64;
            c.w32(9, r432);
            c.logic(r432, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 340; continue; }
            blk = 338; continue;
        }
        338 => {
            c.w64(1, c.r64(0));
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864f58); };
            c.w64(0, t433);
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864f5c); };
            c.w64(3, t434);
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864f60); };
            c.w64(6, t435);
            return c.call(0x7000871cb4, 0x7000864f69, 0x7000864f64);
        }
        339 => {
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000864f69); };
            c.w64(0, t436);
            blk = 359; continue;
        }
        340 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 671; continue; }
            blk = 341; continue;
        }
        341 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000864f7a); }
            let Some(t437) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864f7e); };
            c.logic(t437 & 0x1u64, 8);
            if c.cc_e() { blk = 347; continue; }
            blk = 342; continue;
        }
        342 => {
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864f84); };
            c.w64(2, t438);
            let Some(t439) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000864f88); };
            c.w32(8, t439);
            let r440 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r440);
            let Some(t441) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000864f8f); };
            c.cmp(t441, 0x42u64, 16);
            if c.cc_ne() { blk = 674; continue; }
            blk = 343; continue;
        }
        343 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000864f9b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 674; continue; }
            blk = 344; continue;
        }
        344 => {
            if c.cc_ne() { blk = 674; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t442) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x7000864fbe); };
            c.w64(9, t442);
            let r443 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r443);
            let Some(t444) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x7000864fc8); };
            c.w64(11, t444);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 348; continue; }
            blk = 346; continue;
        }
        346 => {
            blk = 674; continue;
        }
        347 => {
            let Some(t445) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864fd9); };
            c.w64(8, (t445 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 348; continue;
        }
        348 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000864fe0); }
            c.w64(9, c.r64(0));
            blk = 351; continue;
        }
        349 => {
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000864fe9); };
            c.w64(6, t446);
            return c.call(0x70008a1014, 0x7000864ff2, 0x7000864fed);
        }
        350 => {
            c.w64(9, c.r64(0));
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000864ff5); };
            c.w64(8, t447);
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000864ff9); };
            c.w64(0, t448);
            blk = 351; continue;
        }
        351 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 355; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t449) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865003); };
            c.w32(11, t449);
            let r450 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r450);
            let Some(t451) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086500a); };
            c.cmp(t451, c.r32(11), 32);
            if c.cc_e() { blk = 354; continue; }
            blk = 353; continue;
        }
        353 => {
            let Some(t452) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000865013); };
            c.cmp(t452, 0x43u64, 16);
            if c.cc_ne() { blk = 349; continue; }
            blk = 354; continue;
        }
        354 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700086501b); }
            blk = 356; continue;
        }
        355 => {
            let r453 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r453);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 356; continue;
        }
        356 => {
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086502f); };
            c.w64(0, t454);
            let Some(t455) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000865033); };
            c.w32(9, t455);
            let r456 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r456);
            let Some(t457) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700086503a); };
            c.logic(t457 & 0x4u64, 8);
            if c.cc_ne() { blk = 674; continue; }
            blk = 357; continue;
        }
        357 => {
            let Some(t458) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000865045); };
            c.cmp(t458, c.r64(8), 64);
            if c.cc_be() { blk = 674; continue; }
            blk = 358; continue;
        }
        358 => {
            let Some(t459) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086504f); };
            c.w64(9, t459);
            let Some(t460) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000865053); };
            c.w32(11, t460);
            let r461 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r461);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(8)), 0) { return RegfileExit::fault(0x700086505a); }
            blk = 359; continue;
        }
        359 => {
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000865060); };
            c.w64(7, t462);
            let r463 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r463);
            let Some(t464) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000865068); };
            c.cmp(c.r64(7), t464, 64);
            if c.cc_ne() { blk = 330; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000865072); };
            c.w64(8, t465);
            let r466 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r466);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086507d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086507e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000865084); }
            return c.ret(0x0, 0x7000865086);
        }
        361 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 419; continue; }
            blk = 362; continue;
        }
        362 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 364; continue;
        }
        363 => {
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000865095); };
            c.w64(12, t467);
            blk = 364; continue;
        }
        364 => {
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000865099); };
            c.w64(9, t468);
            let Some(t469) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008650a0); };
            c.logic(t469 & 0x4u64, 8);
            if c.cc_ne() { blk = 419; continue; }
            blk = 365; continue;
        }
        365 => {
            let Some(t470) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008650ab); };
            c.w64(3, t470);
            let Some(t471) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008650b0); };
            c.w32(2, t471);
            let r472 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r472);
            let Some(t473) = c.ld8(c.r64(7).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70008650b8); };
            c.w32(3, t473);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008650bc); }
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 676; continue; }
            blk = 366; continue;
        }
        366 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            blk = 367; continue;
        }
        367 => {
            let r474 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r474);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008650d2); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008650d4); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008650d5); }
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008650d6); };
            if !c.push64(t475) { return RegfileExit::fault(0x70008650d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008650d9); }
            let Some(t476) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008650dd); };
            c.w64(6, t476);
            let Some(t477) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008650e1); };
            c.w64(7, t477);
            c.w32(0, 0x3u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2274, 0x70008650f2, 0x70008650ed);
        }
        368 => {
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008650f2); };
            c.w64(8, t478);
            let r479 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r479);
            let Some(t480) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008650fd); };
            c.w64(9, t480);
            let r481 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r481);
            let r482 = c.r32(8) & 0x3fu64;
            c.w32(8, r482);
            c.logic(r482, 32);
            let r483 = c.r32(9) & 0x3fu64;
            c.w32(9, r483);
            c.logic(r483, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 371; continue; }
            blk = 369; continue;
        }
        369 => {
            c.w64(1, c.r64(0));
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865115); };
            c.w64(0, t484);
            let Some(t485) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865119); };
            c.w64(3, t485);
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086511d); };
            c.w64(6, t486);
            return c.call(0x7000871cb4, 0x7000865126, 0x7000865121);
        }
        370 => {
            let Some(t487) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865126); };
            c.w64(0, t487);
            blk = 418; continue;
        }
        371 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 685; continue; }
            blk = 372; continue;
        }
        372 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000865137); }
            let Some(t488) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086513b); };
            c.logic(t488 & 0x1u64, 8);
            if c.cc_e() { blk = 378; continue; }
            blk = 373; continue;
        }
        373 => {
            let Some(t489) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865141); };
            c.w64(2, t489);
            let Some(t490) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865145); };
            c.w32(8, t490);
            let r491 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r491);
            let Some(t492) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086514c); };
            c.cmp(t492, 0x42u64, 16);
            if c.cc_ne() { blk = 688; continue; }
            blk = 374; continue;
        }
        374 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000865158); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 688; continue; }
            blk = 375; continue;
        }
        375 => {
            if c.cc_ne() { blk = 688; continue; }
            blk = 376; continue;
        }
        376 => {
            let Some(t493) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x700086517b); };
            c.w64(9, t493);
            let r494 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r494);
            let Some(t495) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x7000865185); };
            c.w64(11, t495);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 379; continue; }
            blk = 377; continue;
        }
        377 => {
            blk = 688; continue;
        }
        378 => {
            let Some(t496) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865196); };
            c.w64(8, (t496 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 379; continue;
        }
        379 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086519d); }
            c.w64(9, c.r64(0));
            blk = 382; continue;
        }
        380 => {
            let Some(t497) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008651a6); };
            c.w64(6, t497);
            return c.call(0x70008a1014, 0x70008651af, 0x70008651aa);
        }
        381 => {
            c.w64(9, c.r64(0));
            let Some(t498) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008651b2); };
            c.w64(8, t498);
            let Some(t499) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008651b6); };
            c.w64(0, t499);
            blk = 382; continue;
        }
        382 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 411; continue; }
            blk = 383; continue;
        }
        383 => {
            let Some(t500) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008651c4); };
            c.w32(11, t500);
            let r501 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r501);
            let Some(t502) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008651cb); };
            c.cmp(t502, c.r32(11), 32);
            if c.cc_e() { blk = 385; continue; }
            blk = 384; continue;
        }
        384 => {
            let Some(t503) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008651d4); };
            c.cmp(t503, 0x43u64, 16);
            if c.cc_ne() { blk = 380; continue; }
            blk = 385; continue;
        }
        385 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008651dc); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 410; continue; }
            blk = 386; continue;
        }
        386 => {
            c.w64(10, 0x406fe00000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_ae() { blk = 409; continue; }
            blk = 387; continue;
        }
        387 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 395; continue; }
            blk = 388; continue;
        }
        388 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 394; continue; }
            blk = 389; continue;
        }
        389 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 391; continue; }
            blk = 390; continue;
        }
        390 => {
            c.mov128_rr(2, 0);
            blk = 400; continue;
        }
        391 => {
            c.mov128_rr(2, 0);
            let Some(t504) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000865239); };
            c.w64(10, t504);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x7000865240); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(3, 10);
            c.mov128_rr(4, 2);
            c.arith_sd(0, 4, c.xmm_u64(3));
            c.arith_sd(1, 4, c.xmm_u64(3));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(4));
            if c.cc_be() { blk = 393; continue; }
            blk = 392; continue;
        }
        392 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(0, 2, c.xmm_u64(4));
            c.mov128_rr(4, 2);
            blk = 393; continue;
        }
        393 => {
            let Some(t505) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000865279); };
            c.w64(10, t505);
            if !c.xorpd_rm(4, c.r64(10)) { return RegfileExit::fault(0x7000865280); }
            c.mov128_rr(2, 4);
            blk = 400; continue;
        }
        394 => {
            c.mov128_rr(2, 0);
            blk = 400; continue;
        }
        395 => {
            let Some(t506) = c.ld64(0x7000865246u64) else { return RegfileExit::fault(0x700086528e); };
            c.w64(10, t506);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_ae() { blk = 399; continue; }
            blk = 396; continue;
        }
        396 => {
            c.mov128_rr(3, 0);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(0));
            if c.cc_a() { blk = 398; continue; }
            blk = 397; continue;
        }
        397 => {
            c.mov128_rr(2, 3);
            blk = 400; continue;
        }
        398 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.mov128_rr(2, 3);
            blk = 400; continue;
        }
        399 => {
            c.mov128_rr(2, 0);
            blk = 400; continue;
        }
        400 => {
            c.pcmpeqd_rr(3, 3);
            c.psllq(3, 0x37);
            c.psrlq(3, 0x2);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(3));
            if c.cc_a() { blk = 406; continue; }
            blk = 401; continue;
        }
        401 => {
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(0));
            if c.cc_a() { blk = 407; continue; }
            blk = 402; continue;
        }
        402 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x3f);
            c.psrlq(0, 0x1);
            c.mov128_rr(3, 2);
            let r507 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r507);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000865303); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000865308); }
            if !c.movsd_mr(c.r64(4), 3) { return RegfileExit::fault(0x700086530b); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000865310); }
            blk = 403; continue;
        }
        403 => {
            c.fprem();
            c.fnstsw_ax();
            let r508 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r508);
            let r509 = c.r32(0) & 0xffu64;
            c.w32(0, r509);
            c.logic(r509, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086531f); }
            if !c.popfq() { return RegfileExit::fault(0x7000865320); }
            if c.cc_p() { blk = 403; continue; }
            blk = 404; continue;
        }
        404 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x7000865325); }
            if !c.movsd_rm(3, c.r64(4)) { return RegfileExit::fault(0x7000865328); }
            let r510 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r510);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(3));
            if c.cc_p() { blk = 406; continue; }
            blk = 405; continue;
        }
        405 => {
            if c.cc_e() { blk = 407; continue; }
            blk = 406; continue;
        }
        406 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(2));
            c.mov128_rr(2, 0);
            blk = 407; continue;
        }
        407 => {
            let cv = c.cvttsd2si64(c.xmm_u64(2)); c.w64(9, cv);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_o() { blk = 732; continue; }
            blk = 408; continue;
        }
        408 => {
            c.w32(9, c.r32(9));
            blk = 415; continue;
        }
        409 => {
            c.w32(9, 0xffu64);
            c.w64(11, c.r64(9));
            blk = 415; continue;
        }
        410 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 415; continue;
        }
        411 => {
            let r511 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r511);
            c.cmp(c.r32(9), 0xffu64, 32);
            if c.cc_be() { blk = 415; continue; }
            blk = 412; continue;
        }
        412 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 414; continue; }
            blk = 413; continue;
        }
        413 => {
            c.w32(9, 0xffu64);
            blk = 415; continue;
        }
        414 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 415; continue;
        }
        415 => {
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086538f); };
            c.w64(0, t512);
            let Some(t513) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000865393); };
            c.w32(11, t513);
            let r514 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r514);
            let Some(t515) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700086539a); };
            c.logic(t515 & 0x4u64, 8);
            if c.cc_ne() { blk = 688; continue; }
            blk = 416; continue;
        }
        416 => {
            let Some(t516) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008653a5); };
            c.cmp(t516, c.r64(8), 64);
            if c.cc_be() { blk = 688; continue; }
            blk = 417; continue;
        }
        417 => {
            let Some(t517) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008653af); };
            c.w64(11, t517);
            let Some(t518) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008653b3); };
            c.w32(12, t518);
            let r519 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r519);
            if !c.st8(c.r64(8).wrapping_add(c.r64(11)), c.r8(9)) { return RegfileExit::fault(0x70008653ba); }
            blk = 418; continue;
        }
        418 => {
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008653be); };
            c.w64(7, t520);
            let r521 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r521);
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008653c6); };
            c.cmp(c.r64(7), t522, 64);
            if c.cc_ne() { blk = 363; continue; }
            blk = 419; continue;
        }
        419 => {
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008653d0); };
            c.w64(8, t523);
            let r524 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r524);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008653db); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008653dc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008653e2); }
            return c.ret(0x0, 0x70008653e4);
        }
        420 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 465; continue; }
            blk = 421; continue;
        }
        421 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(12));
            blk = 423; continue;
        }
        422 => {
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(8));
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008653fd); };
            c.w64(3, t525);
            blk = 423; continue;
        }
        423 => {
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000865401); };
            c.w64(12, t526);
            let Some(t527) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000865408); };
            c.logic(t527 & 0x4u64, 8);
            if c.cc_ne() { blk = 464; continue; }
            blk = 424; continue;
        }
        424 => {
            let Some(t528) = c.ld64(c.r64(3).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000865414); };
            c.w64(2, t528);
            let Some(t529) = c.ld32(c.r64(3).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000865418); };
            c.w32(1, t529);
            let r530 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r530);
            let Some(t531) = c.ld64(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700086541e); };
            c.w64(2, t531);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000865422); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 428; continue; }
            blk = 425; continue;
        }
        425 => {
            let Some(t532) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086542c); };
            c.w64(1, t532);
            let Some(t533) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000865433); };
            c.w64(6, t533);
            c.w64(7, c.r64(6).wrapping_add(0x10));
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086543a); };
            c.w64(8, t534);
            let Some(t535) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000865441); };
            c.cmp(t535, c.r64(7), 64);
            if c.cc_be() { blk = 690; continue; }
            blk = 426; continue;
        }
        426 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x700086544a); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 427; continue;
        }
        427 => {
            let Some(t536) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000865451); };
            c.w64(6, t536);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000865458); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700086545c); }
            if !c.st64(c.r64(8).wrapping_add(0x7), c.r64(2)) { return RegfileExit::fault(0x7000865464); }
            blk = 431; continue;
        }
        428 => {
            let Some(t537) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086546a); };
            c.w64(8, t537);
            let Some(t538) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000865471); };
            c.w64(2, t538);
            c.w64(6, c.r64(2).wrapping_add(0x8));
            let Some(t539) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000865478); };
            c.w64(1, t539);
            let Some(t540) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086547f); };
            c.cmp(t540, c.r64(6), 64);
            if c.cc_be() { blk = 692; continue; }
            blk = 429; continue;
        }
        429 => {
            if !c.st64(c.r64(8), c.r64(6)) { return RegfileExit::fault(0x7000865488); }
            c.w64(8, c.r64(2).wrapping_add(0x1));
            blk = 430; continue;
        }
        430 => {
            let Some(t541) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700086548f); };
            c.w64(2, t541);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000865496); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700086549a); }
            blk = 431; continue;
        }
        431 => {
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 694; continue; }
            blk = 432; continue;
        }
        432 => {
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            blk = 433; continue;
        }
        433 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008654b3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008654b4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008654b5); }
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008654b7); };
            if !c.push64(t542) { return RegfileExit::fault(0x70008654b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008654ba); }
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008654be); };
            c.w64(6, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008654c2); };
            c.w64(7, t544);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008654d3, 0x70008654ce);
        }
        434 => {
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008654d3); };
            c.w64(8, t545);
            let r546 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r546);
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008654de); };
            c.w64(9, t547);
            let r548 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r548);
            let r549 = c.r32(8) & 0x3fu64;
            c.w32(8, r549);
            c.logic(r549, 32);
            let r550 = c.r32(9) & 0x3fu64;
            c.w32(9, r550);
            c.logic(r550, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 437; continue; }
            blk = 435; continue;
        }
        435 => {
            c.w64(1, c.r64(0));
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008654f6); };
            c.w64(0, t551);
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008654fa); };
            c.w64(3, t552);
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008654fe); };
            c.w64(6, t553);
            return c.call(0x7000871cb4, 0x7000865507, 0x7000865502);
        }
        436 => {
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865507); };
            c.w64(9, t554);
            blk = 458; continue;
        }
        437 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 462; continue; }
            blk = 438; continue;
        }
        438 => {
            let Some(t555) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865518); };
            c.w32(8, t555);
            let r556 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r556);
            let Some(t557) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086551f); };
            c.cmp(t557, 0x41u64, 16);
            if c.cc_e() { blk = 440; continue; }
            blk = 439; continue;
        }
        439 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000865527); }
            let Some(t558) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x7000865528); };
            c.w64(3, t558);
            c.w32(0, 0x1u64);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865534); };
            c.w64(6, t559);
            return c.call(0x700086ed74, 0x700086553d, 0x7000865538);
        }
        440 => {
            let Some(t560) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086553d); };
            c.logic(t560 & 0x1u64, 8);
            if c.cc_e() { blk = 446; continue; }
            blk = 441; continue;
        }
        441 => {
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865543); };
            c.w64(2, t561);
            let Some(t562) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865547); };
            c.w32(8, t562);
            let r563 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r563);
            let Some(t564) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086554e); };
            c.cmp(t564, 0x42u64, 16);
            if c.cc_ne() { blk = 703; continue; }
            blk = 442; continue;
        }
        442 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700086555a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 703; continue; }
            blk = 443; continue;
        }
        443 => {
            if c.cc_ne() { blk = 703; continue; }
            blk = 444; continue;
        }
        444 => {
            let Some(t565) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x700086557d); };
            c.w64(9, t565);
            let r566 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r566);
            let Some(t567) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x7000865587); };
            c.w64(11, t567);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 447; continue; }
            blk = 445; continue;
        }
        445 => {
            blk = 703; continue;
        }
        446 => {
            let Some(t568) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865598); };
            c.w64(8, (t568 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 447; continue;
        }
        447 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700086559f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 460; continue; }
            blk = 448; continue;
        }
        448 => {
            let Some(t569) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008655ab); };
            c.w32(9, t569);
            let r570 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r570);
            let Some(t571) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008655b2); };
            c.cmp(t571, 0x41u64, 16);
            if c.cc_e() { blk = 451; continue; }
            blk = 449; continue;
        }
        449 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008655ba); }
            let Some(t572) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70008655bb); };
            c.w64(3, t572);
            c.w32(0, 0x1u64);
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008655c7); };
            c.w64(6, t573);
            return c.call(0x700086ed74, 0x70008655d0, 0x70008655cb);
        }
        450 => {
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008655d0); };
            c.w64(8, t574);
            blk = 451; continue;
        }
        451 => {
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008655d4); };
            c.w64(9, t575);
            let Some(t576) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008655d8); };
            c.w32(11, t576);
            let r577 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r577);
            let Some(t578) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008655df); };
            c.logic(t578 & 0x4u64, 8);
            if c.cc_ne() { blk = 703; continue; }
            blk = 452; continue;
        }
        452 => {
            let Some(t579) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008655ea); };
            c.cmp(t579, c.r64(8), 64);
            if c.cc_be() { blk = 703; continue; }
            blk = 453; continue;
        }
        453 => {
            let Some(t580) = c.ld64(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008655f4); };
            c.w64(11, t580);
            let Some(t581) = c.ld32(c.r64(9).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008655f8); };
            c.w32(12, t581);
            let Some(t582) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008655fc); };
            c.w32(15, t582);
            c.logic(c.r32(15) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 456; continue; }
            blk = 454; continue;
        }
        454 => {
            let Some(t583) = c.ld64(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000865609); };
            c.w64(0, t583);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 457; continue; }
            blk = 455; continue;
        }
        455 => {
            let r584 = c.neg(c.r64(0), 64);
            c.w64(0, r584);
            blk = 457; continue;
        }
        456 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 457; continue;
        }
        457 => {
            let r585 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r585);
            if !c.st64(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x700086561d); }
            blk = 458; continue;
        }
        458 => {
            let Some(t586) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865621); };
            c.w64(8, t586);
            let r587 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r587);
            let Some(t588) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000865629); };
            c.cmp(c.r64(8), t588, 64);
            if c.cc_ne() { blk = 422; continue; }
            blk = 459; continue;
        }
        459 => {
            blk = 466; continue;
        }
        460 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086563b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086563d); }
            let Some(t589) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086563e); };
            c.w64(3, t589);
            c.w32(0, 0x2u64);
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086564a); };
            c.w64(6, t590);
            return c.call(0x700086ed74, 0x7000865653, 0x700086564e);
        }
        461 => {
            return RegfileExit::fault(0x7000865653);
        }
        462 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086565a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086565c); }
            let Some(t591) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086565d); };
            c.w64(3, t591);
            c.w32(0, 0x2u64);
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865669); };
            c.w64(6, t592);
            return c.call(0x700086ed74, 0x7000865672, 0x700086566d);
        }
        463 => {
            return RegfileExit::fault(0x7000865672);
        }
        464 => {
            c.w64(9, c.r64(0));
            blk = 466; continue;
        }
        465 => {
            c.w64(9, c.r64(0));
            blk = 466; continue;
        }
        466 => {
            let Some(t593) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086567b); };
            c.w64(8, t593);
            let r594 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r594);
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000865689); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086568a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000865690); }
            return c.ret(0x0, 0x7000865692);
        }
        467 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 514; continue; }
            blk = 468; continue;
        }
        468 => {
            c.w64(3, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 470; continue;
        }
        469 => {
            c.w64(3, c.r64(11));
            let Some(t595) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008656a8); };
            c.w64(11, t595);
            c.w64(0, c.r64(9));
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008656b2); };
            c.w64(12, t596);
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008656b6); };
            c.w64(15, t597);
            blk = 470; continue;
        }
        470 => {
            let Some(t598) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008656ba); };
            c.w64(9, t598);
            let Some(t599) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008656c1); };
            c.logic(t599 & 0x4u64, 8);
            if c.cc_ne() { blk = 513; continue; }
            blk = 471; continue;
        }
        471 => {
            let Some(t600) = c.ld64(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008656cc); };
            c.w64(2, t600);
            let Some(t601) = c.ld32(c.r64(12).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008656d1); };
            c.w32(1, t601);
            c.w64(6, c.r64(11));
            let r602 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r602);
            c.w64(7, c.r64(15));
            let r603 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r603);
            let r604 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r604);
            let r605 = c.r32(6) & 0x3fu64;
            c.w32(6, r605);
            c.logic(r605, 32);
            let r606 = c.r32(7) & 0x3fu64;
            c.w32(7, r606);
            c.logic(r606, 32);
            let Some(t607) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008656eb); };
            c.w64(2, t607);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008656ef); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(6)) { return RegfileExit::fault(0x70008656f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008656f7); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 477; continue; }
            blk = 472; continue;
        }
        472 => {
            let Some(t608) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865701); };
            c.w64(1, t608);
            let Some(t609) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000865708); };
            c.w64(9, t609);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            let Some(t610) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086570f); };
            c.w64(12, t610);
            let Some(t611) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000865716); };
            c.cmp(t611, c.r64(11), 64);
            if c.cc_be() { blk = 705; continue; }
            blk = 473; continue;
        }
        473 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x7000865720); }
            let r612 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r612);
            blk = 474; continue;
        }
        474 => {
            let Some(t613) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000865727); };
            c.w64(11, t613);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700086572e); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_g() { blk = 476; continue; }
            blk = 475; continue;
        }
        475 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000865738); }
            let r614 = c.neg(c.r64(2), 64);
            c.w64(2, r614);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(2)) { return RegfileExit::fault(0x7000865743); }
            blk = 480; continue;
        }
        476 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000865749); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(2)) { return RegfileExit::fault(0x7000865751); }
            blk = 480; continue;
        }
        477 => {
            let Some(t615) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865757); };
            c.w64(2, t615);
            let Some(t616) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086575e); };
            c.w64(1, t616);
            c.w64(11, c.r64(1).wrapping_add(0x8));
            let Some(t617) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000865765); };
            c.w64(9, t617);
            let Some(t618) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086576c); };
            c.cmp(t618, c.r64(11), 64);
            if c.cc_be() { blk = 707; continue; }
            blk = 478; continue;
        }
        478 => {
            if !c.st64(c.r64(2), c.r64(11)) { return RegfileExit::fault(0x7000865775); }
            c.w64(9, c.r64(1).wrapping_add(0x1));
            blk = 479; continue;
        }
        479 => {
            let Some(t619) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700086577c); };
            c.w64(11, t619);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000865783); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x7000865787); }
            blk = 480; continue;
        }
        480 => {
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 709; continue; }
            blk = 481; continue;
        }
        481 => {
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            blk = 482; continue;
        }
        482 => {
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008657a0); };
            if !c.push64(t620) { return RegfileExit::fault(0x70008657a0); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008657a3); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008657a5); }
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008657a7); };
            if !c.push64(t621) { return RegfileExit::fault(0x70008657a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008657aa); }
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008657ae); };
            c.w64(6, t622);
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008657b2); };
            c.w64(7, t623);
            c.w32(0, 0x3u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x70008657c3, 0x70008657be);
        }
        483 => {
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008657c3); };
            c.w64(8, t624);
            let Some(t625) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008657c7); };
            c.cmp(c.r32(8), t625, 32);
            if c.cc_e() { blk = 486; continue; }
            blk = 484; continue;
        }
        484 => {
            c.w64(1, c.r64(0));
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008657d0); };
            c.w64(0, t626);
            let Some(t627) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008657d4); };
            c.w64(3, t627);
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008657d8); };
            c.w64(6, t628);
            return c.call(0x7000871cb4, 0x70008657e1, 0x70008657dc);
        }
        485 => {
            let Some(t629) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008657e1); };
            c.w64(9, t629);
            blk = 507; continue;
        }
        486 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 511; continue; }
            blk = 487; continue;
        }
        487 => {
            let Some(t630) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008657f2); };
            c.w32(8, t630);
            let r631 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r631);
            let Some(t632) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008657f9); };
            c.cmp(t632, 0x41u64, 16);
            if c.cc_e() { blk = 489; continue; }
            blk = 488; continue;
        }
        488 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000865801); }
            let Some(t633) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x7000865802); };
            c.w64(3, t633);
            c.w32(0, 0x1u64);
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086580e); };
            c.w64(6, t634);
            return c.call(0x700086ed74, 0x7000865817, 0x7000865812);
        }
        489 => {
            let Some(t635) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865817); };
            c.logic(t635 & 0x1u64, 8);
            if c.cc_e() { blk = 495; continue; }
            blk = 490; continue;
        }
        490 => {
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086581d); };
            c.w64(11, t636);
            let Some(t637) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865821); };
            c.w32(8, t637);
            let r638 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r638);
            let Some(t639) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000865828); };
            c.cmp(t639, 0x42u64, 16);
            if c.cc_ne() { blk = 718; continue; }
            blk = 491; continue;
        }
        491 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000865834); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 718; continue; }
            blk = 492; continue;
        }
        492 => {
            if c.cc_ne() { blk = 718; continue; }
            blk = 493; continue;
        }
        493 => {
            let Some(t640) = c.ld64(0x70008642e0u64) else { return RegfileExit::fault(0x7000865858); };
            c.w64(9, t640);
            let r641 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r641);
            let Some(t642) = c.ld64(0x70008642edu64) else { return RegfileExit::fault(0x7000865862); };
            c.w64(11, t642);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 496; continue; }
            blk = 494; continue;
        }
        494 => {
            blk = 718; continue;
        }
        495 => {
            let Some(t643) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865873); };
            c.w64(8, (t643 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 496; continue;
        }
        496 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700086587a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 509; continue; }
            blk = 497; continue;
        }
        497 => {
            let Some(t644) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865886); };
            c.w32(9, t644);
            let r645 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r645);
            let Some(t646) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086588d); };
            c.cmp(t646, 0x41u64, 16);
            if c.cc_e() { blk = 500; continue; }
            blk = 498; continue;
        }
        498 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000865895); }
            let Some(t647) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x7000865896); };
            c.w64(3, t647);
            c.w32(0, 0x1u64);
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008658a2); };
            c.w64(6, t648);
            return c.call(0x700086ed74, 0x70008658ab, 0x70008658a6);
        }
        499 => {
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008658ab); };
            c.w64(8, t649);
            blk = 500; continue;
        }
        500 => {
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008658af); };
            c.w64(9, t650);
            let Some(t651) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008658b3); };
            c.w32(11, t651);
            let r652 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r652);
            let Some(t653) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008658ba); };
            c.logic(t653 & 0x4u64, 8);
            if c.cc_ne() { blk = 718; continue; }
            blk = 501; continue;
        }
        501 => {
            let Some(t654) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008658c5); };
            c.cmp(t654, c.r64(8), 64);
            if c.cc_be() { blk = 718; continue; }
            blk = 502; continue;
        }
        502 => {
            let Some(t655) = c.ld64(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008658cf); };
            c.w64(11, t655);
            let Some(t656) = c.ld32(c.r64(9).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008658d3); };
            c.w32(12, t656);
            let Some(t657) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008658d7); };
            c.w32(15, t657);
            c.logic(c.r32(15) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 505; continue; }
            blk = 503; continue;
        }
        503 => {
            let Some(t658) = c.ld64(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008658e4); };
            c.w64(0, t658);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 506; continue; }
            blk = 504; continue;
        }
        504 => {
            let r659 = c.neg(c.r64(0), 64);
            c.w64(0, r659);
            blk = 506; continue;
        }
        505 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 506; continue;
        }
        506 => {
            let r660 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r660);
            if !c.st64(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x70008658f8); }
            blk = 507; continue;
        }
        507 => {
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008658fc); };
            c.w64(8, t661);
            let r662 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r662);
            let Some(t663) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000865904); };
            c.w64(11, t663);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ne() { blk = 469; continue; }
            blk = 508; continue;
        }
        508 => {
            blk = 515; continue;
        }
        509 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865919); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086591b); }
            let Some(t664) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086591c); };
            c.w64(3, t664);
            c.w32(0, 0x2u64);
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865928); };
            c.w64(6, t665);
            return c.call(0x700086ed74, 0x7000865931, 0x700086592c);
        }
        510 => {
            return RegfileExit::fault(0x7000865931);
        }
        511 => {
            c.w32(8, 0x28u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865938); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086593a); }
            let Some(t666) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086593b); };
            c.w64(3, t666);
            c.w32(0, 0x2u64);
            let Some(t667) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865947); };
            c.w64(6, t667);
            return c.call(0x700086ed74, 0x7000865950, 0x700086594b);
        }
        512 => {
            return RegfileExit::fault(0x7000865950);
        }
        513 => {
            c.w64(9, c.r64(0));
            blk = 515; continue;
        }
        514 => {
            c.w64(9, c.r64(0));
            blk = 515; continue;
        }
        515 => {
            let Some(t668) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000865959); };
            c.w64(8, t668);
            let r669 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r669);
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000865967); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000865968); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086596e); }
            return c.ret(0x0, 0x7000865970);
        }
        516 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000865971); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t670) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000865977); };
            c.w64(3, t670);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000865986, 0x7000865981);
        }
        517 => {
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865986); };
            c.w64(6, t671);
            let Some(t672) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086598a); };
            c.w64(0, t672);
            blk = 1; continue;
        }
        518 => {
            let Some(t673) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865993); };
            c.w64(2, t673);
            let Some(t674) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086599a); };
            c.w64(1, t674);
            c.w64(7, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008659a1); }
            let Some(t675) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008659a5); };
            c.w64(1, t675);
            let Some(t676) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008659ac); };
            c.cmp(t676, c.r64(7), 64);
            if c.cc_a() { blk = 521; continue; }
            blk = 519; continue;
        }
        519 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008659bb, 0x70008659b6);
        }
        520 => {
            c.w64(2, c.r64(0));
            let Some(t677) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008659be); };
            c.w64(6, t677);
            let Some(t678) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008659c2); };
            c.w64(8, t678);
            let Some(t679) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008659c6); };
            c.w64(12, t679);
            let Some(t680) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008659ca); };
            c.w64(3, t680);
            let Some(t681) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008659ce); };
            c.w64(9, t681);
            let Some(t682) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008659d2); };
            c.w64(11, t682);
            let Some(t683) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008659d6); };
            c.w64(0, t683);
            let Some(t684) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008659dd); };
            c.w64(15, t684);
            blk = 522; continue;
        }
        521 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x70008659e3); }
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008659e6); };
            c.w64(2, t685);
            let r686 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r686);
            blk = 522; continue;
        }
        522 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 526; continue; }
            blk = 523; continue;
        }
        523 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 525; continue; }
            blk = 524; continue;
        }
        524 => {
            let r687 = c.r64(10) | 0x1u64;
            c.w64(10, r687);
            c.logic(r687, 64);
            blk = 525; continue;
        }
        525 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 526; continue;
        }
        526 => {
            let Some(t688) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865a15); };
            c.w64(1, t688);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000865a1c); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000865a1f); }
            blk = 14; continue;
        }
        527 => {
            return RegfileExit::fault(0x7000865a29);
        }
        528 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000865a2a); }
            let Some(t689) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000865a2e); };
            c.w64(2, t689);
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(2));
            return c.call(0x7000870914, 0x7000865a40, 0x7000865a3b);
        }
        529 => {
            let Some(t690) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000865a40); };
            c.cmp(t690, c.r32(0), 32);
            if c.cc_e() { blk = 542; continue; }
            blk = 530; continue;
        }
        530 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 540; continue; }
            blk = 531; continue;
        }
        531 => {
            let Some(t691) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865a55); };
            c.w32(8, t691);
            let r692 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r692);
            let Some(t693) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000865a5c); };
            c.cmp(t693, 0xbau64, 16);
            if c.cc_b() { blk = 540; continue; }
            blk = 532; continue;
        }
        532 => {
            let Some(t694) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000865a69); };
            c.w64(3, t694);
            let Some(t695) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865a70); };
            c.w64(6, t695);
            return c.call(0x7000870914, 0x7000865a79, 0x7000865a74);
        }
        533 => {
            let Some(t696) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000865a79); };
            c.cmp(t696, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t697) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000865a88); };
            c.cmp(t697, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r698 = c.r32(8) | c.r32(9);
            c.w32(8, r698);
            c.logic(r698, 32);
            if c.cc_ne() { blk = 539; continue; }
            blk = 534; continue;
        }
        534 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 537; continue; }
            blk = 535; continue;
        }
        535 => {
            let Some(t699) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865aa0); };
            c.w32(8, t699);
            let r700 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r700);
            let Some(t701) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000865aa7); };
            c.logic(t701 & 0x40u64, 8);
            if c.cc_e() { blk = 537; continue; }
            blk = 536; continue;
        }
        536 => {
            c.w64(7, c.r64(0));
            blk = 543; continue;
        }
        537 => {
            c.w32(8, 0x26eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865ab9); }
            c.w32(0, 0x1u64);
            let Some(t702) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000865ac0); };
            c.w64(3, t702);
            let Some(t703) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865ac7); };
            c.w64(6, t703);
            return c.call(0x700086ed74, 0x7000865ad0, 0x7000865acb);
        }
        538 => {
            return RegfileExit::fault(0x7000865ad0);
        }
        539 => {
            let Some(t704) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865ad1); };
            c.w64(7, t704);
            blk = 543; continue;
        }
        540 => {
            c.w32(8, 0x54u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865add); }
            let Some(t705) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000865adf); };
            c.w64(8, t705);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865ae6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000865ae8); }
            let Some(t706) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000865ae9); };
            c.w64(3, t706);
            c.w32(0, 0x3u64);
            let Some(t707) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865af5); };
            c.w64(6, t707);
            return c.call(0x700086ed74, 0x7000865afe, 0x7000865af9);
        }
        541 => {
            return RegfileExit::fault(0x7000865afe);
        }
        542 => {
            let Some(t708) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865aff); };
            c.w64(7, t708);
            blk = 543; continue;
        }
        543 => {
            let Some(t709) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865b03); };
            c.w32(8, t709);
            let r710 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r710);
            let Some(t711) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000865b0a); };
            c.logic(t711 & 0x40u64, 8);
            if c.cc_ne() { blk = 545; continue; }
            blk = 544; continue;
        }
        544 => {
            return RegfileExit::fault(0x7000865b11);
        }
        545 => {
            let Some(t712) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865b12); };
            if !c.push64(t712) { return RegfileExit::fault(0x7000865b12); }
            let Some(t713) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000865b15); };
            c.w64(8, t713);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865b1c); }
            let Some(t714) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865b1e); };
            c.w64(6, t714);
            c.w64(2, c.r64(7));
            c.w32(0, 0x1u64);
            return c.call(0x70007f7214, 0x7000865b2f, 0x7000865b2a);
        }
        546 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 724; continue; }
            blk = 547; continue;
        }
        547 => {
            let Some(t715) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865b37); };
            c.w32(8, t715);
            let r716 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r716);
            let Some(t717) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000865b3e); };
            c.cmp(t717, 0x82eu64, 16);
            if c.cc_ne() { blk = 724; continue; }
            blk = 548; continue;
        }
        548 => {
            let Some(t718) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000865b4b); };
            c.w32(8, t718);
            let r719 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r719);
            let Some(t720) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000865b52); };
            c.logic(t720 & 0x4u64, 8);
            if c.cc_ne() { blk = 722; continue; }
            blk = 549; continue;
        }
        549 => {
            blk = 83; continue;
        }
        550 => {
            let Some(t721) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865b62); };
            c.w64(1, t721);
            let Some(t722) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000865b69); };
            c.w64(6, t722);
            c.w64(1, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000865b70); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000865b74); }
            let Some(t723) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000865b78); };
            c.w64(6, t723);
            let Some(t724) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000865b7f); };
            c.cmp(t724, c.r64(1), 64);
            if c.cc_a() { blk = 553; continue; }
            blk = 551; continue;
        }
        551 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000865b91, 0x7000865b8c);
        }
        552 => {
            c.w64(1, c.r64(0));
            let Some(t725) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865b94); };
            c.w64(0, t725);
            let Some(t726) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000865b98); };
            c.w64(8, t726);
            let Some(t727) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000865b9c); };
            c.w64(12, t727);
            let Some(t728) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000865ba0); };
            c.w64(15, t728);
            let Some(t729) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000865ba4); };
            c.w64(11, t729);
            let Some(t730) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000865bab); };
            c.w64(9, t730);
            let Some(t731) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000865bb2); };
            c.w64(7, t731);
            let Some(t732) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865bb6); };
            c.w64(2, t732);
            let Some(t733) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000865bba); };
            c.w64(3, t733);
            blk = 554; continue;
        }
        553 => {
            let Some(t734) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865bc0); };
            c.w64(6, t734);
            if !c.st64(c.r64(6), c.r64(1)) { return RegfileExit::fault(0x7000865bc7); }
            let Some(t735) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865bca); };
            c.w64(1, t735);
            let r736 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r736);
            blk = 554; continue;
        }
        554 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 558; continue; }
            blk = 555; continue;
        }
        555 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 557; continue; }
            blk = 556; continue;
        }
        556 => {
            let r737 = c.r64(10) | 0x1u64;
            c.w64(10, r737);
            c.logic(r737, 64);
            blk = 557; continue;
        }
        557 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 558; continue;
        }
        558 => {
            let Some(t738) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865bf9); };
            c.w64(6, t738);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000865c00); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000865c03); }
            blk = 105; continue;
        }
        559 => {
            let Some(t739) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865c0d); };
            c.w32(8, t739);
            let Some(t740) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865c11); };
            c.cmp(t740, c.r32(8), 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 560; continue;
        }
        560 => {
            let Some(t741) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865c1e); };
            c.w64(6, t741);
            return c.call(0x70008a1014, 0x7000865c27, 0x7000865c22);
        }
        561 => {
            blk = 110; continue;
        }
        562 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865c32); }
            let Some(t742) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000865c34); };
            c.w64(8, t742);
            let Some(t743) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x7000865c3b); };
            c.w32(8, t743);
            let r744 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r744);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865c45); }
            let Some(t745) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865c47); };
            c.w64(6, t745);
            let Some(t746) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000865c4b); };
            c.w64(3, t746);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000865c5f, 0x7000865c5a);
        }
        563 => {
            return RegfileExit::fault(0x7000865c5f);
        }
        564 => {
            let Some(t747) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865c60); };
            c.w64(2, t747);
            let Some(t748) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000865c67); };
            c.w64(1, t748);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000865c6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000865c72); }
            let Some(t749) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000865c76); };
            c.w64(1, t749);
            let Some(t750) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000865c7d); };
            c.cmp(t750, c.r64(6), 64);
            if c.cc_a() { blk = 567; continue; }
            blk = 565; continue;
        }
        565 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000865c8c, 0x7000865c87);
        }
        566 => {
            c.w64(2, c.r64(0));
            let Some(t751) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865c8f); };
            c.w64(0, t751);
            let Some(t752) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000865c93); };
            c.w64(12, t752);
            let Some(t753) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000865c97); };
            c.w64(9, t753);
            let Some(t754) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000865c9e); };
            c.w64(7, t754);
            let Some(t755) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865ca2); };
            c.w64(3, t755);
            blk = 568; continue;
        }
        567 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000865ca8); }
            let Some(t756) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865cab); };
            c.w64(2, t756);
            let r757 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r757);
            blk = 568; continue;
        }
        568 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 572; continue; }
            blk = 569; continue;
        }
        569 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 571; continue; }
            blk = 570; continue;
        }
        570 => {
            let r758 = c.r64(10) | 0x1u64;
            c.w64(10, r758);
            c.logic(r758, 64);
            blk = 571; continue;
        }
        571 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 572; continue;
        }
        572 => {
            let Some(t759) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865cda); };
            c.w64(1, t759);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000865ce1); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000865ce4); }
            blk = 137; continue;
        }
        573 => {
            let Some(t760) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865cee); };
            c.w32(8, t760);
            let Some(t761) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865cf2); };
            c.cmp(t761, c.r32(8), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 574; continue;
        }
        574 => {
            let Some(t762) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865cff); };
            c.w64(6, t762);
            return c.call(0x70008a1014, 0x7000865d08, 0x7000865d03);
        }
        575 => {
            blk = 142; continue;
        }
        576 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865d13); }
            let Some(t763) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000865d15); };
            c.w64(8, t763);
            let Some(t764) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x7000865d1c); };
            c.w32(8, t764);
            let r765 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r765);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865d26); }
            let Some(t766) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000865d28); };
            c.w64(3, t766);
            c.w32(0, 0x2u64);
            let Some(t767) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865d34); };
            c.w64(6, t767);
            return c.call(0x700086ed74, 0x7000865d3d, 0x7000865d38);
        }
        577 => {
            return RegfileExit::fault(0x7000865d3d);
        }
        578 => {
            let Some(t768) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865d3e); };
            c.w64(2, t768);
            let Some(t769) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000865d45); };
            c.w64(1, t769);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000865d4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000865d50); }
            let Some(t770) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000865d54); };
            c.w64(1, t770);
            let Some(t771) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000865d5b); };
            c.cmp(t771, c.r64(6), 64);
            if c.cc_a() { blk = 581; continue; }
            blk = 579; continue;
        }
        579 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000865d6a, 0x7000865d65);
        }
        580 => {
            c.w64(2, c.r64(0));
            let Some(t772) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865d6d); };
            c.w64(0, t772);
            let Some(t773) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000865d71); };
            c.w64(12, t773);
            let Some(t774) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000865d75); };
            c.w64(9, t774);
            let Some(t775) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000865d7c); };
            c.w64(7, t775);
            let Some(t776) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865d80); };
            c.w64(3, t776);
            blk = 582; continue;
        }
        581 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000865d86); }
            let Some(t777) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865d89); };
            c.w64(2, t777);
            let r778 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r778);
            blk = 582; continue;
        }
        582 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 586; continue; }
            blk = 583; continue;
        }
        583 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 585; continue; }
            blk = 584; continue;
        }
        584 => {
            let r779 = c.r64(10) | 0x1u64;
            c.w64(10, r779);
            c.logic(r779, 64);
            blk = 585; continue;
        }
        585 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 586; continue;
        }
        586 => {
            let Some(t780) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865db8); };
            c.w64(1, t780);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000865dbf); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000865dc2); }
            blk = 169; continue;
        }
        587 => {
            let Some(t781) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865dcc); };
            c.w32(8, t781);
            let Some(t782) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865dd0); };
            c.cmp(t782, c.r32(8), 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 588; continue;
        }
        588 => {
            let Some(t783) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865ddd); };
            c.w64(6, t783);
            return c.call(0x70008a1014, 0x7000865de6, 0x7000865de1);
        }
        589 => {
            blk = 174; continue;
        }
        590 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865df1); }
            let Some(t784) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000865df3); };
            c.w64(8, t784);
            let Some(t785) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x7000865dfa); };
            c.w32(8, t785);
            let r786 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r786);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865e04); }
            let Some(t787) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000865e06); };
            c.w64(3, t787);
            c.w32(0, 0x2u64);
            let Some(t788) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865e12); };
            c.w64(6, t788);
            return c.call(0x700086ed74, 0x7000865e1b, 0x7000865e16);
        }
        591 => {
            return RegfileExit::fault(0x7000865e1b);
        }
        592 => {
            let Some(t789) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865e1c); };
            c.w64(2, t789);
            let Some(t790) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000865e23); };
            c.w64(1, t790);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000865e2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000865e2e); }
            let Some(t791) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000865e32); };
            c.w64(1, t791);
            let Some(t792) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000865e39); };
            c.cmp(t792, c.r64(6), 64);
            if c.cc_a() { blk = 595; continue; }
            blk = 593; continue;
        }
        593 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000865e48, 0x7000865e43);
        }
        594 => {
            c.w64(2, c.r64(0));
            let Some(t793) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865e4b); };
            c.w64(0, t793);
            let Some(t794) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000865e4f); };
            c.w64(12, t794);
            let Some(t795) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000865e53); };
            c.w64(9, t795);
            let Some(t796) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000865e5a); };
            c.w64(7, t796);
            let Some(t797) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865e5e); };
            c.w64(3, t797);
            blk = 596; continue;
        }
        595 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000865e64); }
            let Some(t798) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865e67); };
            c.w64(2, t798);
            let r799 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r799);
            blk = 596; continue;
        }
        596 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 600; continue; }
            blk = 597; continue;
        }
        597 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 599; continue; }
            blk = 598; continue;
        }
        598 => {
            let r800 = c.r64(10) | 0x1u64;
            c.w64(10, r800);
            c.logic(r800, 64);
            blk = 599; continue;
        }
        599 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 600; continue;
        }
        600 => {
            let Some(t801) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865e96); };
            c.w64(1, t801);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000865e9d); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000865ea0); }
            blk = 201; continue;
        }
        601 => {
            let Some(t802) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865eaa); };
            c.w32(8, t802);
            let Some(t803) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865eae); };
            c.cmp(t803, c.r32(8), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 602; continue;
        }
        602 => {
            let Some(t804) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865ebb); };
            c.w64(6, t804);
            return c.call(0x70008a1014, 0x7000865ec4, 0x7000865ebf);
        }
        603 => {
            blk = 206; continue;
        }
        604 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865ecf); }
            let Some(t805) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000865ed1); };
            c.w64(8, t805);
            let Some(t806) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x7000865ed8); };
            c.w32(8, t806);
            let r807 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r807);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000865ee2); }
            let Some(t808) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000865ee4); };
            c.w64(3, t808);
            c.w32(0, 0x2u64);
            let Some(t809) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000865ef0); };
            c.w64(6, t809);
            return c.call(0x700086ed74, 0x7000865ef9, 0x7000865ef4);
        }
        605 => {
            return RegfileExit::fault(0x7000865ef9);
        }
        606 => {
            let Some(t810) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865efa); };
            c.w64(2, t810);
            let Some(t811) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000865f01); };
            c.w64(1, t811);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000865f08); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x7000865f0c); }
            let Some(t812) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000865f10); };
            c.w64(1, t812);
            let Some(t813) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000865f17); };
            c.cmp(t813, c.r64(6), 64);
            if c.cc_a() { blk = 609; continue; }
            blk = 607; continue;
        }
        607 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000865f26, 0x7000865f21);
        }
        608 => {
            c.w64(2, c.r64(0));
            let Some(t814) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865f29); };
            c.w64(0, t814);
            let Some(t815) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000865f2d); };
            c.w64(12, t815);
            let Some(t816) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000865f31); };
            c.w64(9, t816);
            let Some(t817) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000865f38); };
            c.w64(7, t817);
            let Some(t818) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865f3c); };
            c.w64(3, t818);
            blk = 610; continue;
        }
        609 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000865f42); }
            let Some(t819) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865f45); };
            c.w64(2, t819);
            let r820 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r820);
            blk = 610; continue;
        }
        610 => {
            c.w32(10, c.r32(3));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t821) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865f59); };
            c.w64(3, t821);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000865f60); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000865f63); }
            c.w64(3, c.r64(2));
            blk = 233; continue;
        }
        611 => {
            let Some(t822) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000865f70); };
            c.w64(2, t822);
            let Some(t823) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000865f77); };
            c.w64(1, t823);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000865f7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000865f82); }
            let Some(t824) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000865f86); };
            c.w64(1, t824);
            let Some(t825) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000865f8d); };
            c.cmp(t825, c.r64(6), 64);
            if c.cc_a() { blk = 614; continue; }
            blk = 612; continue;
        }
        612 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000865f9c, 0x7000865f97);
        }
        613 => {
            c.w64(2, c.r64(0));
            let Some(t826) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000865f9f); };
            c.w64(0, t826);
            let Some(t827) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000865fa3); };
            c.w64(12, t827);
            let Some(t828) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000865fa7); };
            c.w64(9, t828);
            let Some(t829) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000865fae); };
            c.w64(7, t829);
            let Some(t830) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000865fb2); };
            c.w64(3, t830);
            blk = 615; continue;
        }
        614 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000865fb8); }
            let Some(t831) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000865fbb); };
            c.w64(2, t831);
            let r832 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r832);
            blk = 615; continue;
        }
        615 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 619; continue; }
            blk = 616; continue;
        }
        616 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 618; continue; }
            blk = 617; continue;
        }
        617 => {
            let r833 = c.r64(10) | 0x1u64;
            c.w64(10, r833);
            c.logic(r833, 64);
            blk = 618; continue;
        }
        618 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 619; continue;
        }
        619 => {
            let Some(t834) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000865fea); };
            c.w64(1, t834);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000865ff1); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000865ff4); }
            blk = 235; continue;
        }
        620 => {
            let Some(t835) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000865ffe); };
            c.w32(8, t835);
            let Some(t836) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866002); };
            c.cmp(t836, c.r32(8), 32);
            if c.cc_e() { blk = 240; continue; }
            blk = 621; continue;
        }
        621 => {
            let Some(t837) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086600f); };
            c.w64(6, t837);
            return c.call(0x70008a1014, 0x7000866018, 0x7000866013);
        }
        622 => {
            blk = 240; continue;
        }
        623 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866023); }
            let Some(t838) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000866025); };
            c.w64(8, t838);
            let Some(t839) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x700086602c); };
            c.w32(8, t839);
            let r840 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r840);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866036); }
            let Some(t841) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000866038); };
            c.w64(3, t841);
            c.w32(0, 0x2u64);
            let Some(t842) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000866044); };
            c.w64(6, t842);
            return c.call(0x700086ed74, 0x700086604d, 0x7000866048);
        }
        624 => {
            return RegfileExit::fault(0x700086604d);
        }
        625 => {
            let Some(t843) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086604e); };
            c.w64(2, t843);
            let Some(t844) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000866055); };
            c.w64(1, t844);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700086605c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x7000866060); }
            let Some(t845) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000866064); };
            c.w64(1, t845);
            let Some(t846) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086606b); };
            c.cmp(t846, c.r64(6), 64);
            if c.cc_a() { blk = 628; continue; }
            blk = 626; continue;
        }
        626 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700086607a, 0x7000866075);
        }
        627 => {
            c.w64(2, c.r64(0));
            let Some(t847) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086607d); };
            c.w64(0, t847);
            let Some(t848) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000866081); };
            c.w64(12, t848);
            let Some(t849) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000866085); };
            c.w64(9, t849);
            let Some(t850) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086608c); };
            c.w64(7, t850);
            let Some(t851) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000866090); };
            c.w64(3, t851);
            blk = 629; continue;
        }
        628 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000866096); }
            let Some(t852) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000866099); };
            c.w64(2, t852);
            let r853 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r853);
            blk = 629; continue;
        }
        629 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            let Some(t854) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008660a9); };
            c.w64(3, t854);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008660b0); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008660b3); }
            c.w64(3, c.r64(2));
            blk = 267; continue;
        }
        630 => {
            let Some(t855) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008660c0); };
            c.w64(2, t855);
            let Some(t856) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008660c7); };
            c.w64(1, t856);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008660ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008660d2); }
            let Some(t857) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008660d6); };
            c.w64(1, t857);
            let Some(t858) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008660dd); };
            c.cmp(t858, c.r64(6), 64);
            if c.cc_a() { blk = 633; continue; }
            blk = 631; continue;
        }
        631 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008660ec, 0x70008660e7);
        }
        632 => {
            c.w64(2, c.r64(0));
            let Some(t859) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008660ef); };
            c.w64(0, t859);
            let Some(t860) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008660f3); };
            c.w64(12, t860);
            let Some(t861) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008660f7); };
            c.w64(9, t861);
            let Some(t862) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008660fe); };
            c.w64(7, t862);
            let Some(t863) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000866102); };
            c.w64(3, t863);
            blk = 634; continue;
        }
        633 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000866108); }
            let Some(t864) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086610b); };
            c.w64(2, t864);
            let r865 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r865);
            blk = 634; continue;
        }
        634 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 638; continue; }
            blk = 635; continue;
        }
        635 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 637; continue; }
            blk = 636; continue;
        }
        636 => {
            let r866 = c.r64(10) | 0x1u64;
            c.w64(10, r866);
            c.logic(r866, 64);
            blk = 637; continue;
        }
        637 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 638; continue;
        }
        638 => {
            let Some(t867) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086613a); };
            c.w64(1, t867);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000866141); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000866144); }
            blk = 269; continue;
        }
        639 => {
            let Some(t868) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086614e); };
            c.w32(8, t868);
            let Some(t869) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866152); };
            c.cmp(t869, c.r32(8), 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 640; continue;
        }
        640 => {
            let Some(t870) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086615f); };
            c.w64(6, t870);
            return c.call(0x70008a1014, 0x7000866168, 0x7000866163);
        }
        641 => {
            blk = 274; continue;
        }
        642 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866173); }
            let Some(t871) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000866175); };
            c.w64(8, t871);
            let Some(t872) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x700086617c); };
            c.w32(8, t872);
            let r873 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r873);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866186); }
            let Some(t874) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000866188); };
            c.w64(3, t874);
            c.w32(0, 0x2u64);
            let Some(t875) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000866194); };
            c.w64(6, t875);
            return c.call(0x700086ed74, 0x700086619d, 0x7000866198);
        }
        643 => {
            return RegfileExit::fault(0x700086619d);
        }
        644 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008661a3); }
            return c.call(0x70007febf4, 0x70008661ad, 0x70008661a8);
        }
        645 => {
            c.w64(3, c.r64(0));
            let Some(t876) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008661b0); };
            c.w64(0, t876);
            let Some(t877) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008661b4); };
            c.w64(12, t877);
            let Some(t878) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008661b8); };
            c.w64(9, t878);
            let Some(t879) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008661bf); };
            c.w64(7, t879);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008661c3); }
            blk = 301; continue;
        }
        646 => {
            let Some(t880) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008661cd); };
            c.w64(1, t880);
            let Some(t881) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008661d4); };
            c.w64(6, t881);
            c.w64(1, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008661db); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008661df); }
            let Some(t882) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008661e3); };
            c.w64(6, t882);
            let Some(t883) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008661ea); };
            c.cmp(t883, c.r64(1), 64);
            if c.cc_a() { blk = 649; continue; }
            blk = 647; continue;
        }
        647 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008661fc, 0x70008661f7);
        }
        648 => {
            c.w64(1, c.r64(0));
            let Some(t884) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008661ff); };
            c.w64(0, t884);
            let Some(t885) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000866203); };
            c.w64(12, t885);
            let Some(t886) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000866207); };
            c.w64(9, t886);
            let Some(t887) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086620e); };
            c.w64(7, t887);
            let Some(t888) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000866212); };
            c.w64(3, t888);
            let Some(t889) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866216); };
            c.w64(2, t889);
            blk = 650; continue;
        }
        649 => {
            let Some(t890) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086621f); };
            c.w64(6, t890);
            if !c.st64(c.r64(6), c.r64(1)) { return RegfileExit::fault(0x7000866226); }
            let Some(t891) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000866229); };
            c.w64(1, t891);
            let r892 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r892);
            blk = 650; continue;
        }
        650 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000866231); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 654; continue; }
            blk = 651; continue;
        }
        651 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 653; continue; }
            blk = 652; continue;
        }
        652 => {
            let r893 = c.r64(10) | 0x1u64;
            c.w64(10, r893);
            c.logic(r893, 64);
            blk = 653; continue;
        }
        653 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 654; continue;
        }
        654 => {
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700086625b); }
            c.w64(2, c.r64(1));
            blk = 303; continue;
        }
        655 => {
            let Some(t894) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000866268); };
            c.w32(8, t894);
            let Some(t895) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086626c); };
            c.cmp(t895, c.r32(8), 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 656; continue;
        }
        656 => {
            let Some(t896) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000866279); };
            c.w64(6, t896);
            return c.call(0x70008a1014, 0x7000866282, 0x700086627d);
        }
        657 => {
            blk = 308; continue;
        }
        658 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086628d); }
            let Some(t897) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086628f); };
            c.w64(8, t897);
            let Some(t898) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x7000866296); };
            c.w32(8, t898);
            let r899 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r899);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008662a0); }
            let Some(t900) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008662a2); };
            c.w64(3, t900);
            c.w32(0, 0x2u64);
            let Some(t901) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008662ae); };
            c.w64(6, t901);
            return c.call(0x700086ed74, 0x70008662b7, 0x70008662b2);
        }
        659 => {
            return RegfileExit::fault(0x70008662b7);
        }
        660 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008662bd); }
            return c.call(0x70007febf4, 0x70008662c7, 0x70008662c2);
        }
        661 => {
            c.w64(3, c.r64(0));
            let Some(t902) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008662ca); };
            c.w64(0, t902);
            let Some(t903) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008662ce); };
            c.w64(12, t903);
            let Some(t904) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008662d2); };
            c.w64(9, t904);
            let Some(t905) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008662d9); };
            c.w64(7, t905);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008662dd); }
            blk = 334; continue;
        }
        662 => {
            let Some(t906) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008662e7); };
            c.w64(1, t906);
            let Some(t907) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008662ee); };
            c.w64(6, t907);
            c.w64(1, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008662f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008662f9); }
            let Some(t908) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008662fd); };
            c.w64(6, t908);
            let Some(t909) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000866304); };
            c.cmp(t909, c.r64(1), 64);
            if c.cc_a() { blk = 665; continue; }
            blk = 663; continue;
        }
        663 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000866316, 0x7000866311);
        }
        664 => {
            c.w64(1, c.r64(0));
            let Some(t910) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000866319); };
            c.w64(0, t910);
            let Some(t911) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700086631d); };
            c.w64(12, t911);
            let Some(t912) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000866321); };
            c.w64(9, t912);
            let Some(t913) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000866328); };
            c.w64(7, t913);
            let Some(t914) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086632c); };
            c.w64(3, t914);
            let Some(t915) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866330); };
            c.w64(2, t915);
            blk = 666; continue;
        }
        665 => {
            let Some(t916) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000866339); };
            c.w64(6, t916);
            if !c.st64(c.r64(6), c.r64(1)) { return RegfileExit::fault(0x7000866340); }
            let Some(t917) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000866343); };
            c.w64(1, t917);
            let r918 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r918);
            blk = 666; continue;
        }
        666 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700086634b); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 670; continue; }
            blk = 667; continue;
        }
        667 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 669; continue; }
            blk = 668; continue;
        }
        668 => {
            let r919 = c.r64(10) | 0x1u64;
            c.w64(10, r919);
            c.logic(r919, 64);
            blk = 669; continue;
        }
        669 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 670; continue;
        }
        670 => {
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000866375); }
            c.w64(2, c.r64(1));
            blk = 336; continue;
        }
        671 => {
            let Some(t920) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000866382); };
            c.w32(8, t920);
            let Some(t921) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866386); };
            c.cmp(t921, c.r32(8), 32);
            if c.cc_e() { blk = 341; continue; }
            blk = 672; continue;
        }
        672 => {
            let Some(t922) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000866393); };
            c.w64(6, t922);
            return c.call(0x70008a1014, 0x700086639c, 0x7000866397);
        }
        673 => {
            blk = 341; continue;
        }
        674 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008663a7); }
            let Some(t923) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008663a9); };
            c.w64(8, t923);
            let Some(t924) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x70008663b0); };
            c.w32(8, t924);
            let r925 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r925);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008663ba); }
            let Some(t926) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008663bc); };
            c.w64(3, t926);
            c.w32(0, 0x2u64);
            let Some(t927) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008663c8); };
            c.w64(6, t927);
            return c.call(0x700086ed74, 0x70008663d1, 0x70008663cc);
        }
        675 => {
            return RegfileExit::fault(0x70008663d1);
        }
        676 => {
            let Some(t928) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008663d2); };
            c.w64(2, t928);
            let Some(t929) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008663d9); };
            c.w64(1, t929);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008663e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008663e4); }
            let Some(t930) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008663e8); };
            c.w64(1, t930);
            let Some(t931) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008663ef); };
            c.cmp(t931, c.r64(6), 64);
            if c.cc_a() { blk = 679; continue; }
            blk = 677; continue;
        }
        677 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008663fe, 0x70008663f9);
        }
        678 => {
            c.w64(2, c.r64(0));
            let Some(t932) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000866401); };
            c.w64(0, t932);
            let Some(t933) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000866405); };
            c.w64(12, t933);
            let Some(t934) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000866409); };
            c.w64(9, t934);
            let Some(t935) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000866410); };
            c.w64(7, t935);
            let Some(t936) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000866414); };
            c.w64(3, t936);
            blk = 680; continue;
        }
        679 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x700086641a); }
            let Some(t937) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086641d); };
            c.w64(2, t937);
            let r938 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r938);
            blk = 680; continue;
        }
        680 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 684; continue; }
            blk = 681; continue;
        }
        681 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 683; continue; }
            blk = 682; continue;
        }
        682 => {
            let r939 = c.r64(10) | 0x1u64;
            c.w64(10, r939);
            c.logic(r939, 64);
            blk = 683; continue;
        }
        683 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 684; continue;
        }
        684 => {
            let Some(t940) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086644c); };
            c.w64(1, t940);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000866453); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000866456); }
            blk = 367; continue;
        }
        685 => {
            let Some(t941) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000866460); };
            c.w32(8, t941);
            let Some(t942) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866464); };
            c.cmp(t942, c.r32(8), 32);
            if c.cc_e() { blk = 372; continue; }
            blk = 686; continue;
        }
        686 => {
            let Some(t943) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000866471); };
            c.w64(6, t943);
            return c.call(0x70008a1014, 0x700086647a, 0x7000866475);
        }
        687 => {
            blk = 372; continue;
        }
        688 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866485); }
            let Some(t944) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000866487); };
            c.w64(8, t944);
            let Some(t945) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x700086648e); };
            c.w32(8, t945);
            let r946 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r946);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866498); }
            let Some(t947) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086649a); };
            c.w64(3, t947);
            c.w32(0, 0x2u64);
            let Some(t948) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008664a6); };
            c.w64(6, t948);
            return c.call(0x700086ed74, 0x70008664af, 0x70008664aa);
        }
        689 => {
            return RegfileExit::fault(0x70008664af);
        }
        690 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008664b0); }
            c.w32(2, 0x10u64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febb4, 0x70008664c1, 0x70008664bc);
        }
        691 => {
            c.w64(8, c.r64(0));
            let Some(t949) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008664c4); };
            c.w64(0, t949);
            let Some(t950) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008664c8); };
            c.w64(3, t950);
            let Some(t951) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008664cc); };
            c.w64(12, t951);
            let Some(t952) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008664d3); };
            c.w64(9, t952);
            let Some(t953) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008664d7); };
            c.w64(2, t953);
            blk = 427; continue;
        }
        692 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008664ea, 0x70008664e5);
        }
        693 => {
            c.w64(8, c.r64(0));
            let Some(t954) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008664ed); };
            c.w64(0, t954);
            let Some(t955) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008664f1); };
            c.w64(3, t955);
            let Some(t956) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008664f5); };
            c.w64(12, t956);
            let Some(t957) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008664fc); };
            c.w64(9, t957);
            blk = 430; continue;
        }
        694 => {
            let Some(t958) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000866505); };
            c.w64(2, t958);
            let Some(t959) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086650c); };
            c.w64(1, t959);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x7000866513); }
            let Some(t960) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000866517); };
            c.w64(7, t960);
            let Some(t961) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086651e); };
            c.cmp(t961, c.r64(6), 64);
            if c.cc_a() { blk = 697; continue; }
            blk = 695; continue;
        }
        695 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700086652d, 0x7000866528);
        }
        696 => {
            c.w64(2, c.r64(0));
            let Some(t962) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000866530); };
            c.w64(0, t962);
            let Some(t963) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000866534); };
            c.w64(3, t963);
            let Some(t964) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000866538); };
            c.w64(12, t964);
            let Some(t965) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086653f); };
            c.w64(9, t965);
            let Some(t966) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000866543); };
            c.w64(8, t966);
            blk = 698; continue;
        }
        697 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000866549); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 698; continue;
        }
        698 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 702; continue; }
            blk = 699; continue;
        }
        699 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 701; continue; }
            blk = 700; continue;
        }
        700 => {
            let r967 = c.r64(10) | 0x1u64;
            c.w64(10, r967);
            c.logic(r967, 64);
            blk = 701; continue;
        }
        701 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 702; continue;
        }
        702 => {
            let Some(t968) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866577); };
            c.w64(1, t968);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700086657e); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000866581); }
            blk = 433; continue;
        }
        703 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866591); }
            let Some(t969) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000866593); };
            c.w64(8, t969);
            let Some(t970) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x700086659a); };
            c.w32(8, t970);
            let r971 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r971);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008665a4); }
            let Some(t972) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008665a6); };
            c.w64(3, t972);
            c.w32(0, 0x2u64);
            let Some(t973) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008665b2); };
            c.w64(6, t973);
            return c.call(0x700086ed74, 0x70008665bb, 0x70008665b6);
        }
        704 => {
            return RegfileExit::fault(0x70008665bb);
        }
        705 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008665bc); }
            c.w32(2, 0x10u64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febb4, 0x70008665cd, 0x70008665c8);
        }
        706 => {
            c.w64(9, c.r64(0));
            let Some(t974) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008665d0); };
            c.w64(0, t974);
            let Some(t975) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008665d4); };
            c.w64(3, t975);
            let Some(t976) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008665d8); };
            c.w64(8, t976);
            let Some(t977) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008665dc); };
            c.w64(7, t977);
            let Some(t978) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008665e0); };
            c.w64(6, t978);
            let Some(t979) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008665e4); };
            c.w64(15, t979);
            let Some(t980) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008665e8); };
            c.w64(2, t980);
            blk = 474; continue;
        }
        707 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008665fb, 0x70008665f6);
        }
        708 => {
            c.w64(9, c.r64(0));
            let Some(t981) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008665fe); };
            c.w64(0, t981);
            let Some(t982) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000866602); };
            c.w64(3, t982);
            let Some(t983) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000866606); };
            c.w64(8, t983);
            let Some(t984) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086660a); };
            c.w64(7, t984);
            let Some(t985) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700086660e); };
            c.w64(6, t985);
            let Some(t986) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000866612); };
            c.w64(12, t986);
            let Some(t987) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000866616); };
            c.w64(15, t987);
            blk = 479; continue;
        }
        709 => {
            let Some(t988) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086661f); };
            c.w64(11, t988);
            let Some(t989) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000866626); };
            c.w64(12, t989);
            c.w64(2, c.r64(12).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x700086662e); }
            let Some(t990) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000866632); };
            c.w64(1, t990);
            let Some(t991) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000866639); };
            c.cmp(t991, c.r64(2), 64);
            if c.cc_a() { blk = 712; continue; }
            blk = 710; continue;
        }
        710 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000866648, 0x7000866643);
        }
        711 => {
            c.w64(11, c.r64(0));
            let Some(t992) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086664b); };
            c.w64(0, t992);
            let Some(t993) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086664f); };
            c.w64(3, t993);
            let Some(t994) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000866653); };
            c.w64(8, t994);
            let Some(t995) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000866657); };
            c.w64(7, t995);
            let Some(t996) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700086665b); };
            c.w64(6, t996);
            let Some(t997) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700086665f); };
            c.w64(9, t997);
            let Some(t998) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000866663); };
            c.w64(15, t998);
            blk = 713; continue;
        }
        712 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x7000866669); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 713; continue;
        }
        713 => {
            let Some(t999) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866671); };
            c.w64(12, t999);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000866678); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(8) as i64));
            c.logic(c.r64(8) & c.r64(8), 64);
            if c.cc_ns() { blk = 717; continue; }
            blk = 714; continue;
        }
        714 => {
            c.w64(10, c.r64(8));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 716; continue; }
            blk = 715; continue;
        }
        715 => {
            let r1000 = c.r64(10) | 0x1u64;
            c.w64(10, r1000);
            c.logic(r1000, 64);
            blk = 716; continue;
        }
        716 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 717; continue;
        }
        717 => {
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008666a3); }
            blk = 482; continue;
        }
        718 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008666b4); }
            let Some(t1001) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008666b6); };
            c.w64(8, t1001);
            let Some(t1002) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x70008666bd); };
            c.w32(8, t1002);
            let r1003 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1003);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008666c7); }
            let Some(t1004) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008666c9); };
            c.w64(3, t1004);
            c.w32(0, 0x2u64);
            let Some(t1005) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008666d5); };
            c.w64(6, t1005);
            return c.call(0x700086ed74, 0x70008666de, 0x70008666d9);
        }
        719 => {
            return RegfileExit::fault(0x70008666de);
        }
        720 => {
            c.w32(8, 0x296u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008666e5); }
            let Some(t1006) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008666e7); };
            c.w64(3, t1006);
            c.w32(0, 0x1u64);
            let Some(t1007) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008666f3); };
            c.w64(6, t1007);
            return c.call(0x700086ed74, 0x70008666fc, 0x70008666f7);
        }
        721 => {
            return RegfileExit::fault(0x70008666fc);
        }
        722 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866703); }
            let Some(t1008) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000866705); };
            c.w64(8, t1008);
            let Some(t1009) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x700086670c); };
            c.w32(8, t1009);
            let r1010 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1010);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866716); }
            let Some(t1011) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000866718); };
            c.w64(3, t1011);
            c.w32(0, 0x2u64);
            let Some(t1012) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000866724); };
            c.w64(6, t1012);
            return c.call(0x700086ed74, 0x700086672d, 0x7000866728);
        }
        723 => {
            return RegfileExit::fault(0x700086672d);
        }
        724 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866734); }
            let Some(t1013) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000866736); };
            c.w64(8, t1013);
            let Some(t1014) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x700086673d); };
            c.w32(8, t1014);
            let r1015 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1015);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866747); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000866749); }
            let Some(t1016) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086674a); };
            c.w64(3, t1016);
            c.w32(0, 0x3u64);
            let Some(t1017) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000866756); };
            c.w64(6, t1017);
            return c.call(0x700086ed74, 0x700086675f, 0x700086675a);
        }
        725 => {
            return RegfileExit::fault(0x700086675f);
        }
        726 => {
            c.w32(9, 0x32u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000866766); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866768); }
            let Some(t1018) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086676a); };
            c.w64(3, t1018);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700086677b, 0x7000866776);
        }
        727 => {
            return RegfileExit::fault(0x700086677b);
        }
        728 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866782); }
            let Some(t1019) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000866784); };
            c.w64(8, t1019);
            let Some(t1020) = c.ld32(c.r64(8).wrapping_add(0xd3)) else { return RegfileExit::fault(0x700086678b); };
            c.w32(8, t1020);
            let r1021 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1021);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000866795); }
            let Some(t1022) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000866797); };
            c.w64(3, t1022);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008667a8, 0x70008667a3);
        }
        729 => {
            return RegfileExit::fault(0x70008667a8);
        }
        730 => {
            c.w32(8, 0xd6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008667af); }
            let Some(t1023) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008667b1); };
            c.w64(3, t1023);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008667c2, 0x70008667bd);
        }
        731 => {
            return RegfileExit::fault(0x70008667c2);
        }
        732 => {
            let r1024 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1024);
            if !c.movsd_mr(c.r64(4), 2) { return RegfileExit::fault(0x70008667c7); }
            return c.call(0x70008708b4, 0x70008667d1, 0x70008667cc);
        }
        733 => {
            let Some(t1025) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008667d1); };
            c.w32(9, t1025);
            let r1026 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1026);
            blk = 408; continue;
        }
        734 => {
            let r1027 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1027);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008667e2); }
            return c.call(0x70008708b4, 0x70008667ec, 0x70008667e7);
        }
        735 => {
            let Some(t1028) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008667ec); };
            c.w32(9, t1028);
            let r1029 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1029);
            blk = 288; continue;
        }
        736 => {
            let r1030 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1030);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008667fd); }
            return c.call(0x70008708b4, 0x7000866807, 0x7000866802);
        }
        737 => {
            let Some(t1031) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000866807); };
            c.w32(9, t1031);
            let r1032 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1032);
            blk = 254; continue;
        }
        738 => {
            let r1033 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1033);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000866818); }
            return c.call(0x70008708b4, 0x7000866822, 0x700086681d);
        }
        739 => {
            let Some(t1034) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000866822); };
            c.w32(9, t1034);
            let r1035 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1035);
            blk = 220; continue;
        }
        740 => {
            let r1036 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1036);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000866833); }
            return c.call(0x70008708b4, 0x700086683d, 0x7000866838);
        }
        741 => {
            let Some(t1037) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700086683d); };
            c.w32(9, t1037);
            let r1038 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1038);
            blk = 188; continue;
        }
        742 => {
            let r1039 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1039);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700086684e); }
            return c.call(0x70008708b4, 0x7000866858, 0x7000866853);
        }
        743 => {
            let Some(t1040) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000866858); };
            c.w32(9, t1040);
            let r1041 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1041);
            blk = 156; continue;
        }
        744 => {
            let r1042 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1042);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000866869); }
            return c.call(0x70008708b4, 0x7000866873, 0x700086686e);
        }
        745 => {
            let Some(t1043) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000866873); };
            c.w32(9, t1043);
            let r1044 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1044);
            blk = 124; continue;
        }
        746 => {
            return RegfileExit::jump(0x7000866884);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000866894); }
            c.w64(5, c.r64(4));
            if !c.push64(0xcu64) { return RegfileExit::fault(0x7000866898); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700086689e); }
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008668a2); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008668a8); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008668ae); };
            c.w32(6, t5);
            let r6 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r6);
            let ea7 = c.r64(7).wrapping_add(0x1f);
            let Some(t8) = c.ld32(ea7) else { return RegfileExit::fault(0x70008668b4); };
            let r9 = c.add(t8, 0xfffffffffffffffeu64, 32);
            if !c.st32(ea7, r9) { return RegfileExit::fault(0x70008668b4); }
            if c.cc_le() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008668be); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008668c5); };
            c.w64(11, t12);
            let Some(t13) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008668c9); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008668cc); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(11).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008668d0); };
            c.w64(11, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008668d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008668d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008668dc); }
            c.logic(c.r32(1) & c.r32(1), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(10, c.r64(4).wrapping_sub(0x8));
            if let Some(sr) = c.shl(c.r64(1), 0x3u64, 64) { c.w64(1, sr); }
            let r16 = c.sub(c.r64(4), c.r64(1), 64);
            c.w64(4, r16);
            c.w64(15, c.r64(4).wrapping_sub(0x8));
            let r17 = c.sub(c.r64(4), c.r64(1), 64);
            c.w64(4, r17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008668fc); };
            c.w64(1, t18);
            c.w32(3, 0x18u64);
            c.w64(2, c.r64(1));
            if let Some(sr) = c.shl(c.r64(2), 0x3u64, 64) { c.w64(2, sr); }
            let r19 = c.add(c.r64(2), 0x18u64, 64);
            c.w64(2, r19);
            if let Some(sr) = c.shl(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let r20 = c.add(c.r64(11), c.r64(8), 64);
            c.w64(11, r20);
            blk = 3; continue;
        }
        3 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000866917); };
            c.w64(0, t21);
            let Some(t22) = c.ld32(c.r64(11)) else { return RegfileExit::fault(0x700086691c); };
            c.w32(12, t22);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_ne() { blk = 24; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w32(0, c.r32(0));
            if !c.st64(c.r64(10), c.r64(0)) { return RegfileExit::fault(0x7000866939); }
            let r23 = c.sub(c.r64(10), 0x8u64, 64);
            c.w64(10, r23);
            blk = 6; continue;
        }
        6 => {
            let r24 = c.add(c.r64(3), 0x8u64, 64);
            c.w64(3, r24);
            let r25 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r25);
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ne() { blk = 3; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086694d); };
            c.w64(0, t26);
            if let Some(sr) = c.shl(c.r64(0), 0x3u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000866955); }
            c.w64(7, c.r64(5));
            let r27 = c.add(c.r64(7), 0xffffffffffffffc0u64, 64);
            c.w64(7, r27);
            let r28 = c.sub(c.r64(7), c.r64(0), 64);
            c.w64(7, r28);
            if !c.movsd_rm(1, c.r64(7)) { return RegfileExit::fault(0x7000866963); }
            if !c.movsd_rm(2, c.r64(7).wrapping_sub(0x8)) { return RegfileExit::fault(0x7000866967); }
            if !c.movsd_rm(3, c.r64(7).wrapping_sub(0x10)) { return RegfileExit::fault(0x700086696c); }
            if !c.movsd_rm(4, c.r64(7).wrapping_sub(0x18)) { return RegfileExit::fault(0x7000866971); }
            if !c.movsd_rm(5, c.r64(7).wrapping_sub(0x20)) { return RegfileExit::fault(0x7000866976); }
            if !c.movsd_rm(6, c.r64(7).wrapping_sub(0x28)) { return RegfileExit::fault(0x700086697b); }
            let r29 = c.sub(c.r64(7), 0x28u64, 64);
            c.w64(7, r29);
            c.w64(8, c.r64(5));
            let r30 = c.add(c.r64(8), 0xffffffffffffffc0u64, 64);
            c.w64(8, r30);
            let Some(t31) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700086698b); };
            c.w64(0, t31);
            let Some(t32) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700086698e); };
            c.w64(2, t32);
            let Some(t33) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000866992); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(8).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000866996); };
            c.w64(3, t34);
            let Some(t35) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086699a); };
            c.w64(9, t35);
            let r36 = c.sub(c.r64(8), 0x20u64, 64);
            c.w64(8, r36);
            let r37 = c.add(c.r64(10), 0x8u64, 64);
            c.w64(10, r37);
            let r38 = c.add(c.r64(15), 0x8u64, 64);
            c.w64(15, r38);
            blk = 8; continue;
        }
        8 => {
            let r39 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r39);
            c.cmp(c.r64(8), c.r64(10), 64);
            if c.cc_g() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_le() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t40) = c.ld32(c.r64(11)) else { return RegfileExit::fault(0x70008669c0); };
            c.w32(12, t40);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(12), 0x4u64, 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008669eb);
        }
        15 => {
            c.cmp(c.r64(8), c.r64(10), 64);
            if c.cc_le() { blk = 8; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t41) = c.ld64(c.r64(10)) else { return RegfileExit::fault(0x70008669f1); };
            if !c.push64(t41) { return RegfileExit::fault(0x70008669f1); }
            let r42 = c.add(c.r64(10), 0x8u64, 64);
            c.w64(10, r42);
            blk = 8; continue;
        }
        17 => {
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_le() { blk = 8; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t43) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008669ff); };
            if !c.push64(t43) { return RegfileExit::fault(0x70008669ff); }
            let r44 = c.add(c.r64(15), 0x8u64, 64);
            c.w64(15, r44);
            blk = 8; continue;
        }
        19 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000866a08); };
            c.w64(7, t45);
            blk = 20; continue;
        }
        20 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x3090)) else { return RegfileExit::fault(0x7000866a0c); };
            c.w64(12, t46);
            if !c.st32(c.r64(12), 0x1u64) { return RegfileExit::fault(0x7000866a13); }
            let Some(t47) = c.ld64(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000866a1b); };
            c.w64(7, t47);
            if !c.st64(c.r64(5).wrapping_sub(0x10), 0x0u64) { return RegfileExit::fault(0x7000866a1f); }
            return c.call(c.r64(7), 0x7000866a29, 0x7000866a27);
        }
        21 => {
            c.w64(4, c.r64(5).wrapping_sub(0x38));
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x3090)) else { return RegfileExit::fault(0x7000866a2d); };
            c.w64(8, t48);
            if !c.st32(c.r64(8), 0x0u64) { return RegfileExit::fault(0x7000866a34); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000866a3b); };
            c.w64(8, t49);
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000866a49); };
            c.w64(0, t50);
            blk = 23; continue;
        }
        23 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000866a50); };
            c.w64(1, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000866a54); };
            c.w64(2, t52);
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_l() { c.w64(1, c.r64(2)); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000866a62); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x7000866a63); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000866a69); }
            return c.ret(0x0, 0x7000866a6a);
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), 0x2u64) { return RegfileExit::fault(0x7000866a6b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000866a73); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000866a74); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000866a75); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000866a77); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000866a79); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000866a7b); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000866a7c); }
            let Some(t53) = c.ld32(c.r64(6).wrapping_add(0xc7)) else { return RegfileExit::fault(0x7000866a7d); };
            c.w32(6, t53);
            let r54 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r54);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_e() { blk = 31; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r64(12), 0x4u64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x7000866aae);
        }
        29 => {
            return c.call(0x70008ecfd4, 0x7000866ab4, 0x7000866aaf);
        }
        30 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000866ab4); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x7000866ab5); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x7000866ab6); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x7000866ab8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000866aba); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x7000866abc); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x7000866abd); }
            if !c.st64(c.r64(10), c.r64(0)) { return RegfileExit::fault(0x7000866abe); }
            let r55 = c.sub(c.r64(10), 0x8u64, 64);
            c.w64(10, r55);
            blk = 6; continue;
        }
        31 => {
            return c.call(0x70007ff914, 0x7000866acf, 0x7000866aca);
        }
        32 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000866acf); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x7000866ad0); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x7000866ad1); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x7000866ad3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000866ad5); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x7000866ad7); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x7000866ad8); }
            if !c.st64(c.r64(10), c.r64(0)) { return RegfileExit::fault(0x7000866ad9); }
            let r56 = c.sub(c.r64(10), 0x8u64, 64);
            c.w64(10, r56);
            blk = 6; continue;
        }
        33 => {
            return c.call(0x70008ed054, 0x7000866aea, 0x7000866ae5);
        }
        34 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000866aea); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x7000866aeb); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x7000866aec); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x7000866aee); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000866af0); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x7000866af2); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x7000866af3); }
            c.xorpd_rr(1, 1);
            c.cvtsd2ss(1, c.xmm_u64(0));
            if !c.movsd_mr(c.r64(15), 1) { return RegfileExit::fault(0x7000866afc); }
            let r57 = c.sub(c.r64(15), 0x8u64, 64);
            c.w64(15, r57);
            blk = 6; continue;
        }
        35 => {
            return c.call(0x70008ed054, 0x7000866b0f, 0x7000866b0a);
        }
        36 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000866b0f); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x7000866b10); }
            if !c.pop64_reg(11) { return RegfileExit::fault(0x7000866b11); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x7000866b13); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000866b15); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x7000866b17); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x7000866b18); }
            if !c.movsd_mr(c.r64(15), 0) { return RegfileExit::fault(0x7000866b19); }
            let r58 = c.sub(c.r64(15), 0x8u64, 64);
            c.w64(15, r58);
            blk = 6; continue;
        }
        37 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000866b27); };
            c.w64(11, t59);
            let Some(t60) = c.ld32(c.r64(11)) else { return RegfileExit::fault(0x7000866b2b); };
            c.w32(12, t60);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(12), 0x4u64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x7000866b56);
        }
        42 => {
            c.w64(3, c.r64(0));
            let r61 = c.add(c.r32(3), c.r32(0), 32);
            c.w32(3, r61);
            if c.cc_o() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            let r62 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r62);
            blk = 23; continue;
        }
        44 => {
            return c.call(0x70008ecf54, 0x7000866b6e, 0x7000866b69);
        }
        45 => {
            blk = 23; continue;
        }
        46 => {
            return c.call(0x70007ff9b4, 0x7000866b78, 0x7000866b73);
        }
        47 => {
            blk = 23; continue;
        }
        48 => {
            c.movss_rr(0, 1);
            return c.call(0x7000866d54, 0x7000866b86, 0x7000866b81);
        }
        49 => {
            blk = 23; continue;
        }
        50 => {
            c.movsd_rr(0, 1);
            return c.call(0x7000866df4, 0x7000866b94, 0x7000866b8f);
        }
        51 => {
            blk = 23; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), 0x4u64) { return RegfileExit::fault(0x7000866b99); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000866ba1); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000866ba2); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000866ba3); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000866ba4); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2bd8)) else { return RegfileExit::fault(0x7000866bac); };
            c.w64(3, t63);
            return c.call(0x700086ed74, 0x7000866bb8, 0x7000866bb3);
        }
        53 => {
            c.w64(9, c.r64(0));
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000866bbb); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x7000866bbc); }
            blk = 1; continue;
        }
        54 => {
            return RegfileExit::fault(0x7000866bc2);
        }
        55 => {
            return RegfileExit::fault(0x7000866bc3);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.pop64_reg(15) { return RegfileExit::fault(0x7000866bd4); }
            let r0 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r0);
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000866bd9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x14u64) { return RegfileExit::fault(0x7000866bdd); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000866bdf); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000866be0); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000866be1); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000866be2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000866be3); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000866be4); }
            let r1 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r1);
            if !c.mov128_mr(c.r64(4), 1) { return RegfileExit::fault(0x7000866bea); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 2) { return RegfileExit::fault(0x7000866bf0); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 3) { return RegfileExit::fault(0x7000866bf7); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 4) { return RegfileExit::fault(0x7000866bfe); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 5) { return RegfileExit::fault(0x7000866c05); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 6) { return RegfileExit::fault(0x7000866c0c); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000866c13); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000866c14); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2bd0)) else { return RegfileExit::fault(0x7000866c1d); };
            c.w64(3, t2);
            return c.call(0x700086ed74, 0x7000866c29, 0x7000866c24);
        }
        1 => {
            c.w64(15, c.r64(0));
            if !c.mov128_rm(6, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x7000866c2c); }
            if !c.mov128_rm(5, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x7000866c33); }
            if !c.mov128_rm(4, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x7000866c3a); }
            if !c.mov128_rm(3, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x7000866c41); }
            if !c.mov128_rm(2, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x7000866c48); }
            if !c.mov128_rm(1, c.r64(4)) { return RegfileExit::fault(0x7000866c4f); }
            let r3 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r3);
            if !c.pop64_reg(9) { return RegfileExit::fault(0x7000866c59); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x7000866c5b); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x7000866c5c); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x7000866c5d); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x7000866c5e); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x7000866c5f); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000866c63); }
            return RegfileExit::jump(c.r64(15));
        }
        2 => {
            return RegfileExit::fault(0x7000866c67);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866c74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000866c74); }
            c.w64(5, c.r64(4));
            if !c.push64(0xeu64) { return RegfileExit::fault(0x7000866c78); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000866c7a); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000866c7c); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000866c7d); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000866c7e); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000866c7f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000866c80); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000866c81); }
            let r0 = c.sub(c.r64(4), 0x80u64, 64);
            c.w64(4, r0);
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 7) { return RegfileExit::fault(0x7000866c89); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 6) { return RegfileExit::fault(0x7000866c90); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 5) { return RegfileExit::fault(0x7000866c97); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 4) { return RegfileExit::fault(0x7000866c9e); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 3) { return RegfileExit::fault(0x7000866ca5); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 2) { return RegfileExit::fault(0x7000866cac); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 1) { return RegfileExit::fault(0x7000866cb3); }
            if !c.mov128_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000866cba); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2be8)) else { return RegfileExit::fault(0x7000866cc4); };
            c.w64(3, t1);
            return c.call(0x700086ed74, 0x7000866cd0, 0x7000866ccb);
        }
        1 => {
            if !c.mov128_rm(0, c.r64(4)) { return RegfileExit::fault(0x7000866cd0); }
            if !c.mov128_rm(1, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x7000866cd6); }
            if !c.mov128_rm(2, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x7000866cdd); }
            if !c.mov128_rm(3, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x7000866ce4); }
            if !c.mov128_rm(4, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x7000866ceb); }
            if !c.mov128_rm(5, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x7000866cf2); }
            if !c.mov128_rm(6, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x7000866cf9); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x7000866d00); }
            let r2 = c.add(c.r64(4), 0x80u64, 64);
            c.w64(4, r2);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x7000866d0e); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x7000866d0f); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x7000866d10); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x7000866d11); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x7000866d12); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000866d13); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x7000866d14); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000866d19); }
            return c.ret(0x0, 0x7000866d1a);
        }
        2 => {
            return RegfileExit::fault(0x7000866d1b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866d34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000866d34); };
            c.w64(10, t0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), 0x0u64) { return RegfileExit::fault(0x7000866d38); }
            return RegfileExit::jump(c.r64(10));
        }
        1 => {
            return RegfileExit::fault(0x7000866d43);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let cv = c.cvttss2si32(c.xmm_u32(0)); c.w32(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2ss(1, (c.r32(3) as u32 as i32 as i64));
            c.ucomiss(c.xmm_u32(1), c.xmm_u32(0));
            if c.cc_p() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            if c.cc_ne() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.movd_rx(1, 0);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(1, c.r64(3));
            let r0 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r0);
            if c.cc_o() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x7000866d7e);
        }
        6 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000866d7f); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000866d86); };
            c.w64(1, t2);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000866d8d); };
            c.w64(7, t3);
            let Some(t4) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000866d94); };
            c.cmp(t4, c.r64(6), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000866d99); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            c.cvtss2sd(0, c.xmm_u32(0));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866da4); };
            c.w64(1, t5);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000866dab); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000866dae); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x7000866db6);
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000866db7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000866dbb); }
            let r6 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r6);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000866dc6); }
            return c.call(0x70007febf4, 0x7000866dd0, 0x7000866dcb);
        }
        10 => {
            c.w64(3, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000866dd3); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000866ddb); }
            blk = 8; continue;
        }
        11 => {
            return RegfileExit::jump(0x7000866de0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866df4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            if c.cc_ne() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.movq_rx(1, 0);
            if let Some(sr) = c.shr(c.r64(1), 0x20u64, 64) { c.w64(1, sr); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(1, c.r64(3));
            let r0 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r0);
            if c.cc_o() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x7000866e25);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000866e26); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000866e2a); }
            let r1 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1);
            blk = 9; continue;
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000866e32); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000866e36); }
            let r2 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r2);
            blk = 9; continue;
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000866e3e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000866e42); }
            let r3 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r3);
            blk = 9; continue;
        }
        9 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000866e48); };
            c.w64(3, t4);
            let Some(t5) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000866e4f); };
            c.w64(1, t5);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000866e56); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000866e5b); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000866e62); };
            c.cmp(t7, c.r64(6), 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000866e71, 0x7000866e6c);
        }
        11 => {
            blk = 13; continue;
        }
        12 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000866e73); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(3));
            blk = 13; continue;
        }
        13 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000866e7d); };
            c.w64(3, t8);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000866e84); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000866e87); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000866e8c); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000866e94); }
            return c.ret(0x0, 0x7000866e95);
        }
        14 => {
            return RegfileExit::jump(0x7000866e98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866eb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            return RegfileExit::fault(0x7000866eb4);
        }
        1 => {
            return RegfileExit::jump(0x7000866eb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866ed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            return RegfileExit::fault(0x7000866ed4);
        }
        1 => {
            return RegfileExit::jump(0x7000866ed8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x3090)) else { return RegfileExit::fault(0x7000866ef4); };
            c.w64(3, t0);
            if !c.st32(c.r64(3), 0x0u64) { return RegfileExit::fault(0x7000866efb); }
            let Some(t1) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000866f01); };
            c.w64(0, t1);
            return c.ret(0x8, 0x7000866f06);
        }
        1 => {
            return RegfileExit::jump(0x7000866f0c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000866f14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000866f14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000866f18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000866f19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000866f1a); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000866f1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000866f23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 151; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(5));
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000866f39); };
            c.w64(1, t2);
            c.w64(8, c.r64(1));
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000866f42); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000866f49); };
            c.w64(1, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000866f50); }
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000866f54); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_e() { blk = 302; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000866f61); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000866f68); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000866f6f); };
            c.w32(11, t10);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000866f79); }
            c.cmp(c.r64(11), 0x74u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000866f83);
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0x1d7)) else { return RegfileExit::fault(0x7000866f84); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000866f97); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            let Some(t15) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000866f9e); };
            c.logic(t15 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000866faa); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000866fbb); };
            c.w32(15, t18);
            let r19 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r19);
            let Some(t20) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000866fc3); };
            c.cmp(t20, 0xacu64, 16);
            if c.cc_ne() { blk = 36; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000866fd0); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000866fdb); };
            c.logic(t22 & 0x200000u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000866fe6); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000866fed); };
            c.w64(15, t24);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000866ff6); };
            c.w64(11, t25);
            blk = 23; continue;
        }
        14 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000867002); };
            c.w64(11, t26);
            let Some(t27) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000867009); };
            c.w64(15, t27);
            c.w64(0, c.r64(15).wrapping_add(0x4c));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000867010); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000867017); };
            c.cmp(t29, c.r64(0), 64);
            if c.cc_be() { blk = 153; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x7000867020); }
            let r30 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r30);
            blk = 16; continue;
        }
        16 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x7000867027); };
            c.w64(0, t31);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700086702e); }
            c.w32(0, 0x22u64);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000867037); }
            let Some(t32) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086703b); };
            c.w32(0, t32);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 155; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000867051); }
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867055); };
            c.w32(2, t33);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 156; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x7000867069); }
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086706d); };
            c.w32(2, t34);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 157; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0x8u64);
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x7000867086); }
            let Some(t35) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086708a); };
            c.w32(2, t35);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_be() { blk = 158; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, 0x2u64);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008670a3); }
            let Some(t36) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008670a7); };
            c.w32(2, t36);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_be() { blk = 159; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x70008670bb); }
            c.w64(2, c.r64(15).wrapping_add(0x4b));
            c.w64(7, c.r64(15).wrapping_add(0x1b));
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st32(c.r64(7), c.r32(1)) { return RegfileExit::fault(0x70008670cc); }
            let r37 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r37);
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ne() { blk = 22; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t38) = c.ld8(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008670d7); };
            c.w32(2, t38);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let Some(t39) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008670e1); };
            c.w64(7, t39);
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t40) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008670e8); };
            c.cmp(t40, c.r64(8), 64);
            if c.cc_be() { blk = 160; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x70008670f1); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008670fc); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000867100); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000867104); };
            c.w64(15, t41);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086710b); }
            let Some(t42) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086710f); };
            c.w32(15, t42);
            c.w64(3, c.r64(15));
            let r43 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r43);
            c.logic(r43, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            let r44 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r44);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000867129); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 27; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 35; continue;
        }
        29 => {
            let r45 = c.sub(c.r32(15), 0x20000000u64, 32);
            c.w32(15, r45);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700086713c); }
            let Some(t46) = c.ld8(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000867141); };
            c.w32(15, t46);
            c.w64(15, c.r64(15).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x7000867154); };
            c.w64(2, t47);
            blk = 31; continue;
        }
        31 => {
            let r48 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r48);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700086715f); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 31; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let r49 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r49);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000867172); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            let r50 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r50);
            if c.cc_e() { blk = 162; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(0, c.r64(8));
            blk = 37; continue;
        }
        36 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086718d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086718f); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2408)) else { return RegfileExit::fault(0x7000867190); };
            c.w64(3, t51);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008671a1, 0x700086719c);
        }
        37 => {
            c.w64(12, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008671a4); };
            c.w64(6, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008671a8); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008671af); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008671b3); };
            c.w64(9, t55);
            blk = 42; continue;
        }
        38 => {
            let Some(t56) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008671b9); };
            c.w32(11, t56);
            let r57 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008671c0); };
            c.w64(12, t58);
            let Some(t59) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008671c7); };
            c.w64(15, t59);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008671cb); };
            c.w32(0, t60);
            c.w64(2, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008671dc); };
            c.w64(3, t61);
            let Some(t62) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008671e3); };
            c.cmp(t62, c.r64(0), 64);
            if c.cc_be() { blk = 164; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008671ec); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 40; continue;
        }
        40 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008671f4); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008671f9); };
            c.w64(11, t63);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000867200); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000867205); }
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r64 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r64);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x7000867218); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 41; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(12)) { return RegfileExit::fault(0x7000867222); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x7000867230); };
            c.w64(11, t65);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000867239); };
            c.cmp(t66, c.r32(3), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(0, c.r64(8));
            let Some(t67) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867246); };
            c.w32(8, t67);
            let r68 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r68);
            let Some(t69) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086724d); };
            c.cmp(t69, 0x834u64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(15, 0x1u64);
            blk = 47; continue;
        }
        46 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        47 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t70) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700086726a); };
            c.w32(15, t70);
            let r71 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r71);
            let r72 = c.r32(15) & 0x3fu64;
            c.w32(15, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_a() { blk = 56; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t73) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086727d); };
            c.w32(15, t73);
            let r74 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r74);
            let Some(t75) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000867284); };
            c.w32(15, t75);
            let r76 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r76);
            let Some(t77) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086728b); };
            c.w32(2, t77);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4cu64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x700086729b);
        }
        51 => {
            let Some(t78) = c.ld32(c.r64(15).wrapping_add(0x137)) else { return RegfileExit::fault(0x700086729c); };
            c.w32(15, t78);
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008672a3); };
            c.cmp(t79, c.r32(15), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008672a9); };
            c.w64(8, t80);
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008672b0); };
            c.cmp(t81, c.r32(3), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(11, 0x1u64);
            blk = 56; continue;
        }
        54 => {
            c.w64(0, c.r64(8));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(8));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(8, c.r64(3));
            blk = 59; continue;
        }
        58 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008672d9); };
            c.w32(8, t82);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 59; continue;
        }
        59 => {
            c.w64(15, c.r64(8));
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(8, c.r64(15).wrapping_add(c.r64(8)).wrapping_sub(0x1));
            c.w64(15, c.r64(8));
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r83 = c.r64(15) | c.r64(8);
            c.w64(15, r83);
            c.logic(r83, 64);
            c.w64(8, c.r64(15));
            if let Some(sr) = c.shr(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let r84 = c.r64(8) | c.r64(15);
            c.w64(8, r84);
            c.logic(r84, 64);
            c.w64(15, c.r64(8));
            if let Some(sr) = c.shr(c.r64(15), 0x4u64, 64) { c.w64(15, sr); }
            let r85 = c.r64(15) | c.r64(8);
            c.w64(15, r85);
            c.logic(r85, 64);
            c.w64(8, c.r64(15));
            if let Some(sr) = c.shr(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            let r86 = c.r64(8) | c.r64(15);
            c.w64(8, r86);
            c.logic(r86, 64);
            c.w64(15, c.r64(8));
            if let Some(sr) = c.shr(c.r64(15), 0x10u64, 64) { c.w64(15, sr); }
            let r87 = c.r64(15) | c.r64(8);
            c.w64(15, r87);
            c.logic(r87, 64);
            c.w64(8, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(8), 0x4u64, 64);
            c.w8(15, (c.cc_ge() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x700086732f); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(15)) { return RegfileExit::fault(0x7000867333); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000867337); }
            blk = 61; continue;
        }
        60 => {
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000867340); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000867344); };
            c.w64(12, t89);
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867348); };
            c.w64(1, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086734f); };
            c.w64(0, t91);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000867353); }
            blk = 61; continue;
        }
        61 => {
            let Some(t92) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000867357); };
            c.cmp(t92, 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(2, 0x4u64);
            blk = 64; continue;
        }
        63 => {
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000867364); };
            c.w64(2, t93);
            blk = 64; continue;
        }
        64 => {
            c.w64(7, c.r64(2).wrapping_mul(2).wrapping_add(0x3));
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 300; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086737d); };
            c.w64(8, t94);
            let Some(t95) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000867384); };
            c.w64(8, t95);
            c.w64(15, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(15).wrapping_add(c.r64(8)));
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000867393); };
            c.w64(11, t96);
            let Some(t97) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086739a); };
            c.cmp(t97, c.r64(6), 64);
            if c.cc_be() { blk = 166; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 166; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008673b0); };
            c.w64(11, t98);
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x70008673b7); }
            let r99 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r99);
            blk = 68; continue;
        }
        68 => {
            c.w32(11, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008673c2); };
            c.w64(15, t100);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008673c9); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008673cd); }
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2d8)) else { return RegfileExit::fault(0x70008673d1); };
            c.w64(11, t101);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008673d8); }
            let Some(t102) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008673dc); };
            c.w32(11, t102);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 172; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008673f0); }
            let Some(t103) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008673f4); };
            c.w32(11, t103);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 173; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000867408); }
            let Some(t104) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086740c); };
            c.w32(11, t104);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_be() { blk = 174; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w32(11, c.r64(2).wrapping_add(c.r64(2)));
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000867424); }
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x13u64, 64);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let r105 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r105);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700086743a); }
            c.cmp(c.r64(11), 0x13u64, 64);
            if c.cc_ne() { blk = 72; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000867444); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r106 = c.r64(11) & c.r64(12);
            c.w64(11, r106);
            c.logic(r106, 64);
            let Some(t107) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000867459); };
            c.logic(t107 & 0x4u64, 8);
            if c.cc_ne() { blk = 304; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t108) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000867464); };
            c.cmp(t108, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t109) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867473); };
            c.cmp(t109, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let r110 = c.r32(8) | c.r32(11);
            c.w32(8, r110);
            c.logic(r110, 32);
            if c.cc_ne() { blk = 150; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t111) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086748b); };
            c.w32(8, t111);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x5bu64, 64);
            if c.cc_be() { blk = 133; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t112) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086749f); };
            c.w32(11, t112);
            let r113 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r113);
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008674a7); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008674ae); };
            c.w32(11, t116);
            let r117 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r117);
            let Some(t118) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008674b5); };
            c.w32(15, t118);
            let r119 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r119);
            let Some(t120) = c.ld32(c.r64(9).wrapping_add(0x173)) else { return RegfileExit::fault(0x70008674bc); };
            c.cmp(t120, c.r32(11), 32);
            if c.cc_ne() { blk = 175; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t121) = c.ld32(c.r64(15).wrapping_add(0x37)) else { return RegfileExit::fault(0x70008674c9); };
            c.w32(11, t121);
            c.w32(11, c.r32(11));
            let r122 = c.r32(11) & 0x8u64;
            c.w32(11, r122);
            c.logic(r122, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_ne() { blk = 175; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t123) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008674de); };
            c.cmp(t123, 0x0u64, 32);
            if c.cc_e() { blk = 175; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t124) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008674e8); };
            c.w32(11, t124);
            let r125 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r125);
            let Some(t126) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008674ef); };
            c.w32(15, t126);
            let r127 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r127);
            let Some(t128) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008674f6); };
            c.w32(11, t128);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008674fb); }
            c.cmp(c.r64(8), 0xf7u64, 64);
            if c.cc_be() { blk = 134; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t129) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086750c); };
            c.w32(8, t129);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t130) = c.ld32(c.r64(9).wrapping_add(0x3e3)) else { return RegfileExit::fault(0x7000867513); };
            c.w32(7, t130);
            let r131 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r131);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000867520); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000867524); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 150; continue; }
            blk = 82; continue;
        }
        82 => {
            let r132 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r132);
            let r133 = c.r32(11) & 0x3fu64;
            c.w32(11, r133);
            c.logic(r133, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_a() { blk = 135; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w64(11, c.r64(3));
            blk = 85; continue;
        }
        84 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000867549); };
            c.w64(7, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086754d); };
            c.w64(15, t135);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 85; continue;
        }
        85 => {
            let Some(t136) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867553); };
            c.w32(8, t136);
            let r137 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086755b); };
            c.w64(6, t138);
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700086755f); };
            c.cmp(t139, c.r32(8), 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867568); };
            c.w64(8, t140);
            blk = 87; continue;
        }
        87 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 129; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t141) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867579); };
            c.w32(9, t141);
            let r142 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r142);
            let Some(t143) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867580); };
            c.cmp(t143, 0x834u64, 16);
            if c.cc_ne() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), 0x1u64) { return RegfileExit::fault(0x7000867589); }
            blk = 91; continue;
        }
        90 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), 0x0u64) { return RegfileExit::fault(0x7000867593); }
            blk = 91; continue;
        }
        91 => {
            let Some(t144) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086759b); };
            c.cmp(t144, 0x0u64, 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 92; continue;
        }
        92 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008675a5); }
            let Some(t145) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008675a9); };
            c.w32(11, t145);
            let r146 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r146);
            let r147 = c.r32(11) & 0x3fu64;
            c.w32(11, r147);
            c.logic(r147, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_a() { blk = 128; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t148) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008675c0); };
            c.w32(11, t148);
            let r149 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r149);
            let Some(t150) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008675c7); };
            c.w32(11, t150);
            let r151 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r151);
            let Some(t152) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008675ce); };
            c.w32(12, t152);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4cu64, 64);
            if c.cc_be() { blk = 141; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t153) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008675e2); };
            c.w32(11, t153);
            let Some(t154) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008675e9); };
            c.cmp(t154, c.r32(11), 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008675f3); };
            c.w64(9, t155);
            let Some(t156) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008675fa); };
            c.cmp(t156, c.r32(3), 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867604); };
            c.w32(9, t157);
            let r158 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r158);
            let Some(t159) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086760b); };
            c.w32(11, t159);
            let r160 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r160);
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000867612); };
            c.w32(8, t161);
            let Some(t162) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867616); };
            c.w32(12, t162);
            let r163 = c.sub(c.r32(12), 0x77u64, 32);
            c.w32(12, r163);
            c.cmp(c.r32(12), 0xcu64, 32);
            if c.cc_a() { blk = 107; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_g() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086762f); };
            c.w64(11, t164);
            blk = 102; continue;
        }
        99 => {
            let Some(t165) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867638); };
            c.w32(11, t165);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 142; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t166) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086764c); };
            c.w32(11, t166);
            let r167 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000867653); };
            c.cmp(t168, c.r32(11), 32);
            if c.cc_ne() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086765c); };
            c.w64(11, t169);
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_g() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867669); };
            c.w64(8, t170);
            blk = 125; continue;
        }
        104 => {
            let Some(t171) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867675); };
            c.w32(8, t171);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 143; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t172) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000867689); };
            c.w32(8, t172);
            let r173 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r173);
            let Some(t174) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000867690); };
            c.cmp(t174, c.r32(8), 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086769d); };
            c.w64(8, t175);
            blk = 125; continue;
        }
        107 => {
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008676a9); };
            c.cmp(t176, c.r32(11), 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_g() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008676bc); };
            c.w64(11, t177);
            blk = 117; continue;
        }
        110 => {
            let Some(t178) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008676c5); };
            c.w32(11, t178);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 144; continue; }
            blk = 111; continue;
        }
        111 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x7)) { return RegfileExit::fault(0x70008676d9); }
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t179) = c.ld64(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008676e9); };
            c.cmp(t179, c.r64(11), 64);
            if c.cc_e() { blk = 115; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008676ef); };
            c.w64(11, t180);
            let Some(t181) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008676f6); };
            c.w64(12, t181);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008676fe); };
            c.w64(0, t182);
            let Some(t183) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000867705); };
            c.cmp(t183, c.r64(15), 64);
            if c.cc_be() { blk = 168; continue; }
            blk = 113; continue;
        }
        113 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700086770e); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 114; continue;
        }
        114 => {
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000867716); };
            c.w64(12, t184);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086771d); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000867721); }
            blk = 116; continue;
        }
        115 => {
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867729); };
            c.w64(11, t185);
            blk = 116; continue;
        }
        116 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000867730); };
            c.w64(15, t186);
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_g() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086773a); };
            c.w64(8, t187);
            blk = 125; continue;
        }
        119 => {
            let Some(t188) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867743); };
            c.w32(8, t188);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 145; continue; }
            blk = 120; continue;
        }
        120 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0xf)) { return RegfileExit::fault(0x7000867757); }
            let Some(t189) = c.ld64(0x70008676e1u64) else { return RegfileExit::fault(0x700086775d); };
            c.w64(8, t189);
            let Some(t190) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000867764); };
            c.cmp(t190, c.r64(8), 64);
            if c.cc_e() { blk = 124; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086776a); };
            c.w64(8, t191);
            let Some(t192) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000867771); };
            c.w64(9, t192);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000867778); };
            c.w64(0, t193);
            let Some(t194) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086777f); };
            c.cmp(t194, c.r64(12), 64);
            if c.cc_be() { blk = 170; continue; }
            blk = 122; continue;
        }
        122 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000867788); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 123; continue;
        }
        123 => {
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086778f); };
            c.w64(9, t195);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000867796); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700086779a); }
            blk = 125; continue;
        }
        124 => {
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008677a2); };
            c.w64(8, t196);
            blk = 125; continue;
        }
        125 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008677a9); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008677ab); }
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008677ad); };
            if !c.push64(t197) { return RegfileExit::fault(0x70008677ad); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2274, 0x70008677ba, 0x70008677b5);
        }
        126 => {
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008677ba); };
            c.w64(11, t198);
            let r199 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008677c2); };
            c.w64(1, t200);
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_ne() { blk = 84; continue; }
            blk = 127; continue;
        }
        127 => {
            blk = 146; continue;
        }
        128 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 130; continue;
        }
        129 => {
            c.w32(1, 0x1u64);
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 148; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t201) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008677e6); };
            c.w32(1, t201);
            let r202 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r202);
            let Some(t203) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008677ed); };
            c.w32(1, t203);
            let r204 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r204);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 148; continue; }
            blk = 132; continue;
        }
        132 => {
            blk = 60; continue;
        }
        133 => {
            return RegfileExit::fault(0x7000867808);
        }
        134 => {
            return RegfileExit::fault(0x7000867809);
        }
        135 => {
            let Some(t205) = c.ld64(0x70008676e1u64) else { return RegfileExit::fault(0x700086780a); };
            c.w64(8, t205);
            c.w64(9, c.r64(15));
            let Some(t206) = c.ld64(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867814); };
            c.cmp(t206, c.r64(8), 64);
            if c.cc_e() { blk = 139; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x7)) { return RegfileExit::fault(0x700086781a); }
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000867820); };
            c.w64(1, t207);
            let Some(t208) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000867827); };
            c.w64(8, t208);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086782e); };
            c.w64(11, t209);
            let Some(t210) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000867835); };
            c.cmp(t210, c.r64(9), 64);
            if c.cc_be() { blk = 298; continue; }
            blk = 137; continue;
        }
        137 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700086783e); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 138; continue;
        }
        138 => {
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000867845); };
            c.w64(8, t211);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086784c); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000867850); }
            blk = 139; continue;
        }
        139 => {
            c.w32(8, 0x92u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086785b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086785d); }
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086785e); };
            c.w64(3, t212);
            c.w32(0, 0x2u64);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086786a); };
            c.w64(6, t213);
            return c.call(0x700086ed74, 0x7000867873, 0x700086786e);
        }
        140 => {
            return RegfileExit::fault(0x7000867873);
        }
        141 => {
            return RegfileExit::fault(0x7000867874);
        }
        142 => {
            return RegfileExit::fault(0x7000867875);
        }
        143 => {
            return RegfileExit::fault(0x7000867876);
        }
        144 => {
            return RegfileExit::fault(0x7000867877);
        }
        145 => {
            return RegfileExit::fault(0x7000867878);
        }
        146 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000867879); };
            c.w64(12, t214);
            blk = 150; continue;
        }
        147 => {
            return RegfileExit::fault(0x700086787f);
        }
        148 => {
            c.w32(1, 0x92u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000867885); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000867886); }
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000867888); };
            c.w64(3, t215);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000867899, 0x7000867894);
        }
        149 => {
            return RegfileExit::fault(0x7000867899);
        }
        150 => {
            c.w64(0, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008678a0); }
            return c.ret(0x0, 0x70008678a1);
        }
        151 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008678a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008678a6); }
            let Some(t216) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008678aa); };
            c.w64(3, t216);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008678bb, 0x70008678b6);
        }
        152 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008678bb); };
            c.w64(6, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008678bf); };
            c.w64(2, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008678c3); };
            c.w64(0, t219);
            blk = 1; continue;
        }
        153 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008678d1); }
            return c.call(0x70007febf4, 0x70008678da, 0x70008678d5);
        }
        154 => {
            c.w64(15, c.r64(0));
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008678dd); };
            c.w64(6, t220);
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008678e1); };
            c.w64(1, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008678e8); };
            c.w64(8, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008678ec); };
            c.w64(9, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008678f0); };
            c.w64(12, t224);
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008678f4); };
            c.w64(11, t225);
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008678fb); };
            c.w64(3, t226);
            blk = 16; continue;
        }
        155 => {
            return RegfileExit::fault(0x7000867907);
        }
        156 => {
            return RegfileExit::fault(0x7000867908);
        }
        157 => {
            return RegfileExit::fault(0x7000867909);
        }
        158 => {
            return RegfileExit::fault(0x700086790a);
        }
        159 => {
            return RegfileExit::fault(0x700086790b);
        }
        160 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700086790c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x7000867910); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000867914); }
            return c.call(0x70007febf4, 0x700086791d, 0x7000867918);
        }
        161 => {
            c.w64(8, c.r64(0));
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867920); };
            c.w64(6, t227);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867926); };
            c.w64(1, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086792d); };
            c.w64(9, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000867931); };
            c.w64(12, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000867935); };
            c.w64(15, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000867939); };
            c.w64(2, t232);
            blk = 25; continue;
        }
        162 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000867942); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x7000867944); }
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x7000867948); };
            c.w64(3, t233);
            c.w64(6, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700086795f, 0x700086795a);
        }
        163 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086795f); };
            c.w64(6, t234);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867965); };
            c.w64(1, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086796c); };
            c.w64(9, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000867970); };
            c.w64(8, t237);
            blk = 35; continue;
        }
        164 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000867979); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700086797d); }
            return c.call(0x70007febf4, 0x7000867986, 0x7000867981);
        }
        165 => {
            c.w64(12, c.r64(0));
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867989); };
            c.w64(6, t238);
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086798d); };
            c.w64(1, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867994); };
            c.w64(8, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000867998); };
            c.w64(9, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086799c); };
            c.w64(2, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008679a0); };
            c.w64(11, t243);
            blk = 40; continue;
        }
        166 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008679a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008679ad); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008679b9, 0x70008679b4);
        }
        167 => {
            c.w64(8, c.r64(0));
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008679be); };
            c.w64(12, t244);
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008679c2); };
            c.w64(1, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008679c9); };
            c.w64(0, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008679cd); };
            c.w64(9, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008679d1); };
            c.w64(7, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008679d5); };
            c.w64(2, t249);
            blk = 68; continue;
        }
        168 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008679e3); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70008679e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008679ec); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008679f8, 0x70008679f3);
        }
        169 => {
            c.w64(11, c.r64(0));
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008679fb); };
            c.w64(6, t250);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000867a01); };
            c.w64(7, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000867a05); };
            c.w64(9, t252);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000867a09); }
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000867a0e); };
            c.w64(8, t253);
            blk = 114; continue;
        }
        170 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000867a1c); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x7000867a20); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000867a2d, 0x7000867a28);
        }
        171 => {
            c.w64(8, c.r64(0));
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867a30); };
            c.w64(6, t254);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000867a36); };
            c.w64(15, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000867a3a); };
            c.w64(7, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000867a3e); };
            c.w64(11, t257);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000867a42); }
            blk = 123; continue;
        }
        172 => {
            return RegfileExit::fault(0x7000867a4c);
        }
        173 => {
            return RegfileExit::fault(0x7000867a4d);
        }
        174 => {
            return RegfileExit::fault(0x7000867a4e);
        }
        175 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867a4f); };
            c.w64(6, t258);
            c.w64(0, c.r64(12));
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x7000867a56); };
            c.w64(3, t259);
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x7000867a65, 0x7000867a60);
        }
        176 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000867a65); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 296; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t260) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867a71); };
            c.w32(1, t260);
            let r261 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r261);
            let Some(t262) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000867a77); };
            c.logic(t262 & 0x2u64, 8);
            if c.cc_e() { blk = 296; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000867a81); };
            c.w64(3, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867a88); };
            c.w64(0, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867a8c); };
            c.w64(6, t265);
            return c.call(0x7000870914, 0x7000867a95, 0x7000867a90);
        }
        179 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 294; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t266) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867a9d); };
            c.w32(1, t266);
            let r267 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r267);
            let Some(t268) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000867aa3); };
            c.logic(t268 & 0x2u64, 8);
            if c.cc_e() { blk = 294; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867aad); };
            if !c.push64(t269) { return RegfileExit::fault(0x7000867aad); }
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867ab0); };
            c.w64(6, t270);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x7000867ac1, 0x7000867abc);
        }
        182 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000867ac1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 292; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t271) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867acd); };
            c.w32(1, t271);
            let r272 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r272);
            let Some(t273) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867ad3); };
            c.cmp(t273, 0xbau64, 16);
            if c.cc_b() { blk = 292; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000867adf); };
            c.w64(3, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867ae6); };
            c.w64(6, t275);
            return c.call(0x7000870914, 0x7000867aef, 0x7000867aea);
        }
        185 => {
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000867aef); };
            c.w64(1, t276);
            let Some(t277) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867af3); };
            c.w32(8, t277);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000867afd); }
            c.cmp(c.r64(8), 0x68u64, 64);
            if c.cc_a() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            return RegfileExit::fault(0x7000867b07);
        }
        187 => {
            let Some(t278) = c.ld32(c.r64(1).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x7000867b08); };
            c.w32(1, t278);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x7000867b0e); }
            blk = 188; continue;
        }
        188 => {
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000867b12); };
            if !c.push64(t279) { return RegfileExit::fault(0x7000867b12); }
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867b15); };
            c.w64(6, t280);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x70007f21d4, 0x7000867b26, 0x7000867b21);
        }
        189 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000867b26); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 290; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t281) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867b32); };
            c.w32(1, t281);
            let r282 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r282);
            let Some(t283) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000867b38); };
            c.cmp(c.r32(1), t283, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t284) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867b41); };
            c.cmp(t284, 0xbau64, 16);
            if c.cc_b() { blk = 290; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x7000867b4d); };
            c.w64(3, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867b54); };
            c.w64(6, t286);
            return c.call(0x7000870914, 0x7000867b5d, 0x7000867b58);
        }
        193 => {
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000867b5d); };
            c.cmp(t287, c.r32(0), 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 194; continue;
        }
        194 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 205; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t288) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000867b70); };
            c.cmp(t288, c.r32(0), 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t289) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867b79); };
            c.w32(1, t289);
            let r290 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r290);
            let Some(t291) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000867b7f); };
            c.logic(t291 & 0x10u64, 8);
            if c.cc_ne() { blk = 203; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t292) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000867b85); };
            c.cmp(t292, c.r32(1), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t293) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867b8e); };
            c.cmp(t293, 0x41u64, 16);
            if c.cc_ne() { blk = 273; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t294) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867b99); };
            c.logic(t294 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 273; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867ba6); };
            c.w64(0, t295);
            blk = 219; continue;
        }
        201 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000867baf); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 273; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867bc2); };
            c.w64(0, t296);
            blk = 219; continue;
        }
        203 => {
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867bcb); };
            c.w64(0, t297);
            blk = 219; continue;
        }
        204 => {
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867bd4); };
            c.w64(0, t298);
            blk = 219; continue;
        }
        205 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 273; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867be6); };
            c.w64(0, t299);
            blk = 219; continue;
        }
        207 => {
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867bef); };
            c.w64(0, t300);
            blk = 219; continue;
        }
        208 => {
            let Some(t301) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000867bf8); };
            c.w32(1, t301);
            let r302 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r302);
            let Some(t303) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000867bfe); };
            c.cmp(t303, c.r32(1), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w32(8, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 218; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t304) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000867c0f); };
            c.cmp(t304, c.r32(1), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t305) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867c18); };
            c.w32(8, t305);
            let r306 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r306);
            let Some(t307) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000867c1f); };
            c.logic(t307 & 0x10u64, 8);
            if c.cc_ne() { blk = 219; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t308) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000867c26); };
            c.cmp(t308, c.r32(8), 32);
            if c.cc_e() { blk = 216; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t309) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867c2f); };
            c.cmp(t309, 0x41u64, 16);
            if c.cc_ne() { blk = 273; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t310) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867c3b); };
            c.logic(t310 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 215; continue;
        }
        215 => {
            blk = 273; continue;
        }
        216 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000867c49); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 273; continue; }
            blk = 217; continue;
        }
        217 => {
            blk = 219; continue;
        }
        218 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 273; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000867c68); };
            c.w64(1, t311);
            let Some(t312) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867c6c); };
            c.cmp(t312, c.r32(1), 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x7000867c71); };
            c.w64(3, t313);
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867c78); };
            c.w64(6, t314);
            return c.call(0x7000870914, 0x7000867c81, 0x7000867c7c);
        }
        221 => {
            blk = 223; continue;
        }
        222 => {
            let Some(t315) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000867c83); };
            c.w32(0, t315);
            let r316 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r316);
            blk = 223; continue;
        }
        223 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000867c89); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 265; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t317) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867c95); };
            c.w32(1, t317);
            let r318 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r318);
            let Some(t319) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867c9b); };
            c.cmp(t319, 0x834u64, 16);
            if c.cc_ne() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w32(9, 0x1u64);
            blk = 227; continue;
        }
        226 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 227; continue;
        }
        227 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t320) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000867cbb); };
            c.w32(9, t320);
            let r321 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r321);
            let r322 = c.r32(9) & 0x3fu64;
            c.w32(9, r322);
            c.logic(r322, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_a() { blk = 266; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867cd2); };
            c.w64(6, t323);
            let Some(t324) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867cd6); };
            c.w32(9, t324);
            let r325 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r325);
            let Some(t326) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000867cdd); };
            c.w32(9, t326);
            let r327 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r327);
            let Some(t328) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867ce4); };
            c.w32(11, t328);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_be() { blk = 274; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t329) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000867cf8); };
            c.w32(9, t329);
            let Some(t330) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000867cff); };
            c.cmp(t330, c.r32(9), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t331) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000867d09); };
            c.w64(1, t331);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t332) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000867d12); };
            c.cmp(t332, c.r32(3), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t333) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867d1b); };
            c.w32(1, t333);
            let r334 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r334);
            let Some(t335) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867d21); };
            c.w32(8, t335);
            let r336 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r336);
            let Some(t337) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000867d28); };
            c.w32(9, t337);
            let Some(t338) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867d2c); };
            c.w32(11, t338);
            let r339 = c.sub(c.r32(11), 0x77u64, 32);
            c.w32(11, r339);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000867d35); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000867d39); }
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_a() { blk = 243; continue; }
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_g() { blk = 235; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t340) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867d4d); };
            c.w64(8, t340);
            blk = 238; continue;
        }
        235 => {
            let Some(t341) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867d56); };
            c.w32(8, t341);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 275; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t342) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867d6a); };
            c.w32(8, t342);
            let r343 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r343);
            let Some(t344) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000867d71); };
            c.cmp(t344, c.r32(8), 32);
            if c.cc_ne() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867d7a); };
            c.w64(8, t345);
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_g() { blk = 240; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867d87); };
            c.w64(1, t346);
            blk = 264; continue;
        }
        240 => {
            let Some(t347) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867d93); };
            c.w32(9, t347);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 276; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t348) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000867da7); };
            c.w32(1, t348);
            let r349 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r349);
            let Some(t350) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000867dad); };
            c.cmp(t350, c.r32(1), 32);
            if c.cc_ne() { blk = 264; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t351) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867dba); };
            c.w64(1, t351);
            blk = 264; continue;
        }
        243 => {
            let Some(t352) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000867dc6); };
            c.cmp(t352, c.r32(8), 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 244; continue;
        }
        244 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_g() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867dd9); };
            c.w64(8, t353);
            blk = 254; continue;
        }
        246 => {
            let Some(t354) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867de5); };
            c.w32(8, t354);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 277; continue; }
            blk = 247; continue;
        }
        247 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x7)) { return RegfileExit::fault(0x7000867df9); }
            let Some(t355) = c.ld64(0x70008676e1u64) else { return RegfileExit::fault(0x7000867dfe); };
            c.w64(8, t355);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x7000867e05); }
            let Some(t356) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867e0a); };
            c.cmp(t356, c.r64(8), 64);
            if c.cc_e() { blk = 253; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000867e10); };
            c.w64(8, t357);
            let Some(t358) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000867e17); };
            c.w64(11, t358);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t359) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000867e1e); };
            c.w64(15, t359);
            let Some(t360) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000867e25); };
            c.cmp(t360, c.r64(12), 64);
            if c.cc_a() { blk = 251; continue; }
            blk = 249; continue;
        }
        249 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000867e34, 0x7000867e2f);
        }
        250 => {
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000867e34); };
            c.w64(9, t361);
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867e38); };
            c.w64(6, t362);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000867e3e); };
            c.w64(1, t363);
            blk = 252; continue;
        }
        251 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000867e44); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 252; continue;
        }
        252 => {
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000867e4e); };
            c.w64(8, t364);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000867e55); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000867e59); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000867e5e); }
            c.w64(8, c.r64(0));
            blk = 254; continue;
        }
        253 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867e68); };
            c.w64(8, t365);
            blk = 254; continue;
        }
        254 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000867e6f); }
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_g() { blk = 256; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867e79); };
            c.w64(1, t366);
            blk = 264; continue;
        }
        256 => {
            let Some(t367) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000867e85); };
            c.w32(9, t367);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 278; continue; }
            blk = 257; continue;
        }
        257 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0xf)) { return RegfileExit::fault(0x7000867e99); }
            let Some(t368) = c.ld64(0x70008676e1u64) else { return RegfileExit::fault(0x7000867e9e); };
            c.w64(9, t368);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x7000867ea5); }
            let Some(t369) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000867eaa); };
            c.cmp(t369, c.r64(9), 64);
            if c.cc_e() { blk = 263; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000867eb0); };
            c.w64(1, t370);
            let Some(t371) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000867eb7); };
            c.w64(9, t371);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t372) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000867ebe); };
            c.w64(12, t372);
            let Some(t373) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000867ec5); };
            c.cmp(t373, c.r64(11), 64);
            if c.cc_a() { blk = 261; continue; }
            blk = 259; continue;
        }
        259 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000867ed5, 0x7000867ed0);
        }
        260 => {
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000867ed5); };
            c.w64(8, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867ed9); };
            c.w64(6, t375);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 262; continue;
        }
        261 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x7000867ee1); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 262; continue;
        }
        262 => {
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000867eeb); };
            c.w64(1, t376);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000867ef2); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000867ef5); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000867efa); }
            c.w64(1, c.r64(0));
            blk = 264; continue;
        }
        263 => {
            let Some(t377) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867f04); };
            c.w64(1, t377);
            blk = 264; continue;
        }
        264 => {
            c.w64(0, c.r64(1));
            blk = 271; continue;
        }
        265 => {
            c.w32(8, 0x1u64);
            blk = 266; continue;
        }
        266 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 281; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t378) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000867f1c); };
            c.w32(1, t378);
            let r379 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r379);
            let Some(t380) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000867f22); };
            c.w32(1, t380);
            let r381 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r381);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 281; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867f34); };
            c.w64(6, t382);
            c.w32(3, 0);
            c.logic(0, 32);
            return c.call(0x7000870914, 0x7000867f3f, 0x7000867f3a);
        }
        269 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000867f3f); }
            c.w32(3, 0x2u64);
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000867f48); };
            c.w64(0, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867f4c); };
            c.w64(6, t384);
            return c.call(0x7000870914, 0x7000867f55, 0x7000867f50);
        }
        270 => {
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867f55); };
            c.w64(6, t385);
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000867f59); };
            c.w64(8, t386);
            blk = 271; continue;
        }
        271 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000867f5d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000867f5e); }
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000867f60); };
            if !c.push64(t387) { return RegfileExit::fault(0x7000867f60); }
            c.w32(0, 0x2u64);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000867f68); };
            c.w64(7, t388);
            return c.call(0x70007f2274, 0x7000867f71, 0x7000867f6c);
        }
        272 => {
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000867f71); };
            c.w64(0, t389);
            blk = 188; continue;
        }
        273 => {
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000867f7a); };
            c.w64(12, t390);
            blk = 150; continue;
        }
        274 => {
            return RegfileExit::fault(0x7000867f83);
        }
        275 => {
            return RegfileExit::fault(0x7000867f84);
        }
        276 => {
            return RegfileExit::fault(0x7000867f85);
        }
        277 => {
            return RegfileExit::fault(0x7000867f86);
        }
        278 => {
            return RegfileExit::fault(0x7000867f87);
        }
        279 => {
            return RegfileExit::fault(0x7000867f88);
        }
        280 => {
            blk = 283; continue;
        }
        281 => {
            c.w32(1, 0x92u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000867f90); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000867f91); }
            let Some(t391) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000867f92); };
            c.w64(3, t391);
            c.w32(0, 0x2u64);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867f9e); };
            c.w64(6, t392);
            return c.call(0x700086ed74, 0x7000867fa7, 0x7000867fa2);
        }
        282 => {
            blk = 289; continue;
        }
        283 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000867fa9); }
            let Some(t393) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x7000867fad); };
            c.w64(3, t393);
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000867fb4); };
            c.w64(0, t394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867fb8); };
            c.w64(6, t395);
            return c.call(0x7000870914, 0x7000867fc1, 0x7000867fbc);
        }
        284 => {
            let Some(t396) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000867fc1); };
            c.cmp(t396, c.r32(0), 32);
            if c.cc_e() { blk = 287; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t397) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000867fca); };
            c.cmp(t397, c.r32(0), 32);
            if c.cc_e() { blk = 287; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000867fd3); };
            if !c.push64(t398) { return RegfileExit::fault(0x7000867fd3); }
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867fd6); };
            c.w64(6, t399);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x7000867fe7, 0x7000867fe2);
        }
        287 => {
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000867fe7); };
            if !c.push64(t400) { return RegfileExit::fault(0x7000867fe7); }
            let Some(t401) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000867fea); };
            c.w64(3, t401);
            c.w32(0, 0x1u64);
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000867ff6); };
            c.w64(6, t402);
            return c.call(0x700086ed74, 0x7000867fff, 0x7000867ffa);
        }
        288 => {
            return RegfileExit::fault(0x7000867fff);
        }
        289 => {
            return RegfileExit::fault(0x7000868000);
        }
        290 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000868001); }
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x7000868002); };
            c.w64(3, t403);
            c.w32(0, 0x1u64);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086800e); };
            c.w64(6, t404);
            return c.call(0x700086ed74, 0x7000868017, 0x7000868012);
        }
        291 => {
            return RegfileExit::fault(0x7000868017);
        }
        292 => {
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x7000868018); };
            c.w64(3, t405);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868021); };
            c.w64(6, t406);
            return c.call(0x700086ed74, 0x700086802a, 0x7000868025);
        }
        293 => {
            return RegfileExit::fault(0x700086802a);
        }
        294 => {
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086802b); };
            if !c.push64(t407) { return RegfileExit::fault(0x700086802b); }
            let Some(t408) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x700086802e); };
            c.w64(3, t408);
            c.w32(0, 0x1u64);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086803a); };
            c.w64(6, t409);
            return c.call(0x700086ed74, 0x7000868043, 0x700086803e);
        }
        295 => {
            return RegfileExit::fault(0x7000868043);
        }
        296 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000868049); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086804a); }
            let Some(t410) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x700086804b); };
            c.w64(1, t410);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000868052); }
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000868053); };
            if !c.push64(t411) { return RegfileExit::fault(0x7000868053); }
            let Some(t412) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000868056); };
            c.w64(3, t412);
            c.w32(0, 0x4u64);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868062); };
            c.w64(6, t413);
            return c.call(0x700086ed74, 0x700086806b, 0x7000868066);
        }
        297 => {
            return RegfileExit::fault(0x700086806b);
        }
        298 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000868071); }
            return c.call(0x70007febf4, 0x700086807b, 0x7000868076);
        }
        299 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700086807e); }
            blk = 138; continue;
        }
        300 => {
            let Some(t414) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000868088); };
            c.w64(1, t414);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x700086809c, 0x7000868097);
        }
        301 => {
            return RegfileExit::fault(0x700086809c);
        }
        302 => {
            c.w32(1, 0x52u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008680a2); }
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0xb60)) else { return RegfileExit::fault(0x70008680a3); };
            c.w64(1, t415);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008680aa); }
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008680ab); };
            c.w64(3, t416);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008680bc, 0x70008680b7);
        }
        303 => {
            return RegfileExit::fault(0x70008680bc);
        }
        304 => {
            c.w32(8, c.r32(8));
            let r417 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r417);
            c.w64(11, 0xfffffffffffc0000u64);
            let r418 = c.r64(11) & c.r64(8);
            c.w64(11, r418);
            c.logic(r418, 64);
            let Some(t419) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008680cd); };
            c.logic(t419 & 0x2u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w64(15, c.r64(12).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008680dd); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008680de); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008680eb, 0x70008680e6);
        }
        306 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008680eb); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008680ed); }
            blk = 75; continue;
        }
        307 => {
            return RegfileExit::jump(0x70008680f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000868114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086811d); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868123); };
            c.cmp(t2, 0xbau64, 16);
            if c.cc_b() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086812f); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086813a); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868140); };
            c.w32(1, t7);
            c.cmp(c.r16(1), 0xb0u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(1), 0x7bu64, 16);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868155); };
            c.w32(1, t8);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000868168); };
            c.w32(2, t9);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 10; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868173); };
            c.w32(2, t10);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if let Some(sr) = c.shr(c.r64(2), 0xau64, 64) { c.w64(2, sr); }
            let r11 = c.r64(2) & 0xfffffu64;
            c.w64(2, r11);
            c.logic(r11, 64);
            blk = 10; continue;
        }
        9 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000868195); };
            c.w32(1, t12);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r13 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r13);
            let r14 = c.r64(2) & c.r64(1);
            c.w64(2, r14);
            c.logic(r14, 64);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        12 => {
            let r15 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r15);
            let r16 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r16);
            c.w64(7, c.r64(1));
            let r17 = c.r64(7) & c.r64(2);
            c.w64(7, r17);
            c.logic(r17, 64);
            c.w64(2, c.r64(7));
            blk = 13; continue;
        }
        13 => {
            c.w64(7, c.r64(2).wrapping_mul(2).wrapping_add(0x3));
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008681c1); };
            c.w32(8, t18);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008681c6); };
            c.cmp(t19, c.r32(8), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(3, c.r64(7).wrapping_add(0x1));
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008681db);
        }
        16 => {
            c.w64(0, 0xfffffffffffffffeu64);
            return c.ret(0x0, 0x70008681e3);
        }
        17 => {
            return RegfileExit::fault(0x70008681e4);
        }
        18 => {
            return RegfileExit::jump(0x70008681e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008681f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008681f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008681f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008681f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008681fa); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008681ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000868205); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868212); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868218); };
            c.cmp(t5, 0x831u64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000868220); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000868226); }
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086822a); };
            c.w64(3, t8);
            return c.call(0x7000868114, 0x7000868233, 0x700086822e);
        }
        4 => {
            c.w64(2, 0xfffffffffffffffeu64);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086823e); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868242); };
            c.w32(1, t10);
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_ae() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086824c); };
            c.w32(0, t11);
            let r12 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r12);
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000868253); };
            c.w64(1, t13);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086825a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x10, 0x7000868261);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000868264); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086826b); }
            return c.ret(0x0, 0x700086826d);
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086826e); };
            c.w64(0, t14);
            blk = 7; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000868277); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086827b); };
            c.w64(3, t15);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000868289, 0x7000868284);
        }
        12 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868289); };
            c.w64(6, t16);
            blk = 1; continue;
        }
        13 => {
            return RegfileExit::fault(0x7000868292);
        }
        14 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000868298); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000868299); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0xd7)) else { return RegfileExit::fault(0x70008682a0); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008682a9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008682aa); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008682ab); };
            c.w64(3, t20);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008682bc, 0x70008682b7);
        }
        15 => {
            return RegfileExit::fault(0x70008682bc);
        }
        16 => {
            return RegfileExit::jump(0x70008682c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008682d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008682d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008682d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008682d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008682da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008682df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008682e5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008682ee); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008682f4); };
            c.cmp(t5, 0x831u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008682fc); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000868302); };
            c.w64(3, t8);
            return c.call(0x7000868114, 0x700086830b, 0x7000868306);
        }
        4 => {
            c.w64(2, 0xfffffffffffffffeu64);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000868316); };
            c.w64(0, t9);
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086831d); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000868324); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x700086832b);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086832e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000868335); }
            return c.ret(0x0, 0x7000868337);
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000868338); };
            c.w64(0, t11);
            blk = 6; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000868341); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000868345); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000868353, 0x700086834e);
        }
        11 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868353); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        12 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086835e); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086835f); };
            c.w64(1, t14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0xdb)) else { return RegfileExit::fault(0x7000868366); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086836f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000868370); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000868371); };
            c.w64(3, t17);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000868382, 0x700086837d);
        }
        13 => {
            return RegfileExit::fault(0x7000868382);
        }
        14 => {
            return RegfileExit::jump(0x7000868384);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000868394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000868394); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000868398); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000868399); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086839a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086839f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008683a5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008683b1); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008683b7); };
            c.cmp(t5, 0x831u64, 16);
            if c.cc_ne() { blk = 13; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008683bf); };
            c.w64(3, t6);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008683c8); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008683ce); };
            c.cmp(t9, 0xbau64, 16);
            if c.cc_b() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008683d6); };
            c.w64(1, t10);
            return c.call(0x70008694f4, 0x70008683df, 0x70008683da);
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008683df); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008683e6); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x18, 0x70008683ed);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008683f0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008683f7); }
            return c.ret(0x0, 0x70008683f9);
        }
        9 => {
            c.w32(2, 0x1a6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008683ff); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000868400); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000868401); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000868412, 0x700086840d);
        }
        10 => {
            return RegfileExit::fault(0x7000868412);
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000868413); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000868417); };
            c.w64(3, t13);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000868425, 0x7000868420);
        }
        12 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868425); };
            c.w64(6, t14);
            blk = 1; continue;
        }
        13 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000868433); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000868434); };
            c.w64(2, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0xdf)) else { return RegfileExit::fault(0x700086843b); };
            c.w32(2, t16);
            let r17 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r17);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000868444); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000868445); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000868446); };
            c.w64(3, t18);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000868457, 0x7000868452);
        }
        14 => {
            return RegfileExit::fault(0x7000868457);
        }
        15 => {
            return RegfileExit::jump(0x700086845c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000868474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000868474); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000868478); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000868479); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086847a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086847f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000868485); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086848d); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868493); };
            c.cmp(t5, 0x831u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086849b); };
            c.w64(3, t6);
            return c.call(0x7000869334, 0x70008684a4, 0x700086849f);
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008684a4); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008684ab); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x10, 0x70008684b2);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008684b5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008684bc); }
            return c.ret(0x0, 0x70008684be);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008684bf); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008684c3); };
            c.w64(3, t8);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008684d1, 0x70008684cc);
        }
        8 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008684d1); };
            c.w64(6, t9);
            blk = 1; continue;
        }
        9 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008684dc); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008684dd); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0xe3)) else { return RegfileExit::fault(0x70008684e4); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008684ed); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008684ee); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008684ef); };
            c.w64(3, t13);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000868500, 0x70008684fb);
        }
        10 => {
            return RegfileExit::fault(0x7000868500);
        }
        11 => {
            return RegfileExit::jump(0x7000868504);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000868514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000868514); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000868518); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000868519); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086851a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700086851f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000868523); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 98; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(5));
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000868539); };
            c.w64(1, t2);
            c.w64(8, c.r64(1));
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868542); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868549); };
            c.w64(1, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000868550); }
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868554); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868561); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000868568); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086856f); };
            c.w32(11, t10);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000868579); }
            c.cmp(c.r64(11), 0x75u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000868583);
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0x1db)) else { return RegfileExit::fault(0x7000868584); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868597); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            let Some(t15) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086859e); };
            c.logic(t15 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008685aa); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008685bb); };
            c.w32(15, t18);
            let r19 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r19);
            let Some(t20) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008685c3); };
            c.cmp(t20, 0xacu64, 16);
            if c.cc_ne() { blk = 36; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008685d0); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008685db); };
            c.logic(t22 & 0x200000u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008685e6); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008685ed); };
            c.w64(15, t24);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008685f6); };
            c.w64(11, t25);
            blk = 23; continue;
        }
        14 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000868602); };
            c.w64(11, t26);
            let Some(t27) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000868609); };
            c.w64(15, t27);
            c.w64(0, c.r64(15).wrapping_add(0x4c));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000868610); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000868617); };
            c.cmp(t29, c.r64(0), 64);
            if c.cc_be() { blk = 100; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x7000868620); }
            let r30 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r30);
            blk = 16; continue;
        }
        16 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x7000868627); };
            c.w64(0, t31);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700086862e); }
            c.w32(0, 0x22u64);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000868637); }
            let Some(t32) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086863b); };
            c.w32(0, t32);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 102; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000868651); }
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868655); };
            c.w32(2, t33);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 103; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x7000868669); }
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086866d); };
            c.w32(2, t34);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 104; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0x8u64);
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x7000868686); }
            let Some(t35) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086868a); };
            c.w32(2, t35);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_be() { blk = 105; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, 0x2u64);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008686a3); }
            let Some(t36) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008686a7); };
            c.w32(2, t36);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x70008686bb); }
            c.w64(2, c.r64(15).wrapping_add(0x4b));
            c.w64(7, c.r64(15).wrapping_add(0x1b));
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st32(c.r64(7), c.r32(1)) { return RegfileExit::fault(0x70008686cc); }
            let r37 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r37);
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ne() { blk = 22; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t38) = c.ld8(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008686d7); };
            c.w32(2, t38);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let Some(t39) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008686e1); };
            c.w64(7, t39);
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t40) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008686e8); };
            c.cmp(t40, c.r64(8), 64);
            if c.cc_be() { blk = 107; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x70008686f1); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008686fc); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000868700); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000868704); };
            c.w64(15, t41);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086870b); }
            let Some(t42) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086870f); };
            c.w32(15, t42);
            c.w64(3, c.r64(15));
            let r43 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r43);
            c.logic(r43, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            let r44 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r44);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000868729); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 27; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 35; continue;
        }
        29 => {
            let r45 = c.sub(c.r32(15), 0x20000000u64, 32);
            c.w32(15, r45);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700086873c); }
            let Some(t46) = c.ld8(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000868741); };
            c.w32(15, t46);
            c.w64(15, c.r64(15).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x7000868754); };
            c.w64(2, t47);
            blk = 31; continue;
        }
        31 => {
            let r48 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r48);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700086875f); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 31; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let r49 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r49);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000868772); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            let r50 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r50);
            if c.cc_e() { blk = 109; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(0, c.r64(8));
            blk = 37; continue;
        }
        36 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086878d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086878f); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2408)) else { return RegfileExit::fault(0x7000868790); };
            c.w64(3, t51);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008687a1, 0x700086879c);
        }
        37 => {
            c.w64(12, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008687a4); };
            c.w64(6, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008687a8); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008687af); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008687b3); };
            c.w64(9, t55);
            blk = 42; continue;
        }
        38 => {
            let Some(t56) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008687b9); };
            c.w32(11, t56);
            let r57 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008687c0); };
            c.w64(12, t58);
            let Some(t59) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008687c7); };
            c.w64(15, t59);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008687cb); };
            c.w32(0, t60);
            c.w64(2, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008687dc); };
            c.w64(3, t61);
            let Some(t62) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008687e3); };
            c.cmp(t62, c.r64(0), 64);
            if c.cc_be() { blk = 111; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008687ec); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 40; continue;
        }
        40 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008687f4); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008687f9); };
            c.w64(11, t63);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000868800); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000868805); }
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r64 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r64);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x7000868818); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 41; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000868822); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x7000868830); };
            c.w64(11, t65);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086883a); };
            c.cmp(t66, c.r32(15), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(0, c.r64(8));
            let Some(t67) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868847); };
            c.w32(8, t67);
            let r68 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r68);
            let Some(t69) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086884e); };
            c.cmp(t69, 0x834u64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(3, 0x1u64);
            blk = 47; continue;
        }
        46 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        47 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t70) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000868868); };
            c.w32(3, t70);
            let r71 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r71);
            let r72 = c.r32(3) & 0x3fu64;
            c.w32(3, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_a() { blk = 56; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t73) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868878); };
            c.w32(3, t73);
            let r74 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r74);
            let Some(t75) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086887e); };
            c.w32(3, t75);
            let r76 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r76);
            let Some(t77) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868884); };
            c.w32(2, t77);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4cu64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x7000868893);
        }
        51 => {
            let Some(t78) = c.ld32(c.r64(3).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000868894); };
            c.w32(3, t78);
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086889a); };
            c.cmp(t79, c.r32(3), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008688a0); };
            c.w64(8, t80);
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008688a7); };
            c.cmp(t81, c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(11, 0x1u64);
            blk = 56; continue;
        }
        54 => {
            c.w64(0, c.r64(8));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(8));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        56 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008688c6); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(8, c.r64(15));
            blk = 59; continue;
        }
        58 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008688d5); };
            c.w32(8, t82);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 59; continue;
        }
        59 => {
            c.w64(3, c.r64(8));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(8, c.r64(3).wrapping_add(c.r64(8)).wrapping_sub(0x1));
            c.w64(3, c.r64(8));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r83 = c.r64(3) | c.r64(8);
            c.w64(3, r83);
            c.logic(r83, 64);
            c.w64(8, c.r64(3));
            if let Some(sr) = c.shr(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let r84 = c.r64(8) | c.r64(3);
            c.w64(8, r84);
            c.logic(r84, 64);
            c.w64(3, c.r64(8));
            if let Some(sr) = c.shr(c.r64(3), 0x4u64, 64) { c.w64(3, sr); }
            let r85 = c.r64(3) | c.r64(8);
            c.w64(3, r85);
            c.logic(r85, 64);
            c.w64(8, c.r64(3));
            if let Some(sr) = c.shr(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            let r86 = c.r64(8) | c.r64(3);
            c.w64(8, r86);
            c.logic(r86, 64);
            c.w64(3, c.r64(8));
            if let Some(sr) = c.shr(c.r64(3), 0x10u64, 64) { c.w64(3, sr); }
            let r87 = c.r64(3) | c.r64(8);
            c.w64(3, r87);
            c.logic(r87, 64);
            c.w64(8, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_ge() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(8, 0x4u64);
            blk = 61; continue;
        }
        61 => {
            c.w64(3, c.r64(8).wrapping_mul(2).wrapping_add(0x3));
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 185; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000868940); };
            c.w64(2, t88);
            let Some(t89) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000868947); };
            c.w64(2, t89);
            c.w64(7, c.r64(3).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(7).wrapping_add(c.r64(2)));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000868956); };
            c.w64(11, t90);
            let Some(t91) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086895d); };
            c.cmp(t91, c.r64(6), 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 113; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000868973); };
            c.w64(11, t92);
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x700086897a); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            blk = 65; continue;
        }
        65 => {
            c.w32(2, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000868984); };
            c.w64(6, t93);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700086898b); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700086898f); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2d8)) else { return RegfileExit::fault(0x7000868993); };
            c.w64(2, t94);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700086899a); }
            let Some(t95) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086899e); };
            c.w32(2, t95);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 115; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008689b2); }
            let Some(t96) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008689b6); };
            c.w32(2, t96);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 116; continue; }
            blk = 67; continue;
        }
        67 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008689ca); }
            let Some(t97) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008689ce); };
            c.w32(2, t97);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 117; continue; }
            blk = 68; continue;
        }
        68 => {
            let r98 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r98);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008689e5); }
            c.w64(8, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x13u64, 64);
            if c.cc_e() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let r99 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r99);
            if !c.st32(c.r64(8).wrapping_add(c.r64(11)), c.r32(1)) { return RegfileExit::fault(0x70008689fb); }
            c.cmp(c.r64(8), 0x13u64, 64);
            if c.cc_ne() { blk = 69; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000868a05); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r100 = c.r64(8) & c.r64(12);
            c.w64(8, r100);
            c.logic(r100, 64);
            let Some(t101) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000868a1a); };
            c.logic(t101 & 0x4u64, 8);
            if c.cc_ne() { blk = 189; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t102) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000868a25); };
            c.cmp(t102, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868a34); };
            c.cmp(t103, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let r104 = c.r32(8) | c.r32(11);
            c.w32(8, r104);
            c.logic(r104, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(8, c.r64(9));
            let Some(t105) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868a4f); };
            c.w32(9, t105);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x5cu64, 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            return RegfileExit::fault(0x7000868a5f);
        }
        75 => {
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868a60); };
            c.w32(11, t106);
            let r107 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r107);
            let Some(t108) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000868a68); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            let Some(t110) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868a6f); };
            c.w32(11, t110);
            let r111 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r111);
            let Some(t112) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000868a76); };
            c.w32(3, t112);
            let r113 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r113);
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(0x177)) else { return RegfileExit::fault(0x7000868a7d); };
            c.cmp(t114, c.r32(11), 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t115) = c.ld32(c.r64(3).wrapping_add(0x37)) else { return RegfileExit::fault(0x7000868a8a); };
            c.w32(11, t115);
            c.w32(11, c.r32(11));
            let r116 = c.r32(11) & 0x8u64;
            c.w32(11, r116);
            c.logic(r116, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t117) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000868a9f); };
            c.cmp(t117, 0x0u64, 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t118) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868aa9); };
            c.w32(11, t118);
            let r119 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r119);
            let Some(t120) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868ab0); };
            c.w32(3, t120);
            let r121 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r121);
            let Some(t122) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000868ab6); };
            c.w32(11, t122);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000868abb); }
            c.cmp(c.r64(9), 0xfau64, 64);
            if c.cc_a() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            return RegfileExit::fault(0x7000868ac8);
        }
        80 => {
            let Some(t123) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000868ac9); };
            c.w32(9, t123);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t124) = c.ld32(c.r64(8).wrapping_add(0x3ef)) else { return RegfileExit::fault(0x7000868ad0); };
            c.w32(7, t124);
            let r125 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r125);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000868add); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000868ae1); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 97; continue; }
            blk = 81; continue;
        }
        81 => {
            let r126 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r126);
            let r127 = c.r32(11) & 0x3fu64;
            c.w32(11, r127);
            c.logic(r127, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_be() { blk = 91; continue; }
            blk = 82; continue;
        }
        82 => {
            blk = 84; continue;
        }
        83 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000868b03); };
            c.w64(3, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868b07); };
            c.w64(9, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868b0b); };
            c.w64(7, t130);
            blk = 84; continue;
        }
        84 => {
            c.w64(8, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x7000868b19); }
            let Some(t131) = c.ld64(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868b1d); };
            c.cmp(t131, c.r64(8), 64);
            if c.cc_e() { blk = 184; continue; }
            blk = 85; continue;
        }
        85 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(15).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000868b28); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000868b2f); };
            c.w64(11, t132);
            let Some(t133) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000868b36); };
            c.w64(2, t133);
            c.w64(6, c.r64(2).wrapping_add(0xc));
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000868b3d); };
            c.w64(8, t134);
            let Some(t135) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000868b44); };
            c.cmp(t135, c.r64(6), 64);
            if c.cc_be() { blk = 182; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x7000868b4d); }
            c.w64(8, c.r64(2).wrapping_add(0x1));
            blk = 87; continue;
        }
        87 => {
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000868b54); };
            c.w64(11, t136);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000868b5b); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000868b5f); }
            blk = 88; continue;
        }
        88 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000868b65); }
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000868b67); };
            if !c.push64(t137) { return RegfileExit::fault(0x7000868b67); }
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868b6a); };
            c.w64(6, t138);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x7000868b78, 0x7000868b73);
        }
        89 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868b78); };
            c.w64(0, t139);
            c.w64(15, c.r64(0).wrapping_add(0x1));
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868b80); };
            c.cmp(c.r64(15), t140, 64);
            if c.cc_ne() { blk = 83; continue; }
            blk = 90; continue;
        }
        90 => {
            blk = 97; continue;
        }
        91 => {
            c.w64(8, c.r64(3));
            blk = 93; continue;
        }
        92 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000868b91); };
            c.w64(8, t141);
            c.w64(9, c.r64(1));
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868b98); };
            c.w64(7, t142);
            blk = 93; continue;
        }
        93 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868b9c); };
            c.w32(11, t143);
            let r144 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r144);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x7000868ba4); }
            let Some(t145) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000868ba8); };
            c.cmp(t145, c.r32(11), 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868bb1); };
            c.w64(11, t146);
            blk = 95; continue;
        }
        95 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000868bb8); }
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000868bba); };
            if !c.push64(t147) { return RegfileExit::fault(0x7000868bba); }
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868bbd); };
            c.w64(6, t148);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x7000868bcb, 0x7000868bc6);
        }
        96 => {
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868bcb); };
            c.w64(0, t149);
            c.w64(15, c.r64(0).wrapping_add(0x1));
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868bd3); };
            c.w64(1, t150);
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_ne() { blk = 92; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000868bdc); };
            c.w64(0, t151);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000868be3); }
            return c.ret(0x0, 0x7000868be4);
        }
        98 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000868be5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000868be9); }
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000868bed); };
            c.w64(3, t152);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000868bfe, 0x7000868bf9);
        }
        99 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868bfe); };
            c.w64(6, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000868c02); };
            c.w64(2, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868c06); };
            c.w64(0, t155);
            blk = 1; continue;
        }
        100 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000868c14); }
            return c.call(0x70007febf4, 0x7000868c1d, 0x7000868c18);
        }
        101 => {
            c.w64(15, c.r64(0));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868c20); };
            c.w64(6, t156);
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868c24); };
            c.w64(1, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868c2b); };
            c.w64(8, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868c2f); };
            c.w64(9, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868c33); };
            c.w64(12, t160);
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000868c37); };
            c.w64(11, t161);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000868c3e); };
            c.w64(3, t162);
            blk = 16; continue;
        }
        102 => {
            return RegfileExit::fault(0x7000868c4a);
        }
        103 => {
            return RegfileExit::fault(0x7000868c4b);
        }
        104 => {
            return RegfileExit::fault(0x7000868c4c);
        }
        105 => {
            return RegfileExit::fault(0x7000868c4d);
        }
        106 => {
            return RegfileExit::fault(0x7000868c4e);
        }
        107 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000868c4f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000868c53); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000868c57); }
            return c.call(0x70007febf4, 0x7000868c60, 0x7000868c5b);
        }
        108 => {
            c.w64(8, c.r64(0));
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868c63); };
            c.w64(6, t163);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868c69); };
            c.w64(1, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868c70); };
            c.w64(9, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868c74); };
            c.w64(12, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000868c78); };
            c.w64(15, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000868c7c); };
            c.w64(2, t168);
            blk = 25; continue;
        }
        109 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000868c85); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000868c87); }
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x7000868c8b); };
            c.w64(3, t169);
            c.w64(6, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000868ca2, 0x7000868c9d);
        }
        110 => {
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868ca2); };
            c.w64(6, t170);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868ca8); };
            c.w64(1, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868caf); };
            c.w64(9, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000868cb3); };
            c.w64(8, t173);
            blk = 35; continue;
        }
        111 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000868cbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000868cc0); }
            return c.call(0x70007febf4, 0x7000868cc9, 0x7000868cc4);
        }
        112 => {
            c.w64(12, c.r64(0));
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868ccc); };
            c.w64(6, t174);
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868cd0); };
            c.w64(1, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868cd7); };
            c.w64(8, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868cdb); };
            c.w64(9, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000868cdf); };
            c.w64(2, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868ce3); };
            c.w64(11, t179);
            blk = 40; continue;
        }
        113 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000868cec); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000868cf0); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x7000868cfc, 0x7000868cf7);
        }
        114 => {
            c.w64(11, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000868d02); };
            c.w64(12, t180);
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868d06); };
            c.w64(1, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868d0d); };
            c.w64(0, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868d11); };
            c.w64(9, t183);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868d15); };
            c.w64(3, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000868d19); };
            c.w64(8, t185);
            blk = 65; continue;
        }
        115 => {
            return RegfileExit::fault(0x7000868d22);
        }
        116 => {
            return RegfileExit::fault(0x7000868d23);
        }
        117 => {
            return RegfileExit::fault(0x7000868d24);
        }
        118 => {
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x4b8)) else { return RegfileExit::fault(0x7000868d25); };
            c.w64(3, t186);
            c.w64(0, c.r64(12));
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868d2f); };
            c.w64(6, t187);
            return c.call(0x7000870914, 0x7000868d38, 0x7000868d33);
        }
        119 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000868d38); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 180; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t188) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868d44); };
            c.w32(1, t188);
            let r189 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r189);
            let Some(t190) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000868d4a); };
            c.logic(t190 & 0x2u64, 8);
            if c.cc_e() { blk = 180; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000868d54); };
            c.w64(3, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868d5b); };
            c.w64(0, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868d5f); };
            c.w64(6, t193);
            return c.call(0x7000870914, 0x7000868d68, 0x7000868d63);
        }
        122 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t194) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868d70); };
            c.w32(1, t194);
            let r195 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r195);
            let Some(t196) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000868d76); };
            c.logic(t196 & 0x2u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868d80); };
            if !c.push64(t197) { return RegfileExit::fault(0x7000868d80); }
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868d83); };
            c.w64(6, t198);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x7000868d94, 0x7000868d8f);
        }
        125 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000868d94); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 176; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t199) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868da0); };
            c.w32(1, t199);
            let r200 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r200);
            let Some(t201) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868da6); };
            c.cmp(t201, 0xbau64, 16);
            if c.cc_b() { blk = 176; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000868db2); };
            c.w64(3, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868db9); };
            c.w64(6, t203);
            return c.call(0x7000870914, 0x7000868dc2, 0x7000868dbd);
        }
        128 => {
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868dc2); };
            c.w64(8, t204);
            let Some(t205) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868dc6); };
            c.w32(1, t205);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000868dd0); }
            c.cmp(c.r64(1), 0x68u64, 64);
            if c.cc_a() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            return RegfileExit::fault(0x7000868dda);
        }
        130 => {
            let Some(t206) = c.ld32(c.r64(8).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x7000868ddb); };
            c.w32(1, t206);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x7000868de2); }
            blk = 131; continue;
        }
        131 => {
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868de6); };
            if !c.push64(t207) { return RegfileExit::fault(0x7000868de6); }
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868de9); };
            c.w64(6, t208);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x70007f21d4, 0x7000868dfa, 0x7000868df5);
        }
        132 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000868dfa); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 174; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t209) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868e06); };
            c.w32(1, t209);
            let r210 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r210);
            let Some(t211) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868e0c); };
            c.cmp(c.r32(1), t211, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t212) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868e15); };
            c.cmp(t212, 0xbau64, 16);
            if c.cc_b() { blk = 174; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x7000868e21); };
            c.w64(3, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868e28); };
            c.w64(6, t214);
            return c.call(0x7000870914, 0x7000868e31, 0x7000868e2c);
        }
        136 => {
            let Some(t215) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000868e31); };
            c.cmp(t215, c.r32(0), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t216) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000868e44); };
            c.cmp(t216, c.r32(0), 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t217) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868e4d); };
            c.w32(1, t217);
            let r218 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r218);
            let Some(t219) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000868e53); };
            c.logic(t219 & 0x10u64, 8);
            if c.cc_ne() { blk = 146; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t220) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000868e59); };
            c.cmp(t220, c.r32(1), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t221) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868e62); };
            c.cmp(t221, 0x41u64, 16);
            if c.cc_ne() { blk = 97; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t222) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868e6d); };
            c.logic(t222 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868e7a); };
            c.w64(0, t223);
            blk = 162; continue;
        }
        144 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000868e83); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 97; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868e96); };
            c.w64(0, t224);
            blk = 162; continue;
        }
        146 => {
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868e9f); };
            c.w64(0, t225);
            blk = 162; continue;
        }
        147 => {
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868ea8); };
            c.w64(0, t226);
            blk = 162; continue;
        }
        148 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868eba); };
            c.w64(0, t227);
            blk = 162; continue;
        }
        150 => {
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868ec3); };
            c.w64(0, t228);
            blk = 162; continue;
        }
        151 => {
            let Some(t229) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000868ecc); };
            c.w32(1, t229);
            let r230 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000868ed2); };
            c.cmp(t231, c.r32(1), 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w32(8, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 161; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t232) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000868ee3); };
            c.cmp(t232, c.r32(1), 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t233) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868eec); };
            c.w32(8, t233);
            let r234 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r234);
            let Some(t235) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000868ef3); };
            c.logic(t235 & 0x10u64, 8);
            if c.cc_ne() { blk = 162; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t236) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000868efa); };
            c.cmp(t236, c.r32(8), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t237) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000868f03); };
            c.cmp(t237, 0x41u64, 16);
            if c.cc_ne() { blk = 97; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t238) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000868f0f); };
            c.logic(t238 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 158; continue;
        }
        158 => {
            blk = 97; continue;
        }
        159 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000868f1d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 97; continue; }
            blk = 160; continue;
        }
        160 => {
            blk = 162; continue;
        }
        161 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000868f3c); };
            c.w64(1, t239);
            let Some(t240) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000868f40); };
            c.cmp(t240, c.r32(1), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x7000868f45); };
            c.w64(3, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868f4c); };
            c.w64(6, t242);
            return c.call(0x7000870914, 0x7000868f55, 0x7000868f50);
        }
        164 => {
            blk = 166; continue;
        }
        165 => {
            let Some(t243) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000868f57); };
            c.w32(0, t243);
            let r244 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r244);
            blk = 166; continue;
        }
        166 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000868f5d); }
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000868f5e); };
            if !c.push64(t245) { return RegfileExit::fault(0x7000868f5e); }
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000868f61); };
            c.w64(7, t246);
            c.w32(0, 0x1u64);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868f6a); };
            c.w64(6, t247);
            return c.call(0x70007f2274, 0x7000868f73, 0x7000868f6e);
        }
        167 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000868f73); };
            c.w64(0, t248);
            blk = 131; continue;
        }
        168 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000868f7c); }
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x7000868f80); };
            c.w64(3, t249);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868f87); };
            c.w64(0, t250);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868f8b); };
            c.w64(6, t251);
            return c.call(0x7000870914, 0x7000868f94, 0x7000868f8f);
        }
        169 => {
            let Some(t252) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000868f94); };
            c.cmp(t252, c.r32(0), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t253) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000868f9d); };
            c.cmp(t253, c.r32(0), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000868fa6); };
            if !c.push64(t254) { return RegfileExit::fault(0x7000868fa6); }
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868fa9); };
            c.w64(6, t255);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x7000868fba, 0x7000868fb5);
        }
        172 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868fba); };
            if !c.push64(t256) { return RegfileExit::fault(0x7000868fba); }
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000868fbd); };
            c.w64(3, t257);
            c.w32(0, 0x1u64);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868fc9); };
            c.w64(6, t258);
            return c.call(0x700086ed74, 0x7000868fd2, 0x7000868fcd);
        }
        173 => {
            return RegfileExit::fault(0x7000868fd2);
        }
        174 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000868fd3); }
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x7000868fd4); };
            c.w64(3, t259);
            c.w32(0, 0x1u64);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868fe0); };
            c.w64(6, t260);
            return c.call(0x700086ed74, 0x7000868fe9, 0x7000868fe4);
        }
        175 => {
            return RegfileExit::fault(0x7000868fe9);
        }
        176 => {
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x7000868fea); };
            c.w64(3, t261);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000868ff3); };
            c.w64(6, t262);
            return c.call(0x700086ed74, 0x7000868ffc, 0x7000868ff7);
        }
        177 => {
            return RegfileExit::fault(0x7000868ffc);
        }
        178 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000868ffd); };
            if !c.push64(t263) { return RegfileExit::fault(0x7000868ffd); }
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x7000869000); };
            c.w64(3, t264);
            c.w32(0, 0x1u64);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086900c); };
            c.w64(6, t265);
            return c.call(0x700086ed74, 0x7000869015, 0x7000869010);
        }
        179 => {
            return RegfileExit::fault(0x7000869015);
        }
        180 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086901b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086901c); }
            let Some(t266) = c.ld64(c.r64(13).wrapping_add(0x4b8)) else { return RegfileExit::fault(0x700086901d); };
            c.w64(1, t266);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000869024); }
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000869025); };
            if !c.push64(t267) { return RegfileExit::fault(0x7000869025); }
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000869028); };
            c.w64(3, t268);
            c.w32(0, 0x4u64);
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000869034); };
            c.w64(6, t269);
            return c.call(0x700086ed74, 0x700086903d, 0x7000869038);
        }
        181 => {
            return RegfileExit::fault(0x700086903d);
        }
        182 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000869043); }
            return c.call(0x70007febf4, 0x700086904d, 0x7000869048);
        }
        183 => {
            c.w64(8, c.r64(0));
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000869050); };
            c.w64(9, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000869054); };
            c.w64(7, t271);
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000869058); };
            c.w64(3, t272);
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086905c); };
            c.w64(15, t273);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000869060); }
            blk = 87; continue;
        }
        184 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086906a); };
            c.w64(8, t274);
            blk = 88; continue;
        }
        185 => {
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000869076); };
            c.w64(3, t275);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x7000869087, 0x7000869082);
        }
        186 => {
            return RegfileExit::fault(0x7000869087);
        }
        187 => {
            c.w32(1, 0x52u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086908d); }
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0xb70)) else { return RegfileExit::fault(0x700086908e); };
            c.w64(1, t276);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000869095); }
            let Some(t277) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000869096); };
            c.w64(3, t277);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008690a7, 0x70008690a2);
        }
        188 => {
            return RegfileExit::fault(0x70008690a7);
        }
        189 => {
            c.w32(11, c.r32(11));
            let r278 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r278);
            c.w64(8, 0xfffffffffffc0000u64);
            let r279 = c.r64(8) & c.r64(11);
            c.w64(8, r279);
            c.logic(r279, 64);
            let Some(t280) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008690b8); };
            c.logic(t280 & 0x2u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 190; continue;
        }
        190 => {
            c.w64(3, c.r64(12).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008690c8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008690c9); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008690d6, 0x70008690d1);
        }
        191 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008690d6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008690d8); }
            blk = 72; continue;
        }
        192 => {
            return RegfileExit::jump(0x70008690e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008690f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008690f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008690f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008690f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008690fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008690ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000869105); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086910e); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869114); };
            c.cmp(t5, 0x832u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086911c); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000869122); };
            c.w64(3, t8);
            return c.call(0x7000868114, 0x700086912b, 0x7000869126);
        }
        4 => {
            c.w64(2, 0xfffffffffffffffeu64);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000869136); };
            c.w64(0, t9);
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086913d); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000869144); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x700086914b);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086914e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000869155); }
            return c.ret(0x0, 0x7000869157);
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000869158); };
            c.w64(0, t11);
            blk = 6; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000869161); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000869165); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000869173, 0x700086916e);
        }
        11 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000869173); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        12 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086917e); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086917f); };
            c.w64(1, t14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0xe7)) else { return RegfileExit::fault(0x7000869186); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086918f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000869190); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000869191); };
            c.w64(3, t17);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008691a2, 0x700086919d);
        }
        13 => {
            return RegfileExit::fault(0x70008691a2);
        }
        14 => {
            return RegfileExit::jump(0x70008691a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008691b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008691b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008691b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008691b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008691ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008691bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008691c5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008691d1); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008691d7); };
            c.cmp(t5, 0x832u64, 16);
            if c.cc_ne() { blk = 13; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008691df); };
            c.w64(3, t6);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008691e8); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008691ee); };
            c.cmp(t9, 0xbau64, 16);
            if c.cc_b() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008691f6); };
            c.w64(1, t10);
            return c.call(0x70008694f4, 0x7000869202, 0x70008691fd);
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000869202); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000869209); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x7000869210);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000869213); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086921a); }
            return c.ret(0x0, 0x700086921c);
        }
        9 => {
            c.w32(2, 0x1a8u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000869222); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000869223); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000869224); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000869235, 0x7000869230);
        }
        10 => {
            return RegfileExit::fault(0x7000869235);
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000869236); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086923a); };
            c.w64(3, t13);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000869248, 0x7000869243);
        }
        12 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000869248); };
            c.w64(6, t14);
            blk = 1; continue;
        }
        13 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000869256); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000869257); };
            c.w64(2, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0xeb)) else { return RegfileExit::fault(0x700086925e); };
            c.w32(2, t16);
            let r17 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r17);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000869267); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000869268); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000869269); };
            c.w64(3, t18);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700086927a, 0x7000869275);
        }
        14 => {
            return RegfileExit::fault(0x700086927a);
        }
        15 => {
            return RegfileExit::jump(0x700086927c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000869294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000869294); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000869298); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000869299); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086929a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086929f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008692a5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008692ad); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008692b3); };
            c.cmp(t5, 0x832u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008692bb); };
            c.w64(3, t6);
            return c.call(0x7000869334, 0x70008692c4, 0x70008692bf);
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008692c4); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008692cb); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x10, 0x70008692d2);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008692d5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008692dc); }
            return c.ret(0x0, 0x70008692de);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008692df); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008692e3); };
            c.w64(3, t8);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008692f1, 0x70008692ec);
        }
        8 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008692f1); };
            c.w64(6, t9);
            blk = 1; continue;
        }
        9 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008692fc); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008692fd); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0xef)) else { return RegfileExit::fault(0x7000869304); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086930d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086930e); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086930f); };
            c.w64(3, t13);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000869320, 0x700086931b);
        }
        10 => {
            return RegfileExit::fault(0x7000869320);
        }
        11 => {
            return RegfileExit::jump(0x7000869324);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000869334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086933d); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869343); };
            c.cmp(t2, 0xbau64, 16);
            if c.cc_b() { blk = 23; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086934f); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086935a); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869360); };
            c.w32(1, t7);
            c.cmp(c.r16(1), 0xb0u64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(1), 0x7bu64, 16);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869375); };
            c.w32(1, t8);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_be() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000869388); };
            c.w32(2, t9);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 10; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869393); };
            c.w32(2, t10);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if let Some(sr) = c.shr(c.r64(2), 0xau64, 64) { c.w64(2, sr); }
            let r11 = c.r64(2) & 0xfffffu64;
            c.w64(2, r11);
            c.logic(r11, 64);
            blk = 10; continue;
        }
        9 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008693b9); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008693bf); };
            c.w32(7, t14);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7).wrapping_sub(0x1));
            c.w64(9, c.r64(2));
            let r15 = c.r64(9) & c.r64(8);
            c.w64(9, r15);
            c.logic(r15, 64);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        12 => {
            let r16 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r16);
            let r17 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r17);
            c.w64(12, c.r64(8));
            let r18 = c.r64(12) & c.r64(9);
            c.w64(12, r18);
            c.logic(r18, 64);
            c.w64(9, c.r64(12));
            blk = 13; continue;
        }
        13 => {
            c.w64(12, c.r64(9).wrapping_mul(2).wrapping_add(0x3));
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008693ef); };
            c.w32(15, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008693f4); };
            c.cmp(t20, c.r32(15), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869406); };
            c.w32(8, t21);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.shr(c.r64(7), 0x2u64, 64) { c.w64(7, sr); }
            let r22 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r22);
            c.cmp(c.r64(8), 0x10u64, 64);
            c.w8(9, (c.cc_ge() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r64(8), c.r64(7), 64);
            c.w8(7, (c.cc_le() as u64));
            c.w32(7, c.r8(7));
            c.logic(c.r32(7) & c.r32(9), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869438); };
            c.w32(2, t23);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_ae() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700086944a); };
            c.w64(2, t24);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x7000869451); }
            let Some(t25) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869456); };
            c.w32(7, t25);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_ae() { blk = 26; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x7000869469); }
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086946e); };
            c.w32(2, t26);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000869474); };
            c.w32(7, t27);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, c.r64(8).wrapping_add(c.r64(8)));
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x7000869484); }
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869487); };
            c.w32(2, t28);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 28; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(2, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r29 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r29);
            let r30 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r30);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008694a2); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008694a5); };
            c.w64(0, t31);
            return c.ret(0x0, 0x70008694ac);
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008694ad); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008694b1); }
            let r32 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008694b5); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008694b6); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008694b7); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1df8)) else { return RegfileExit::fault(0x70008694b8); };
            c.w64(3, t33);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008694c9, 0x70008694c4);
        }
        22 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008694cc); }
            return c.ret(0x0, 0x70008694cd);
        }
        23 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008694ce); };
            c.w64(0, t34);
            return c.ret(0x0, 0x70008694d5);
        }
        24 => {
            return RegfileExit::fault(0x70008694d6);
        }
        25 => {
            return RegfileExit::fault(0x70008694d7);
        }
        26 => {
            return RegfileExit::fault(0x70008694d8);
        }
        27 => {
            return RegfileExit::fault(0x70008694d9);
        }
        28 => {
            return RegfileExit::fault(0x70008694da);
        }
        29 => {
            return RegfileExit::jump(0x70008694dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008694f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008694f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008694f8); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008694fe); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869504); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086950b); };
            c.w32(9, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086950f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000869513); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000869517); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086951b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700086951f); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869529); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let Some(t8) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869530); };
            c.w32(11, t8);
            c.cmp(c.r16(11), 0xb0u64, 16);
            if c.cc_e() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(11), 0x7bu64, 16);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869549); };
            c.w32(11, t9);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 30; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700086955d); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 8; continue;
        }
        6 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869569); };
            c.w32(8, t11);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.shr(c.r64(8), 0xau64, 64) { c.w64(8, sr); }
            let r12 = c.r64(8) & 0xfffffu64;
            c.w64(8, r12);
            c.logic(r12, 64);
            blk = 8; continue;
        }
        7 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700086958c); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r13 = c.r64(11) & c.r64(8);
            c.w64(11, r13);
            c.logic(r13, 64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        10 => {
            c.w64(15, c.r64(9).wrapping_sub(0x1));
            let r14 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r14);
            let r15 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r15);
            let r16 = c.r64(15) & c.r64(11);
            c.w64(15, r16);
            c.logic(r16, 64);
            c.w64(11, c.r64(15));
            blk = 11; continue;
        }
        11 => {
            c.w64(15, c.r64(11).wrapping_mul(2).wrapping_add(0x3));
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008695bf); };
            c.w32(2, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008695c4); };
            c.cmp(t18, c.r32(2), 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008695d5); };
            c.w32(8, t19);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 31; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x70008695f4); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r20 = c.r64(9) & c.r64(7);
            c.w64(9, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000869607); };
            c.logic(t21 & 0x4u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000869615); }
            return c.ret(0x0, 0x7000869616);
        }
        17 => {
            c.w64(10, c.r64(4));
            let r22 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r22);
            let r23 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r23);
            c.logic(r23, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000869622); }
            c.w64(2, c.r64(3));
            c.w64(1, c.r64(13));
            c.w64(8, c.r64(1));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x13b0)) else { return RegfileExit::fault(0x7000869630); };
            c.w64(0, t24);
            c.w64(10, 0x700086963eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700086963e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000869642); }
            return c.call(c.r64(0), 0x7000869648, 0x7000869646);
        }
        18 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000869648); }
            let Some(t25) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000869650); };
            c.w64(4, t25);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086965e); };
            c.w64(3, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000869662); };
            c.w64(1, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000869666); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086966a); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086966e); };
            c.w64(9, t30);
            blk = 19; continue;
        }
        19 => {
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869672); };
            c.w32(11, t31);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t32) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000869679); };
            c.w32(12, t32);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r33 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r33);
            c.w64(15, c.r64(11));
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(9));
            let r34 = c.sub(c.r64(2), c.r64(11), 64);
            c.w64(2, r34);
            let r35 = c.add(c.r64(15), c.r64(11), 64);
            c.w64(15, r35);
            c.w64(0, c.r64(12));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(9), c.r64(15), 64);
            c.w8(15, (c.cc_l() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r64(2), c.r64(0), 64);
            c.w8(0, (c.cc_l() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r64(11), c.r64(12), 64);
            c.w8(12, (c.cc_le() as u64));
            c.w32(12, c.r8(12));
            let r36 = c.r32(15) | c.r32(0);
            c.w32(15, r36);
            c.logic(r36, 32);
            let r37 = c.r32(15) | c.r32(12);
            c.w32(15, r37);
            c.logic(r37, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r38 = c.r64(12) & c.r64(8);
            c.w64(12, r38);
            c.logic(r38, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 22; continue;
        }
        21 => {
            let r39 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r39);
            c.w64(15, c.r64(9).wrapping_sub(0x1));
            let r40 = c.add(c.r64(12), c.r64(8), 64);
            c.w64(12, r40);
            let r41 = c.r64(15) & c.r64(12);
            c.w64(15, r41);
            c.logic(r41, 64);
            c.w64(12, c.r64(15));
            blk = 22; continue;
        }
        22 => {
            c.w64(15, c.r64(12).wrapping_mul(2).wrapping_add(0x3));
            c.w64(0, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            let Some(t42) = c.ld32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008696f7); };
            c.w32(2, t42);
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008696fc); };
            c.cmp(t43, c.r32(2), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700086970b); };
            c.cmp(t44, c.r32(2), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let r45 = c.r32(6) | c.r32(2);
            c.w32(6, r45);
            c.logic(r45, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(15).wrapping_add(0x1));
            c.w32(9, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(3));
            if !c.st32(c.r64(7).wrapping_add(c.r64(0)), c.r32(11)) { return RegfileExit::fault(0x7000869727); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r46 = c.r64(12) & c.r64(7);
            c.w64(12, r46);
            c.logic(r46, 64);
            let Some(t47) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086973b); };
            c.logic(t47 & 0x4u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.w64(12, c.r64(1));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x7000869752); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r48 = c.r64(11) & c.r64(7);
            c.w64(11, r48);
            c.logic(r48, 64);
            let Some(t49) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000869766); };
            c.logic(t49 & 0x4u64, 8);
            if c.cc_ne() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700086976d); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000869771); };
            c.w64(0, t50);
            blk = 16; continue;
        }
        28 => {
            c.w32(7, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086977e); };
            if !c.push64(t51) { return RegfileExit::fault(0x700086977e); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000869781); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000869782); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000869783); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1e00)) else { return RegfileExit::fault(0x7000869784); };
            c.w64(3, t52);
            c.w32(0, 0x4u64);
            return c.call(0x700086ed74, 0x7000869795, 0x7000869790);
        }
        29 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000869795); };
            c.w64(0, t53);
            blk = 16; continue;
        }
        30 => {
            return RegfileExit::fault(0x700086979e);
        }
        31 => {
            return RegfileExit::fault(0x700086979f);
        }
        32 => {
            c.w32(12, c.r32(12));
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            c.w64(11, 0xfffffffffffc0000u64);
            let r55 = c.r64(11) & c.r64(12);
            c.w64(11, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008697b0); };
            c.logic(t56 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(15, c.r64(7).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008697bb); }
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008697c5, 0x70008697c0);
        }
        34 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008697c5); }
            blk = 27; continue;
        }
        35 => {
            c.w32(11, c.r32(11));
            let r57 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r57);
            c.w64(12, 0xfffffffffffc0000u64);
            let r58 = c.r64(12) & c.r64(11);
            c.w64(12, r58);
            c.logic(r58, 64);
            let Some(t59) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008697d9); };
            c.logic(t59 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(15, c.r64(7).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008697e9); }
            c.w64(8, c.r64(15));
            return c.call(0x70007f1914, 0x70008697f3, 0x70008697ee);
        }
        37 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008697f3); }
            blk = 25; continue;
        }
        38 => {
            c.w32(1, c.r32(1));
            let r60 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r60);
            c.w64(9, 0xfffffffffffc0000u64);
            let r61 = c.r64(9) & c.r64(1);
            c.w64(9, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000869809); };
            c.logic(t62 & 0x2u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000869818); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000869822, 0x700086981d);
        }
        40 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000869822); }
            blk = 16; continue;
        }
        41 => {
            return RegfileExit::jump(0x700086982c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000869834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000869834); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000869838); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086983e); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869844); };
            c.w32(8, t3);
            if !c.st32(c.r64(0).wrapping_add(0x27), c.r32(8)) { return RegfileExit::fault(0x7000869848); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r4 = c.r64(9) & c.r64(0);
            c.w64(9, r4);
            c.logic(r4, 64);
            let Some(t5) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086985c); };
            c.logic(t5 & 0x4u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000869867); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086986d); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000869874); };
            c.w64(8, t9);
            c.w64(9, c.r64(8).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086987b); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086987f); };
            c.w64(11, t10);
            let Some(t11) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000869886); };
            c.cmp(t11, c.r64(9), 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700086988b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869892); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000869899); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008698a0); };
            c.w32(9, t16);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x68u64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008698b0);
        }
        6 => {
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008698b1); };
            c.w32(8, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008698b8); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008698bc); };
            c.w64(8, t18);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008698c3); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008698c7); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008698cb); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008698ce); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008698d1); };
            c.w64(1, t19);
            let Some(t20) = c.ld8(c.r64(1)) else { return RegfileExit::fault(0x70008698d8); };
            c.logic(t20 & 0x6u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x1010)) else { return RegfileExit::fault(0x70008698dd); };
            c.w64(1, t21);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008698e7); };
            c.cmp(t22, c.r32(8), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(3, c.r64(7));
            return c.call(0x70008b1bf4, 0x70008698f5, 0x70008698f0);
        }
        9 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008698f5); };
            c.w64(0, t23);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008698fc); }
            return c.ret(0x0, 0x70008698fd);
        }
        10 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000869903); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000869907); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700086990b); }
            return c.call(0x70007febf4, 0x7000869914, 0x700086990f);
        }
        11 => {
            c.w64(7, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000869917); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086991b); };
            c.w64(6, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086991f); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000869923); };
            c.w64(3, t27);
            blk = 4; continue;
        }
        12 => {
            c.w64(3, c.r64(7));
            return c.call(0x70008ba734, 0x7000869934, 0x700086992f);
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            c.w32(8, c.r32(8));
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            c.w64(9, 0xfffffffffffc0000u64);
            let r29 = c.r64(9) & c.r64(8);
            c.w64(9, r29);
            c.logic(r29, 64);
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000869946); };
            c.logic(t30 & 0x2u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, c.r64(0).wrapping_add(0x27));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000869955); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000869956); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000869963, 0x700086995e);
        }
        16 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000869963); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000869965); }
            blk = 2; continue;
        }
        17 => {
            return RegfileExit::jump(0x700086996c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000869974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000869974); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000869978); }
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086997a); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869980); };
            c.w32(7, t2);
            if !c.st32(c.r64(0).wrapping_add(0x27), c.r32(7)) { return RegfileExit::fault(0x7000869983); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r3 = c.r64(8) & c.r64(0);
            c.w64(8, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000869996); };
            c.logic(t4 & 0x4u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086999d); };
            c.w32(0, t5);
            let r6 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008699a3); };
            c.w64(1, t7);
            return c.call(0x70008b1fb4, 0x70008699af, 0x70008699aa);
        }
        3 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008699b2); }
            return c.ret(0x0, 0x70008699b3);
        }
        4 => {
            c.w32(7, c.r32(7));
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            c.w64(8, 0xfffffffffffc0000u64);
            let r9 = c.r64(8) & c.r64(7);
            c.w64(8, r9);
            c.logic(r9, 64);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008699c3); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(0).wrapping_add(0x27));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008699ce); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008699cf); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008699dc, 0x70008699d7);
        }
        6 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008699dc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008699de); }
            blk = 2; continue;
        }
        7 => {
            return RegfileExit::jump(0x70008699e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008699f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008699f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008699f8); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008699fe); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000869a05); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000869a0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000869a10); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000869a14); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000869a18); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000869a1c); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869a29); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869a30); };
            c.cmp(t7, 0x83du64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869a3d); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000869a44); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869a4b); };
            c.w32(12, t12);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x97u64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x7000869a5e);
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0x263)) else { return RegfileExit::fault(0x7000869a5f); };
            c.w32(12, t13);
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000869a66); };
            c.cmp(t14, c.r32(12), 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x7000869a70); };
            c.w64(9, t15);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000869a7a); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869a84); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000869a8b); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000869a92); };
            c.w64(12, t20);
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000869a95); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000869aa1); };
            c.w64(2, t23);
            let Some(t24) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000869aa8); };
            c.cmp(t24, c.r64(15), 64);
            if c.cc_be() { blk = 66; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000869ab1); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869ab9); };
            c.w32(12, t25);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x7du64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x7000869ac9);
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x7000869aca); };
            c.w32(12, t26);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000869ad1); }
            c.w32(12, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000869adb); }
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869adf); };
            c.w32(12, t27);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000869aef);
        }
        12 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869af0); };
            c.w32(12, t28);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000869af4); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000869af8); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x7000869afc); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000869b00); };
            c.w64(12, t29);
            let Some(t30) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000869b07); };
            c.w64(15, t30);
            c.w64(2, c.r64(15).wrapping_add(0x1c));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000869b0f); };
            c.w64(7, t31);
            let Some(t32) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000869b16); };
            c.cmp(t32, c.r64(2), 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x7000869b1f); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            let Some(t33) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869b27); };
            c.w32(15, t33);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xbeu64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x7000869b3a);
        }
        16 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x7000869b3b); };
            c.w32(15, t34);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000869b42); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000869b47); };
            c.w64(15, t35);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000869b4e); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000869b53); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x7000869b58); };
            c.w64(15, t36);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x7000869b5f); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1068)) else { return RegfileExit::fault(0x7000869b64); };
            c.w64(15, t37);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x7000869b6b); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000869b70); }
            let Some(t38) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869b75); };
            c.w32(15, t38);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t39) = c.ld64(c.r64(2).wrapping_add(c.r64(15).wrapping_mul(4))) else { return RegfileExit::fault(0x7000869b83); };
            c.w64(15, t39);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x7000869b87); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000869b8c); };
            c.w64(15, t40);
            let Some(t41) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000869b93); };
            c.w64(2, t41);
            c.w64(7, c.r64(2).wrapping_add(0x1c));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000869b9a); };
            c.w64(1, t42);
            let Some(t43) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000869ba1); };
            c.cmp(t43, c.r64(7), 64);
            if c.cc_be() { blk = 70; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x7000869baa); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 18; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869bb1); };
            c.w32(2, t44);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x7000869bc4);
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x7000869bc5); };
            c.w32(2, t45);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000869bcc); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000869bd0); };
            c.w64(2, t46);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x7000869bd7); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x7000869bdb); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x7000869bdf); };
            c.w64(2, t47);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x7000869be6); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1058)) else { return RegfileExit::fault(0x7000869bea); };
            c.w64(2, t48);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x7000869bf1); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000869bf5); }
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869bf9); };
            c.w32(11, t49);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t50) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x7000869c07); };
            c.w64(11, t50);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x7000869c0b); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x7000869c0f); };
            c.w64(11, t51);
            let Some(t52) = c.ld32(c.r64(11)) else { return RegfileExit::fault(0x7000869c16); };
            c.w32(11, t52);
            c.logic(c.r8(11) & 0xeu64, 8);
            if c.cc_ne() { blk = 97; continue; }
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 72; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000869c2d); };
            c.w64(8, t53);
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(12));
            c.w64(1, c.r64(15));
            c.w64(2, c.r64(8));
            c.w64(6, c.r64(9));
            return c.call(0x70008b2ef4, 0x7000869c48, 0x7000869c43);
        }
        24 => {
            blk = 59; continue;
        }
        25 => {
            let Some(t54) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869c4d); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000869c54); };
            c.w32(9, t56);
            let r57 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000869c5b); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000869c62); };
            c.w64(12, t59);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000869c6a); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000869c71); };
            c.cmp(t61, c.r64(15), 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000869c7a); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 27; continue;
        }
        27 => {
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869c82); };
            c.w32(12, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x7du64, 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x7000869c92);
        }
        29 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x7000869c93); };
            c.w32(12, t63);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000869c9a); }
            c.w32(12, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000869ca4); }
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869ca8); };
            c.w32(12, t64);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x7000869cb8);
        }
        31 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869cb9); };
            c.w32(12, t65);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000869cbd); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000869cc1); }
            c.w64(12, c.r64(0));
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x7000869cc8); }
            let Some(t66) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869ccc); };
            c.w32(15, t66);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x7000869cdf);
        }
        33 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x3af)) else { return RegfileExit::fault(0x7000869ce0); };
            c.w32(15, t67);
            let r68 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000869cea); };
            c.w64(0, t69);
            let Some(t70) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000869cf1); };
            c.w64(2, t70);
            let Some(t71) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000869cf4); };
            c.w32(15, t71);
            c.w64(7, c.r64(2).wrapping_add(0x14));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000869cfc); };
            c.w64(12, t72);
            let Some(t73) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000869d03); };
            c.cmp(t73, c.r64(7), 64);
            if c.cc_be() { blk = 101; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x7000869d0d); }
            let r74 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r74);
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000869d14); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000869d18); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000869d1f); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000869d23); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x7000869d2a); }
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x7000869d2e); }
            let Some(t76) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000869d32); };
            c.w64(15, t76);
            c.w64(7, c.r64(15).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x7000869d39); }
            let Some(t77) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000869d3d); };
            c.cmp(t77, c.r64(7), 64);
            if c.cc_be() { blk = 103; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x7000869d47); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869d4e); };
            c.w32(15, t78);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x7000869d58); }
            c.cmp(c.r64(15), 0xbeu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x7000869d65);
        }
        39 => {
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x7000869d66); };
            c.w32(15, t79);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000869d6d); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000869d72); };
            c.w64(15, t80);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000869d79); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000869d7e); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x7000869d83); };
            c.w64(15, t81);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x7000869d8a); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1068)) else { return RegfileExit::fault(0x7000869d8f); };
            c.w64(15, t82);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x7000869d96); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000869d9b); }
            let Some(t83) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869da0); };
            c.w32(15, t83);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(13).wrapping_add(0x63f0));
            let Some(t84) = c.ld64(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(4))) else { return RegfileExit::fault(0x7000869dae); };
            c.w64(15, t84);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x7000869db2); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000869db7); };
            c.w64(15, t85);
            let Some(t86) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000869dbe); };
            c.w64(0, t86);
            c.w64(7, c.r64(0).wrapping_add(0x1c));
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000869dc5); };
            c.w64(12, t87);
            let Some(t88) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000869dcc); };
            c.cmp(t88, c.r64(7), 64);
            if c.cc_be() { blk = 105; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x7000869dd6); }
            c.w64(12, c.r64(0).wrapping_add(0x1));
            blk = 41; continue;
        }
        41 => {
            let Some(t89) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869ddd); };
            c.w32(15, t89);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000869de7); }
            c.cmp(c.r64(15), 0xbeu64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            return RegfileExit::fault(0x7000869df4);
        }
        43 => {
            let Some(t90) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x7000869df5); };
            c.w32(9, t90);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000869dfc); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000869e01); };
            c.w64(9, t91);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000869e08); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000869e0d); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x7000869e12); };
            c.w64(9, t92);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x7000869e19); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x1058)) else { return RegfileExit::fault(0x7000869e1e); };
            c.w64(9, t93);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000869e25); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000869e2a); }
            let Some(t94) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869e2f); };
            c.w32(9, t94);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(13).wrapping_add(0x63f0));
            let Some(t95) = c.ld64(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x7000869e3d); };
            c.w64(9, t95);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x7000869e41); }
            let Some(t96) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869e46); };
            c.w32(9, t96);
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            let Some(t98) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000869e4d); };
            c.w32(9, t98);
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            let Some(t100) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869e54); };
            c.w32(11, t100);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            return RegfileExit::fault(0x7000869e67);
        }
        45 => {
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x7000869e68); };
            c.w32(7, t101);
            let r102 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r102);
            let Some(t103) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869e72); };
            c.w32(9, t103);
            let r104 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r104);
            let Some(t105) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000869e79); };
            c.logic(t105 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t106) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000869e86); };
            c.w32(9, t106);
            let r107 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r107);
            let Some(t108) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000869e8d); };
            c.w32(9, t108);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(9, c.r64(8));
            blk = 52; continue;
        }
        50 => {
            return RegfileExit::fault(0x7000869eac);
        }
        51 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000869ead); };
            c.w64(9, t109);
            blk = 52; continue;
        }
        52 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000869eb4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000869eb6); }
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000869eb7); };
            c.w64(9, t110);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000869ebe); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2134, 0x7000869eca, 0x7000869ec5);
        }
        53 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000869eca); };
            c.w64(2, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000869ece); };
            c.w64(12, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000869ed2); };
            c.w64(6, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000869ed6); };
            c.w64(3, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000869eda); };
            c.w64(8, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000869ede); };
            c.w64(1, t116);
            blk = 54; continue;
        }
        54 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x7000869ee2); };
            c.w64(9, t117);
            let Some(t118) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x7000869ee9); };
            c.w32(9, t118);
            c.logic(c.r8(9) & 0xeu64, 8);
            if c.cc_ne() { blk = 136; continue; }
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000869f00); };
            c.w64(8, t119);
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000869f07); }
            c.w64(0, c.r64(2));
            return c.call(0x70008ba734, 0x7000869f13, 0x7000869f0e);
        }
        58 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000869f13); };
            c.w64(0, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000869f17); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000869f1b); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000869f1f); };
            c.w64(2, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000869f23); };
            c.w64(6, t124);
            return c.call(0x70008b2ef4, 0x7000869f2c, 0x7000869f27);
        }
        59 => {
            c.w32(8, 0x2u64);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000869f32); };
            c.w64(9, t125);
            if !c.st32(c.r64(9).wrapping_add(0x2b), c.r32(8)) { return RegfileExit::fault(0x7000869f36); }
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000869f3a); };
            c.w64(0, t126);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000869f44); }
            return c.ret(0x0, 0x7000869f45);
        }
        60 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x7000869f46); }
            c.w64(0, c.r64(3));
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000869f4d); };
            c.w64(3, t127);
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x7000869f5c, 0x7000869f57);
        }
        61 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000869f5c); };
            c.w64(11, t128);
            let Some(t129) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000869f60); };
            c.w32(8, t129);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            return RegfileExit::fault(0x7000869f73);
        }
        63 => {
            let Some(t130) = c.ld32(c.r64(11).wrapping_add(0x3af)) else { return RegfileExit::fault(0x7000869f74); };
            c.cmp(t130, c.r32(0), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000869f7d); };
            c.w64(0, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000869f81); };
            c.w64(6, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000869f85); };
            c.w64(3, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000869f89); };
            c.w64(8, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000869f8d); };
            c.w64(1, t135);
            blk = 25; continue;
        }
        65 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000869f96); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000869f9a); };
            c.w64(6, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000869f9e); };
            c.w64(3, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000869fa2); };
            c.w64(8, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000869fa6); };
            c.w64(1, t140);
            blk = 6; continue;
        }
        66 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000869fb4); }
            return c.call(0x70007febf4, 0x7000869fbd, 0x7000869fb8);
        }
        67 => {
            c.w64(11, c.r64(0));
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000869fc0); };
            c.w64(0, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000869fc4); };
            c.w64(6, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000869fc8); };
            c.w64(3, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000869fcc); };
            c.w64(8, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000869fd0); };
            c.w64(1, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000869fd4); };
            c.w64(9, t146);
            blk = 8; continue;
        }
        68 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000869fe2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000869fe6); }
            return c.call(0x70007febf4, 0x7000869fef, 0x7000869fea);
        }
        69 => {
            c.w64(12, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000869ff2); };
            c.w64(0, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000869ff6); };
            c.w64(6, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000869ffa); };
            c.w64(3, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000869ffe); };
            c.w64(8, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a002); };
            c.w64(1, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086a006); };
            c.w64(9, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086a00a); };
            c.w64(11, t153);
            blk = 14; continue;
        }
        70 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086a018); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086a01c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086a020); }
            return c.call(0x70007febf4, 0x700086a029, 0x700086a024);
        }
        71 => {
            c.w64(15, c.r64(0));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086a02c); };
            c.w64(0, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a030); };
            c.w64(6, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a034); };
            c.w64(3, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a038); };
            c.w64(8, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a03c); };
            c.w64(12, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086a040); };
            c.w64(9, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086a044); };
            c.w64(11, t160);
            blk = 18; continue;
        }
        72 => {
            let Some(t161) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a04d); };
            c.w32(8, t161);
            let r162 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r162);
            let Some(t163) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086a054); };
            c.w32(8, t163);
            let r164 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r164);
            let Some(t165) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a05b); };
            c.w32(11, t165);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086a065); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086a069); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086a06d); }
            c.cmp(c.r64(11), 0xeau64, 64);
            if c.cc_a() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            return RegfileExit::fault(0x700086a07a);
        }
        74 => {
            let Some(t166) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086a07b); };
            c.w32(8, t166);
            let r167 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r167);
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a085); };
            c.w64(11, t168);
            let Some(t169) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086a08c); };
            c.w64(2, t169);
            let Some(t170) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086a08f); };
            c.w32(8, t170);
            c.w64(1, c.r64(2).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086a097); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a09b); };
            c.w64(7, t171);
            let Some(t172) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086a0a2); };
            c.cmp(t172, c.r64(1), 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700086a0b1, 0x700086a0ac);
        }
        76 => {
            blk = 78; continue;
        }
        77 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700086a0b3); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 78; continue;
        }
        78 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a0bd); };
            c.w64(8, t173);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086a0c1); }
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086a0c5); };
            c.w64(8, t174);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086a0cc); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086a0d0); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086a0d7); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086a0db); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086a0df); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086a0e3); }
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086a0e7); };
            c.w64(8, t175);
            let Some(t176) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700086a0ee); };
            c.w32(8, t176);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700086a0f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086a0f5); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a109); };
            c.w64(6, t177);
            blk = 93; continue;
        }
        81 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a112); };
            c.w64(6, t178);
            let Some(t179) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a116); };
            c.w32(9, t179);
            let r180 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r180);
            let Some(t181) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086a11d); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a124); };
            c.w32(11, t183);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            return RegfileExit::fault(0x700086a137);
        }
        83 => {
            let Some(t184) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086a138); };
            c.w32(7, t184);
            let r185 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r185);
            let Some(t186) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a142); };
            c.w32(9, t186);
            let r187 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r187);
            let Some(t188) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086a149); };
            c.logic(t188 & 0x2u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a150); };
            c.w64(3, t189);
            let Some(t190) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a154); };
            c.w32(9, t190);
            let r191 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r191);
            let Some(t192) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086a15b); };
            c.w32(9, t192);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 88; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(9, c.r64(3));
            blk = 89; continue;
        }
        87 => {
            return RegfileExit::fault(0x700086a17a);
        }
        88 => {
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086a17b); };
            c.w64(9, t193);
            blk = 89; continue;
        }
        89 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086a182); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086a184); }
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086a185); };
            c.w64(9, t194);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086a18c); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700086a19b, 0x700086a196);
        }
        90 => {
            blk = 92; continue;
        }
        91 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086a19d); }
            c.w32(0, 0x1u64);
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086a1a3); };
            c.w64(3, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a1aa); };
            c.w64(6, t196);
            return c.call(0x700086ed74, 0x700086a1b3, 0x700086a1ae);
        }
        92 => {
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a1b3); };
            c.w64(6, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a1b7); };
            c.w64(0, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a1bb); };
            c.w64(8, t199);
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086a1c5); }
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a1c6); };
            if !c.push64(t200) { return RegfileExit::fault(0x700086a1c6); }
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086a1c9); };
            c.w64(3, t201);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086a1dd, 0x700086a1d8);
        }
        95 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a1dd); };
            c.w64(0, t202);
            blk = 96; continue;
        }
        96 => {
            c.w64(8, c.r64(0));
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a1e4); };
            c.w64(3, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a1e8); };
            c.w64(12, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086a1ec); };
            c.w64(15, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086a1f0); };
            c.w64(9, t206);
            blk = 23; continue;
        }
        97 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086a1f9); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086a1fa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086a1fb); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700086a1fd); }
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a1ff); };
            if !c.push64(t207) { return RegfileExit::fault(0x700086a1ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086a202); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086a206); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086a20a); }
            c.w32(0, 0x5u64);
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x25a0)) else { return RegfileExit::fault(0x700086a213); };
            c.w64(3, t208);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700086a225, 0x700086a220);
        }
        98 => {
            c.w64(8, c.r64(0));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a228); };
            c.w64(3, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a22c); };
            c.w64(12, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086a230); };
            c.w64(15, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086a234); };
            c.w64(9, t212);
            blk = 23; continue;
        }
        99 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700086a242); }
            return c.call(0x70007febf4, 0x700086a24b, 0x700086a246);
        }
        100 => {
            c.w64(11, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086a24e); };
            c.w64(0, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a252); };
            c.w64(6, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a256); };
            c.w64(3, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a25a); };
            c.w64(8, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a25e); };
            c.w64(1, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086a262); };
            c.w64(9, t218);
            blk = 27; continue;
        }
        101 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086a270); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700086a274); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086a278); }
            return c.call(0x70007febf4, 0x700086a281, 0x700086a27c);
        }
        102 => {
            c.w64(2, c.r64(0));
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a284); };
            c.w64(6, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a288); };
            c.w64(3, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a28c); };
            c.w64(8, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a290); };
            c.w64(1, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086a294); };
            c.w64(11, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086a298); };
            c.w64(9, t224);
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a29c); };
            c.w64(0, t225);
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a2a3); };
            c.w64(12, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086a2aa); };
            c.w64(15, t227);
            blk = 35; continue;
        }
        103 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086a2b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700086a2bc); }
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x700086a2c8, 0x700086a2c3);
        }
        104 => {
            c.w64(12, c.r64(0));
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a2cb); };
            c.w64(2, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a2cf); };
            c.w64(6, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a2d3); };
            c.w64(3, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a2d7); };
            c.w64(8, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a2db); };
            c.w64(1, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086a2df); };
            c.w64(11, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086a2e3); };
            c.w64(9, t234);
            blk = 37; continue;
        }
        105 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700086a2f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700086a2f5); }
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x700086a301, 0x700086a2fc);
        }
        106 => {
            c.w64(12, c.r64(0));
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a304); };
            c.w64(2, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a308); };
            c.w64(6, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a30c); };
            c.w64(3, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a310); };
            c.w64(8, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a314); };
            c.w64(1, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086a318); };
            c.w64(11, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086a31c); };
            c.w64(9, t241);
            blk = 41; continue;
        }
        107 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086a325); }
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086a326); };
            c.w64(3, t242);
            c.w32(0, 0x1u64);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a332); };
            c.w64(6, t243);
            return c.call(0x700086ed74, 0x700086a33b, 0x700086a336);
        }
        108 => {
            blk = 53; continue;
        }
        109 => {
            let Some(t244) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a340); };
            c.w32(8, t244);
            let r245 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r245);
            let Some(t246) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086a347); };
            c.w32(8, t246);
            let r247 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r247);
            let Some(t248) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a34e); };
            c.w32(9, t248);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            return RegfileExit::fault(0x700086a361);
        }
        111 => {
            let Some(t249) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086a362); };
            c.w32(8, t249);
            let r250 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r250);
            let Some(t251) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086a36c); };
            c.w32(8, t251);
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a370); };
            c.w64(9, t252);
            let Some(t253) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086a377); };
            c.w64(11, t253);
            c.w64(15, c.r64(11).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086a37e); }
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a382); };
            c.w64(0, t254);
            let Some(t255) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086a389); };
            c.cmp(t255, c.r64(15), 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(2, 0x14u64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700086a39b, 0x700086a396);
        }
        113 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a39b); };
            c.w64(8, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a39f); };
            c.w64(2, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086a3a3); };
            c.w64(12, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a3a7); };
            c.w64(6, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a3ab); };
            c.w64(3, t260);
            blk = 115; continue;
        }
        114 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700086a3b1); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 115; continue;
        }
        115 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086a3bb); }
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086a3bf); };
            c.w64(8, t261);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086a3c6); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086a3ca); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086a3d1); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086a3d5); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086a3d9); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086a3dd); }
            let Some(t262) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086a3e1); };
            c.w64(8, t262);
            let Some(t263) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700086a3e8); };
            c.w32(8, t263);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700086a3eb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086a3ef); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 116; continue;
        }
        116 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t264) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a407); };
            c.w32(9, t264);
            let r265 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r265);
            let Some(t266) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086a40e); };
            c.w32(9, t266);
            let r267 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r267);
            let Some(t268) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a415); };
            c.w32(11, t268);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            return RegfileExit::fault(0x700086a428);
        }
        119 => {
            let Some(t269) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086a429); };
            c.w32(7, t269);
            let r270 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r270);
            let Some(t271) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a433); };
            c.w32(9, t271);
            let r272 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r272);
            let Some(t273) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086a43a); };
            c.logic(t273 & 0x2u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 120; continue;
        }
        120 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 126; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(9, c.r64(2));
            let Some(t274) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a44d); };
            c.w32(11, t274);
            let r275 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r275);
            let Some(t276) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086a454); };
            c.w32(11, t276);
            c.cmp(c.r16(11), 0x83du64, 16);
            if c.cc_ne() { blk = 127; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w32(11, c.r16(11));
            c.cmp(c.r32(11), 0x83du64, 32);
            if c.cc_ne() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(11, c.r64(9));
            blk = 128; continue;
        }
        124 => {
            return RegfileExit::fault(0x700086a473);
        }
        125 => {
            blk = 127; continue;
        }
        126 => {
            c.w64(9, c.r64(2));
            blk = 127; continue;
        }
        127 => {
            let Some(t277) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086a479); };
            c.w64(11, t277);
            blk = 128; continue;
        }
        128 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086a480); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086a482); }
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086a483); };
            c.w64(11, t278);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086a48a); }
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x70007f2134, 0x700086a499, 0x700086a494);
        }
        129 => {
            blk = 131; continue;
        }
        130 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086a49b); }
            let Some(t279) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086a49c); };
            c.w64(3, t279);
            c.w32(0, 0x1u64);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a4a8); };
            c.w64(6, t280);
            return c.call(0x700086ed74, 0x700086a4b1, 0x700086a4ac);
        }
        131 => {
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a4b1); };
            c.w64(2, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086a4b5); };
            c.w64(12, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a4b9); };
            c.w64(6, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a4bd); };
            c.w64(3, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a4c1); };
            c.w64(0, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086a4c5); };
            c.w64(8, t286);
            blk = 132; continue;
        }
        132 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 133; continue;
        }
        133 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086a4cf); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086a4d0); }
            c.w32(0, 0x2u64);
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086a4d6); };
            c.w64(3, t287);
            c.w64(8, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700086a4e8, 0x700086a4e3);
        }
        134 => {
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a4e8); };
            c.w64(2, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086a4ec); };
            c.w64(12, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a4f0); };
            c.w64(6, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a4f4); };
            c.w64(3, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086a4f8); };
            c.w64(0, t292);
            blk = 135; continue;
        }
        135 => {
            c.w64(8, c.r64(0));
            blk = 57; continue;
        }
        136 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086a504); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086a505); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086a506); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700086a508); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086a50a); }
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0x25a8)) else { return RegfileExit::fault(0x700086a50b); };
            c.w64(3, t293);
            c.w32(0, 0x5u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700086a51f, 0x700086a51a);
        }
        137 => {
            c.w64(8, c.r64(0));
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086a522); };
            c.w64(2, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086a526); };
            c.w64(6, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086a52a); };
            c.w64(3, t296);
            blk = 57; continue;
        }
        138 => {
            return RegfileExit::jump(0x700086a534);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086a554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086a554); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086a558); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086a55e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086a565); };
            c.w32(9, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700086a569); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086a56d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700086a571); }
            c.cmp(c.r32(9), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1058)) else { return RegfileExit::fault(0x700086a57b); };
            c.w64(9, t4);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x1070)) else { return RegfileExit::fault(0x700086a582); };
            c.w64(11, t5);
            blk = 3; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1078)) else { return RegfileExit::fault(0x700086a58b); };
            c.w64(9, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1080)) else { return RegfileExit::fault(0x700086a592); };
            c.w64(11, t7);
            blk = 3; continue;
        }
        3 => {
            c.w32(12, 0x2u64);
            if !c.st32(c.r64(0).wrapping_add(0x2b), c.r32(12)) { return RegfileExit::fault(0x700086a59f); }
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086a5a3); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700086a5aa); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a5b7); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086a5be); };
            c.cmp(t12, 0x83du64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a5cc); };
            c.w32(15, t13);
            let r14 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r14);
            let Some(t15) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086a5d3); };
            c.w32(15, t15);
            let r16 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r16);
            let Some(t17) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a5da); };
            c.w32(2, t17);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x97u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x700086a5ed);
        }
        7 => {
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x263)) else { return RegfileExit::fault(0x700086a5ee); };
            c.w32(2, t18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086a5f5); };
            c.cmp(t19, c.r32(2), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x700086a600); };
            c.w64(12, t20);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086a609); };
            c.cmp(t21, c.r32(2), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t22) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a614); };
            c.w32(12, t22);
            let r23 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a61b); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086a622); };
            c.w64(2, t25);
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086a625); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w64(7, c.r64(2).wrapping_add(0x14));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a631); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086a638); };
            c.cmp(t29, c.r64(7), 64);
            if c.cc_be() { blk = 69; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x700086a641); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 11; continue;
        }
        11 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a648); };
            c.w32(2, t30);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x7du64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700086a659);
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700086a65a); };
            c.w32(2, t31);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700086a662); }
            c.w32(2, 0x6u64);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700086a66b); }
            let Some(t32) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a66f); };
            c.w32(2, t32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x700086a680);
        }
        15 => {
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086a681); };
            c.w32(2, t33);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700086a686); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700086a68a); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700086a68e); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a692); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086a699); };
            c.w64(2, t35);
            c.w64(1, c.r64(2).wrapping_add(0x1c));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a6a0); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086a6a7); };
            c.cmp(t37, c.r64(1), 64);
            if c.cc_be() { blk = 71; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700086a6b0); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 17; continue;
        }
        17 => {
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a6b7); };
            c.w32(2, t38);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700086a6cb);
        }
        19 => {
            let Some(t39) = c.ld32(c.r64(12).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086a6cc); };
            c.w32(2, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700086a6d4); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086a6d7); };
            c.w64(2, t40);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700086a6de); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700086a6e1); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086a6e4); };
            c.w64(2, t41);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700086a6eb); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700086a6ee); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x700086a6f2); }
            let Some(t42) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a6f6); };
            c.w32(11, t42);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t43) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x700086a704); };
            c.w64(11, t43);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x700086a708); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a70c); };
            c.w64(11, t44);
            let Some(t45) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086a713); };
            c.w64(2, t45);
            c.w64(1, c.r64(2).wrapping_add(0x1c));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a71a); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086a721); };
            c.cmp(t47, c.r64(1), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700086a72a); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            blk = 21; continue;
        }
        21 => {
            let Some(t48) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a731); };
            c.w32(2, t48);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x700086a745);
        }
        23 => {
            let Some(t49) = c.ld32(c.r64(12).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086a746); };
            c.w32(2, t49);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700086a74e); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086a752); };
            c.w64(2, t50);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700086a759); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700086a75d); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086a761); };
            c.w64(2, t51);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700086a768); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086a76c); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x700086a770); }
            let Some(t52) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a774); };
            c.w32(9, t52);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(13).wrapping_add(0x63f0));
            let Some(t53) = c.ld64(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x700086a782); };
            c.w64(9, t53);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x700086a786); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086a78a); };
            c.w64(9, t54);
            let Some(t55) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700086a791); };
            c.w32(9, t55);
            c.logic(c.r8(9) & 0xeu64, 8);
            if c.cc_ne() { blk = 100; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086a7a8); };
            c.w64(8, t56);
            blk = 26; continue;
        }
        26 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w64(1, c.r64(11));
            c.w64(2, c.r64(8));
            c.w64(6, c.r64(12));
            return c.call(0x70008b2ef4, 0x700086a7c6, 0x700086a7c1);
        }
        27 => {
            blk = 62; continue;
        }
        28 => {
            let Some(t57) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a7cb); };
            c.w32(12, t57);
            let r58 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r58);
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086a7d2); };
            c.w32(12, t59);
            let r60 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r60);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a7da); };
            c.w64(15, t61);
            let Some(t62) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086a7e1); };
            c.w64(2, t62);
            c.w64(7, c.r64(2).wrapping_add(0x14));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a7e8); };
            c.w64(3, t63);
            let Some(t64) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700086a7ef); };
            c.cmp(t64, c.r64(7), 64);
            if c.cc_be() { blk = 102; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x700086a7f8); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 30; continue;
        }
        30 => {
            let Some(t65) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a7ff); };
            c.w32(3, t65);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x7du64, 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x700086a810);
        }
        32 => {
            let Some(t66) = c.ld32(c.r64(12).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700086a811); };
            c.w32(3, t66);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700086a819); }
            c.w32(3, 0x6u64);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700086a822); }
            let Some(t67) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a826); };
            c.w32(3, t67);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x700086a837);
        }
        34 => {
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086a838); };
            c.w32(3, t68);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700086a83d); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700086a841); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700086a845); }
            let Some(t69) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a849); };
            c.w32(0, t69);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xeau64, 64);
            if c.cc_a() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            return RegfileExit::fault(0x700086a85c);
        }
        36 => {
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086a85d); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a868); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700086a86f); };
            c.w64(2, t73);
            let Some(t74) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086a872); };
            c.w32(0, t74);
            c.w64(7, c.r64(2).wrapping_add(0x14));
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a879); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086a880); };
            c.cmp(t76, c.r64(7), 64);
            if c.cc_be() { blk = 104; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x700086a889); }
            let r77 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r77);
            blk = 38; continue;
        }
        38 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700086a890); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086a893); };
            c.w64(0, t78);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700086a89a); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700086a89d); }
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700086a8a2); }
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700086a8a5); }
            let Some(t79) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700086a8a8); };
            c.w64(0, t79);
            c.w64(7, c.r64(0).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700086a8af); }
            let Some(t80) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086a8b3); };
            c.cmp(t80, c.r64(7), 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x700086a8bc); }
            let r81 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r81);
            blk = 40; continue;
        }
        40 => {
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a8c3); };
            c.w32(3, t82);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700086a8ce); }
            c.cmp(c.r64(3), 0xbeu64, 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x700086a8db);
        }
        42 => {
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086a8dc); };
            c.w32(3, t83);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700086a8e4); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086a8e7); };
            c.w64(3, t84);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700086a8ee); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700086a8f1); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086a8f4); };
            c.w64(3, t85);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x700086a8fb); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700086a8fe); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x700086a902); }
            let Some(t86) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a906); };
            c.w32(11, t86);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(13).wrapping_add(0x63f0));
            let Some(t87) = c.ld64(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x700086a914); };
            c.w64(11, t87);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x700086a918); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086a91c); };
            c.w64(11, t88);
            let Some(t89) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086a923); };
            c.w64(3, t89);
            c.w64(1, c.r64(3).wrapping_add(0x1c));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086a92a); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086a931); };
            c.cmp(t91, c.r64(1), 64);
            if c.cc_be() { blk = 108; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700086a93a); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 44; continue;
        }
        44 => {
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a941); };
            c.w32(3, t92);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700086a94c); }
            c.cmp(c.r64(3), 0xbeu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700086a959);
        }
        46 => {
            let Some(t93) = c.ld32(c.r64(12).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086a95a); };
            c.w32(12, t93);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086a962); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086a966); };
            c.w64(12, t94);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086a96d); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086a971); }
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086a975); };
            c.w64(12, t95);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x700086a97c); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086a980); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x700086a984); }
            let Some(t96) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a988); };
            c.w32(9, t96);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(13).wrapping_add(0x63f0));
            let Some(t97) = c.ld64(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x700086a996); };
            c.w64(9, t97);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x700086a99a); }
            let Some(t98) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a99e); };
            c.w32(9, t98);
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            let Some(t100) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086a9a5); };
            c.w32(9, t100);
            let r101 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r101);
            let Some(t102) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086a9ac); };
            c.w32(12, t102);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x8eu64, 64);
            if c.cc_a() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            return RegfileExit::fault(0x700086a9bf);
        }
        48 => {
            let Some(t103) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086a9c0); };
            c.w32(7, t103);
            let r104 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r104);
            let Some(t105) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a9ca); };
            c.w32(9, t105);
            let r106 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r106);
            let Some(t107) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086a9d1); };
            c.logic(t107 & 0x2u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 49; continue;
        }
        49 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t108) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086a9de); };
            c.w32(9, t108);
            let r109 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r109);
            let Some(t110) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086a9e5); };
            c.w32(9, t110);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, c.r64(8));
            blk = 55; continue;
        }
        53 => {
            return RegfileExit::fault(0x700086aa04);
        }
        54 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086aa05); };
            c.w64(9, t111);
            blk = 55; continue;
        }
        55 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086aa0c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086aa0e); }
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086aa0f); };
            c.w64(9, t112);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086aa16); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700086aa25, 0x700086aa20);
        }
        56 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086aa25); };
            c.w64(2, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086aa29); };
            c.w64(0, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086aa2d); };
            c.w64(11, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086aa31); };
            c.w64(6, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086aa35); };
            c.w64(8, t117);
            blk = 57; continue;
        }
        57 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086aa39); };
            c.w64(9, t118);
            let Some(t119) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700086aa40); };
            c.w32(9, t119);
            c.logic(c.r8(9) & 0xeu64, 8);
            if c.cc_ne() { blk = 139; continue; }
            blk = 58; continue;
        }
        58 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 112; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086aa57); };
            c.w64(8, t120);
            blk = 60; continue;
        }
        60 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700086aa5e); }
            c.w64(0, c.r64(2));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086aa65); };
            c.w64(3, t121);
            return c.call(0x70008ba734, 0x700086aa6e, 0x700086aa69);
        }
        61 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086aa6e); };
            c.w64(0, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086aa72); };
            c.w64(3, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086aa76); };
            c.w64(1, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086aa7a); };
            c.w64(2, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086aa7e); };
            c.w64(6, t126);
            return c.call(0x70008b2ef4, 0x700086aa87, 0x700086aa82);
        }
        62 => {
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086aa87); };
            c.w64(0, t127);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086aa91); }
            return c.ret(0x0, 0x700086aa92);
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700086aa93); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700086aa97); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086aa9b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700086aa9f); }
            c.w64(0, c.r64(3));
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x700086aaa6); };
            c.w64(3, t128);
            c.w64(12, c.r64(3));
            return c.call(0x7000870914, 0x700086aab5, 0x700086aab0);
        }
        64 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086aab5); };
            c.w64(15, t129);
            let Some(t130) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086aab9); };
            c.w32(8, t130);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            return RegfileExit::fault(0x700086aacc);
        }
        66 => {
            let Some(t131) = c.ld32(c.r64(15).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086aacd); };
            c.cmp(t131, c.r32(0), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086aad6); };
            c.w64(6, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086aada); };
            c.w64(8, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086aade); };
            c.w64(1, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086aae2); };
            c.w64(9, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086aae6); };
            c.w64(11, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086aaea); };
            c.w64(0, t137);
            blk = 28; continue;
        }
        68 => {
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086aaf3); };
            c.w64(6, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086aaf7); };
            c.w64(3, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086aafb); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086aaff); };
            c.w64(9, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ab03); };
            c.w64(11, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086ab07); };
            c.w64(0, t143);
            blk = 9; continue;
        }
        69 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700086ab15); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700086ab19); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086ab1d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700086ab21); }
            return c.call(0x70007febf4, 0x700086ab2a, 0x700086ab25);
        }
        70 => {
            c.w64(15, c.r64(0));
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ab2d); };
            c.w64(6, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ab31); };
            c.w64(3, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ab35); };
            c.w64(8, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086ab39); };
            c.w64(9, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ab3d); };
            c.w64(11, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086ab41); };
            c.w64(0, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086ab45); };
            c.w64(12, t150);
            blk = 11; continue;
        }
        71 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700086ab53); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700086ab57); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700086ab5b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700086ab5f); }
            return c.call(0x70007febf4, 0x700086ab68, 0x700086ab63);
        }
        72 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086ab68); };
            c.w64(15, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ab6c); };
            c.w64(6, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ab70); };
            c.w64(3, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ab74); };
            c.w64(8, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086ab78); };
            c.w64(9, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ab7c); };
            c.w64(11, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086ab80); };
            c.w64(12, t157);
            blk = 17; continue;
        }
        73 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700086ab8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086ab92); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700086ab96); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700086ab9a); }
            return c.call(0x70007febf4, 0x700086aba3, 0x700086ab9e);
        }
        74 => {
            c.w64(11, c.r64(0));
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086aba6); };
            c.w64(6, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086abaa); };
            c.w64(3, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086abae); };
            c.w64(8, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086abb2); };
            c.w64(9, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086abb6); };
            c.w64(0, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086abba); };
            c.w64(12, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086abbe); };
            c.w64(15, t164);
            blk = 21; continue;
        }
        75 => {
            let Some(t165) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086abc7); };
            c.w32(8, t165);
            let r166 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r166);
            let Some(t167) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086abce); };
            c.w32(8, t167);
            let r168 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r168);
            let Some(t169) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086abd5); };
            c.w32(9, t169);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086abdf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700086abe3); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700086abe7); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x700086abf4);
        }
        77 => {
            let Some(t170) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086abf5); };
            c.w32(8, t170);
            let r171 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r171);
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086abff); };
            c.w64(9, t172);
            let Some(t173) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086ac06); };
            c.w64(15, t173);
            let Some(t174) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086ac09); };
            c.w32(8, t174);
            c.w64(2, c.r64(15).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700086ac11); }
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086ac15); };
            c.w64(1, t175);
            let Some(t176) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086ac1c); };
            c.cmp(t176, c.r64(2), 64);
            if c.cc_a() { blk = 80; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700086ac2b, 0x700086ac26);
        }
        79 => {
            blk = 81; continue;
        }
        80 => {
            if !c.st64(c.r64(9), c.r64(2)) { return RegfileExit::fault(0x700086ac2d); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 81; continue;
        }
        81 => {
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ac37); };
            c.w64(8, t177);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086ac3b); }
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086ac3f); };
            c.w64(8, t178);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086ac46); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086ac4a); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086ac51); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086ac55); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086ac59); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086ac5d); }
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086ac61); };
            c.w64(8, t179);
            let Some(t180) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700086ac68); };
            c.w32(8, t180);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086ac6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700086ac6f); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 82; continue;
        }
        82 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ac83); };
            c.w64(6, t181);
            blk = 96; continue;
        }
        84 => {
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ac8c); };
            c.w64(6, t182);
            let Some(t183) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086ac90); };
            c.w32(9, t183);
            let r184 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r184);
            let Some(t185) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086ac97); };
            c.w32(9, t185);
            let r186 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r186);
            let Some(t187) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086ac9e); };
            c.w32(11, t187);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            return RegfileExit::fault(0x700086acb1);
        }
        86 => {
            let Some(t188) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086acb2); };
            c.w32(7, t188);
            let r189 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r189);
            let Some(t190) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086acbc); };
            c.w32(9, t190);
            let r191 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r191);
            let Some(t192) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086acc3); };
            c.logic(t192 & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086acce); };
            c.w64(3, t193);
            let Some(t194) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086acd2); };
            c.w32(9, t194);
            let r195 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r195);
            let Some(t196) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086acd9); };
            c.w32(9, t196);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 91; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(9, c.r64(3));
            blk = 92; continue;
        }
        90 => {
            return RegfileExit::fault(0x700086acf8);
        }
        91 => {
            let Some(t197) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086acf9); };
            c.w64(9, t197);
            blk = 92; continue;
        }
        92 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086ad00); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086ad02); }
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086ad03); };
            c.w64(9, t198);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086ad0a); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700086ad19, 0x700086ad14);
        }
        93 => {
            blk = 95; continue;
        }
        94 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086ad1b); }
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ad1c); };
            c.w64(6, t199);
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086ad20); };
            c.w64(3, t200);
            c.w32(0, 0x1u64);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700086ad34, 0x700086ad2f);
        }
        95 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ad34); };
            c.w64(6, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ad38); };
            c.w64(0, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ad3c); };
            c.w64(8, t203);
            blk = 96; continue;
        }
        96 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086ad46); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ad47); };
            if !c.push64(t204) { return RegfileExit::fault(0x700086ad47); }
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086ad4a); };
            c.w64(3, t205);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086ad5e, 0x700086ad59);
        }
        98 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ad5e); };
            c.w64(0, t206);
            blk = 99; continue;
        }
        99 => {
            c.w64(8, c.r64(0));
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ad65); };
            c.w64(3, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086ad69); };
            c.w64(0, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086ad6d); };
            c.w64(11, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086ad71); };
            c.w64(12, t210);
            blk = 26; continue;
        }
        100 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086ad7a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086ad7b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086ad7c); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086ad7e); }
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086ad80); };
            if !c.push64(t211) { return RegfileExit::fault(0x700086ad80); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086ad83); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700086ad87); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700086ad8b); }
            c.w32(0, 0x5u64);
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x25a0)) else { return RegfileExit::fault(0x700086ad94); };
            c.w64(3, t212);
            c.w64(8, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700086ada6, 0x700086ada1);
        }
        101 => {
            c.w64(8, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ada9); };
            c.w64(3, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086adad); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086adb1); };
            c.w64(11, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086adb5); };
            c.w64(12, t216);
            blk = 26; continue;
        }
        102 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700086adc3); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700086adc7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700086adcb); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086adcf); }
            return c.call(0x70007febf4, 0x700086add8, 0x700086add3);
        }
        103 => {
            c.w64(15, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086addb); };
            c.w64(6, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086addf); };
            c.w64(8, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086ade3); };
            c.w64(1, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086ade7); };
            c.w64(9, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086adeb); };
            c.w64(12, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086adef); };
            c.w64(11, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086adf3); };
            c.w64(0, t223);
            blk = 30; continue;
        }
        104 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700086ae01); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700086ae05); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700086ae09); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700086ae0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700086ae11); }
            return c.call(0x70007febf4, 0x700086ae1a, 0x700086ae15);
        }
        105 => {
            c.w64(2, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ae1d); };
            c.w64(6, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ae21); };
            c.w64(8, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086ae25); };
            c.w64(9, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086ae29); };
            c.w64(15, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086ae2d); };
            c.w64(12, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ae31); };
            c.w64(11, t229);
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086ae35); };
            c.w64(3, t230);
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086ae3c); };
            c.w64(1, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086ae43); };
            c.w64(0, t232);
            blk = 38; continue;
        }
        106 => {
            c.w32(0, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700086ae51); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700086ae55); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700086ae59); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700086ae5d); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700086ae69, 0x700086ae64);
        }
        107 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ae69); };
            c.w64(11, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086ae6d); };
            c.w64(2, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ae71); };
            c.w64(6, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ae75); };
            c.w64(8, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086ae79); };
            c.w64(9, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086ae7d); };
            c.w64(15, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086ae81); };
            c.w64(12, t239);
            blk = 40; continue;
        }
        108 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700086ae8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700086ae93); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700086ae97); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700086aea3, 0x700086ae9e);
        }
        109 => {
            c.w64(11, c.r64(0));
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086aea6); };
            c.w64(2, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086aeaa); };
            c.w64(0, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086aeae); };
            c.w64(6, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086aeb2); };
            c.w64(8, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086aeb6); };
            c.w64(9, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086aeba); };
            c.w64(15, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086aebe); };
            c.w64(12, t246);
            blk = 44; continue;
        }
        110 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086aec7); }
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086aec8); };
            c.w64(3, t247);
            c.w32(0, 0x1u64);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086aed4); };
            c.w64(6, t248);
            return c.call(0x700086ed74, 0x700086aedd, 0x700086aed8);
        }
        111 => {
            blk = 56; continue;
        }
        112 => {
            let Some(t249) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086aee2); };
            c.w32(8, t249);
            let r250 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r250);
            let Some(t251) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086aee9); };
            c.w32(8, t251);
            let r252 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r252);
            let Some(t253) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086aef0); };
            c.w32(9, t253);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            return RegfileExit::fault(0x700086af03);
        }
        114 => {
            let Some(t254) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086af04); };
            c.w32(8, t254);
            let r255 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r255);
            let Some(t256) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086af0e); };
            c.w32(8, t256);
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086af12); };
            c.w64(9, t257);
            let Some(t258) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086af19); };
            c.w64(12, t258);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086af21); }
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086af25); };
            c.w64(3, t259);
            let Some(t260) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700086af2c); };
            c.cmp(t260, c.r64(15), 64);
            if c.cc_a() { blk = 117; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w32(2, 0x14u64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700086af3e, 0x700086af39);
        }
        116 => {
            c.w64(9, c.r64(0));
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086af41); };
            c.w64(2, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086af45); };
            c.w64(0, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086af49); };
            c.w64(11, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086af4d); };
            c.w64(6, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086af51); };
            c.w64(8, t265);
            blk = 118; continue;
        }
        117 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700086af57); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 118; continue;
        }
        118 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086af5f); }
            let Some(t266) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086af63); };
            c.w64(8, t266);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086af6a); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086af6e); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086af75); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086af79); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086af7d); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086af81); }
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086af85); };
            c.w64(8, t267);
            let Some(t268) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700086af8c); };
            c.w32(8, t268);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700086af8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086af93); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 119; continue;
        }
        119 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t269) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086afab); };
            c.w32(12, t269);
            let r270 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r270);
            let Some(t271) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086afb2); };
            c.w32(12, t271);
            let r272 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r272);
            let Some(t273) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086afba); };
            c.w32(15, t273);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x8eu64, 64);
            if c.cc_a() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            return RegfileExit::fault(0x700086afce);
        }
        122 => {
            let Some(t274) = c.ld32(c.r64(12).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086afcf); };
            c.w32(7, t274);
            let r275 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r275);
            let Some(t276) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086afda); };
            c.w32(12, t276);
            let r277 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r277);
            let Some(t278) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086afe1); };
            c.logic(t278 & 0x2u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 123; continue;
        }
        123 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 129; continue; }
            blk = 124; continue;
        }
        124 => {
            c.w64(12, c.r64(2));
            let Some(t279) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086aff5); };
            c.w32(15, t279);
            let r280 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r280);
            let Some(t281) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086affd); };
            c.w32(15, t281);
            c.cmp(c.r16(15), 0x83du64, 16);
            if c.cc_ne() { blk = 130; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w32(15, c.r16(15));
            c.cmp(c.r32(15), 0x83du64, 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(15, c.r64(12));
            blk = 131; continue;
        }
        127 => {
            return RegfileExit::fault(0x700086b01c);
        }
        128 => {
            blk = 130; continue;
        }
        129 => {
            c.w64(12, c.r64(2));
            blk = 130; continue;
        }
        130 => {
            let Some(t282) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b022); };
            c.w64(15, t282);
            blk = 131; continue;
        }
        131 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700086b029); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086b02b); }
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b02d); };
            c.w64(15, t283);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700086b034); }
            c.w32(0, 0x2u64);
            c.w64(15, c.r64(0));
            return c.call(0x70007f2134, 0x700086b043, 0x700086b03e);
        }
        132 => {
            blk = 134; continue;
        }
        133 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b045); }
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086b046); };
            c.w64(3, t284);
            c.w32(0, 0x1u64);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b052); };
            c.w64(6, t285);
            return c.call(0x700086ed74, 0x700086b05b, 0x700086b056);
        }
        134 => {
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086b05b); };
            c.w64(2, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086b05f); };
            c.w64(0, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b063); };
            c.w64(11, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b067); };
            c.w64(6, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b06b); };
            c.w64(9, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b06f); };
            c.w64(8, t291);
            blk = 135; continue;
        }
        135 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086b079); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086b07b); }
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086b07c); };
            c.w64(3, t292);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086b090, 0x700086b08b);
        }
        137 => {
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086b090); };
            c.w64(2, t293);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086b094); };
            c.w64(0, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b098); };
            c.w64(11, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b09c); };
            c.w64(6, t296);
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b0a0); };
            c.w64(9, t297);
            blk = 138; continue;
        }
        138 => {
            c.w64(8, c.r64(9));
            blk = 60; continue;
        }
        139 => {
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b0ac); };
            if !c.push64(t298) { return RegfileExit::fault(0x700086b0ac); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086b0af); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086b0b0); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086b0b2); }
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b0b4); };
            if !c.push64(t299) { return RegfileExit::fault(0x700086b0b4); }
            let Some(t300) = c.ld64(c.r64(13).wrapping_add(0x25a8)) else { return RegfileExit::fault(0x700086b0b7); };
            c.w64(3, t300);
            c.w32(0, 0x5u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086b0cb, 0x700086b0c6);
        }
        140 => {
            c.w64(8, c.r64(0));
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086b0ce); };
            c.w64(2, t301);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b0d2); };
            c.w64(6, t302);
            blk = 60; continue;
        }
        141 => {
            return RegfileExit::jump(0x700086b0dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086b0f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086b0f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086b0f8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086b0fe); };
            c.w32(7, t1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086b101); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086b108); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700086b10c); }
            blk = 2; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b112); };
            c.w64(6, t4);
            blk = 2; continue;
        }
        2 => {
            c.w32(9, 0x2u64);
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700086b11c); };
            c.cmp(t5, c.r32(9), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b126); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086b133); };
            c.w32(11, t7);
            c.w32(12, c.r32(7));
            c.w32(15, c.r32(11));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(7), 0xfffffffffffffffeu64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086b148); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b14f); };
            c.w64(3, t9);
            return c.call(0x7000869834, 0x700086b15b, 0x700086b156);
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086b15b); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086b15f); };
            c.w32(7, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086b162); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            c.w64(0, c.r64(2));
            blk = 1; continue;
        }
        8 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(7, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(0).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700086b17b); }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086b17e); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700086b185); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700086b189); }
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(7), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st32(c.r64(0).wrapping_add(0x1b), c.r32(11)) { return RegfileExit::fault(0x700086b198); }
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086b19c); };
            c.w32(7, t16);
            let r17 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r17);
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(7));
            return c.call(0x70007fa8f4, 0x700086b1ae, 0x700086b1a9);
        }
        13 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086b1ae); };
            c.w64(0, t18);
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086b1b2); };
            c.w32(7, t19);
            let Some(t20) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086b1b5); };
            c.w32(8, t20);
            let r21 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r21);
            blk = 1; continue;
        }
        14 => {
            return c.call(0x7000869974, 0x700086b1c6, 0x700086b1c1);
        }
        15 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086b1c6); };
            c.w64(0, t22);
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086b1ca); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b1d1); };
            c.w64(7, t25);
            blk = 1; continue;
        }
        16 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b1da); }
            c.w32(0, 0x1u64);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1fd0)) else { return RegfileExit::fault(0x700086b1e0); };
            c.w64(3, t26);
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086b1f2, 0x700086b1ed);
        }
        17 => {
            let Some(t27) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700086b1f2); };
            c.w64(5, t27);
            c.w64(1, c.r64(0));
            let Some(t28) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086b1f9); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700086b1fe); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086b203); };
            c.w64(6, t30);
            let r31 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(0x700086a554);
        }
        18 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b211); };
            c.w64(0, t32);
            blk = 19; continue;
        }
        19 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086b21b); }
            return c.ret(0x0, 0x700086b21c);
        }
        20 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b21d); };
            c.w64(0, t33);
            blk = 19; continue;
        }
        21 => {
            return RegfileExit::jump(0x700086b228);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086b234(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c00)) else { return RegfileExit::fault(0x700086b234); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700086b254(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086b254); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086b258); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086b259); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b25a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700086b25f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086b263); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086b277); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700086b27f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700086b283); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086b28d); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b293); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b29a); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086b2a1); };
            c.w32(12, t7);
            let r8 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086b2a9); };
            c.w32(15, t9);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086b2b4); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700086b2c1);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086b2c2); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086b2cd); };
            c.w64(15, t12);
            let Some(t13) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086b2d4); };
            c.w64(0, t13);
            let Some(t14) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086b2d7); };
            c.w32(12, t14);
            c.w64(3, c.r64(0).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086b2e0); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086b2e7); };
            c.cmp(t16, c.r64(3), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700086b2f0); }
            let r17 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r17);
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086b2f7); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086b2fb); };
            c.w64(12, t18);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086b302); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086b306); }
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700086b30d); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086b311); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700086b315); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086b319); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086b31d); };
            c.w64(3, t19);
            let Some(t20) = c.ld32(c.r64(3)) else { return RegfileExit::fault(0x700086b324); };
            c.w32(3, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086b326); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700086b32a); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b340); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086b346); };
            c.w32(1, t23);
            let r24 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r24);
            let Some(t25) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086b34c); };
            c.w32(7, t25);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x8eu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700086b35e);
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086b35f); };
            c.w32(7, t26);
            let r27 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r27);
            let Some(t28) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b368); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086b36e); };
            c.logic(t30 & 0x2u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b374); };
            c.w64(1, t31);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b380); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086b387); };
            c.w32(8, t34);
            c.cmp(c.r16(8), 0x83du64, 16);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(8, c.r16(8));
            c.cmp(c.r32(8), 0x83du64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700086b3a1);
        }
        17 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086b3a2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b3a3); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700086b3a4); }
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x700086b3b2, 0x700086b3ad);
        }
        18 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b3b2); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b3b6); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b3ba); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b3be); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b3c2); };
            c.w64(8, t39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086b3c6); };
            c.w64(15, t40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086b3cd); };
            c.w64(2, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086b3d4); };
            c.w64(3, t42);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(3) & 0x6u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b3e0); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b3e1); };
            c.w64(3, t43);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086b3e8); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086b3e9); };
            c.w64(3, t44);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700086b3fd, 0x700086b3f8);
        }
        21 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b3fd); };
            c.w64(0, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b401); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b405); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b409); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b40d); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086b411); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086b418); };
            c.w64(2, t51);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b42c); };
            c.w32(3, t52);
            let r53 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r53);
            let Some(t54) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086b433); };
            c.cmp(t54, 0x82cu64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t55) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086b43f); };
            c.w64(3, t55);
            c.w64(1, c.r64(3).wrapping_add(0x14));
            let Some(t56) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086b446); };
            c.cmp(t56, c.r64(1), 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x700086b44f); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xd90)) else { return RegfileExit::fault(0x700086b456); };
            c.w64(3, t57);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700086b45d); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b461); };
            c.w64(3, t58);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700086b468); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086b46c); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086b470); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700086b474); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700086b478); }
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086b47c); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b483); };
            c.cmp(t61, c.r32(8), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 29; continue;
        }
        28 => {
            c.w64(8, c.r64(12));
            blk = 29; continue;
        }
        29 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086b491); };
            c.w32(12, t62);
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b498); };
            c.cmp(t64, c.r32(12), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086b4a1); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r65 = c.r64(12) & c.r64(8);
            c.w64(12, r65);
            c.logic(r65, 64);
            let Some(t66) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086b4b5); };
            c.logic(t66 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 35; continue;
        }
        33 => {
            if !c.st32(c.r64(9).wrapping_add(0x27), c.r32(15)) { return RegfileExit::fault(0x700086b4c3); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r67 = c.r64(8) & c.r64(9);
            c.w64(8, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086b4d7); };
            c.logic(t68 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086b4e2); };
            c.w32(8, t69);
            c.cmp(c.r32(8), 0xfffffffffffffffcu64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, c.r64(9));
            return c.call(0x700086b0f4, 0x700086b4f4, 0x700086b4ef);
        }
        37 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b4f4); };
            c.w64(0, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b4f8); };
            c.w64(11, t71);
            blk = 38; continue;
        }
        38 => {
            c.w64(8, c.r64(11));
            let r72 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r72);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086b506); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086b507); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086b50d); }
            return c.ret(0x0, 0x700086b50f);
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086b510); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086b514); };
            c.w64(3, t73);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086b525, 0x700086b520);
        }
        40 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b525); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b529); };
            c.w64(0, t75);
            blk = 1; continue;
        }
        41 => {
            c.w32(0, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086b537); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700086b543, 0x700086b53e);
        }
        42 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086b543); };
            c.w64(12, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b547); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b54b); };
            c.w64(6, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b54f); };
            c.w64(9, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b553); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086b557); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086b55e); };
            c.w64(2, t82);
            blk = 8; continue;
        }
        43 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b56a); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086b56b); };
            c.w64(3, t83);
            c.w32(0, 0x1u64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b577); };
            c.w64(6, t84);
            return c.call(0x700086ed74, 0x700086b580, 0x700086b57b);
        }
        44 => {
            blk = 18; continue;
        }
        45 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700086b58f, 0x700086b58a);
        }
        46 => {
            c.w64(15, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b592); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b596); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b59a); };
            c.w64(6, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b59e); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b5a2); };
            c.w64(8, t89);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        47 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086b5b4); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086b5b6); };
            c.w64(8, t90);
            let Some(t91) = c.ld32(c.r64(8).wrapping_add(0xf3)) else { return RegfileExit::fault(0x700086b5bd); };
            c.w32(8, t91);
            let r92 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r92);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086b5c7); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086b5c9); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086b5cb); };
            c.w64(3, t93);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086b5df, 0x700086b5da);
        }
        48 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086b5df); };
            c.w64(1, t94);
            c.w64(3, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b5e9); };
            c.w64(0, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b5ed); };
            c.w64(6, t96);
            return c.call(0x70008b1fb4, 0x700086b5f6, 0x700086b5f1);
        }
        49 => {
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b5f6); };
            c.w64(8, t97);
            let r98 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b5fe); };
            c.w64(0, t99);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086b605); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086b606); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086b60c); }
            return c.ret(0x0, 0x700086b60e);
        }
        50 => {
            c.w32(15, c.r32(15));
            let r100 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r100);
            c.w64(8, 0xfffffffffffc0000u64);
            let r101 = c.r64(8) & c.r64(15);
            c.w64(8, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086b61f); };
            c.logic(t102 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(12, c.r64(9).wrapping_add(0x27));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086b62e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086b62f); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700086b63c, 0x700086b637);
        }
        52 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700086b63c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700086b63e); }
            blk = 35; continue;
        }
        53 => {
            c.w32(15, c.r32(15));
            let r103 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r103);
            c.w64(12, 0xfffffffffffc0000u64);
            let r104 = c.r64(12) & c.r64(15);
            c.w64(12, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086b654); };
            c.logic(t105 & 0x2u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(3, c.r64(8).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086b664); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086b665); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x700086b672, 0x700086b66d);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700086b672); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700086b674); }
            blk = 32; continue;
        }
        56 => {
            return RegfileExit::jump(0x700086b67c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086b694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086b694); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086b698); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086b699); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b69a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700086b69f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086b6a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086b6b7); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700086b6bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700086b6c3); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086b6cd); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b6d3); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b6da); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086b6e1); };
            c.w32(12, t7);
            let r8 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086b6e9); };
            c.w32(15, t9);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086b6f4); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700086b701);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086b702); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086b70d); };
            c.w64(15, t12);
            let Some(t13) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086b714); };
            c.w64(0, t13);
            let Some(t14) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086b717); };
            c.w32(12, t14);
            c.w64(3, c.r64(0).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086b720); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086b727); };
            c.cmp(t16, c.r64(3), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700086b730); }
            let r17 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r17);
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086b737); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086b73b); };
            c.w64(12, t18);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086b742); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086b746); }
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700086b74d); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086b751); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700086b755); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086b759); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086b75d); };
            c.w64(12, t19);
            let Some(t20) = c.ld32(c.r64(12)) else { return RegfileExit::fault(0x700086b764); };
            c.w32(12, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086b768); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086b76c); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b784); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086b78a); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086b790); };
            c.w32(1, t25);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x8eu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700086b7a2);
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086b7a3); };
            c.w32(7, t26);
            let r27 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r27);
            let Some(t28) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b7ac); };
            c.w32(3, t28);
            let r29 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r29);
            let Some(t30) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086b7b2); };
            c.logic(t30 & 0x2u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b7b8); };
            c.w64(3, t31);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b7c4); };
            c.w32(1, t32);
            let r33 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r33);
            let Some(t34) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086b7ca); };
            c.w32(1, t34);
            c.cmp(c.r16(1), 0x83du64, 16);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x83du64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700086b7e0);
        }
        17 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086b7e1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b7e2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086b7e3); }
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2134, 0x700086b7f1, 0x700086b7ec);
        }
        18 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b7f1); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b7f5); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b7f9); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b7fd); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b801); };
            c.w64(8, t39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086b805); };
            c.w64(15, t40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086b80c); };
            c.w64(2, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086b813); };
            c.w64(12, t42);
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(12) & 0x6u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b81d); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b81e); };
            c.w64(12, t43);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700086b825); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086b827); };
            c.w64(3, t44);
            c.w32(0, 0x2u64);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700086b83b, 0x700086b836);
        }
        21 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b83b); };
            c.w64(0, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b83f); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b843); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b847); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b84b); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086b84f); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086b856); };
            c.w64(2, t51);
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086b867); };
            c.w32(12, t52);
            let r53 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r53);
            let Some(t54) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086b86e); };
            c.cmp(t54, 0x82cu64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(12, c.r64(15));
            let Some(t55) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086b87f); };
            c.w64(15, t55);
            c.w64(3, c.r64(15).wrapping_add(0x14));
            let Some(t56) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086b887); };
            c.cmp(t56, c.r64(3), 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x700086b890); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xd90)) else { return RegfileExit::fault(0x700086b898); };
            c.w64(15, t57);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086b89f); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b8a4); };
            c.w64(15, t58);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086b8ab); }
            c.w32(3, 0x2u64);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700086b8b5); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086b8ba); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700086b8bf); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086b8c4); }
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086b8c9); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b8d0); };
            c.cmp(t61, c.r32(8), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 29; continue;
        }
        28 => {
            c.w64(8, c.r64(15));
            blk = 29; continue;
        }
        29 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086b8de); };
            c.w32(15, t62);
            let r63 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086b8e5); };
            c.cmp(t64, c.r32(15), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086b8ee); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r65 = c.r64(15) & c.r64(8);
            c.w64(15, r65);
            c.logic(r65, 64);
            let Some(t66) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086b902); };
            c.logic(t66 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 35; continue;
        }
        33 => {
            if !c.st32(c.r64(9).wrapping_add(0x27), c.r32(12)) { return RegfileExit::fault(0x700086b90f); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r67 = c.r64(8) & c.r64(9);
            c.w64(8, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086b923); };
            c.logic(t68 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086b92e); };
            c.w32(8, t69);
            c.cmp(c.r32(8), 0xfffffffffffffffcu64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, c.r64(9));
            return c.call(0x700086b0f4, 0x700086b940, 0x700086b93b);
        }
        37 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b940); };
            c.w64(0, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b944); };
            c.w64(11, t71);
            blk = 38; continue;
        }
        38 => {
            c.w64(8, c.r64(11));
            let r72 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r72);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086b952); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086b953); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086b959); }
            return c.ret(0x0, 0x700086b95b);
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086b95c); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086b960); };
            c.w64(3, t73);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086b971, 0x700086b96c);
        }
        40 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b971); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b975); };
            c.w64(0, t75);
            blk = 1; continue;
        }
        41 => {
            c.w32(0, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086b983); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700086b98f, 0x700086b98a);
        }
        42 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086b98f); };
            c.w64(12, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b993); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b997); };
            c.w64(6, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b99b); };
            c.w64(9, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b99f); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086b9a3); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086b9aa); };
            c.w64(2, t82);
            blk = 8; continue;
        }
        43 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086b9b6); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086b9b7); };
            c.w64(3, t83);
            c.w32(0, 0x1u64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b9c3); };
            c.w64(6, t84);
            return c.call(0x700086ed74, 0x700086b9cc, 0x700086b9c7);
        }
        44 => {
            blk = 18; continue;
        }
        45 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700086b9db, 0x700086b9d6);
        }
        46 => {
            c.w64(12, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086b9de); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086b9e2); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086b9e6); };
            c.w64(6, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086b9ea); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086b9ee); };
            c.w64(8, t89);
            blk = 26; continue;
        }
        47 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086b9fd); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086b9ff); };
            c.w64(8, t90);
            let Some(t91) = c.ld32(c.r64(8).wrapping_add(0xf7)) else { return RegfileExit::fault(0x700086ba06); };
            c.w32(8, t91);
            let r92 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r92);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086ba10); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086ba12); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086ba14); };
            c.w64(3, t93);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086ba28, 0x700086ba23);
        }
        48 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086ba28); };
            c.w64(1, t94);
            c.w64(3, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ba32); };
            c.w64(0, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ba36); };
            c.w64(6, t96);
            return c.call(0x70008b1fb4, 0x700086ba3f, 0x700086ba3a);
        }
        49 => {
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086ba3f); };
            c.w64(8, t97);
            let r98 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086ba47); };
            c.w64(0, t99);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086ba4e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086ba4f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086ba55); }
            return c.ret(0x0, 0x700086ba57);
        }
        50 => {
            c.w32(12, c.r32(12));
            let r100 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r100);
            c.w64(8, 0xfffffffffffc0000u64);
            let r101 = c.r64(8) & c.r64(12);
            c.w64(8, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086ba68); };
            c.logic(t102 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, c.r64(9).wrapping_add(0x27));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086ba77); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086ba78); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700086ba85, 0x700086ba80);
        }
        52 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700086ba85); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700086ba87); }
            blk = 35; continue;
        }
        53 => {
            c.w32(12, c.r32(12));
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            c.w64(15, 0xfffffffffffc0000u64);
            let r104 = c.r64(15) & c.r64(12);
            c.w64(15, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086ba9d); };
            c.logic(t105 & 0x2u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(3, c.r64(8).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086baac); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086baad); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x700086baba, 0x700086bab5);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700086baba); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700086babc); }
            blk = 32; continue;
        }
        56 => {
            return RegfileExit::jump(0x700086bac4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086bad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086bad4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700086bad8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086bad9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086bada); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700086badf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086bae3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086baf7); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700086baff); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700086bb03); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086bb0d); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086bb13); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086bb1a); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086bb21); };
            c.w32(12, t7);
            let r8 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086bb29); };
            c.w32(15, t9);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086bb34); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700086bb41);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086bb42); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086bb4d); };
            c.w64(15, t12);
            let Some(t13) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086bb54); };
            c.w64(0, t13);
            let Some(t14) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086bb57); };
            c.w32(12, t14);
            c.w64(3, c.r64(0).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086bb60); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086bb67); };
            c.cmp(t16, c.r64(3), 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700086bb70); }
            let r17 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r17);
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086bb77); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086bb7b); };
            c.w64(12, t18);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086bb82); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086bb86); }
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700086bb8d); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086bb91); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700086bb95); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086bb99); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086bb9d); };
            c.w64(12, t19);
            let Some(t20) = c.ld32(c.r64(12)) else { return RegfileExit::fault(0x700086bba4); };
            c.w32(12, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086bba8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086bbac); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086bbc4); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086bbca); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086bbd0); };
            c.w32(1, t25);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x8eu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700086bbe2);
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086bbe3); };
            c.w32(7, t26);
            let r27 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r27);
            let Some(t28) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086bbec); };
            c.w32(3, t28);
            let r29 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r29);
            let Some(t30) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086bbf2); };
            c.logic(t30 & 0x2u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086bbf8); };
            c.w64(3, t31);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086bc04); };
            c.w32(1, t32);
            let r33 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r33);
            let Some(t34) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086bc0a); };
            c.w32(1, t34);
            c.cmp(c.r16(1), 0x83du64, 16);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x83du64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700086bc20);
        }
        17 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086bc21); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086bc22); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086bc23); }
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(0));
            return c.call(0x70007f2134, 0x700086bc31, 0x700086bc2c);
        }
        18 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086bc31); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086bc35); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086bc39); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086bc3d); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086bc41); };
            c.w64(8, t39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086bc45); };
            c.w64(15, t40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086bc4c); };
            c.w64(2, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086bc53); };
            c.w64(12, t42);
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(12) & 0x6u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086bc5d); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086bc5e); };
            c.w64(12, t43);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700086bc65); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086bc67); };
            c.w64(3, t44);
            c.w32(0, 0x2u64);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700086bc7b, 0x700086bc76);
        }
        21 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086bc7b); };
            c.w64(0, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086bc7f); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086bc83); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086bc87); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086bc8b); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086bc8f); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086bc96); };
            c.w64(2, t51);
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086bca7); };
            c.w32(12, t52);
            let r53 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r53);
            let Some(t54) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086bcae); };
            c.cmp(t54, 0x82cu64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(12, c.r64(15));
            let Some(t55) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086bcbf); };
            c.w64(15, t55);
            c.w64(3, c.r64(15).wrapping_add(0x14));
            let Some(t56) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086bcc7); };
            c.cmp(t56, c.r64(3), 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x700086bcd0); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xd90)) else { return RegfileExit::fault(0x700086bcd8); };
            c.w64(15, t57);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086bcdf); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086bce4); };
            c.w64(15, t58);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086bceb); }
            c.w32(3, 0x4u64);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700086bcf5); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086bcfa); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700086bcff); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086bd04); }
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086bd09); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086bd10); };
            c.cmp(t61, c.r32(8), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 29; continue;
        }
        28 => {
            c.w64(8, c.r64(15));
            blk = 29; continue;
        }
        29 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086bd1e); };
            c.w32(15, t62);
            let r63 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086bd25); };
            c.cmp(t64, c.r32(15), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086bd2e); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r65 = c.r64(15) & c.r64(8);
            c.w64(15, r65);
            c.logic(r65, 64);
            let Some(t66) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086bd42); };
            c.logic(t66 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 35; continue;
        }
        33 => {
            if !c.st32(c.r64(9).wrapping_add(0x27), c.r32(12)) { return RegfileExit::fault(0x700086bd4f); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r67 = c.r64(8) & c.r64(9);
            c.w64(8, r67);
            c.logic(r67, 64);
            let Some(t68) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086bd63); };
            c.logic(t68 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086bd6e); };
            c.w32(8, t69);
            c.cmp(c.r32(8), 0xfffffffffffffffcu64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, c.r64(9));
            return c.call(0x700086b0f4, 0x700086bd80, 0x700086bd7b);
        }
        37 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086bd80); };
            c.w64(0, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086bd84); };
            c.w64(11, t71);
            blk = 38; continue;
        }
        38 => {
            c.w64(8, c.r64(11));
            let r72 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r72);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086bd92); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086bd93); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086bd99); }
            return c.ret(0x0, 0x700086bd9b);
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086bd9c); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700086bda0); };
            c.w64(3, t73);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086bdb1, 0x700086bdac);
        }
        40 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086bdb1); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086bdb5); };
            c.w64(0, t75);
            blk = 1; continue;
        }
        41 => {
            c.w32(0, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700086bdc3); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700086bdcf, 0x700086bdca);
        }
        42 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086bdcf); };
            c.w64(12, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086bdd3); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086bdd7); };
            c.w64(6, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086bddb); };
            c.w64(9, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086bddf); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086bde3); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086bdea); };
            c.w64(2, t82);
            blk = 8; continue;
        }
        43 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086bdf6); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086bdf7); };
            c.w64(3, t83);
            c.w32(0, 0x1u64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086be03); };
            c.w64(6, t84);
            return c.call(0x700086ed74, 0x700086be0c, 0x700086be07);
        }
        44 => {
            blk = 18; continue;
        }
        45 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700086be1b, 0x700086be16);
        }
        46 => {
            c.w64(12, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086be1e); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086be22); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086be26); };
            c.w64(6, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086be2a); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086be2e); };
            c.w64(8, t89);
            blk = 26; continue;
        }
        47 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086be3d); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086be3f); };
            c.w64(8, t90);
            let Some(t91) = c.ld32(c.r64(8).wrapping_add(0xfb)) else { return RegfileExit::fault(0x700086be46); };
            c.w32(8, t91);
            let r92 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r92);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086be50); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086be52); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2170)) else { return RegfileExit::fault(0x700086be54); };
            c.w64(3, t93);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086be68, 0x700086be63);
        }
        48 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086be68); };
            c.w64(1, t94);
            c.w64(3, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086be72); };
            c.w64(0, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086be76); };
            c.w64(6, t96);
            return c.call(0x70008b1fb4, 0x700086be7f, 0x700086be7a);
        }
        49 => {
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086be7f); };
            c.w64(8, t97);
            let r98 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086be87); };
            c.w64(0, t99);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086be8e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086be8f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086be95); }
            return c.ret(0x0, 0x700086be97);
        }
        50 => {
            c.w32(12, c.r32(12));
            let r100 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r100);
            c.w64(8, 0xfffffffffffc0000u64);
            let r101 = c.r64(8) & c.r64(12);
            c.w64(8, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086bea8); };
            c.logic(t102 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, c.r64(9).wrapping_add(0x27));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086beb7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086beb8); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700086bec5, 0x700086bec0);
        }
        52 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700086bec5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700086bec7); }
            blk = 35; continue;
        }
        53 => {
            c.w32(12, c.r32(12));
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            c.w64(15, 0xfffffffffffc0000u64);
            let r104 = c.r64(15) & c.r64(12);
            c.w64(15, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086bedd); };
            c.logic(t105 & 0x2u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(3, c.r64(8).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086beec); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086beed); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x700086befa, 0x700086bef5);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700086befa); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700086befc); }
            blk = 32; continue;
        }
        56 => {
            return RegfileExit::jump(0x700086bf04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700086bf14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700086bf14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700086bf18); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700086bf1e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086bf25); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700086bf2c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700086bf30); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700086bf34); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086bf38); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086bf45); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086bf4c); };
            c.cmp(t7, 0x83du64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086bf59); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086bf60); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086bf67); };
            c.w32(12, t12);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x97u64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x700086bf7a);
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0x263)) else { return RegfileExit::fault(0x700086bf7b); };
            c.w32(12, t13);
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700086bf82); };
            c.cmp(t14, c.r32(12), 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x700086bf8c); };
            c.w64(9, t15);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086bf96); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086bfa0); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086bfa7); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086bfae); };
            c.w64(12, t20);
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086bfb1); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086bfbd); };
            c.w64(2, t23);
            let Some(t24) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086bfc4); };
            c.cmp(t24, c.r64(15), 64);
            if c.cc_be() { blk = 66; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700086bfcd); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086bfd5); };
            c.w32(12, t25);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x7du64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x700086bfe5);
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700086bfe6); };
            c.w32(12, t26);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086bfed); }
            c.w32(12, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086bff7); }
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086bffb); };
            c.w32(12, t27);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700086c00b);
        }
        12 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086c00c); };
            c.w32(12, t28);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086c010); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086c014); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700086c018); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086c01c); };
            c.w64(12, t29);
            let Some(t30) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086c023); };
            c.w64(15, t30);
            c.w64(2, c.r64(15).wrapping_add(0x1c));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086c02b); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086c032); };
            c.cmp(t32, c.r64(2), 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x700086c03b); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            let Some(t33) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c043); };
            c.w32(15, t33);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xbeu64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x700086c056);
        }
        16 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086c057); };
            c.w32(15, t34);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086c05e); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086c063); };
            c.w64(15, t35);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086c06a); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086c06f); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086c074); };
            c.w64(15, t36);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x700086c07b); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1060)) else { return RegfileExit::fault(0x700086c080); };
            c.w64(15, t37);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700086c087); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700086c08c); }
            let Some(t38) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c091); };
            c.w32(15, t38);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t39) = c.ld64(c.r64(2).wrapping_add(c.r64(15).wrapping_mul(4))) else { return RegfileExit::fault(0x700086c09f); };
            c.w64(15, t39);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x700086c0a3); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086c0a8); };
            c.w64(15, t40);
            let Some(t41) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086c0af); };
            c.w64(2, t41);
            c.w64(1, c.r64(2).wrapping_add(0x1c));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086c0b6); };
            c.w64(7, t42);
            let Some(t43) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086c0bd); };
            c.cmp(t43, c.r64(1), 64);
            if c.cc_be() { blk = 70; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x700086c0c6); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 18; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c0cd); };
            c.w32(2, t44);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xbeu64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x700086c0e0);
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086c0e1); };
            c.w32(2, t45);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700086c0e8); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086c0ec); };
            c.w64(2, t46);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x700086c0f3); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700086c0f7); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086c0fb); };
            c.w64(2, t47);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700086c102); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1058)) else { return RegfileExit::fault(0x700086c106); };
            c.w64(2, t48);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700086c10d); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700086c111); }
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c115); };
            c.w32(11, t49);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t50) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x700086c123); };
            c.w64(11, t50);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x700086c127); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086c12b); };
            c.w64(11, t51);
            let Some(t52) = c.ld32(c.r64(11)) else { return RegfileExit::fault(0x700086c132); };
            c.w32(11, t52);
            c.logic(c.r8(11) & 0xeu64, 8);
            if c.cc_ne() { blk = 97; continue; }
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 72; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c149); };
            c.w64(8, t53);
            blk = 23; continue;
        }
        23 => {
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(12));
            c.w64(1, c.r64(15));
            c.w64(2, c.r64(8));
            c.w64(6, c.r64(9));
            return c.call(0x70008b2ef4, 0x700086c164, 0x700086c15f);
        }
        24 => {
            blk = 59; continue;
        }
        25 => {
            let Some(t54) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c169); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086c170); };
            c.w32(9, t56);
            let r57 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086c177); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086c17e); };
            c.w64(12, t59);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086c186); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086c18d); };
            c.cmp(t61, c.r64(15), 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700086c196); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 27; continue;
        }
        27 => {
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c19e); };
            c.w32(12, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x7du64, 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x700086c1ae);
        }
        29 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x700086c1af); };
            c.w32(12, t63);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700086c1b6); }
            c.w32(12, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700086c1c0); }
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c1c4); };
            c.w32(12, t64);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x700086c1d4);
        }
        31 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086c1d5); };
            c.w32(12, t65);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700086c1d9); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086c1dd); }
            c.w64(12, c.r64(0));
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x700086c1e4); }
            let Some(t66) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c1e8); };
            c.w32(15, t66);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xeau64, 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x700086c1fb);
        }
        33 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086c1fc); };
            c.w32(15, t67);
            let r68 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086c206); };
            c.w64(0, t69);
            let Some(t70) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086c20d); };
            c.w64(2, t70);
            let Some(t71) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086c210); };
            c.w32(15, t71);
            c.w64(1, c.r64(2).wrapping_add(0x14));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086c218); };
            c.w64(7, t72);
            let Some(t73) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086c21f); };
            c.cmp(t73, c.r64(1), 64);
            if c.cc_be() { blk = 101; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700086c228); }
            let r74 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r74);
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700086c22f); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086c233); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700086c23a); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700086c23e); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700086c245); }
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x700086c249); }
            let Some(t76) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086c24d); };
            c.w64(15, t76);
            c.w64(1, c.r64(15).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x700086c254); }
            let Some(t77) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086c258); };
            c.cmp(t77, c.r64(1), 64);
            if c.cc_be() { blk = 103; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700086c261); }
            let r78 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r78);
            blk = 37; continue;
        }
        37 => {
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c268); };
            c.w32(0, t79);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086c272); }
            c.cmp(c.r64(0), 0xbeu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700086c27e);
        }
        39 => {
            let Some(t80) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086c27f); };
            c.w32(0, t80);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700086c286); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086c28a); };
            c.w64(0, t81);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700086c291); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700086c295); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086c299); };
            c.w64(0, t82);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x700086c2a0); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1060)) else { return RegfileExit::fault(0x700086c2a4); };
            c.w64(0, t83);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700086c2ab); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700086c2af); }
            let Some(t84) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c2b3); };
            c.w32(0, t84);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(13).wrapping_add(0x63f0));
            let Some(t85) = c.ld64(c.r64(1).wrapping_add(c.r64(0).wrapping_mul(4))) else { return RegfileExit::fault(0x700086c2c0); };
            c.w64(0, t85);
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x700086c2c4); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086c2c8); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700086c2cf); };
            c.w64(1, t87);
            c.w64(7, c.r64(1).wrapping_add(0x1c));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086c2d6); };
            c.w64(12, t88);
            let Some(t89) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086c2dd); };
            c.cmp(t89, c.r64(7), 64);
            if c.cc_be() { blk = 105; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x700086c2e7); }
            c.w64(12, c.r64(1).wrapping_add(0x1));
            blk = 41; continue;
        }
        41 => {
            let Some(t90) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c2ee); };
            c.w32(0, t90);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700086c2f8); }
            c.cmp(c.r64(0), 0xbeu64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            return RegfileExit::fault(0x700086c304);
        }
        43 => {
            let Some(t91) = c.ld32(c.r64(9).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x700086c305); };
            c.w32(9, t91);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700086c30c); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086c311); };
            c.w64(9, t92);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700086c318); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700086c31d); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x700086c322); };
            c.w64(9, t93);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x700086c329); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x1058)) else { return RegfileExit::fault(0x700086c32e); };
            c.w64(9, t94);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700086c335); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x700086c33a); }
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c33f); };
            c.w32(9, t95);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(13).wrapping_add(0x63f0));
            let Some(t96) = c.ld64(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x700086c34d); };
            c.w64(9, t96);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x700086c351); }
            let Some(t97) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c356); };
            c.w32(9, t97);
            let r98 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r98);
            let Some(t99) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086c35d); };
            c.w32(9, t99);
            let r100 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r100);
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c364); };
            c.w32(11, t101);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            return RegfileExit::fault(0x700086c377);
        }
        45 => {
            let Some(t102) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086c378); };
            c.w32(7, t102);
            let r103 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r103);
            let Some(t104) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c382); };
            c.w32(9, t104);
            let r105 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r105);
            let Some(t106) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086c389); };
            c.logic(t106 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t107) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c396); };
            c.w32(9, t107);
            let r108 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r108);
            let Some(t109) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086c39d); };
            c.w32(9, t109);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(9, c.r64(8));
            blk = 52; continue;
        }
        50 => {
            return RegfileExit::fault(0x700086c3bc);
        }
        51 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c3bd); };
            c.w64(9, t110);
            blk = 52; continue;
        }
        52 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086c3c4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086c3c6); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c3c7); };
            c.w64(9, t111);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086c3ce); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2134, 0x700086c3da, 0x700086c3d5);
        }
        53 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c3da); };
            c.w64(2, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c3de); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c3e2); };
            c.w64(12, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c3e6); };
            c.w64(6, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c3ea); };
            c.w64(3, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c3ee); };
            c.w64(8, t117);
            blk = 54; continue;
        }
        54 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086c3f2); };
            c.w64(9, t118);
            let Some(t119) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x700086c3f9); };
            c.w32(9, t119);
            c.logic(c.r8(9) & 0xeu64, 8);
            if c.cc_ne() { blk = 136; continue; }
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c410); };
            c.w64(8, t120);
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700086c417); }
            c.w64(0, c.r64(2));
            return c.call(0x70008ba734, 0x700086c423, 0x700086c41e);
        }
        58 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c423); };
            c.w64(0, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c427); };
            c.w64(3, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c42b); };
            c.w64(1, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c42f); };
            c.w64(2, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c433); };
            c.w64(6, t125);
            return c.call(0x70008b2ef4, 0x700086c43c, 0x700086c437);
        }
        59 => {
            c.w32(8, 0x2u64);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c442); };
            c.w64(9, t126);
            if !c.st32(c.r64(9).wrapping_add(0x2b), c.r32(8)) { return RegfileExit::fault(0x700086c446); }
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c44a); };
            c.w64(0, t127);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700086c454); }
            return c.ret(0x0, 0x700086c455);
        }
        60 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700086c456); }
            c.w64(0, c.r64(3));
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x700086c45d); };
            c.w64(3, t128);
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x700086c46c, 0x700086c467);
        }
        61 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c46c); };
            c.w64(11, t129);
            let Some(t130) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c470); };
            c.w32(8, t130);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            return RegfileExit::fault(0x700086c483);
        }
        63 => {
            let Some(t131) = c.ld32(c.r64(11).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086c484); };
            c.cmp(t131, c.r32(0), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c48d); };
            c.w64(0, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c491); };
            c.w64(6, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c495); };
            c.w64(3, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c499); };
            c.w64(8, t135);
            blk = 25; continue;
        }
        65 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c4a2); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c4a6); };
            c.w64(6, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c4aa); };
            c.w64(3, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c4ae); };
            c.w64(8, t139);
            blk = 6; continue;
        }
        66 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086c4bc); }
            return c.call(0x70007febf4, 0x700086c4c5, 0x700086c4c0);
        }
        67 => {
            c.w64(11, c.r64(0));
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c4c8); };
            c.w64(0, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c4cc); };
            c.w64(6, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c4d0); };
            c.w64(3, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c4d4); };
            c.w64(8, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c4d8); };
            c.w64(9, t144);
            blk = 8; continue;
        }
        68 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086c4e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086c4ea); }
            return c.call(0x70007febf4, 0x700086c4f3, 0x700086c4ee);
        }
        69 => {
            c.w64(12, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c4f6); };
            c.w64(0, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c4fa); };
            c.w64(6, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c4fe); };
            c.w64(3, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c502); };
            c.w64(8, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c506); };
            c.w64(9, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c50a); };
            c.w64(11, t150);
            blk = 14; continue;
        }
        70 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086c518); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086c51c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086c520); }
            return c.call(0x70007febf4, 0x700086c529, 0x700086c524);
        }
        71 => {
            c.w64(15, c.r64(0));
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c52c); };
            c.w64(0, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c530); };
            c.w64(6, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c534); };
            c.w64(3, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c538); };
            c.w64(8, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c53c); };
            c.w64(12, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c540); };
            c.w64(9, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c544); };
            c.w64(11, t157);
            blk = 18; continue;
        }
        72 => {
            let Some(t158) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c54d); };
            c.w32(8, t158);
            let r159 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r159);
            let Some(t160) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086c554); };
            c.w32(8, t160);
            let r161 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r161);
            let Some(t162) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c55b); };
            c.w32(11, t162);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086c565); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086c569); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086c56d); }
            c.cmp(c.r64(11), 0xeau64, 64);
            if c.cc_a() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            return RegfileExit::fault(0x700086c57a);
        }
        74 => {
            let Some(t163) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086c57b); };
            c.w32(8, t163);
            let r164 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r164);
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086c585); };
            c.w64(11, t165);
            let Some(t166) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086c58c); };
            c.w64(2, t166);
            let Some(t167) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086c58f); };
            c.w32(8, t167);
            c.w64(1, c.r64(2).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086c597); }
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086c59b); };
            c.w64(7, t168);
            let Some(t169) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700086c5a2); };
            c.cmp(t169, c.r64(1), 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700086c5b1, 0x700086c5ac);
        }
        76 => {
            blk = 78; continue;
        }
        77 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x700086c5b3); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 78; continue;
        }
        78 => {
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c5bd); };
            c.w64(8, t170);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086c5c1); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086c5c5); };
            c.w64(8, t171);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086c5cc); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086c5d0); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086c5d7); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086c5db); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086c5df); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086c5e3); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086c5e7); };
            c.w64(8, t172);
            let Some(t173) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700086c5ee); };
            c.w32(8, t173);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700086c5f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086c5f5); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c609); };
            c.w64(6, t174);
            blk = 93; continue;
        }
        81 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c612); };
            c.w64(6, t175);
            let Some(t176) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c616); };
            c.w32(9, t176);
            let r177 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r177);
            let Some(t178) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086c61d); };
            c.w32(9, t178);
            let r179 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r179);
            let Some(t180) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c624); };
            c.w32(11, t180);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            return RegfileExit::fault(0x700086c637);
        }
        83 => {
            let Some(t181) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086c638); };
            c.w32(7, t181);
            let r182 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r182);
            let Some(t183) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c642); };
            c.w32(9, t183);
            let r184 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r184);
            let Some(t185) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086c649); };
            c.logic(t185 & 0x2u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c650); };
            c.w64(3, t186);
            let Some(t187) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c654); };
            c.w32(9, t187);
            let r188 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r188);
            let Some(t189) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086c65b); };
            c.w32(9, t189);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 88; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(9, c.r64(3));
            blk = 89; continue;
        }
        87 => {
            return RegfileExit::fault(0x700086c67a);
        }
        88 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c67b); };
            c.w64(9, t190);
            blk = 89; continue;
        }
        89 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086c682); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086c684); }
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c685); };
            c.w64(9, t191);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086c68c); }
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700086c69b, 0x700086c696);
        }
        90 => {
            blk = 92; continue;
        }
        91 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086c69d); }
            c.w32(0, 0x1u64);
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086c6a3); };
            c.w64(3, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c6aa); };
            c.w64(6, t193);
            return c.call(0x700086ed74, 0x700086c6b3, 0x700086c6ae);
        }
        92 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c6b3); };
            c.w64(6, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c6b7); };
            c.w64(0, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c6bb); };
            c.w64(8, t196);
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086c6c5); }
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c6c6); };
            if !c.push64(t197) { return RegfileExit::fault(0x700086c6c6); }
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086c6c9); };
            c.w64(3, t198);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086c6dd, 0x700086c6d8);
        }
        95 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c6dd); };
            c.w64(0, t199);
            blk = 96; continue;
        }
        96 => {
            c.w64(8, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c6e4); };
            c.w64(3, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c6e8); };
            c.w64(12, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c6ec); };
            c.w64(15, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c6f0); };
            c.w64(9, t203);
            blk = 23; continue;
        }
        97 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086c6f9); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086c6fa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086c6fb); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700086c6fd); }
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086c6ff); };
            c.w64(8, t204);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086c706); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700086c708); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086c70c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700086c710); }
            c.w32(0, 0x5u64);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x25a0)) else { return RegfileExit::fault(0x700086c719); };
            c.w64(3, t205);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700086c72b, 0x700086c726);
        }
        98 => {
            c.w64(8, c.r64(0));
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c72e); };
            c.w64(3, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c732); };
            c.w64(12, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c736); };
            c.w64(15, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c73a); };
            c.w64(9, t209);
            blk = 23; continue;
        }
        99 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700086c748); }
            return c.call(0x70007febf4, 0x700086c751, 0x700086c74c);
        }
        100 => {
            c.w64(11, c.r64(0));
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c754); };
            c.w64(0, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c758); };
            c.w64(6, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c75c); };
            c.w64(3, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c760); };
            c.w64(8, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086c764); };
            c.w64(9, t214);
            blk = 27; continue;
        }
        101 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086c772); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700086c776); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700086c77a); }
            return c.call(0x70007febf4, 0x700086c783, 0x700086c77e);
        }
        102 => {
            c.w64(2, c.r64(0));
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c786); };
            c.w64(12, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c78a); };
            c.w64(6, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c78e); };
            c.w64(3, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c792); };
            c.w64(8, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c796); };
            c.w64(11, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086c79a); };
            c.w64(9, t220);
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086c79e); };
            c.w64(0, t221);
            let Some(t222) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086c7a5); };
            c.w64(7, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c7ac); };
            c.w64(15, t223);
            blk = 35; continue;
        }
        103 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086c7ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700086c7be); }
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x700086c7ca, 0x700086c7c5);
        }
        104 => {
            c.w64(15, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700086c7cd); };
            c.w64(12, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c7d1); };
            c.w64(2, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c7d5); };
            c.w64(6, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c7d9); };
            c.w64(3, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c7dd); };
            c.w64(8, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c7e1); };
            c.w64(11, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086c7e5); };
            c.w64(9, t230);
            blk = 37; continue;
        }
        105 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700086c7f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700086c7f7); }
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x700086c803, 0x700086c7fe);
        }
        106 => {
            c.w64(12, c.r64(0));
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c806); };
            c.w64(2, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c80a); };
            c.w64(15, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c80e); };
            c.w64(6, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c812); };
            c.w64(3, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c816); };
            c.w64(8, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c81a); };
            c.w64(11, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086c81e); };
            c.w64(9, t237);
            blk = 41; continue;
        }
        107 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086c827); }
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086c828); };
            c.w64(3, t238);
            c.w32(0, 0x1u64);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c834); };
            c.w64(6, t239);
            return c.call(0x700086ed74, 0x700086c83d, 0x700086c838);
        }
        108 => {
            blk = 53; continue;
        }
        109 => {
            let Some(t240) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c842); };
            c.w32(8, t240);
            let r241 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r241);
            let Some(t242) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086c849); };
            c.w32(8, t242);
            let r243 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r243);
            let Some(t244) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c850); };
            c.w32(9, t244);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            return RegfileExit::fault(0x700086c863);
        }
        111 => {
            let Some(t245) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x700086c864); };
            c.w32(8, t245);
            let r246 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r246);
            let Some(t247) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086c86e); };
            c.w32(8, t247);
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086c872); };
            c.w64(9, t248);
            let Some(t249) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086c879); };
            c.w64(11, t249);
            c.w64(0, c.r64(11).wrapping_add(0x14));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086c880); }
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086c884); };
            c.w64(1, t250);
            let Some(t251) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086c88b); };
            c.cmp(t251, c.r64(0), 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(2, 0x14u64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700086c89d, 0x700086c898);
        }
        113 => {
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c89d); };
            c.w64(8, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c8a1); };
            c.w64(2, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c8a5); };
            c.w64(15, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c8a9); };
            c.w64(12, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c8ad); };
            c.w64(6, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c8b1); };
            c.w64(3, t257);
            blk = 115; continue;
        }
        114 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x700086c8b7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 115; continue;
        }
        115 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086c8c1); }
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086c8c5); };
            c.w64(8, t258);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700086c8cc); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700086c8d0); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086c8d7); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086c8db); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700086c8df); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700086c8e3); }
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700086c8e7); };
            c.w64(8, t259);
            let Some(t260) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700086c8ee); };
            c.w32(8, t260);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700086c8f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700086c8f5); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 116; continue;
        }
        116 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t261) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c90d); };
            c.w32(9, t261);
            let r262 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r262);
            let Some(t263) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086c914); };
            c.w32(9, t263);
            let r264 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r264);
            let Some(t265) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086c91b); };
            c.w32(11, t265);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            return RegfileExit::fault(0x700086c92e);
        }
        119 => {
            let Some(t266) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x700086c92f); };
            c.w32(7, t266);
            let r267 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r267);
            let Some(t268) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c939); };
            c.w32(9, t268);
            let r269 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r269);
            let Some(t270) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086c940); };
            c.logic(t270 & 0x2u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 120; continue;
        }
        120 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 126; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(9, c.r64(2));
            let Some(t271) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086c953); };
            c.w32(11, t271);
            let r272 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r272);
            let Some(t273) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086c95a); };
            c.w32(11, t273);
            c.cmp(c.r16(11), 0x83du64, 16);
            if c.cc_ne() { blk = 127; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w32(11, c.r16(11));
            c.cmp(c.r32(11), 0x83du64, 32);
            if c.cc_ne() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(11, c.r64(9));
            blk = 128; continue;
        }
        124 => {
            return RegfileExit::fault(0x700086c979);
        }
        125 => {
            blk = 127; continue;
        }
        126 => {
            c.w64(9, c.r64(2));
            blk = 127; continue;
        }
        127 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c97f); };
            c.w64(11, t274);
            blk = 128; continue;
        }
        128 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086c986); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086c988); }
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086c989); };
            c.w64(11, t275);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700086c990); }
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x70007f2134, 0x700086c99f, 0x700086c99a);
        }
        129 => {
            blk = 131; continue;
        }
        130 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086c9a1); }
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700086c9a2); };
            c.w64(3, t276);
            c.w32(0, 0x1u64);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c9ae); };
            c.w64(6, t277);
            return c.call(0x700086ed74, 0x700086c9b7, 0x700086c9b2);
        }
        131 => {
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c9b7); };
            c.w64(2, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c9bb); };
            c.w64(15, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c9bf); };
            c.w64(12, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c9c3); };
            c.w64(6, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086c9c7); };
            c.w64(3, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086c9cb); };
            c.w64(0, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086c9cf); };
            c.w64(8, t284);
            blk = 132; continue;
        }
        132 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 133; continue;
        }
        133 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086c9d9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086c9da); }
            c.w32(0, 0x2u64);
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x700086c9e0); };
            c.w64(3, t285);
            c.w64(8, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700086c9f2, 0x700086c9ed);
        }
        134 => {
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086c9f2); };
            c.w64(2, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086c9f6); };
            c.w64(15, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086c9fa); };
            c.w64(12, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086c9fe); };
            c.w64(6, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ca02); };
            c.w64(3, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086ca06); };
            c.w64(0, t291);
            blk = 135; continue;
        }
        135 => {
            c.w64(8, c.r64(0));
            blk = 57; continue;
        }
        136 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700086ca12); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086ca13); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086ca14); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700086ca16); }
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700086ca18); };
            c.w64(8, t292);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086ca1f); }
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0x25a8)) else { return RegfileExit::fault(0x700086ca21); };
            c.w64(3, t293);
            c.w32(0, 0x5u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700086ca35, 0x700086ca30);
        }
        137 => {
            c.w64(8, c.r64(0));
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086ca38); };
            c.w64(2, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086ca3c); };
            c.w64(6, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086ca40); };
            c.w64(3, t296);
            blk = 57; continue;
        }
        138 => {
            return RegfileExit::jump(0x700086ca4c);
        }
            _ => unreachable!(),
        }
    }
}
