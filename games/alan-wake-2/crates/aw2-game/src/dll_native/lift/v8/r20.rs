#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000894bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000894bd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000894bd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000894bd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000894bda); }
            let r0 = c.sub(c.r64(4), 0xc0u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000894be2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000894be6); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000894bea); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 589; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(5));
            c.w64(11, c.r64(9).wrapping_add(0x18));
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000894c02); };
            c.w64(12, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000894c07); }
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(11)) { return RegfileExit::fault(0x7000894c0b); }
            c.w64(0, c.r64(12));
            return c.call(0x70008a2054, 0x7000894c1a, 0x7000894c15);
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000894c24); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000894c2e); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894c35); };
            c.cmp(t5, 0x834u64, 16);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894c3e); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894c45); };
            c.w64(6, t8);
            blk = 28; continue;
        }
        5 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 7; continue;
        }
        6 => {
            c.w32(11, 0x1u64);
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894c5f); };
            c.w64(6, t9);
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000894c63); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000894c6a); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000894c71); };
            c.w32(12, t14);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000894c78); };
            c.w32(15, t15);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xb0u64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x7000894c88);
        }
        10 => {
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x7000894c89); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 591; continue;
        }
        12 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894c97); };
            c.w64(6, t17);
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 593; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000894ca5); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000894cac); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000894cb3); };
            c.w32(11, t22);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t23) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000894cba); };
            c.w32(12, t23);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x7000894cc7);
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x117)) else { return RegfileExit::fault(0x7000894cc8); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(11), 0xaeu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x7000894cda);
        }
        19 => {
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x7000894cdb); };
            c.cmp(t25, c.r32(12), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(11), 0xafu64, 64);
            if c.cc_a() { blk = 592; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000894cf1);
        }
        22 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894cf2); };
            c.w32(9, t26);
            let r27 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r27);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a17f4, 0x7000894d07, 0x7000894d02);
        }
        25 => {
            c.w64(9, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894d0a); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894d0e); };
            c.w64(0, t29);
            blk = 28; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ge() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000894d1d); };
            c.w64(11, (t30 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x7000894d21); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000894d25); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000894d2f); };
            c.w64(12, t31);
            let Some(t32) = c.ld64(c.r64(12).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000894d33); };
            c.w64(12, t32);
            blk = 31; continue;
        }
        30 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000894d3a); };
            c.w64(12, t33);
            blk = 31; continue;
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x7000894d41); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, c.r64(12));
            blk = 35; continue;
        }
        33 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x7000894d58, 0x7000894d53);
        }
        34 => {
            c.w64(15, c.r64(0));
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000894d5b); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894d5f); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894d63); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894d67); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000894d6b); };
            c.w64(9, t38);
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.w32(2, c.r32(9));
            c.w32(1, c.r32(15));
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(3), 0x0u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000894d96); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x7000894d9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(8)) { return RegfileExit::fault(0x7000894d9e); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894daa); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 60; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 40; continue;
        }
        38 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_l() { blk = 60; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(1, 0x1u64);
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894dd6); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894de1); }
            blk = 46; continue;
        }
        43 => {
            c.w64(1, c.r64(9));
            let r39 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r39);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 46; continue;
        }
        44 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(1, c.r64(15));
            let r40 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r40);
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894e02); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 46; continue;
        }
        46 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 59; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 49; continue;
        }
        48 => {
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_le() { blk = 59; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894e26); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894e31); }
            blk = 55; continue;
        }
        52 => {
            c.w64(1, c.r64(15));
            let r41 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 55; continue;
        }
        53 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(1, c.r64(9));
            let r42 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r42);
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894e52); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 55; continue;
        }
        55 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 75; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 58; continue;
        }
        57 => {
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_le() { blk = 75; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000894e75); };
            c.w64(15, t43);
            blk = 75; continue;
        }
        59 => {
            c.w64(15, c.r64(9));
            blk = 75; continue;
        }
        60 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 595; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 595; continue; }
            blk = 62; continue;
        }
        62 => {
            let r44 = c.add(c.r32(1), c.r32(2), 32);
            c.w32(1, r44);
            if c.cc_o() { blk = 595; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(15, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 64; continue;
        }
        64 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894ead); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 75; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 69; continue;
        }
        67 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ge() { blk = 75; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w32(1, 0x1u64);
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894ed1); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 74; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 73; continue;
        }
        72 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_le() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000894ee9); };
            c.w64(15, t45);
            blk = 75; continue;
        }
        74 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 75; continue;
        }
        75 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(15)) { return RegfileExit::fault(0x7000894ef5); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 125; continue; }
            blk = 76; continue;
        }
        76 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000894f10); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_e() { blk = 119; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000894f24); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000894f2b); };
            c.w64(9, t47);
            c.w64(7, c.r64(9));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000894f32); };
            c.w64(9, t48);
            blk = 80; continue;
        }
        79 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000894f38); };
            c.w64(7, t49);
            blk = 80; continue;
        }
        80 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000899a14, 0x7000894f4d, 0x7000894f48);
        }
        82 => {
            c.w64(7, c.r64(0));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000894f50); };
            c.w64(12, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894f54); };
            c.w64(6, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894f58); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894f5c); };
            c.w64(0, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000894f60); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000894f64); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000894f68); };
            c.w64(2, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000894f6c); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000894f73); };
            c.w64(15, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000894f7a); };
            c.w64(1, t59);
            blk = 83; continue;
        }
        83 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894f84); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 94; continue; }
            blk = 85; continue;
        }
        85 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), 0x0u64) { return RegfileExit::fault(0x7000894f93); }
            blk = 88; continue;
        }
        86 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ge() { blk = 94; continue; }
            blk = 87; continue;
        }
        87 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), 0x1u64) { return RegfileExit::fault(0x7000894fa2); }
            blk = 88; continue;
        }
        88 => {
            let Some(t60) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000894faa); };
            c.cmp(t60, 0x0u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 89; continue;
        }
        89 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894fb0); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 93; continue; }
            blk = 90; continue;
        }
        90 => {
            blk = 92; continue;
        }
        91 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_le() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000894fc6); };
            c.w64(7, t61);
            blk = 94; continue;
        }
        93 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 613; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 613; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w32(12, c.r32(15));
            let r62 = c.sub(c.r32(2), c.r32(12), 32);
            c.w32(2, r62);
            if c.cc_o() { blk = 612; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(12, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 102; continue; }
            blk = 99; continue;
        }
        99 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700089500c); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000895016); }
            blk = 104; continue;
        }
        101 => {
            c.w64(11, c.r64(12));
            let r63 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r63);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 104; continue;
        }
        102 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(11, c.r64(7));
            let r64 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r64);
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700089503b); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 104; continue;
        }
        104 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 118; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 107; continue;
        }
        106 => {
            c.cmp(c.r32(12), c.r32(7), 32);
            if c.cc_le() { blk = 118; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 108; continue;
        }
        108 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000895061); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700089506d); }
            blk = 113; continue;
        }
        110 => {
            c.w64(11, c.r64(7));
            let r65 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r65);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 113; continue;
        }
        111 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 115; continue; }
            blk = 112; continue;
        }
        112 => {
            let r66 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r66);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700089508d); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 113; continue;
        }
        113 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 117; continue; }
            blk = 114; continue;
        }
        114 => {
            blk = 116; continue;
        }
        115 => {
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_le() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x70008950a8); };
            c.w64(12, t67);
            blk = 118; continue;
        }
        117 => {
            c.w64(12, c.r64(7));
            blk = 118; continue;
        }
        118 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008950b4); };
            c.w64(11, t68);
            let r69 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r69);
            let r70 = c.sub(c.r32(11), 0x4u64, 32);
            c.w32(11, r70);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(12));
            blk = 124; continue;
        }
        119 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 632; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 632; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(7, c.r32(15));
            let r71 = c.sub(c.r32(2), c.r32(7), 32);
            c.w32(2, r71);
            if c.cc_o() { blk = 631; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 123; continue;
        }
        123 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 124; continue;
        }
        124 => {
            c.w64(12, c.r64(2));
            blk = 126; continue;
        }
        125 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(11));
            blk = 126; continue;
        }
        126 => {
            c.w32(2, c.r32(11));
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000895103); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x7000895107); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x700089510b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700089510f); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 650; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 650; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000895126); };
            c.w64(7, t72);
            let r73 = c.add(c.r32(7), c.r32(2), 32);
            c.w32(7, r73);
            if c.cc_o() { blk = 650; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 130; continue;
        }
        130 => {
            c.w32(9, c.r32(12));
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(9)) { return RegfileExit::fault(0x7000895151); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x7000895155); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 668; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 668; continue; }
            blk = 132; continue;
        }
        132 => {
            let r74 = c.sub(c.r32(11), c.r32(9), 32);
            c.w32(11, r74);
            if c.cc_o() { blk = 668; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 134; continue;
        }
        134 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000895184); };
            c.w64(7, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(0x15b)) else { return RegfileExit::fault(0x700089518b); };
            c.w32(7, t76);
            let r77 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r77);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 135; continue;
        }
        135 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700089519a); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008951a5); }
            blk = 140; continue;
        }
        137 => {
            let r78 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r78);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 140; continue;
        }
        138 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 139; continue;
        }
        139 => {
            let r79 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r79);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008951c3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 140; continue;
        }
        140 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 587; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 143; continue;
        }
        142 => {
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_l() { blk = 587; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 423; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t80) = c.ld8(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008951f0); };
            c.logic(t80 & 0x1u64, 8);
            if c.cc_ne() { blk = 423; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 423; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895206); };
            c.w64(11, t81);
            let Some(t82) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089520a); };
            c.w32(15, t82);
            let r83 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r83);
            let Some(t84) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000895211); };
            c.cmp(t84, 0x834u64, 16);
            if c.cc_ne() { blk = 423; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t85) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089521e); };
            c.w32(7, t85);
            let r86 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r86);
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000895224); };
            c.w32(7, t87);
            let r88 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r88);
            let Some(t89) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089522a); };
            c.w32(9, t89);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 149; continue; }
            blk = 148; continue;
        }
        148 => {
            return RegfileExit::fault(0x700089523a);
        }
        149 => {
            let Some(t90) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x700089523b); };
            c.w32(9, t90);
            let Some(t91) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000895242); };
            c.cmp(t91, c.r32(9), 32);
            if c.cc_ne() { blk = 423; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700089524c); };
            c.w64(9, t92);
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t93) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000895255); };
            c.cmp(t93, c.r32(7), 32);
            if c.cc_e() { blk = 423; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xfc0)) else { return RegfileExit::fault(0x700089525f); };
            c.w64(9, t94);
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000895266); };
            c.cmp(t95, c.r32(7), 32);
            if c.cc_e() { blk = 423; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t96) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000895270); };
            c.w32(9, t96);
            c.w64(11, c.r64(9));
            let r97 = c.r32(11) & 0x8100000u64;
            c.w32(11, r97);
            c.logic(r97, 32);
            c.cmp(c.r32(11), 0x8000000u64, 32);
            if c.cc_ne() { blk = 423; continue; }
            blk = 153; continue;
        }
        153 => {
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 423; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t98) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000895298); };
            c.w32(9, t98);
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            let Some(t100) = c.ld16(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089529f); };
            c.w32(11, t100);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            return RegfileExit::fault(0x70008952aa);
        }
        156 => {
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008952ab); };
            c.w32(9, t101);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 423; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t102) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008952b9); };
            c.w32(9, t102);
            let r103 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r103);
            let r104 = c.r32(9) & 0x3fu64;
            c.w32(9, r104);
            c.logic(r104, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_a() { blk = 423; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w64(11, c.r64(9));
            c.w32(15, 0x2u64);
            blk = 160; continue;
        }
        159 => {
            let r105 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r105);
            blk = 160; continue;
        }
        160 => {
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008952df); };
            c.w64(12, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008952e3); };
            c.w64(0, t107);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ge() { blk = 193; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t108) = c.ld64(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(8))) else { return RegfileExit::fault(0x70008952f8); };
            c.w64(12, t108);
            blk = 164; continue;
        }
        163 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008952fe); };
            c.w64(12, t109);
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_be() { blk = 179; continue; }
            blk = 165; continue;
        }
        165 => {
            c.w64(0, c.r64(11));
            let r110 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r110);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_ne() { blk = 159; continue; }
            blk = 166; continue;
        }
        166 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t111) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089531f); };
            c.w32(12, t111);
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000895324); };
            c.cmp(t112, c.r32(12), 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w32(12, 0x1u64);
            blk = 170; continue;
        }
        169 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 159; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_ne() { blk = 159; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w32(11, 0x3u64);
            blk = 159; continue;
        }
        176 => {
            c.w32(11, 0x2u64);
            blk = 159; continue;
        }
        177 => {
            c.w32(11, 0x3u64);
            blk = 159; continue;
        }
        178 => {
            c.w32(11, 0x2u64);
            blk = 159; continue;
        }
        179 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t113) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089538c); };
            c.w32(12, t113);
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let Some(t114) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089539b); };
            c.cmp(t114, c.r32(12), 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 181; continue;
        }
        181 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_ne() { blk = 159; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w32(11, 0x5u64);
            blk = 159; continue;
        }
        184 => {
            c.w32(11, 0x4u64);
            blk = 159; continue;
        }
        185 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 186; continue;
        }
        186 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 187; continue;
        }
        187 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 188; continue;
        }
        188 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_ne() { blk = 159; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w32(11, 0x3u64);
            blk = 159; continue;
        }
        190 => {
            c.w32(11, 0x2u64);
            blk = 159; continue;
        }
        191 => {
            c.w32(11, 0x3u64);
            blk = 159; continue;
        }
        192 => {
            c.w32(11, 0x2u64);
            blk = 159; continue;
        }
        193 => {
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(11)) { return RegfileExit::fault(0x7000895410); }
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 194; continue;
        }
        194 => {
            c.w32(9, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895420); };
            if !c.push64(t115) { return RegfileExit::fault(0x7000895420); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000895423); }
            c.w64(15, c.r64(0));
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1cb0)) else { return RegfileExit::fault(0x7000895428); };
            c.w64(3, t116);
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700089543c, 0x7000895437);
        }
        195 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089543c); };
            c.w64(6, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000895440); };
            c.w64(12, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000895444); };
            c.w64(1, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000895448); };
            c.w64(2, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089544c); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000895450); };
            c.w64(8, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000895457); };
            c.w64(0, t123);
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000895460); };
            c.w64(11, t124);
            blk = 196; continue;
        }
        196 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895467); };
            c.w64(9, t125);
            let Some(t126) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089546b); };
            c.w32(15, t126);
            c.w32(9, c.r32(15));
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(15)) { return RegfileExit::fault(0x7000895472); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x7000895479); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 423; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089548a); };
            c.w64(3, t127);
            c.cmp(c.r32(3), c.r32(15), 32);
            if c.cc_ne() { blk = 422; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t128) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000895497); };
            let r129 = c.sub(c.r32(2), t128, 32);
            c.w32(2, r129);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700089549a); }
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089549e); };
            c.w64(0, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008954a2); };
            c.w64(3, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008954a9); };
            c.w64(1, t132);
            return c.call(0x70008283b4, 0x70008954b2, 0x70008954ad);
        }
        199 => {
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008954b2); };
            c.w64(9, t133);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008954b6); }
            let Some(t134) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008954ba); };
            let r135 = c.add(c.r32(9), t134, 32);
            c.w32(9, r135);
            if c.cc_e() { blk = 420; continue; }
            blk = 200; continue;
        }
        200 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008954c4); }
            let Some(t136) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008954c8); };
            c.cmp(t136, 0x3u64, 32);
            if c.cc_be() { blk = 298; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008954d5); };
            c.w64(11, t137);
            let Some(t138) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008954d9); };
            c.w32(12, t138);
            let r139 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r139);
            let Some(t140) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008954e0); };
            c.w32(15, t140);
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008954e5); };
            c.cmp(t141, c.r32(15), 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t142) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008954f2); };
            c.w32(15, t142);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 227; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t143) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000895500); };
            c.w32(3, t143);
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_a() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_ae() { blk = 208; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000895519); };
            c.w64(2, t144);
            let Some(t145) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000895520); };
            c.w32(2, t145);
            let r146 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r146);
            return c.call(0x7000824234, 0x700089552e, 0x7000895529);
        }
        206 => {
            return RegfileExit::fault(0x700089552e);
        }
        207 => {
            return RegfileExit::fault(0x700089552f);
        }
        208 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 223; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 687; continue; }
            blk = 210; continue;
        }
        210 => {
            let r147 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r147);
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(1, c.r32(1));
            let r148 = c.imul2(c.r32(1), 0x4u64, 32);
            c.w32(1, r148);
            if c.cc_o() { blk = 222; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(6, 0x8u64);
            let r149 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r149);
            if c.cc_o() { blk = 221; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089556e); };
            c.w64(7, t150);
            let Some(t151) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000895575); };
            c.w64(7, t151);
            c.w64(8, c.r64(7).wrapping_add(c.r64(1)));
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089557c); };
            c.w64(0, t152);
            let Some(t153) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000895583); };
            c.cmp(t153, c.r64(8), 64);
            if c.cc_be() { blk = 685; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 685; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000895599); };
            c.w64(0, t154);
            if !c.st64(c.r64(0), c.r64(8)) { return RegfileExit::fault(0x70008955a0); }
            c.w64(0, c.r64(7).wrapping_add(0x1));
            blk = 215; continue;
        }
        215 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008955a7); };
            c.w64(1, t155);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008955ae); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008955b1); }
            c.w64(3, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(6));
            blk = 217; continue;
        }
        216 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008955c1); }
            let r156 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r156);
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r64(3), c.r64(6), 64);
            if c.cc_e() { blk = 224; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w64(1, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_e() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t157) = c.ld32(c.r64(12).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008955db); };
            c.w32(1, t157);
            let r158 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r158);
            blk = 216; continue;
        }
        220 => {
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008955e6); };
            c.w64(1, t159);
            blk = 216; continue;
        }
        221 => {
            return RegfileExit::fault(0x70008955ef);
        }
        222 => {
            return RegfileExit::fault(0x70008955f0);
        }
        223 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008955f1); };
            c.w64(0, t160);
            c.w64(12, c.r64(0));
            blk = 224; continue;
        }
        224 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008955fb); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r161 = c.r64(12) & c.r64(11);
            c.w64(12, r161);
            c.logic(r161, 64);
            let Some(t162) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089560d); };
            c.logic(t162 & 0x4u64, 8);
            if c.cc_ne() { blk = 1042; continue; }
            blk = 226; continue;
        }
        226 => {
            blk = 228; continue;
        }
        227 => {
            return RegfileExit::fault(0x700089561b);
        }
        228 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089561c); };
            c.w64(12, t163);
            let Some(t164) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000895620); };
            c.cmp(c.r32(12), t164, 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(9, c.r64(11));
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895629); };
            c.w64(3, t165);
            blk = 283; continue;
        }
        230 => {
            let Some(t166) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000895635); };
            c.w32(15, t166);
            let r167 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r167);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700089563c); }
            let Some(t168) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000895640); };
            c.cmp(c.r32(12), t168, 32);
            if c.cc_l() { blk = 276; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089564a); };
            c.w64(1, t169);
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_l() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            c.w64(9, c.r64(11));
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895656); };
            c.w64(3, t170);
            blk = 283; continue;
        }
        233 => {
            let Some(t171) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000895662); };
            c.w32(0, t171);
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_le() { blk = 274; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089566f); };
            c.cmp(t172, c.r32(15), 32);
            if c.cc_ne() { blk = 245; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(0, c.r64(9));
            let r173 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r173);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r174 = c.r64(0) & 0xfffffffffffffffeu64;
            c.w64(0, r174);
            c.logic(r174, 64);
            c.w32(0, c.r32(0));
            c.w32(0, c.r64(0).wrapping_add(c.r64(9)).wrapping_add(0x20));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000895698); }
            c.cmp(c.r64(3), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 693; continue; }
            blk = 236; continue;
        }
        236 => {
            c.w64(3, c.r64(0).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 689; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w64(2, c.r64(0));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r175 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r175);
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008956c6); };
            c.w64(6, t176);
            let Some(t177) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008956cd); };
            c.w64(7, t177);
            let Some(t178) = c.ld64(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008956d0); };
            c.w64(6, t178);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008956d4); }
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_be() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            let r179 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r179);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008956e3); }
            c.w32(3, 0x4u64);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008956e9); }
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008956ea); };
            c.w64(3, t180);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008956fd, 0x70008956f8);
        }
        239 => {
            blk = 242; continue;
        }
        240 => {
            c.w64(8, c.r64(3).wrapping_add(c.r64(7)));
            c.cmp(c.r64(6), c.r64(8), 64);
            if c.cc_be() { blk = 691; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089570c); };
            c.w64(3, t181);
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x7000895713); }
            c.w64(3, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(3));
            blk = 242; continue;
        }
        242 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700089571d); };
            c.w64(9, t182);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000895724); }
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000895728); };
            c.w64(9, t183);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089572c); }
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000895730); };
            c.w64(9, t184);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000895737); }
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089573d); };
            c.w64(9, t185);
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 244; continue; }
            blk = 243; continue;
        }
        243 => {
            let r186 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r186);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(0)), 0) { return RegfileExit::fault(0x7000895753); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 243; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089575f); };
            c.w64(7, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895763); };
            c.w64(11, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000895767); };
            c.w64(12, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089576b); };
            c.w64(1, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700089576f); };
            c.w64(3, t191);
            blk = 269; continue;
        }
        245 => {
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700089577b); };
            c.w64(3, t192);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_a() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ae() { blk = 250; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000895792); };
            c.w64(2, t193);
            let Some(t194) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000895799); };
            c.w32(2, t194);
            let r195 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r195);
            return c.call(0x7000824234, 0x70008957a7, 0x70008957a2);
        }
        248 => {
            return RegfileExit::fault(0x70008957a7);
        }
        249 => {
            return RegfileExit::fault(0x70008957a8);
        }
        250 => {
            c.w64(0, c.r64(9));
            let r196 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r196);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r197 = c.r64(0) & 0xfffffffffffffffeu64;
            c.w64(0, r197);
            c.logic(r197, 64);
            c.w32(0, c.r32(0));
            c.w32(0, c.r64(0).wrapping_add(c.r64(9)).wrapping_add(0x20));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 268; continue; }
            blk = 251; continue;
        }
        251 => {
            c.cmp(c.r64(0), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 698; continue; }
            blk = 252; continue;
        }
        252 => {
            let r198 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r198);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w32(0, c.r32(0));
            let r199 = c.imul2(c.r32(0), 0x8u64, 32);
            c.w32(0, r199);
            if c.cc_o() { blk = 267; continue; }
            blk = 253; continue;
        }
        253 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(2, 0x8u64);
            let r200 = c.add(c.r64(0), c.r64(2), 64);
            c.w64(0, r200);
            if c.cc_o() { blk = 266; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008957fc); };
            c.w64(6, t201);
            let Some(t202) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000895803); };
            c.w64(6, t202);
            c.w64(7, c.r64(6).wrapping_add(c.r64(0)));
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089580a); };
            c.w64(8, t203);
            let Some(t204) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000895811); };
            c.cmp(t204, c.r64(7), 64);
            if c.cc_be() { blk = 696; continue; }
            blk = 255; continue;
        }
        255 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 696; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000895826); };
            c.w64(0, t205);
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x700089582d); }
            c.w64(0, c.r64(6).wrapping_add(0x1));
            blk = 257; continue;
        }
        257 => {
            c.w64(6, c.r64(9));
            let r206 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r206);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let r207 = c.r64(6) & 0xfffffffffffffffeu64;
            c.w64(6, r207);
            c.logic(r207, 64);
            c.w32(6, c.r32(6));
            c.w32(6, c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x20));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r208 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r208);
            c.w64(7, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000895852); };
            c.w64(8, t209);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000895859); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x700089585d); }
            c.w64(6, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(7, c.r64(15));
            c.w64(15, c.r64(2));
            blk = 259; continue;
        }
        258 => {
            let r210 = c.add(c.r64(2), 0x8u64, 64);
            c.w64(2, r210);
            blk = 259; continue;
        }
        259 => {
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_e() { blk = 269; continue; }
            blk = 260; continue;
        }
        260 => {
            c.w64(8, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_e() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(c.r64(15)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000895889); }
            c.w64(8, c.r64(15).wrapping_add(0x8));
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t211) = c.ld64(c.r64(7).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089589e); };
            c.cmp(t211, c.r64(9), 64);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w64(15, c.r64(8));
            blk = 263; continue;
        }
        262 => {
            c.xorpd_rr(0, 0);
            c.w32(9, 0x1u64);
            blk = 263; continue;
        }
        263 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(0).wrapping_add(c.r64(2)).wrapping_sub(0x1), 0) { return RegfileExit::fault(0x70008958ca); }
            blk = 258; continue;
        }
        265 => {
            let Some(t212) = c.ld64(0x7000895896u64) else { return RegfileExit::fault(0x70008958d2); };
            c.w64(9, t212);
            if !c.st64(c.r64(0).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r64(9)) { return RegfileExit::fault(0x70008958d9); }
            blk = 258; continue;
        }
        266 => {
            return RegfileExit::fault(0x70008958e0);
        }
        267 => {
            return RegfileExit::fault(0x70008958e1);
        }
        268 => {
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008958e2); };
            c.w64(0, t213);
            c.w64(7, c.r64(15));
            blk = 269; continue;
        }
        269 => {
            c.w64(9, c.r64(11));
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008958ef); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 271; continue; }
            blk = 270; continue;
        }
        270 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r214 = c.r64(11) & c.r64(9);
            c.w64(11, r214);
            c.logic(r214, 64);
            let Some(t215) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000895901); };
            c.logic(t215 & 0x4u64, 8);
            if c.cc_ne() { blk = 1039; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t216) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089590c); };
            c.w32(11, t216);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_le() { blk = 283; continue; }
            blk = 272; continue;
        }
        272 => {
            c.w32(11, c.r32(3));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700089591d); };
            c.w64(15, t217);
            let Some(t218) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000895924); };
            let r219 = c.sub(c.r32(15), t218, 32);
            c.w32(15, r219);
            c.w32(2, c.r64(3).wrapping_add(c.r64(1)));
            let r220 = c.sub(c.r32(15), c.r32(11), 32);
            c.w32(15, r220);
            c.w64(11, c.r64(12));
            c.w32(12, c.r64(3).wrapping_add(c.r64(11)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(15).wrapping_mul(4));
            c.w64(2, c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r221 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r221);
            let r222 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r222);
            c.logic(r222, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089595b); }
            let Some(t223) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000895960); };
            c.w64(0, t223);
            c.w64(10, 0x700089596eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089596e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000895972); }
            return c.call(c.r64(0), 0x7000895978, 0x7000895976);
        }
        273 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000895978); }
            let Some(t224) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000895980); };
            c.w64(4, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895985); };
            c.w64(9, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000895989); };
            c.w64(12, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700089598d); };
            c.w64(3, t227);
            blk = 283; continue;
        }
        274 => {
            let Some(t228) = c.ld32(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895999); };
            c.w32(0, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700089599f); };
            c.w64(3, t229);
            let Some(t230) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008959a6); };
            let r231 = c.sub(c.r32(3), t230, 32);
            c.w32(3, r231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008959a9); };
            c.w64(2, t232);
            let r233 = c.add(c.r32(1), c.r32(2), 32);
            c.w32(1, r233);
            let r234 = c.sub(c.r32(3), c.r32(0), 32);
            c.w32(3, r234);
            c.w32(0, c.r64(2).wrapping_add(c.r64(12)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(3).wrapping_mul(4));
            c.w64(3, c.r64(15).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7));
            c.w64(1, c.r64(15).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r235 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r235);
            let r236 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r236);
            c.logic(r236, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008959de); }
            c.w64(2, c.r64(3));
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x70008959e6); };
            c.w64(0, t237);
            c.w64(10, 0x70008959f4u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008959f4); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008959f8); }
            return c.call(c.r64(0), 0x70008959fe, 0x70008959fc);
        }
        275 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008959fe); }
            let Some(t238) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000895a06); };
            c.w64(4, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895a0b); };
            c.w64(9, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000895a0f); };
            c.w64(12, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895a13); };
            c.w64(3, t241);
            blk = 283; continue;
        }
        276 => {
            let Some(t242) = c.ld32(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895a1f); };
            c.w32(0, t242);
            c.w64(3, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000895a28); };
            c.w64(1, t243);
            let Some(t244) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000895a2f); };
            let r245 = c.sub(c.r32(1), t244, 32);
            c.w32(1, r245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000895a32); };
            c.w64(2, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895a36); };
            c.w64(6, t247);
            let r248 = c.add(c.r32(2), c.r32(6), 32);
            c.w32(2, r248);
            let r249 = c.sub(c.r32(1), c.r32(0), 32);
            c.w32(1, r249);
            c.w32(0, c.r64(6).wrapping_add(c.r64(12)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(1).wrapping_mul(4));
            c.w64(2, c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7));
            c.w64(1, c.r64(15).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r250 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r250);
            let r251 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r251);
            c.logic(r251, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000895a6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000895a70); }
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x7000895a74); };
            c.w64(0, t252);
            c.w64(10, 0x7000895a82u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000895a82); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000895a86); }
            return c.call(c.r64(0), 0x7000895a8c, 0x7000895a8a);
        }
        277 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000895a8c); }
            let Some(t253) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000895a94); };
            c.w64(4, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000895a99); };
            c.w64(3, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000895a9d); };
            c.w64(9, t255);
            blk = 279; continue;
        }
        278 => {
            c.w32(11, c.r64(3).wrapping_add(0x2));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let Some(t256) = c.ld64(0x7000895896u64) else { return RegfileExit::fault(0x7000895ab1); };
            c.w64(15, t256);
            if !c.st64(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0x7), c.r64(15)) { return RegfileExit::fault(0x7000895ab8); }
            c.w64(3, c.r64(11));
            blk = 279; continue;
        }
        279 => {
            let Some(t257) = c.ld32(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000895ac0); };
            c.cmp(c.r32(3), t257, 32);
            if c.cc_ge() { blk = 282; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t258) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000895ac8); };
            c.w32(11, t258);
            c.cmp(c.r32(3), c.r32(11), 32);
            if c.cc_b() { blk = 278; continue; }
            blk = 281; continue;
        }
        281 => {
            blk = 297; continue;
        }
        282 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895ad6); };
            c.w64(9, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000895ada); };
            c.w64(12, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895ade); };
            c.w64(3, t261);
            blk = 283; continue;
        }
        283 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_g() { blk = 285; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000895aeb); };
            c.w64(15, t262);
            c.w64(0, c.r64(9));
            blk = 295; continue;
        }
        285 => {
            c.w64(0, c.r64(9));
            let Some(t263) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000895afa); };
            c.w32(9, t263);
            let r264 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r264);
            c.w32(11, 0x2u64);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000895b07); };
            c.w64(15, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000895b0b); };
            c.w64(1, t266);
            blk = 287; continue;
        }
        286 => {
            c.w32(12, c.r64(3).wrapping_add(0x2));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000895b28); }
            let r267 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r267);
            c.w64(3, c.r64(12));
            blk = 287; continue;
        }
        287 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ge() { blk = 295; continue; }
            blk = 288; continue;
        }
        288 => {
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t268) = c.ld64(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000895b40); };
            c.w64(12, t268);
            blk = 291; continue;
        }
        290 => {
            let Some(t269) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000895b46); };
            c.w64(12, t269);
            blk = 291; continue;
        }
        291 => {
            let Some(t270) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000895b4d); };
            c.w32(2, t270);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_ae() { blk = 296; continue; }
            blk = 292; continue;
        }
        292 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 294; continue; }
            blk = 293; continue;
        }
        293 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000895b5b); }
            blk = 286; continue;
        }
        294 => {
            let r271 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r271);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 286; continue;
        }
        295 => {
            let Some(t272) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000895b72); };
            c.w64(9, (t272 as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000895b76); }
            blk = 421; continue;
        }
        296 => {
            return RegfileExit::fault(0x7000895b7f);
        }
        297 => {
            return RegfileExit::fault(0x7000895b80);
        }
        298 => {
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895b81); };
            c.w64(11, t273);
            let Some(t274) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000895b85); };
            c.w32(12, t274);
            let r275 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r275);
            let Some(t276) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000895b8c); };
            c.w32(15, t276);
            let Some(t277) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000895b91); };
            c.cmp(t277, c.r32(15), 32);
            if c.cc_ne() { blk = 325; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t278) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000895b9e); };
            c.w32(15, t278);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 324; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t279) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000895bac); };
            c.w32(3, t279);
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_a() { blk = 302; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_ae() { blk = 305; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t280) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000895bc5); };
            c.w64(2, t280);
            let Some(t281) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000895bcc); };
            c.w32(2, t281);
            let r282 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r282);
            return c.call(0x7000824234, 0x7000895bda, 0x7000895bd5);
        }
        303 => {
            return RegfileExit::fault(0x7000895bda);
        }
        304 => {
            return RegfileExit::fault(0x7000895bdb);
        }
        305 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 320; continue; }
            blk = 306; continue;
        }
        306 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 703; continue; }
            blk = 307; continue;
        }
        307 => {
            let r283 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r283);
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(1, c.r32(1));
            let r284 = c.imul2(c.r32(1), 0x4u64, 32);
            c.w32(1, r284);
            if c.cc_o() { blk = 319; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(6, 0x8u64);
            let r285 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r285);
            if c.cc_o() { blk = 318; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t286) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000895c1a); };
            c.w64(7, t286);
            let Some(t287) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000895c21); };
            c.w64(7, t287);
            c.w64(8, c.r64(7).wrapping_add(c.r64(1)));
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000895c28); };
            c.w64(0, t288);
            let Some(t289) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000895c2f); };
            c.cmp(t289, c.r64(8), 64);
            if c.cc_be() { blk = 701; continue; }
            blk = 310; continue;
        }
        310 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 701; continue; }
            blk = 311; continue;
        }
        311 => {
            let Some(t290) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000895c45); };
            c.w64(0, t290);
            if !c.st64(c.r64(0), c.r64(8)) { return RegfileExit::fault(0x7000895c4c); }
            c.w64(0, c.r64(7).wrapping_add(0x1));
            blk = 312; continue;
        }
        312 => {
            let Some(t291) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000895c53); };
            c.w64(1, t291);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000895c5a); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000895c5d); }
            c.w64(3, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(6));
            blk = 314; continue;
        }
        313 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000895c6d); }
            let r292 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r292);
            blk = 314; continue;
        }
        314 => {
            c.cmp(c.r64(3), c.r64(6), 64);
            if c.cc_e() { blk = 321; continue; }
            blk = 315; continue;
        }
        315 => {
            c.w64(1, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_e() { blk = 317; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t293) = c.ld32(c.r64(12).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000895c87); };
            c.w32(1, t293);
            let r294 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r294);
            blk = 313; continue;
        }
        317 => {
            let Some(t295) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000895c92); };
            c.w64(1, t295);
            blk = 313; continue;
        }
        318 => {
            return RegfileExit::fault(0x7000895c9b);
        }
        319 => {
            return RegfileExit::fault(0x7000895c9c);
        }
        320 => {
            let Some(t296) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000895c9d); };
            c.w64(0, t296);
            c.w64(12, c.r64(0));
            blk = 321; continue;
        }
        321 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000895ca7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 323; continue; }
            blk = 322; continue;
        }
        322 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r297 = c.r64(12) & c.r64(11);
            c.w64(12, r297);
            c.logic(r297, 64);
            let Some(t298) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000895cb9); };
            c.logic(t298 & 0x4u64, 8);
            if c.cc_ne() { blk = 1036; continue; }
            blk = 323; continue;
        }
        323 => {
            blk = 325; continue;
        }
        324 => {
            return RegfileExit::fault(0x7000895cc7);
        }
        325 => {
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000895cc8); };
            c.w64(12, t299);
            let Some(t300) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000895ccc); };
            c.cmp(c.r32(12), t300, 32);
            if c.cc_ne() { blk = 327; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895cd2); };
            c.w64(3, t301);
            blk = 405; continue;
        }
        327 => {
            let Some(t302) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000895cde); };
            c.w32(15, t302);
            let r303 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r303);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000895ce5); }
            let Some(t304) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000895ce9); };
            c.cmp(c.r32(12), t304, 32);
            if c.cc_l() { blk = 382; continue; }
            blk = 328; continue;
        }
        328 => {
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000895cf3); };
            c.w64(0, t305);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_l() { blk = 330; continue; }
            blk = 329; continue;
        }
        329 => {
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895cfc); };
            c.w64(3, t306);
            blk = 405; continue;
        }
        330 => {
            let Some(t307) = c.ld32(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895d08); };
            c.w32(3, t307);
            let Some(t308) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000895d0e); };
            c.w32(2, t308);
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_le() { blk = 365; continue; }
            blk = 331; continue;
        }
        331 => {
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895d1b); };
            c.w64(3, t309);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_a() { blk = 333; continue; }
            blk = 332; continue;
        }
        332 => {
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_ae() { blk = 336; continue; }
            blk = 333; continue;
        }
        333 => {
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000895d32); };
            c.w64(2, t310);
            let Some(t311) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000895d39); };
            c.w32(2, t311);
            let r312 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r312);
            return c.call(0x7000824234, 0x7000895d47, 0x7000895d42);
        }
        334 => {
            return RegfileExit::fault(0x7000895d47);
        }
        335 => {
            return RegfileExit::fault(0x7000895d48);
        }
        336 => {
            c.w64(2, c.r64(9));
            let r313 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r313);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r314 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r314);
            c.logic(r314, 64);
            c.w32(2, c.r32(2));
            c.w32(2, c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x20));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 351; continue; }
            blk = 337; continue;
        }
        337 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 708; continue; }
            blk = 338; continue;
        }
        338 => {
            let r315 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r315);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(2, c.r32(2));
            let r316 = c.imul2(c.r32(2), 0x4u64, 32);
            c.w32(2, r316);
            if c.cc_o() { blk = 350; continue; }
            blk = 339; continue;
        }
        339 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, 0x8u64);
            let r317 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r317);
            if c.cc_o() { blk = 349; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000895d9d); };
            c.w64(6, t318);
            let Some(t319) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000895da4); };
            c.w64(6, t319);
            c.w64(7, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000895dab); };
            c.w64(8, t320);
            let Some(t321) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000895db2); };
            c.cmp(t321, c.r64(7), 64);
            if c.cc_be() { blk = 706; continue; }
            blk = 341; continue;
        }
        341 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 706; continue; }
            blk = 342; continue;
        }
        342 => {
            let Some(t322) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000895dc8); };
            c.w64(2, t322);
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x7000895dcf); }
            c.w64(2, c.r64(6).wrapping_add(0x1));
            blk = 343; continue;
        }
        343 => {
            c.w64(6, c.r64(9));
            let r323 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r323);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let r324 = c.r64(6) & 0xfffffffffffffffeu64;
            c.w64(6, r324);
            c.logic(r324, 64);
            c.w32(6, c.r32(6));
            c.w32(6, c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x20));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r325 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r325);
            c.w64(7, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t326) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000895df4); };
            c.w64(8, t326);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000895dfb); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000895dff); }
            c.w64(6, c.r64(7).wrapping_mul(2).wrapping_add(0x8));
            c.w64(7, c.r64(1));
            blk = 345; continue;
        }
        344 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000895e0f); }
            let r327 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r327);
            blk = 345; continue;
        }
        345 => {
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_e() { blk = 352; continue; }
            blk = 346; continue;
        }
        346 => {
            c.w64(8, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(8).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 348; continue; }
            blk = 347; continue;
        }
        347 => {
            let Some(t328) = c.ld32(c.r64(15).wrapping_add(c.r64(7)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000895e2d); };
            c.w32(8, t328);
            let r329 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r329);
            blk = 344; continue;
        }
        348 => {
            let Some(t330) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000895e38); };
            c.w64(8, t330);
            blk = 344; continue;
        }
        349 => {
            return RegfileExit::fault(0x7000895e41);
        }
        350 => {
            return RegfileExit::fault(0x7000895e42);
        }
        351 => {
            let Some(t331) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000895e43); };
            c.w64(2, t331);
            blk = 352; continue;
        }
        352 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x7000895e4a); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 354; continue; }
            blk = 353; continue;
        }
        353 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r332 = c.r64(1) & c.r64(11);
            c.w64(1, r332);
            c.logic(r332, 64);
            let Some(t333) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000895e5d); };
            c.logic(t333 & 0x4u64, 8);
            if c.cc_ne() { blk = 1033; continue; }
            blk = 354; continue;
        }
        354 => {
            let Some(t334) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000895e67); };
            c.w32(1, t334);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_le() { blk = 405; continue; }
            blk = 355; continue;
        }
        355 => {
            c.w32(1, c.r32(3));
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000895e76); };
            c.w64(6, t335);
            let Some(t336) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000895e7d); };
            let r337 = c.sub(c.r32(6), t336, 32);
            c.w32(6, r337);
            let r338 = c.add(c.r32(0), c.r32(3), 32);
            c.w32(0, r338);
            let r339 = c.sub(c.r32(6), c.r32(1), 32);
            c.w32(6, r339);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(2));
            let r340 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r340);
            c.logic(r340, 64);
            c.w64(7, c.r64(0).wrapping_mul(2).wrapping_add(0x7));
            let Some(t341) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000895e9c); };
            c.logic(t341 & 0x4u64, 8);
            if c.cc_e() { blk = 362; continue; }
            blk = 356; continue;
        }
        356 => {
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r342 = c.add(c.r64(1), c.r64(0), 64);
            c.w64(1, r342);
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_e() { blk = 364; continue; }
            blk = 357; continue;
        }
        357 => {
            c.w32(6, c.r64(3).wrapping_add(c.r64(12)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r343 = c.sub(c.r64(6), c.r64(0), 64);
            c.w64(6, r343);
            blk = 358; continue;
        }
        358 => {
            let Some(t344) = c.ld32(c.r64(15).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000895ec9); };
            c.w32(0, t344);
            c.w64(8, c.r64(7).wrapping_add(c.r64(6).wrapping_mul(4)));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000895ed1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 360; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r345 = c.r64(9) & c.r64(2);
            c.w64(9, r345);
            c.logic(r345, 64);
            let Some(t346) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000895ee3); };
            c.logic(t346 & 0x4u64, 8);
            if c.cc_ne() { blk = 1030; continue; }
            blk = 360; continue;
        }
        360 => {
            let r347 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r347);
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 358; continue; }
            blk = 361; continue;
        }
        361 => {
            blk = 364; continue;
        }
        362 => {
            c.w32(0, c.r64(3).wrapping_add(c.r64(12)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(1).wrapping_add(c.r64(1)));
            let r348 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r348);
            c.w64(1, c.r64(2).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r349 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r349);
            let r350 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r350);
            c.logic(r350, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000895f17); }
            c.w64(2, c.r64(15));
            let Some(t351) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000895f1f); };
            c.w64(0, t351);
            c.w64(10, 0x7000895f2du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000895f2d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000895f31); }
            return c.call(c.r64(0), 0x7000895f37, 0x7000895f35);
        }
        363 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000895f37); }
            let Some(t352) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000895f3f); };
            c.w64(4, t352);
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000895f44); };
            c.w64(12, t353);
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895f48); };
            c.w64(3, t354);
            blk = 364; continue;
        }
        364 => {
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000895f4f); };
            c.w64(11, t355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000895f53); };
            c.w64(9, t356);
            blk = 405; continue;
        }
        365 => {
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000895f5c); };
            c.w64(2, t357);
            c.w32(1, c.r64(2).wrapping_add(c.r64(0)));
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000895f66); };
            c.w64(6, t358);
            let Some(t359) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000895f6d); };
            let r360 = c.sub(c.r32(6), t359, 32);
            c.w32(6, r360);
            c.w32(7, c.r64(2).wrapping_add(c.r64(12)));
            let r361 = c.sub(c.r32(6), c.r32(3), 32);
            c.w32(6, r361);
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(15));
            let r362 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r362);
            c.logic(r362, 64);
            c.w64(8, c.r64(3).wrapping_mul(2).wrapping_add(0x7));
            let Some(t363) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000895f91); };
            c.logic(t363 & 0x4u64, 8);
            if c.cc_e() { blk = 379; continue; }
            blk = 366; continue;
        }
        366 => {
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r364 = c.sub(c.r64(1), c.r64(3), 64);
            c.w64(1, r364);
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if let Some(sr) = c.shl(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            let r365 = c.add(c.r64(3), c.r64(6), 64);
            c.w64(3, r365);
            c.w64(3, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_l() { blk = 373; continue; }
            blk = 367; continue;
        }
        367 => {
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_e() { blk = 381; continue; }
            blk = 368; continue;
        }
        368 => {
            let r366 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r366);
            c.w32(1, c.r64(2).wrapping_add(c.r64(12)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r367 = c.sub(c.r64(1), c.r64(0), 64);
            c.w64(1, r367);
            blk = 369; continue;
        }
        369 => {
            let r368 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r368);
            let Some(t369) = c.ld32(c.r64(3).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000895fde); };
            c.w32(0, t369);
            c.w64(6, c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)));
            if !c.st32(c.r64(15).wrapping_add(c.r64(6)), c.r32(0)) { return RegfileExit::fault(0x7000895fe6); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 371; continue; }
            blk = 370; continue;
        }
        370 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r370 = c.r64(7) & c.r64(15);
            c.w64(7, r370);
            c.logic(r370, 64);
            let Some(t371) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000895ff8); };
            c.logic(t371 & 0x4u64, 8);
            if c.cc_ne() { blk = 1027; continue; }
            blk = 371; continue;
        }
        371 => {
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_ne() { blk = 369; continue; }
            blk = 372; continue;
        }
        372 => {
            blk = 381; continue;
        }
        373 => {
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_e() { blk = 381; continue; }
            blk = 374; continue;
        }
        374 => {
            let r372 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r372);
            c.w32(1, c.r64(2).wrapping_add(c.r64(12)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r373 = c.sub(c.r64(1), c.r64(0), 64);
            c.w64(1, r373);
            blk = 375; continue;
        }
        375 => {
            let Some(t374) = c.ld32(c.r64(15).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700089602a); };
            c.w32(0, t374);
            c.w64(6, c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)));
            if !c.st32(c.r64(15).wrapping_add(c.r64(6)), c.r32(0)) { return RegfileExit::fault(0x7000896032); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 376; continue;
        }
        376 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r375 = c.r64(7) & c.r64(15);
            c.w64(7, r375);
            c.logic(r375, 64);
            let Some(t376) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000896044); };
            c.logic(t376 & 0x4u64, 8);
            if c.cc_ne() { blk = 1024; continue; }
            blk = 377; continue;
        }
        377 => {
            let r377 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r377);
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_ne() { blk = 375; continue; }
            blk = 378; continue;
        }
        378 => {
            blk = 381; continue;
        }
        379 => {
            c.w64(0, c.r64(6).wrapping_add(c.r64(6)));
            c.w64(3, c.r64(8).wrapping_add(c.r64(15)));
            c.w64(1, c.r64(15).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r378 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r378);
            let r379 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r379);
            c.logic(r379, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000896071); }
            c.w64(8, c.r64(0));
            c.w64(2, c.r64(3));
            let Some(t380) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x700089607c); };
            c.w64(0, t380);
            c.w64(10, 0x700089608au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089608a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089608e); }
            return c.call(c.r64(0), 0x7000896094, 0x7000896092);
        }
        380 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000896094); }
            let Some(t381) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089609c); };
            c.w64(4, t381);
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008960a1); };
            c.w64(11, t382);
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008960a5); };
            c.w64(12, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008960a9); };
            c.w64(2, t384);
            blk = 381; continue;
        }
        381 => {
            c.w64(3, c.r64(2));
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008960b3); };
            c.w64(9, t385);
            blk = 405; continue;
        }
        382 => {
            let Some(t386) = c.ld32(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008960bc); };
            c.w32(0, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008960c2); };
            c.w64(2, t387);
            let Some(t388) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008960c9); };
            let r389 = c.sub(c.r32(2), t388, 32);
            c.w32(2, r389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008960cc); };
            c.w64(3, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008960d0); };
            c.w64(1, t391);
            c.w32(6, c.r64(1).wrapping_add(c.r64(3)));
            let r392 = c.sub(c.r32(2), c.r32(0), 32);
            c.w32(2, r392);
            c.w32(0, c.r64(1).wrapping_add(c.r64(12)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(15));
            let r393 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r393);
            c.logic(r393, 64);
            c.w64(8, c.r64(6).wrapping_mul(2).wrapping_add(0x7));
            let Some(t394) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008960fb); };
            c.logic(t394 & 0x4u64, 8);
            if c.cc_e() { blk = 397; continue; }
            blk = 383; continue;
        }
        383 => {
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r395 = c.sub(c.r64(0), c.r64(6), 64);
            c.w64(0, r395);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if let Some(sr) = c.shl(c.r64(0), 0x2u64, 64) { c.w64(0, sr); }
            let r396 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r396);
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_l() { blk = 390; continue; }
            blk = 384; continue;
        }
        384 => {
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_e() { blk = 399; continue; }
            blk = 385; continue;
        }
        385 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(3)));
            c.w32(3, c.r64(1).wrapping_add(c.r64(12)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r397 = c.sub(c.r64(3), c.r64(0), 64);
            c.w64(3, r397);
            blk = 386; continue;
        }
        386 => {
            let r398 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r398);
            let Some(t399) = c.ld32(c.r64(2).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000896149); };
            c.w32(0, t399);
            c.w64(6, c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)));
            if !c.st32(c.r64(15).wrapping_add(c.r64(6)), c.r32(0)) { return RegfileExit::fault(0x7000896151); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 388; continue; }
            blk = 387; continue;
        }
        387 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r400 = c.r64(7) & c.r64(15);
            c.w64(7, r400);
            c.logic(r400, 64);
            let Some(t401) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000896163); };
            c.logic(t401 & 0x4u64, 8);
            if c.cc_ne() { blk = 1021; continue; }
            blk = 388; continue;
        }
        388 => {
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_ne() { blk = 386; continue; }
            blk = 389; continue;
        }
        389 => {
            blk = 399; continue;
        }
        390 => {
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_e() { blk = 399; continue; }
            blk = 391; continue;
        }
        391 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(3)));
            c.w32(3, c.r64(1).wrapping_add(c.r64(12)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r402 = c.sub(c.r64(3), c.r64(0), 64);
            c.w64(3, r402);
            blk = 393; continue;
        }
        392 => {
            c.w64(8, c.r64(0));
            blk = 393; continue;
        }
        393 => {
            c.w64(0, c.r64(8).wrapping_add(0x4));
            let Some(t403) = c.ld32(c.r64(15).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700089619f); };
            c.w32(6, t403);
            c.w64(7, c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)));
            if !c.st32(c.r64(15).wrapping_add(c.r64(7)), c.r32(6)) { return RegfileExit::fault(0x70008961a7); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 395; continue; }
            blk = 394; continue;
        }
        394 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r404 = c.r64(8) & c.r64(15);
            c.w64(8, r404);
            c.logic(r404, 64);
            let Some(t405) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008961bb); };
            c.logic(t405 & 0x4u64, 8);
            if c.cc_ne() { blk = 1018; continue; }
            blk = 395; continue;
        }
        395 => {
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 392; continue; }
            blk = 396; continue;
        }
        396 => {
            blk = 399; continue;
        }
        397 => {
            c.w64(3, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(2, c.r64(8).wrapping_add(c.r64(15)));
            c.w64(0, c.r64(15).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r406 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r406);
            let r407 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r407);
            c.logic(r407, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008961e5); }
            c.w64(1, c.r64(0));
            c.w64(8, c.r64(3));
            let Some(t408) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x70008961f0); };
            c.w64(0, t408);
            c.w64(10, 0x70008961feu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008961fe); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000896202); }
            return c.call(c.r64(0), 0x7000896208, 0x7000896206);
        }
        398 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000896208); }
            let Some(t409) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000896210); };
            c.w64(4, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896215); };
            c.w64(11, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000896219); };
            c.w64(12, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700089621d); };
            c.w64(1, t412);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000896224); };
            c.w64(15, t413);
            blk = 399; continue;
        }
        399 => {
            let Some(t414) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896228); };
            c.w64(9, (t414 as u32 as i32 as i64 as u64));
            blk = 401; continue;
        }
        400 => {
            c.w32(0, c.r64(9).wrapping_add(0x2));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000896238); };
            c.w64(3, t415);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700089623f); }
            c.w64(9, c.r64(0));
            blk = 401; continue;
        }
        401 => {
            let Some(t416) = c.ld32(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000896247); };
            c.cmp(c.r32(9), t416, 32);
            if c.cc_ge() { blk = 404; continue; }
            blk = 402; continue;
        }
        402 => {
            let Some(t417) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000896250); };
            c.w32(0, t417);
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_b() { blk = 400; continue; }
            blk = 403; continue;
        }
        403 => {
            blk = 419; continue;
        }
        404 => {
            c.w64(3, c.r64(1));
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896261); };
            c.w64(9, t418);
            blk = 405; continue;
        }
        405 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_g() { blk = 407; continue; }
            blk = 406; continue;
        }
        406 => {
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089626b); };
            c.w64(15, t419);
            c.w64(0, c.r64(11));
            blk = 417; continue;
        }
        407 => {
            c.w64(0, c.r64(11));
            let Some(t420) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089627a); };
            c.w32(11, t420);
            let r421 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r421);
            c.w32(12, 0x2u64);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000896287); };
            c.w64(15, t422);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x700089628b); };
            c.w64(7, t423);
            blk = 411; continue;
        }
        408 => {
            c.w32(1, c.r64(3).wrapping_add(0x2));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(11).wrapping_add(c.r64(3)), c.r32(2)) { return RegfileExit::fault(0x70008962a5); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 410; continue; }
            blk = 409; continue;
        }
        409 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r424 = c.r64(6) & c.r64(11);
            c.w64(6, r424);
            c.logic(r424, 64);
            let Some(t425) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008962b8); };
            c.logic(t425 & 0x4u64, 8);
            if c.cc_ne() { blk = 1015; continue; }
            blk = 410; continue;
        }
        410 => {
            let r426 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r426);
            c.w64(3, c.r64(1));
            blk = 411; continue;
        }
        411 => {
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_ge() { blk = 417; continue; }
            blk = 412; continue;
        }
        412 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 414; continue; }
            blk = 413; continue;
        }
        413 => {
            let Some(t427) = c.ld64(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(8))) else { return RegfileExit::fault(0x70008962d3); };
            c.w64(2, t427);
            blk = 415; continue;
        }
        414 => {
            let Some(t428) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008962d9); };
            c.w64(2, t428);
            blk = 415; continue;
        }
        415 => {
            let Some(t429) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008962e0); };
            c.w32(1, t429);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_b() { blk = 408; continue; }
            blk = 416; continue;
        }
        416 => {
            blk = 418; continue;
        }
        417 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008962ed); }
            blk = 421; continue;
        }
        418 => {
            return RegfileExit::fault(0x70008962f3);
        }
        419 => {
            return RegfileExit::fault(0x70008962f4);
        }
        420 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008962f5); }
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008962f9); };
            c.w64(9, t430);
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008962fd); };
            c.w64(11, t431);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000896304); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700089630b); }
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089630f); };
            c.w64(15, t432);
            blk = 421; continue;
        }
        421 => {
            c.w64(9, c.r64(15).wrapping_add(0x1));
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000896317); };
            c.w64(0, t433);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089631e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089631f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000896325); }
            return c.ret(0x0, 0x7000896327);
        }
        422 => {
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000896328); };
            c.w64(9, t434);
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700089632c); };
            c.w64(11, t435);
            c.w64(12, c.r64(9));
            blk = 423; continue;
        }
        423 => {
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896336); };
            if !c.push64(t436) { return RegfileExit::fault(0x7000896336); }
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000896339); };
            c.w64(11, t437);
            c.w32(0, 0x1u64);
            let Some(t438) = c.ld64(c.r64(13).wrapping_add(0x1c80)) else { return RegfileExit::fault(0x7000896342); };
            c.w64(3, t438);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x7000896351, 0x700089634c);
        }
        424 => {
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000896351); };
            if !c.push64(t439) { return RegfileExit::fault(0x7000896351); }
            let Some(t440) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000896354); };
            c.w64(9, t440);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700089635b); }
            c.w64(7, c.r64(0));
            c.w64(2, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896368); };
            c.w64(6, t441);
            return c.call(0x70007f7214, 0x7000896371, 0x700089636c);
        }
        425 => {
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000896374); }
            blk = 426; continue;
        }
        426 => {
            c.w32(15, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000896385); };
            c.w64(12, t442);
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000896397); };
            c.w64(11, t443);
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(15)) { return RegfileExit::fault(0x700089639e); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(3)) { return RegfileExit::fault(0x70008963a5); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 430; continue; }
            blk = 427; continue;
        }
        427 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008963b1); }
            let Some(t444) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008963b7); };
            c.cmp(t444, 0x0u64, 32);
            if c.cc_ne() { blk = 429; continue; }
            blk = 428; continue;
        }
        428 => {
            if !c.movsd_rm(2, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008963bd); }
            blk = 432; continue;
        }
        429 => {
            c.w64(2, c.r64(12));
            let r445 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r445);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(2) as u32 as i32 as i64));
            blk = 432; continue;
        }
        430 => {
            let Some(t446) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008963d5); };
            c.cmp(t446, 0x0u64, 32);
            if c.cc_ne() { blk = 434; continue; }
            blk = 431; continue;
        }
        431 => {
            c.w64(2, c.r64(9));
            let r447 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r447);
            if !c.movsd_rm(2, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008963e0); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 432; continue;
        }
        432 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_a() { blk = 435; continue; }
            blk = 433; continue;
        }
        433 => {
            blk = 446; continue;
        }
        434 => {
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_ge() { blk = 446; continue; }
            blk = 435; continue;
        }
        435 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x7000896416); }
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(2)) { return RegfileExit::fault(0x700089641d); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 710; continue; }
            blk = 436; continue;
        }
        436 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 710; continue; }
            blk = 437; continue;
        }
        437 => {
            c.w32(6, c.r32(11));
            let r448 = c.add(c.r32(6), c.r32(15), 32);
            c.w32(6, r448);
            if c.cc_o() { blk = 710; continue; }
            blk = 438; continue;
        }
        438 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 439; continue;
        }
        439 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(1)) { return RegfileExit::fault(0x7000896445); }
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089644c); };
            c.w64(0, t449);
            c.w64(3, c.r64(1));
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896453); };
            c.w64(6, t450);
            return c.call(0x7000820354, 0x700089645c, 0x7000896457);
        }
        440 => {
            let Some(t451) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089645c); };
            c.cmp(t451, c.r32(0), 32);
            if c.cc_ne() { blk = 443; continue; }
            blk = 441; continue;
        }
        441 => {
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896465); };
            c.w64(0, t452);
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000896469); };
            c.w64(3, t453);
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896470); };
            c.w64(6, t454);
            return c.call(0x7000870914, 0x7000896479, 0x7000896474);
        }
        442 => {
            c.w64(1, c.r64(0));
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089647c); };
            c.w64(0, t455);
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000896480); };
            c.w64(3, t456);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896487); };
            c.w64(6, t457);
            return c.call(0x7000899bb4, 0x7000896490, 0x700089648b);
        }
        443 => {
            let Some(t458) = c.ld32(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000896490); };
            c.cmp(t458, 0x0u64, 32);
            if c.cc_ne() { blk = 727; continue; }
            blk = 444; continue;
        }
        444 => {
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x700089649d); };
            c.w64(9, t459);
            let r460 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r460);
            if c.cc_o() { blk = 727; continue; }
            blk = 445; continue;
        }
        445 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 426; continue;
        }
        446 => {
            let Some(t461) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008964b6); };
            c.w64(3, t461);
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008964bd); };
            c.w64(0, t462);
            c.w64(1, c.r64(12));
            let Some(t463) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008964c4); };
            c.w64(6, t463);
            return c.call(0x7000871cb4, 0x70008964cd, 0x70008964c8);
        }
        447 => {
            let Some(t464) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008964cd); };
            c.cmp(t464, 0x0u64, 32);
            if c.cc_ne() { blk = 451; continue; }
            blk = 448; continue;
        }
        448 => {
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008964d3); };
            c.w64(11, t465);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008964d7); }
            let Some(t466) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008964dd); };
            c.cmp(t466, 0x0u64, 32);
            if c.cc_ne() { blk = 450; continue; }
            blk = 449; continue;
        }
        449 => {
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008964e3); };
            c.w64(12, t467);
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008964e7); }
            blk = 453; continue;
        }
        450 => {
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008964f0); };
            c.w64(12, t468);
            let r469 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r469);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000896500); };
            c.w64(12, t470);
            blk = 453; continue;
        }
        451 => {
            let Some(t471) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000896506); };
            c.cmp(t471, 0x0u64, 32);
            if c.cc_ne() { blk = 455; continue; }
            blk = 452; continue;
        }
        452 => {
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089650c); };
            c.w64(11, t472);
            let r473 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r473);
            let Some(t474) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000896513); };
            c.w64(12, t474);
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896517); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000896527); };
            c.w64(11, t475);
            blk = 453; continue;
        }
        453 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 506; continue; }
            blk = 454; continue;
        }
        454 => {
            blk = 457; continue;
        }
        455 => {
            let Some(t476) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000896537); };
            c.w64(11, t476);
            let Some(t477) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089653b); };
            c.cmp(c.r32(11), t477, 32);
            if c.cc_l() { blk = 505; continue; }
            blk = 456; continue;
        }
        456 => {
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000896545); };
            c.w64(12, t478);
            blk = 457; continue;
        }
        457 => {
            let Some(t479) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000896549); };
            c.cmp(t479, 0x0u64, 32);
            if c.cc_ne() { blk = 461; continue; }
            blk = 458; continue;
        }
        458 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700089654f); }
            let Some(t480) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000896556); };
            c.cmp(t480, 0x0u64, 32);
            if c.cc_ne() { blk = 460; continue; }
            blk = 459; continue;
        }
        459 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700089655c); }
            blk = 463; continue;
        }
        460 => {
            c.w64(9, c.r64(11));
            let r481 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r481);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 463; continue;
        }
        461 => {
            let Some(t482) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000896575); };
            c.cmp(t482, 0x0u64, 32);
            if c.cc_ne() { blk = 465; continue; }
            blk = 462; continue;
        }
        462 => {
            c.w64(9, c.r64(12));
            let r483 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r483);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896581); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 463; continue;
        }
        463 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 467; continue; }
            blk = 464; continue;
        }
        464 => {
            blk = 466; continue;
        }
        465 => {
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_l() { blk = 467; continue; }
            blk = 466; continue;
        }
        466 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008965ab); };
            c.w64(0, t484);
            blk = 565; continue;
        }
        467 => {
            let Some(t485) = c.ld32(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008965b7); };
            c.cmp(t485, 0x0u64, 32);
            if c.cc_ne() { blk = 742; continue; }
            blk = 468; continue;
        }
        468 => {
            let Some(t486) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008965c4); };
            c.cmp(t486, 0x0u64, 32);
            if c.cc_e() { blk = 742; continue; }
            blk = 469; continue;
        }
        469 => {
            let Some(t487) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008965ce); };
            c.w64(9, t487);
            let Some(t488) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008965d2); };
            let r489 = c.sub(c.r32(9), t488, 32);
            c.w32(9, r489);
            if c.cc_o() { blk = 742; continue; }
            blk = 470; continue;
        }
        470 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 471; continue;
        }
        471 => {
            let Some(t490) = c.ld8(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008965df); };
            c.logic(t490 & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(15)) { return RegfileExit::fault(0x70008965ee); }
            blk = 473; continue;
        }
        472 => {
            let Some(t491) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008965f7); };
            c.w64(12, t491);
            let Some(t492) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008965fb); };
            c.w64(11, t492);
            blk = 473; continue;
        }
        473 => {
            c.w32(3, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t493) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700089661a); };
            c.w64(0, t493);
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(3)) { return RegfileExit::fault(0x7000896621); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(2)) { return RegfileExit::fault(0x7000896628); }
            let Some(t494) = c.ld32(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x700089662f); };
            c.cmp(t494, 0x0u64, 32);
            if c.cc_ne() { blk = 477; continue; }
            blk = 474; continue;
        }
        474 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896638); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 476; continue; }
            blk = 475; continue;
        }
        475 => {
            if !c.movsd_rm(2, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896642); }
            blk = 479; continue;
        }
        476 => {
            c.w64(1, c.r64(9));
            let r495 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r495);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            blk = 479; continue;
        }
        477 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 481; continue; }
            blk = 478; continue;
        }
        478 => {
            c.w64(1, c.r64(0));
            let r496 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r496);
            if !c.movsd_rm(2, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896663); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 479; continue;
        }
        479 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_a() { blk = 482; continue; }
            blk = 480; continue;
        }
        480 => {
            blk = 565; continue;
        }
        481 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ge() { blk = 565; continue; }
            blk = 482; continue;
        }
        482 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x700089668e); }
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(1)) { return RegfileExit::fault(0x7000896695); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 760; continue; }
            blk = 483; continue;
        }
        483 => {
            let Some(t497) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008966a5); };
            c.cmp(t497, 0x0u64, 32);
            if c.cc_e() { blk = 760; continue; }
            blk = 484; continue;
        }
        484 => {
            c.w64(6, c.r64(3));
            let Some(t498) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008966b2); };
            let r499 = c.add(c.r32(6), t498, 32);
            c.w32(6, r499);
            if c.cc_o() { blk = 760; continue; }
            blk = 485; continue;
        }
        485 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 486; continue;
        }
        486 => {
            c.w32(7, c.r32(6));
            c.logic(c.r8(6) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 777; continue; }
            blk = 487; continue;
        }
        487 => {
            let r500 = c.sub(c.r32(7), 0x2u64, 32);
            c.w32(7, r500);
            if c.cc_o() { blk = 777; continue; }
            blk = 488; continue;
        }
        488 => {
            c.w64(6, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 489; continue;
        }
        489 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc8), c.r64(6)) { return RegfileExit::fault(0x70008966e2); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 791; continue; }
            blk = 490; continue;
        }
        490 => {
            let Some(t501) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008966f2); };
            c.cmp(t501, 0x0u64, 32);
            if c.cc_e() { blk = 791; continue; }
            blk = 491; continue;
        }
        491 => {
            c.w64(7, c.r64(3));
            let Some(t502) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008966ff); };
            let r503 = c.add(c.r32(7), t502, 32);
            c.w32(7, r503);
            if c.cc_o() { blk = 791; continue; }
            blk = 492; continue;
        }
        492 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 493; continue;
        }
        493 => {
            c.w32(8, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 808; continue; }
            blk = 494; continue;
        }
        494 => {
            let r504 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r504);
            if c.cc_o() { blk = 808; continue; }
            blk = 495; continue;
        }
        495 => {
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 496; continue;
        }
        496 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(9)) { return RegfileExit::fault(0x7000896731); }
            let Some(t505) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896738); };
            c.w64(0, t505);
            c.w64(3, c.r64(6));
            let Some(t506) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089673f); };
            c.w64(6, t506);
            return c.call(0x7000820354, 0x7000896748, 0x7000896743);
        }
        497 => {
            let Some(t507) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000896748); };
            c.cmp(t507, c.r32(0), 32);
            if c.cc_e() { blk = 500; continue; }
            blk = 498; continue;
        }
        498 => {
            c.w32(1, 0x2u64);
            let Some(t508) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896756); };
            c.w64(0, t508);
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700089675a); };
            c.w64(3, t509);
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896761); };
            c.w64(6, t510);
            return c.call(0x7000820e54, 0x700089676a, 0x7000896765);
        }
        499 => {
            blk = 502; continue;
        }
        500 => {
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089676c); };
            c.w64(0, t511);
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000896770); };
            c.w64(3, t512);
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896777); };
            c.w64(6, t513);
            return c.call(0x7000870914, 0x7000896780, 0x700089677b);
        }
        501 => {
            c.w64(1, c.r64(0));
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896783); };
            c.w64(0, t514);
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000896787); };
            c.w64(3, t515);
            let Some(t516) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089678e); };
            c.w64(6, t516);
            return c.call(0x7000871cb4, 0x7000896797, 0x7000896792);
        }
        502 => {
            let Some(t517) = c.ld32(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000896797); };
            c.cmp(t517, 0x0u64, 32);
            if c.cc_ne() { blk = 822; continue; }
            blk = 503; continue;
        }
        503 => {
            let Some(t518) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008967a4); };
            c.w64(9, t518);
            let r519 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r519);
            if c.cc_o() { blk = 822; continue; }
            blk = 504; continue;
        }
        504 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 472; continue;
        }
        505 => {
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008967bd); };
            c.w64(12, t520);
            blk = 506; continue;
        }
        506 => {
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008967c1); };
            c.w64(0, t521);
            blk = 508; continue;
        }
        507 => {
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008967ca); };
            c.w64(12, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008967ce); };
            c.w64(11, t523);
            blk = 508; continue;
        }
        508 => {
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008967d2); };
            c.w64(9, t524);
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t525) = c.ld32(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008967e4); };
            c.cmp(t525, 0x0u64, 32);
            if c.cc_ne() { blk = 837; continue; }
            blk = 509; continue;
        }
        509 => {
            let Some(t526) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008967f1); };
            c.cmp(t526, 0x0u64, 32);
            if c.cc_e() { blk = 837; continue; }
            blk = 510; continue;
        }
        510 => {
            let Some(t527) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008967fb); };
            c.w64(15, t527);
            let Some(t528) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008967ff); };
            let r529 = c.sub(c.r32(15), t528, 32);
            c.w32(15, r529);
            if c.cc_o() { blk = 837; continue; }
            blk = 511; continue;
        }
        511 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            blk = 512; continue;
        }
        512 => {
            c.w32(3, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(3)) { return RegfileExit::fault(0x7000896820); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(2)) { return RegfileExit::fault(0x7000896827); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 516; continue; }
            blk = 513; continue;
        }
        513 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896833); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 515; continue; }
            blk = 514; continue;
        }
        514 => {
            if !c.movsd_rm(2, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x700089683d); }
            blk = 518; continue;
        }
        515 => {
            let r530 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r530);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(15) as u32 as i32 as i64));
            blk = 518; continue;
        }
        516 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 520; continue; }
            blk = 517; continue;
        }
        517 => {
            c.w64(1, c.r64(0));
            let r531 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r531);
            if !c.movsd_rm(2, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x700089685d); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 518; continue;
        }
        518 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_a() { blk = 521; continue; }
            blk = 519; continue;
        }
        519 => {
            blk = 538; continue;
        }
        520 => {
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_ge() { blk = 538; continue; }
            blk = 521; continue;
        }
        521 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(0)) { return RegfileExit::fault(0x700089688a); }
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(15)) { return RegfileExit::fault(0x7000896891); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 854; continue; }
            blk = 522; continue;
        }
        522 => {
            let Some(t532) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008968a2); };
            c.cmp(t532, 0x0u64, 32);
            if c.cc_e() { blk = 854; continue; }
            blk = 523; continue;
        }
        523 => {
            c.w64(1, c.r64(3));
            let Some(t533) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008968af); };
            let r534 = c.add(c.r32(1), t533, 32);
            c.w32(1, r534);
            if c.cc_o() { blk = 854; continue; }
            blk = 524; continue;
        }
        524 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 525; continue;
        }
        525 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc8), c.r64(1)) { return RegfileExit::fault(0x70008968bb); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 871; continue; }
            blk = 526; continue;
        }
        526 => {
            let Some(t535) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008968cc); };
            c.cmp(t535, 0x0u64, 32);
            if c.cc_e() { blk = 871; continue; }
            blk = 527; continue;
        }
        527 => {
            c.w64(6, c.r64(3));
            let Some(t536) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008968d9); };
            let r537 = c.add(c.r32(6), t536, 32);
            c.w32(6, r537);
            if c.cc_o() { blk = 871; continue; }
            blk = 528; continue;
        }
        528 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 529; continue;
        }
        529 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(6)) { return RegfileExit::fault(0x70008968e5); }
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008968ec); };
            c.w64(0, t538);
            c.w64(3, c.r64(1));
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008968f3); };
            c.w64(6, t539);
            return c.call(0x7000820354, 0x70008968fc, 0x70008968f7);
        }
        530 => {
            let Some(t540) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008968fc); };
            c.cmp(t540, c.r32(0), 32);
            if c.cc_e() { blk = 533; continue; }
            blk = 531; continue;
        }
        531 => {
            c.w32(1, 0x2u64);
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089690a); };
            c.w64(0, t541);
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700089690e); };
            c.w64(3, t542);
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896915); };
            c.w64(6, t543);
            return c.call(0x7000820e54, 0x700089691e, 0x7000896919);
        }
        532 => {
            blk = 535; continue;
        }
        533 => {
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896920); };
            c.w64(0, t544);
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000896924); };
            c.w64(3, t545);
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089692b); };
            c.w64(6, t546);
            return c.call(0x7000870914, 0x7000896934, 0x700089692f);
        }
        534 => {
            c.w64(1, c.r64(0));
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896937); };
            c.w64(0, t547);
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700089693b); };
            c.w64(3, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896942); };
            c.w64(6, t549);
            return c.call(0x7000871cb4, 0x700089694b, 0x7000896946);
        }
        535 => {
            let Some(t550) = c.ld32(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x700089694b); };
            c.cmp(t550, 0x0u64, 32);
            if c.cc_ne() { blk = 888; continue; }
            blk = 536; continue;
        }
        536 => {
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000896958); };
            c.w64(9, t551);
            let r552 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r552);
            if c.cc_o() { blk = 888; continue; }
            blk = 537; continue;
        }
        537 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 507; continue;
        }
        538 => {
            c.w64(15, c.r64(11));
            c.w64(11, c.r64(9));
            blk = 540; continue;
        }
        539 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000896987); };
            c.w64(15, t553);
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089698b); };
            c.w64(11, t554);
            let Some(t555) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089698f); };
            c.w64(12, t555);
            blk = 540; continue;
        }
        540 => {
            let Some(t556) = c.ld32(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000896993); };
            c.cmp(t556, 0x0u64, 32);
            if c.cc_ne() { blk = 902; continue; }
            blk = 541; continue;
        }
        541 => {
            let Some(t557) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008969a0); };
            c.cmp(t557, 0x0u64, 32);
            if c.cc_e() { blk = 902; continue; }
            blk = 542; continue;
        }
        542 => {
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008969aa); };
            c.w64(0, t558);
            let Some(t559) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008969ae); };
            let r560 = c.sub(c.r32(0), t559, 32);
            c.w32(0, r560);
            if c.cc_o() { blk = 902; continue; }
            blk = 543; continue;
        }
        543 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 544; continue;
        }
        544 => {
            c.w32(3, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 919; continue; }
            blk = 545; continue;
        }
        545 => {
            let Some(t561) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008969cd); };
            c.cmp(t561, 0x0u64, 32);
            if c.cc_e() { blk = 919; continue; }
            blk = 546; continue;
        }
        546 => {
            let Some(t562) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008969d7); };
            let r563 = c.add(c.r32(3), t562, 32);
            c.w32(3, r563);
            if c.cc_o() { blk = 919; continue; }
            blk = 547; continue;
        }
        547 => {
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 548; continue;
        }
        548 => {
            c.w32(3, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(3)) { return RegfileExit::fault(0x70008969f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(2)) { return RegfileExit::fault(0x70008969f7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 552; continue; }
            blk = 549; continue;
        }
        549 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896a02); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 551; continue; }
            blk = 550; continue;
        }
        550 => {
            if !c.movsd_rm(2, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896a0c); }
            blk = 554; continue;
        }
        551 => {
            c.w64(0, c.r64(9));
            let r564 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r564);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 554; continue;
        }
        552 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 556; continue; }
            blk = 553; continue;
        }
        553 => {
            let r565 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r565);
            if !c.movsd_rm(2, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896a2a); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 554; continue;
        }
        554 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_a() { blk = 557; continue; }
            blk = 555; continue;
        }
        555 => {
            blk = 564; continue;
        }
        556 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ge() { blk = 564; continue; }
            blk = 557; continue;
        }
        557 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x7000896a4e); }
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(0)) { return RegfileExit::fault(0x7000896a55); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 936; continue; }
            blk = 558; continue;
        }
        558 => {
            c.w64(1, c.r64(3));
            let r566 = c.sub(c.r32(1), 0x2u64, 32);
            c.w32(1, r566);
            if c.cc_o() { blk = 936; continue; }
            blk = 559; continue;
        }
        559 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 560; continue;
        }
        560 => {
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896a74); };
            c.w64(6, t567);
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896a78); };
            c.w64(0, t568);
            c.w64(3, c.r64(1));
            c.w32(1, 0x2u64);
            return c.call(0x7000820e54, 0x7000896a89, 0x7000896a84);
        }
        561 => {
            let Some(t569) = c.ld32(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000896a89); };
            c.cmp(t569, 0x0u64, 32);
            if c.cc_ne() { blk = 950; continue; }
            blk = 562; continue;
        }
        562 => {
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000896a96); };
            c.w64(9, t570);
            let r571 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r571);
            if c.cc_o() { blk = 950; continue; }
            blk = 563; continue;
        }
        563 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 539; continue;
        }
        564 => {
            c.w64(11, c.r64(15));
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000896ab2); };
            c.w64(0, t572);
            blk = 565; continue;
        }
        565 => {
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000896ab9); };
            c.cmp(t573, 0x2u64, 64);
            if c.cc_g() { blk = 567; continue; }
            blk = 566; continue;
        }
        566 => {
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000896ac0); };
            c.w64(15, t574);
            blk = 577; continue;
        }
        567 => {
            c.w32(9, 0x2u64);
            blk = 569; continue;
        }
        568 => {
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000896ad1); };
            c.w64(9, t575);
            let r576 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r576);
            blk = 569; continue;
        }
        569 => {
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000896adc); };
            c.w64(15, t577);
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000896ae0); };
            c.w64(3, t578);
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_ge() { blk = 577; continue; }
            blk = 570; continue;
        }
        570 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x7000896aec); }
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(0)) { return RegfileExit::fault(0x7000896af3); }
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_be() { blk = 572; continue; }
            blk = 571; continue;
        }
        571 => {
            let Some(t579) = c.ld64(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x7000896aff); };
            c.w64(2, t579);
            blk = 573; continue;
        }
        572 => {
            let Some(t580) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000896b05); };
            c.w64(2, t580);
            blk = 573; continue;
        }
        573 => {
            c.w32(1, c.r32(0));
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(1)) { return RegfileExit::fault(0x7000896b0e); }
            c.w64(3, c.r64(0));
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896b18); };
            c.w64(0, t581);
            c.w64(1, c.r64(2));
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896b1f); };
            c.w64(6, t582);
            return c.call(0x7000871cb4, 0x7000896b28, 0x7000896b23);
        }
        574 => {
            let Some(t583) = c.ld8(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000896b28); };
            c.logic(t583 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 965; continue; }
            blk = 575; continue;
        }
        575 => {
            let Some(t584) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000896b41); };
            c.w64(11, t584);
            let r585 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r585);
            if c.cc_o() { blk = 965; continue; }
            blk = 576; continue;
        }
        576 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 568; continue;
        }
        577 => {
            let Some(t586) = c.ld32(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000896b5a); };
            c.cmp(t586, 0x0u64, 32);
            if c.cc_ne() { blk = 979; continue; }
            blk = 578; continue;
        }
        578 => {
            let Some(t587) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000896b67); };
            c.cmp(t587, 0x0u64, 32);
            if c.cc_e() { blk = 979; continue; }
            blk = 579; continue;
        }
        579 => {
            let Some(t588) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000896b71); };
            c.w64(9, t588);
            let Some(t589) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000896b75); };
            let r590 = c.sub(c.r32(9), t589, 32);
            c.w32(9, r590);
            if c.cc_o() { blk = 979; continue; }
            blk = 580; continue;
        }
        580 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 581; continue;
        }
        581 => {
            c.w32(11, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 997; continue; }
            blk = 582; continue;
        }
        582 => {
            let Some(t591) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000896b9b); };
            c.cmp(t591, 0x0u64, 32);
            if c.cc_e() { blk = 997; continue; }
            blk = 583; continue;
        }
        583 => {
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000896ba5); };
            c.w64(0, t592);
            let r593 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r593);
            if c.cc_o() { blk = 997; continue; }
            blk = 584; continue;
        }
        584 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 585; continue;
        }
        585 => {
            let Some(t594) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000896bb5); };
            c.w64(3, t594);
            let Some(t595) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896bbc); };
            c.w64(0, t595);
            c.w64(1, c.r64(9));
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896bc3); };
            c.w64(6, t596);
            return c.call(0x7000871cb4, 0x7000896bcc, 0x7000896bc7);
        }
        586 => {
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000896bcc); };
            c.w64(9, t597);
            let r598 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r598);
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000896bd4); };
            c.w64(0, t599);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000896bdb); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000896bdc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000896be2); }
            return c.ret(0x0, 0x7000896be4);
        }
        587 => {
            c.w32(9, 0x192u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000896beb); }
            let Some(t600) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000896bed); };
            if !c.push64(t600) { return RegfileExit::fault(0x7000896bed); }
            let Some(t601) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000896bf0); };
            c.w64(3, t601);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000896c01, 0x7000896bfc);
        }
        588 => {
            return RegfileExit::fault(0x7000896c01);
        }
        589 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t602) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000896c04); };
            c.w64(3, t602);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000896c13, 0x7000896c0e);
        }
        590 => {
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896c13); };
            c.w64(6, t603);
            blk = 1; continue;
        }
        591 => {
            let Some(t604) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000896c1c); };
            c.w32(9, t604);
            let r605 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r605);
            blk = 23; continue;
        }
        592 => {
            let Some(t606) = c.ld32(c.r64(9).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000896c28); };
            c.cmp(t606, c.r32(12), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 593; continue;
        }
        593 => {
            let Some(t607) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000896c35); };
            c.w64(3, t607);
            return c.call(0x7000870914, 0x7000896c41, 0x7000896c3c);
        }
        594 => {
            c.w64(9, c.r64(0));
            let Some(t608) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896c44); };
            c.w64(6, t608);
            let Some(t609) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896c48); };
            c.w64(0, t609);
            blk = 23; continue;
        }
        595 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 597; continue; }
            blk = 596; continue;
        }
        596 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896c56); }
            blk = 598; continue;
        }
        597 => {
            let r610 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r610);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 598; continue;
        }
        598 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 600; continue; }
            blk = 599; continue;
        }
        599 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896c6f); }
            blk = 601; continue;
        }
        600 => {
            c.w64(15, c.r64(9));
            let r611 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r611);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            blk = 601; continue;
        }
        601 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(15, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x7000896c98); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 607; continue; }
            blk = 602; continue;
        }
        602 => {
            if c.cc_ne() { blk = 607; continue; }
            blk = 603; continue;
        }
        603 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 605; continue; }
            blk = 604; continue;
        }
        604 => {
            c.movq_rx(1, 0);
            if let Some(sr) = c.shr(c.r64(1), 0x20u64, 64) { c.w64(1, sr); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 607; continue; }
            blk = 605; continue;
        }
        605 => {
            c.w64(1, c.r64(15));
            let r612 = c.add(c.r32(1), c.r32(15), 32);
            c.w32(1, r612);
            if c.cc_o() { blk = 607; continue; }
            blk = 606; continue;
        }
        606 => {
            c.w64(15, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 64; continue;
        }
        607 => {
            let Some(t613) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000896cc9); };
            c.w64(15, t613);
            let Some(t614) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000896cd0); };
            c.w64(1, t614);
            c.w64(7, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x7000896cd7); }
            let Some(t615) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000896cdb); };
            c.w64(1, t615);
            let Some(t616) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000896ce2); };
            c.cmp(t616, c.r64(7), 64);
            if c.cc_a() { blk = 610; continue; }
            blk = 608; continue;
        }
        608 => {
            c.w32(2, 0xcu64);
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x7000896cf4, 0x7000896cef);
        }
        609 => {
            c.w64(15, c.r64(0));
            let Some(t617) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000896cf7); };
            c.w64(12, t617);
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896cfb); };
            c.w64(6, t618);
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000896cff); };
            c.w64(11, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896d03); };
            c.w64(0, t620);
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000896d07); };
            c.w64(9, t621);
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000896d0b); };
            c.w64(3, t622);
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000896d0f); };
            c.w64(2, t623);
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000896d13); };
            c.w64(8, t624);
            blk = 611; continue;
        }
        610 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x7000896d1c); }
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000896d1f); };
            c.w64(15, t625);
            let r626 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r626);
            blk = 611; continue;
        }
        611 => {
            let Some(t627) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000896d27); };
            c.w64(1, t627);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000896d2e); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x7000896d32); }
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000896d37); }
            blk = 64; continue;
        }
        612 => {
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000896d42); };
            c.w64(2, t628);
            let Some(t629) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000896d46); };
            c.w64(12, t629);
            blk = 613; continue;
        }
        613 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000896d4a); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 615; continue; }
            blk = 614; continue;
        }
        614 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896d53); }
            blk = 616; continue;
        }
        615 => {
            c.w64(1, c.r64(15));
            let r630 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r630);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 616; continue;
        }
        616 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 618; continue; }
            blk = 617; continue;
        }
        617 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896d6d); }
            blk = 619; continue;
        }
        618 => {
            c.w64(1, c.r64(9));
            let r631 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r631);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 619; continue;
        }
        619 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(1, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 1) { return RegfileExit::fault(0x7000896d92); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 625; continue; }
            blk = 620; continue;
        }
        620 => {
            if c.cc_ne() { blk = 625; continue; }
            blk = 621; continue;
        }
        621 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 623; continue; }
            blk = 622; continue;
        }
        622 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 625; continue; }
            blk = 623; continue;
        }
        623 => {
            c.w64(9, c.r64(1));
            let r632 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r632);
            if c.cc_o() { blk = 625; continue; }
            blk = 624; continue;
        }
        624 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 630; continue;
        }
        625 => {
            let Some(t633) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000896dc3); };
            c.w64(9, t633);
            let Some(t634) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000896dca); };
            c.w64(1, t634);
            c.w64(9, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(1)) { return RegfileExit::fault(0x7000896dd1); }
            let Some(t635) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000896dd5); };
            c.w64(1, t635);
            let Some(t636) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000896ddc); };
            c.cmp(t636, c.r64(9), 64);
            if c.cc_a() { blk = 628; continue; }
            blk = 626; continue;
        }
        626 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x7000896dee, 0x7000896de9);
        }
        627 => {
            c.w64(9, c.r64(0));
            let Some(t637) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000896df1); };
            c.w64(12, t637);
            let Some(t638) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896df5); };
            c.w64(6, t638);
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000896df9); };
            c.w64(11, t639);
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896dfd); };
            c.w64(0, t640);
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000896e01); };
            c.w64(3, t641);
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000896e05); };
            c.w64(2, t642);
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000896e09); };
            c.w64(8, t643);
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000896e10); };
            c.w64(15, t644);
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000896e17); };
            c.w64(7, t645);
            blk = 629; continue;
        }
        628 => {
            let Some(t646) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000896e1d); };
            c.w64(1, t646);
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x7000896e24); }
            let Some(t647) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000896e27); };
            c.w64(9, t647);
            let r648 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r648);
            blk = 629; continue;
        }
        629 => {
            let Some(t649) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000896e2f); };
            c.w64(1, t649);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000896e36); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x7000896e3a); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000896e3f); }
            blk = 630; continue;
        }
        630 => {
            c.w64(12, c.r64(9));
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000896e48); };
            c.w64(9, t650);
            blk = 98; continue;
        }
        631 => {
            let Some(t651) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000896e51); };
            c.w64(2, t651);
            blk = 632; continue;
        }
        632 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 634; continue; }
            blk = 633; continue;
        }
        633 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896e5a); }
            blk = 635; continue;
        }
        634 => {
            c.w64(1, c.r64(15));
            let r652 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r652);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 635; continue;
        }
        635 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 637; continue; }
            blk = 636; continue;
        }
        636 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896e74); }
            blk = 638; continue;
        }
        637 => {
            c.w64(1, c.r64(9));
            let r653 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r653);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 638; continue;
        }
        638 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(1, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 1) { return RegfileExit::fault(0x7000896e99); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 644; continue; }
            blk = 639; continue;
        }
        639 => {
            if c.cc_ne() { blk = 644; continue; }
            blk = 640; continue;
        }
        640 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 642; continue; }
            blk = 641; continue;
        }
        641 => {
            c.movq_rx(7, 1);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 644; continue; }
            blk = 642; continue;
        }
        642 => {
            c.w64(7, c.r64(1));
            let r654 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r654);
            if c.cc_o() { blk = 644; continue; }
            blk = 643; continue;
        }
        643 => {
            c.w64(1, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 649; continue;
        }
        644 => {
            let Some(t655) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000896ec8); };
            c.w64(1, t655);
            let Some(t656) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000896ecf); };
            c.w64(7, t656);
            c.w64(1, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000896ed6); }
            let Some(t657) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000896eda); };
            c.w64(7, t657);
            let Some(t658) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000896ee1); };
            c.cmp(t658, c.r64(1), 64);
            if c.cc_a() { blk = 647; continue; }
            blk = 645; continue;
        }
        645 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000896ef3, 0x7000896eee);
        }
        646 => {
            c.w64(1, c.r64(0));
            let Some(t659) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000896ef6); };
            c.w64(12, t659);
            let Some(t660) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896efa); };
            c.w64(6, t660);
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000896efe); };
            c.w64(11, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896f02); };
            c.w64(0, t662);
            let Some(t663) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000896f06); };
            c.w64(9, t663);
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000896f0a); };
            c.w64(3, t664);
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000896f0e); };
            c.w64(2, t665);
            let Some(t666) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000896f12); };
            c.w64(8, t666);
            let Some(t667) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000896f19); };
            c.w64(15, t667);
            blk = 648; continue;
        }
        647 => {
            let Some(t668) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000896f22); };
            c.w64(7, t668);
            if !c.st64(c.r64(7), c.r64(1)) { return RegfileExit::fault(0x7000896f29); }
            let Some(t669) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000896f2c); };
            c.w64(1, t669);
            let r670 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r670);
            blk = 648; continue;
        }
        648 => {
            let Some(t671) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000896f34); };
            c.w64(7, t671);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000896f3b); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x7000896f3e); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000896f43); }
            blk = 649; continue;
        }
        649 => {
            c.w64(2, c.r64(1));
            blk = 123; continue;
        }
        650 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 652; continue; }
            blk = 651; continue;
        }
        651 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896f55); }
            blk = 653; continue;
        }
        652 => {
            c.w64(7, c.r64(11));
            let r672 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r672);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 653; continue;
        }
        653 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 655; continue; }
            blk = 654; continue;
        }
        654 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000896f6f); }
            blk = 656; continue;
        }
        655 => {
            c.w64(7, c.r64(9));
            let r673 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r673);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 656; continue;
        }
        656 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x70), 0) { return RegfileExit::fault(0x7000896f94); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 662; continue; }
            blk = 657; continue;
        }
        657 => {
            if c.cc_ne() { blk = 662; continue; }
            blk = 658; continue;
        }
        658 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 660; continue; }
            blk = 659; continue;
        }
        659 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 662; continue; }
            blk = 660; continue;
        }
        660 => {
            c.w64(9, c.r64(7));
            let r674 = c.add(c.r32(9), c.r32(7), 32);
            c.w32(9, r674);
            if c.cc_o() { blk = 662; continue; }
            blk = 661; continue;
        }
        661 => {
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            blk = 667; continue;
        }
        662 => {
            let Some(t675) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000896fc8); };
            c.w64(9, t675);
            let Some(t676) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000896fcf); };
            c.w64(7, t676);
            c.w64(9, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(7)) { return RegfileExit::fault(0x7000896fd6); }
            let Some(t677) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000896fda); };
            c.w64(7, t677);
            let Some(t678) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000896fe1); };
            c.cmp(t678, c.r64(9), 64);
            if c.cc_a() { blk = 665; continue; }
            blk = 663; continue;
        }
        663 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x7000896ff3, 0x7000896fee);
        }
        664 => {
            c.w64(9, c.r64(0));
            let Some(t679) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000896ff6); };
            c.w64(6, t679);
            let Some(t680) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000896ffa); };
            c.w64(0, t680);
            let Some(t681) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000896ffe); };
            c.w64(11, t681);
            let Some(t682) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000897002); };
            c.w64(1, t682);
            let Some(t683) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000897006); };
            c.w64(2, t683);
            let Some(t684) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089700a); };
            c.w64(3, t684);
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089700e); };
            c.w64(12, t685);
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000897012); };
            c.w64(8, t686);
            let Some(t687) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000897019); };
            c.w64(15, t687);
            blk = 666; continue;
        }
        665 => {
            let Some(t688) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897022); };
            c.w64(7, t688);
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000897029); }
            let Some(t689) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700089702c); };
            c.w64(9, t689);
            let r690 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r690);
            blk = 666; continue;
        }
        666 => {
            let Some(t691) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897034); };
            c.w64(7, t691);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089703b); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x70)) { return RegfileExit::fault(0x700089703f); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897044); }
            blk = 667; continue;
        }
        667 => {
            c.w64(7, c.r64(9));
            blk = 130; continue;
        }
        668 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 670; continue; }
            blk = 669; continue;
        }
        669 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897057); }
            blk = 671; continue;
        }
        670 => {
            c.w64(11, c.r64(12));
            let r692 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r692);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 671; continue;
        }
        671 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 673; continue; }
            blk = 672; continue;
        }
        672 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897075); }
            blk = 674; continue;
        }
        673 => {
            let r693 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r693);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 674; continue;
        }
        674 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(11, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x90), 1) { return RegfileExit::fault(0x7000897098); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 680; continue; }
            blk = 675; continue;
        }
        675 => {
            if c.cc_ne() { blk = 680; continue; }
            blk = 676; continue;
        }
        676 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 678; continue; }
            blk = 677; continue;
        }
        677 => {
            c.movq_rx(15, 1);
            if let Some(sr) = c.shr(c.r64(15), 0x20u64, 64) { c.w64(15, sr); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_l() { blk = 680; continue; }
            blk = 678; continue;
        }
        678 => {
            c.w64(15, c.r64(11));
            let r694 = c.add(c.r32(15), c.r32(11), 32);
            c.w32(15, r694);
            if c.cc_o() { blk = 680; continue; }
            blk = 679; continue;
        }
        679 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            blk = 134; continue;
        }
        680 => {
            let Some(t695) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008970cd); };
            c.w64(11, t695);
            let Some(t696) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008970d4); };
            c.w64(15, t696);
            c.w64(7, c.r64(15).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(15)) { return RegfileExit::fault(0x70008970db); }
            let Some(t697) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008970e2); };
            c.w64(15, t697);
            let Some(t698) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008970e9); };
            c.cmp(t698, c.r64(7), 64);
            if c.cc_a() { blk = 683; continue; }
            blk = 681; continue;
        }
        681 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008970fb, 0x70008970f6);
        }
        682 => {
            c.w64(11, c.r64(0));
            let Some(t699) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008970fe); };
            c.w64(6, t699);
            let Some(t700) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000897102); };
            c.w64(1, t700);
            let Some(t701) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000897106); };
            c.w64(2, t701);
            let Some(t702) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089710a); };
            c.w64(3, t702);
            let Some(t703) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089710e); };
            c.w64(12, t703);
            let Some(t704) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000897112); };
            c.w64(0, t704);
            let Some(t705) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000897116); };
            c.w64(9, t705);
            let Some(t706) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700089711a); };
            c.w64(8, t706);
            blk = 684; continue;
        }
        683 => {
            if !c.st64(c.r64(11), c.r64(7)) { return RegfileExit::fault(0x7000897123); }
            let Some(t707) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000897126); };
            c.w64(11, t707);
            let r708 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r708);
            blk = 684; continue;
        }
        684 => {
            let Some(t709) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897131); };
            c.w64(15, t709);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000897138); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x90)) { return RegfileExit::fault(0x700089713c); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897144); }
            blk = 134; continue;
        }
        685 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700089714f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000897153); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x7000897157); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700089715b); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000897167, 0x7000897162);
        }
        686 => {
            let Some(t710) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000897167); };
            c.w64(3, t710);
            let Some(t711) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089716b); };
            c.w64(11, t711);
            let Some(t712) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089716f); };
            c.w64(9, t712);
            let Some(t713) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000897173); };
            c.w64(12, t713);
            let Some(t714) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000897177); };
            c.w64(15, t714);
            c.w32(6, 0x8u64);
            let Some(t715) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000897180); };
            c.w64(2, t715);
            blk = 215; continue;
        }
        687 => {
            let Some(t716) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000897189); };
            c.w64(3, t716);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000897199, 0x7000897194);
        }
        688 => {
            return RegfileExit::fault(0x7000897199);
        }
        689 => {
            let Some(t717) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700089719a); };
            c.w64(3, t717);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008971aa, 0x70008971a5);
        }
        690 => {
            return RegfileExit::fault(0x70008971aa);
        }
        691 => {
            let r718 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r718);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008971ad); }
            c.w32(3, 0x4u64);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008971b3); }
            let Some(t719) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008971b4); };
            c.w64(3, t719);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008971c7, 0x70008971c2);
        }
        692 => {
            blk = 242; continue;
        }
        693 => {
            let Some(t720) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008971cc); };
            c.w64(3, t720);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008971dc, 0x70008971d7);
        }
        694 => {
            return RegfileExit::fault(0x70008971dc);
        }
        695 => {
            blk = 247; continue;
        }
        696 => {
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x70008971ea, 0x70008971e5);
        }
        697 => {
            c.w32(2, 0x8u64);
            let Some(t721) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008971ef); };
            c.w64(11, t721);
            let Some(t722) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008971f3); };
            c.w64(12, t722);
            let Some(t723) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008971f7); };
            c.w64(1, t723);
            let Some(t724) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008971fb); };
            c.w64(3, t724);
            let Some(t725) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000897202); };
            c.w64(9, t725);
            let Some(t726) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000897206); };
            c.w64(15, t726);
            blk = 257; continue;
        }
        698 => {
            let Some(t727) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700089720f); };
            c.w64(3, t727);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089721f, 0x700089721a);
        }
        699 => {
            return RegfileExit::fault(0x700089721f);
        }
        700 => {
            blk = 302; continue;
        }
        701 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000897225); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000897229); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x700089722d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000897231); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x700089723d, 0x7000897238);
        }
        702 => {
            let Some(t728) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089723d); };
            c.w64(3, t728);
            let Some(t729) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000897241); };
            c.w64(11, t729);
            let Some(t730) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000897245); };
            c.w64(9, t730);
            let Some(t731) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000897249); };
            c.w64(12, t731);
            let Some(t732) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089724d); };
            c.w64(15, t732);
            c.w32(6, 0x8u64);
            let Some(t733) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000897256); };
            c.w64(2, t733);
            blk = 312; continue;
        }
        703 => {
            let Some(t734) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700089725f); };
            c.w64(3, t734);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089726f, 0x700089726a);
        }
        704 => {
            return RegfileExit::fault(0x700089726f);
        }
        705 => {
            blk = 333; continue;
        }
        706 => {
            return c.call(0x70007febb4, 0x700089727a, 0x7000897275);
        }
        707 => {
            c.w64(2, c.r64(0));
            let Some(t735) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089727d); };
            c.w64(11, t735);
            let Some(t736) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000897281); };
            c.w64(12, t736);
            let Some(t737) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897285); };
            c.w64(0, t737);
            let Some(t738) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000897289); };
            c.w64(3, t738);
            let Some(t739) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000897290); };
            c.w64(9, t739);
            let Some(t740) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000897294); };
            c.w64(15, t740);
            c.w32(1, 0x8u64);
            blk = 343; continue;
        }
        708 => {
            let Some(t741) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008972a2); };
            c.w64(3, t741);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008972b2, 0x70008972ad);
        }
        709 => {
            return RegfileExit::fault(0x70008972b2);
        }
        710 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 712; continue; }
            blk = 711; continue;
        }
        711 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008972b8); }
            blk = 713; continue;
        }
        712 => {
            c.w64(6, c.r64(9));
            let r742 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r742);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(6) as u32 as i32 as i64));
            blk = 713; continue;
        }
        713 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 715; continue; }
            blk = 714; continue;
        }
        714 => {
            if !c.movsd_rm(2, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008972d2); }
            blk = 716; continue;
        }
        715 => {
            c.w64(1, c.r64(11));
            let r743 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r743);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            blk = 716; continue;
        }
        716 => {
            c.arith_sd(0, 1, c.xmm_u64(2));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(1, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0xc0), 1) { return RegfileExit::fault(0x70008972f7); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 722; continue; }
            blk = 717; continue;
        }
        717 => {
            if c.cc_ne() { blk = 722; continue; }
            blk = 718; continue;
        }
        718 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 720; continue; }
            blk = 719; continue;
        }
        719 => {
            c.movq_rx(6, 1);
            if let Some(sr) = c.shr(c.r64(6), 0x20u64, 64) { c.w64(6, sr); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_l() { blk = 722; continue; }
            blk = 720; continue;
        }
        720 => {
            c.w64(6, c.r64(1));
            let r744 = c.add(c.r32(6), c.r32(1), 32);
            c.w32(6, r744);
            if c.cc_o() { blk = 722; continue; }
            blk = 721; continue;
        }
        721 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 439; continue;
        }
        722 => {
            let Some(t745) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897329); };
            c.w64(1, t745);
            let Some(t746) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000897330); };
            c.w64(6, t746);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            let Some(t747) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897337); };
            c.w64(8, t747);
            let Some(t748) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089733e); };
            c.cmp(t748, c.r64(7), 64);
            if c.cc_a() { blk = 725; continue; }
            blk = 723; continue;
        }
        723 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000897350, 0x700089734b);
        }
        724 => {
            blk = 726; continue;
        }
        725 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000897352); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 726; continue;
        }
        726 => {
            let Some(t749) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089735c); };
            c.w64(9, t749);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000897363); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0xc0)) { return RegfileExit::fault(0x7000897367); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089736f); }
            c.w64(1, c.r64(0));
            let Some(t750) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897377); };
            c.w64(12, t750);
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t751) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000897389); };
            c.w64(11, t751);
            let Some(t752) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897390); };
            c.w64(9, t752);
            let Some(t753) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000897397); };
            c.w64(3, t753);
            let Some(t754) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x700089739e); };
            c.w64(15, t754);
            let Some(t755) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x70008973a5); };
            c.w64(2, t755);
            blk = 439; continue;
        }
        727 => {
            let Some(t756) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008973b1); };
            c.cmp(t756, 0x0u64, 32);
            if c.cc_ne() { blk = 729; continue; }
            blk = 728; continue;
        }
        728 => {
            let Some(t757) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008973ba); };
            c.w64(9, t757);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008973c1); }
            blk = 730; continue;
        }
        729 => {
            let Some(t758) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008973c9); };
            c.w64(9, t758);
            let r759 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r759);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 730; continue;
        }
        730 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x90), 0) { return RegfileExit::fault(0x70008973fc); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 736; continue; }
            blk = 731; continue;
        }
        731 => {
            if c.cc_ne() { blk = 736; continue; }
            blk = 732; continue;
        }
        732 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 734; continue; }
            blk = 733; continue;
        }
        733 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 736; continue; }
            blk = 734; continue;
        }
        734 => {
            c.w64(11, c.r64(9));
            let r760 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r760);
            if c.cc_o() { blk = 736; continue; }
            blk = 735; continue;
        }
        735 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 741; continue;
        }
        736 => {
            let Some(t761) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897431); };
            c.w64(9, t761);
            let Some(t762) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000897438); };
            c.w64(11, t762);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t763) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089743f); };
            c.w64(15, t763);
            let Some(t764) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000897446); };
            c.cmp(t764, c.r64(12), 64);
            if c.cc_a() { blk = 739; continue; }
            blk = 737; continue;
        }
        737 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000897455, 0x7000897450);
        }
        738 => {
            blk = 740; continue;
        }
        739 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000897457); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 740; continue;
        }
        740 => {
            let Some(t765) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897461); };
            c.w64(9, t765);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000897468); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x90)) { return RegfileExit::fault(0x700089746c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897474); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            blk = 741; continue;
        }
        741 => {
            c.w64(9, c.r64(0));
            blk = 426; continue;
        }
        742 => {
            let Some(t766) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700089748f); };
            c.cmp(t766, 0x0u64, 32);
            if c.cc_ne() { blk = 744; continue; }
            blk = 743; continue;
        }
        743 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897495); }
            blk = 745; continue;
        }
        744 => {
            c.w64(9, c.r64(12));
            let r767 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r767);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 745; continue;
        }
        745 => {
            let Some(t768) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008974ad); };
            c.cmp(t768, 0x0u64, 32);
            if c.cc_ne() { blk = 747; continue; }
            blk = 746; continue;
        }
        746 => {
            let Some(t769) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008974b3); };
            c.w64(9, t769);
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008974b7); }
            blk = 748; continue;
        }
        747 => {
            let Some(t770) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008974bf); };
            c.w64(9, t770);
            let r771 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r771);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 748; continue;
        }
        748 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(9, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x90), 1) { return RegfileExit::fault(0x70008974e1); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 754; continue; }
            blk = 749; continue;
        }
        749 => {
            if c.cc_ne() { blk = 754; continue; }
            blk = 750; continue;
        }
        750 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 752; continue; }
            blk = 751; continue;
        }
        751 => {
            c.movq_rx(15, 1);
            if let Some(sr) = c.shr(c.r64(15), 0x20u64, 64) { c.w64(15, sr); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_l() { blk = 754; continue; }
            blk = 752; continue;
        }
        752 => {
            c.w64(15, c.r64(9));
            let r772 = c.add(c.r32(15), c.r32(9), 32);
            c.w32(15, r772);
            if c.cc_o() { blk = 754; continue; }
            blk = 753; continue;
        }
        753 => {
            c.w64(9, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 759; continue;
        }
        754 => {
            let Some(t773) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897516); };
            c.w64(9, t773);
            let Some(t774) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089751d); };
            c.w64(15, t774);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t775) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897524); };
            c.w64(3, t775);
            let Some(t776) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700089752b); };
            c.cmp(t776, c.r64(0), 64);
            if c.cc_a() { blk = 757; continue; }
            blk = 755; continue;
        }
        755 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089753a, 0x7000897535);
        }
        756 => {
            blk = 758; continue;
        }
        757 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x700089753c); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 758; continue;
        }
        758 => {
            let Some(t777) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897546); };
            c.w64(9, t777);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089754d); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x90)) { return RegfileExit::fault(0x7000897551); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897559); }
            let Some(t778) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089755e); };
            c.w64(12, t778);
            let Some(t779) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000897562); };
            c.w64(11, t779);
            blk = 759; continue;
        }
        759 => {
            c.w64(9, c.r64(0));
            blk = 471; continue;
        }
        760 => {
            let Some(t780) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700089756e); };
            c.cmp(t780, 0x0u64, 32);
            if c.cc_ne() { blk = 762; continue; }
            blk = 761; continue;
        }
        761 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897574); }
            blk = 763; continue;
        }
        762 => {
            c.w64(6, c.r64(12));
            let r781 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r781);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(6) as u32 as i32 as i64));
            blk = 763; continue;
        }
        763 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 765; continue; }
            blk = 764; continue;
        }
        764 => {
            if !c.movsd_rm(2, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700089758f); }
            blk = 766; continue;
        }
        765 => {
            c.w64(6, c.r64(9));
            let r782 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r782);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(6) as u32 as i32 as i64));
            blk = 766; continue;
        }
        766 => {
            c.arith_sd(0, 1, c.xmm_u64(2));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(6, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(6) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0xc0), 1) { return RegfileExit::fault(0x70008975b4); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 772; continue; }
            blk = 767; continue;
        }
        767 => {
            if c.cc_ne() { blk = 772; continue; }
            blk = 768; continue;
        }
        768 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 770; continue; }
            blk = 769; continue;
        }
        769 => {
            c.movq_rx(7, 1);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 772; continue; }
            blk = 770; continue;
        }
        770 => {
            c.w64(7, c.r64(6));
            let r783 = c.add(c.r32(7), c.r32(6), 32);
            c.w32(7, r783);
            if c.cc_o() { blk = 772; continue; }
            blk = 771; continue;
        }
        771 => {
            c.w64(6, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 486; continue;
        }
        772 => {
            let Some(t784) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008975e6); };
            c.w64(6, t784);
            let Some(t785) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008975ed); };
            c.w64(7, t785);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0xc8), c.r64(7)) { return RegfileExit::fault(0x70008975f4); }
            let Some(t786) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008975fb); };
            c.w64(7, t786);
            let Some(t787) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000897602); };
            c.cmp(t787, c.r64(8), 64);
            if c.cc_a() { blk = 775; continue; }
            blk = 773; continue;
        }
        773 => {
            c.w32(2, 0xcu64);
            c.w64(6, c.r64(2));
            return c.call(0x70007febf4, 0x7000897614, 0x700089760f);
        }
        774 => {
            blk = 776; continue;
        }
        775 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x7000897616); }
            let Some(t788) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000897619); };
            c.w64(6, t788);
            let r789 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r789);
            c.w64(0, c.r64(6));
            blk = 776; continue;
        }
        776 => {
            let Some(t790) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897627); };
            c.w64(9, t790);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089762e); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0xc0)) { return RegfileExit::fault(0x7000897632); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089763a); }
            c.w64(6, c.r64(0));
            let Some(t791) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000897642); };
            c.w64(11, t791);
            let Some(t792) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897646); };
            c.w64(12, t792);
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t793) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000897658); };
            c.w64(0, t793);
            let Some(t794) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700089765f); };
            c.w64(9, t794);
            let Some(t795) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000897666); };
            c.w64(2, t795);
            let Some(t796) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x700089766d); };
            c.w64(3, t796);
            let Some(t797) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000897674); };
            c.w64(1, t797);
            blk = 486; continue;
        }
        777 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 779; continue; }
            blk = 778; continue;
        }
        778 => {
            if !c.movsd_rm(1, c.r64(6).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897686); }
            blk = 780; continue;
        }
        779 => {
            let r798 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r798);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(6) as u32 as i32 as i64));
            blk = 780; continue;
        }
        780 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(6, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(6) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0xc0), 1) { return RegfileExit::fault(0x70008976a7); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 786; continue; }
            blk = 781; continue;
        }
        781 => {
            if c.cc_ne() { blk = 786; continue; }
            blk = 782; continue;
        }
        782 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 784; continue; }
            blk = 783; continue;
        }
        783 => {
            c.movq_rx(7, 1);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 786; continue; }
            blk = 784; continue;
        }
        784 => {
            c.w64(7, c.r64(6));
            let r799 = c.add(c.r32(7), c.r32(6), 32);
            c.w32(7, r799);
            if c.cc_o() { blk = 786; continue; }
            blk = 785; continue;
        }
        785 => {
            c.w64(6, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 489; continue;
        }
        786 => {
            let Some(t800) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008976d9); };
            c.w64(6, t800);
            let Some(t801) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008976e0); };
            c.w64(7, t801);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0xc8), c.r64(7)) { return RegfileExit::fault(0x70008976e7); }
            let Some(t802) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008976ee); };
            c.w64(7, t802);
            let Some(t803) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008976f5); };
            c.cmp(t803, c.r64(8), 64);
            if c.cc_a() { blk = 789; continue; }
            blk = 787; continue;
        }
        787 => {
            c.w32(2, 0xcu64);
            c.w64(6, c.r64(2));
            return c.call(0x70007febf4, 0x7000897707, 0x7000897702);
        }
        788 => {
            c.w64(6, c.r64(0));
            let Some(t804) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089770a); };
            c.w64(11, t804);
            let Some(t805) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089770e); };
            c.w64(12, t805);
            let Some(t806) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000897712); };
            c.w64(0, t806);
            let Some(t807) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897719); };
            c.w64(9, t807);
            let Some(t808) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000897720); };
            c.w64(2, t808);
            let Some(t809) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000897727); };
            c.w64(3, t809);
            let Some(t810) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x700089772e); };
            c.w64(1, t810);
            blk = 790; continue;
        }
        789 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x7000897737); }
            let Some(t811) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x700089773a); };
            c.w64(6, t811);
            let r812 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r812);
            blk = 790; continue;
        }
        790 => {
            let Some(t813) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897745); };
            c.w64(7, t813);
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089774c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0xc0)) { return RegfileExit::fault(0x700089774f); }
            if !c.movsd_mr(c.r64(6).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897757); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            blk = 489; continue;
        }
        791 => {
            let Some(t814) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089776f); };
            c.cmp(t814, 0x0u64, 32);
            if c.cc_ne() { blk = 793; continue; }
            blk = 792; continue;
        }
        792 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897775); }
            blk = 794; continue;
        }
        793 => {
            c.w64(7, c.r64(11));
            let r815 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r815);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 794; continue;
        }
        794 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 796; continue; }
            blk = 795; continue;
        }
        795 => {
            if !c.movsd_rm(2, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700089778f); }
            blk = 797; continue;
        }
        796 => {
            c.w64(7, c.r64(9));
            let r816 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r816);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(7) as u32 as i32 as i64));
            blk = 797; continue;
        }
        797 => {
            c.arith_sd(0, 1, c.xmm_u64(2));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(7, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0xc0), 1) { return RegfileExit::fault(0x70008977b4); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 803; continue; }
            blk = 798; continue;
        }
        798 => {
            if c.cc_ne() { blk = 803; continue; }
            blk = 799; continue;
        }
        799 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 801; continue; }
            blk = 800; continue;
        }
        800 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 803; continue; }
            blk = 801; continue;
        }
        801 => {
            c.w64(8, c.r64(7));
            let r817 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r817);
            if c.cc_o() { blk = 803; continue; }
            blk = 802; continue;
        }
        802 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 493; continue;
        }
        803 => {
            let Some(t818) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008977e8); };
            c.w64(7, t818);
            let Some(t819) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008977ef); };
            c.w64(8, t819);
            c.w64(7, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(8)) { return RegfileExit::fault(0x70008977f6); }
            let Some(t820) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008977fd); };
            c.w64(8, t820);
            let Some(t821) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000897804); };
            c.cmp(t821, c.r64(7), 64);
            if c.cc_a() { blk = 806; continue; }
            blk = 804; continue;
        }
        804 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x7000897816, 0x7000897811);
        }
        805 => {
            blk = 807; continue;
        }
        806 => {
            let Some(t822) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897818); };
            c.w64(8, t822);
            if !c.st64(c.r64(8), c.r64(7)) { return RegfileExit::fault(0x700089781f); }
            let Some(t823) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000897822); };
            c.w64(7, t823);
            let r824 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r824);
            c.w64(0, c.r64(7));
            blk = 807; continue;
        }
        807 => {
            let Some(t825) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897830); };
            c.w64(9, t825);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000897837); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0xc0)) { return RegfileExit::fault(0x700089783b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897843); }
            c.w64(7, c.r64(0));
            let Some(t826) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089784b); };
            c.w64(11, t826);
            let Some(t827) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089784f); };
            c.w64(12, t827);
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t828) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000897861); };
            c.w64(0, t828);
            let Some(t829) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897868); };
            c.w64(9, t829);
            let Some(t830) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x700089786f); };
            c.w64(2, t830);
            let Some(t831) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000897876); };
            c.w64(3, t831);
            let Some(t832) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x700089787d); };
            c.w64(1, t832);
            let Some(t833) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000897884); };
            c.w64(6, t833);
            blk = 493; continue;
        }
        808 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 810; continue; }
            blk = 809; continue;
        }
        809 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897896); }
            blk = 811; continue;
        }
        810 => {
            let r834 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r834);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 811; continue;
        }
        811 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(9, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0xc0), 1) { return RegfileExit::fault(0x70008978b9); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 817; continue; }
            blk = 812; continue;
        }
        812 => {
            if c.cc_ne() { blk = 817; continue; }
            blk = 813; continue;
        }
        813 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 815; continue; }
            blk = 814; continue;
        }
        814 => {
            c.movq_rx(7, 1);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 817; continue; }
            blk = 815; continue;
        }
        815 => {
            c.w64(7, c.r64(9));
            let r835 = c.add(c.r32(7), c.r32(9), 32);
            c.w32(7, r835);
            if c.cc_o() { blk = 817; continue; }
            blk = 816; continue;
        }
        816 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 496; continue;
        }
        817 => {
            let Some(t836) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008978ed); };
            c.w64(9, t836);
            let Some(t837) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008978f4); };
            c.w64(7, t837);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(7)) { return RegfileExit::fault(0x70008978fb); }
            let Some(t838) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897902); };
            c.w64(7, t838);
            let Some(t839) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000897909); };
            c.cmp(t839, c.r64(8), 64);
            if c.cc_a() { blk = 820; continue; }
            blk = 818; continue;
        }
        818 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700089791b, 0x7000897916);
        }
        819 => {
            c.w64(9, c.r64(0));
            let Some(t840) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089791e); };
            c.w64(11, t840);
            let Some(t841) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897922); };
            c.w64(12, t841);
            let Some(t842) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000897926); };
            c.w64(0, t842);
            let Some(t843) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x700089792d); };
            c.w64(2, t843);
            let Some(t844) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000897934); };
            c.w64(3, t844);
            let Some(t845) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x700089793b); };
            c.w64(1, t845);
            let Some(t846) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000897942); };
            c.w64(6, t846);
            blk = 821; continue;
        }
        820 => {
            if !c.st64(c.r64(9), c.r64(8)) { return RegfileExit::fault(0x700089794b); }
            let Some(t847) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x700089794e); };
            c.w64(9, t847);
            let r848 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r848);
            blk = 821; continue;
        }
        821 => {
            let Some(t849) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897959); };
            c.w64(7, t849);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000897960); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0xc0)) { return RegfileExit::fault(0x7000897964); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089796c); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            blk = 496; continue;
        }
        822 => {
            let Some(t850) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000897985); };
            c.cmp(t850, 0x0u64, 32);
            if c.cc_ne() { blk = 824; continue; }
            blk = 823; continue;
        }
        823 => {
            let Some(t851) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700089798e); };
            c.w64(9, t851);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897995); }
            blk = 825; continue;
        }
        824 => {
            let Some(t852) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700089799d); };
            c.w64(9, t852);
            let r853 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r853);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 825; continue;
        }
        825 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x90), 0) { return RegfileExit::fault(0x70008979d0); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 831; continue; }
            blk = 826; continue;
        }
        826 => {
            if c.cc_ne() { blk = 831; continue; }
            blk = 827; continue;
        }
        827 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 829; continue; }
            blk = 828; continue;
        }
        828 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 831; continue; }
            blk = 829; continue;
        }
        829 => {
            c.w64(11, c.r64(9));
            let r854 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r854);
            if c.cc_o() { blk = 831; continue; }
            blk = 830; continue;
        }
        830 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 836; continue;
        }
        831 => {
            let Some(t855) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897a05); };
            c.w64(9, t855);
            let Some(t856) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000897a0c); };
            c.w64(11, t856);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t857) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897a13); };
            c.w64(15, t857);
            let Some(t858) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000897a1a); };
            c.cmp(t858, c.r64(12), 64);
            if c.cc_a() { blk = 834; continue; }
            blk = 832; continue;
        }
        832 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000897a29, 0x7000897a24);
        }
        833 => {
            blk = 835; continue;
        }
        834 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000897a2b); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 835; continue;
        }
        835 => {
            let Some(t859) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897a35); };
            c.w64(9, t859);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000897a3c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x90)) { return RegfileExit::fault(0x7000897a40); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897a48); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            blk = 836; continue;
        }
        836 => {
            c.w64(9, c.r64(0));
            blk = 472; continue;
        }
        837 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(0)) { return RegfileExit::fault(0x7000897a63); }
            let Some(t860) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000897a6a); };
            c.cmp(t860, 0x0u64, 32);
            if c.cc_ne() { blk = 839; continue; }
            blk = 838; continue;
        }
        838 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897a70); }
            blk = 840; continue;
        }
        839 => {
            c.w64(15, c.r64(12));
            let r861 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r861);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            blk = 840; continue;
        }
        840 => {
            let Some(t862) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000897a88); };
            c.cmp(t862, 0x0u64, 32);
            if c.cc_ne() { blk = 842; continue; }
            blk = 841; continue;
        }
        841 => {
            if !c.movsd_rm(2, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897a8e); }
            blk = 843; continue;
        }
        842 => {
            c.w64(15, c.r64(9));
            let r863 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r863);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(15) as u32 as i32 as i64));
            blk = 843; continue;
        }
        843 => {
            c.arith_sd(1, 2, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(2)); c.w32(15, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x98), 2) { return RegfileExit::fault(0x7000897ab7); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(2));
            if c.cc_p() { blk = 849; continue; }
            blk = 844; continue;
        }
        844 => {
            if c.cc_ne() { blk = 849; continue; }
            blk = 845; continue;
        }
        845 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 847; continue; }
            blk = 846; continue;
        }
        846 => {
            c.movq_rx(3, 2);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 849; continue; }
            blk = 847; continue;
        }
        847 => {
            c.w64(3, c.r64(15));
            let r864 = c.add(c.r32(3), c.r32(15), 32);
            c.w32(3, r864);
            if c.cc_o() { blk = 849; continue; }
            blk = 848; continue;
        }
        848 => {
            c.w64(15, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 512; continue;
        }
        849 => {
            let Some(t865) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897aeb); };
            c.w64(15, t865);
            let Some(t866) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000897af2); };
            c.w64(3, t866);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            let Some(t867) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897af9); };
            c.w64(1, t867);
            let Some(t868) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000897b00); };
            c.cmp(t868, c.r64(2), 64);
            if c.cc_a() { blk = 852; continue; }
            blk = 850; continue;
        }
        850 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000897b0f, 0x7000897b0a);
        }
        851 => {
            c.w64(15, c.r64(0));
            let Some(t869) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000897b12); };
            c.w64(11, t869);
            let Some(t870) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000897b16); };
            c.w64(9, t870);
            let Some(t871) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897b1a); };
            c.w64(12, t871);
            let Some(t872) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897b1e); };
            c.w64(0, t872);
            blk = 853; continue;
        }
        852 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x7000897b27); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 853; continue;
        }
        853 => {
            let Some(t873) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897b2e); };
            c.w64(3, t873);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000897b35); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x98)) { return RegfileExit::fault(0x7000897b39); }
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897b41); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            blk = 512; continue;
        }
        854 => {
            let Some(t874) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000897b5a); };
            c.cmp(t874, 0x0u64, 32);
            if c.cc_ne() { blk = 856; continue; }
            blk = 855; continue;
        }
        855 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897b60); }
            blk = 857; continue;
        }
        856 => {
            c.w64(1, c.r64(12));
            let r875 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r875);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 857; continue;
        }
        857 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 859; continue; }
            blk = 858; continue;
        }
        858 => {
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897b7b); }
            blk = 860; continue;
        }
        859 => {
            c.w64(1, c.r64(0));
            let r876 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r876);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            blk = 860; continue;
        }
        860 => {
            c.arith_sd(0, 1, c.xmm_u64(2));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(1, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0xc0), 1) { return RegfileExit::fault(0x7000897b9f); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 866; continue; }
            blk = 861; continue;
        }
        861 => {
            if c.cc_ne() { blk = 866; continue; }
            blk = 862; continue;
        }
        862 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 864; continue; }
            blk = 863; continue;
        }
        863 => {
            c.movq_rx(6, 1);
            if let Some(sr) = c.shr(c.r64(6), 0x20u64, 64) { c.w64(6, sr); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_l() { blk = 866; continue; }
            blk = 864; continue;
        }
        864 => {
            c.w64(6, c.r64(1));
            let r877 = c.add(c.r32(6), c.r32(1), 32);
            c.w32(6, r877);
            if c.cc_o() { blk = 866; continue; }
            blk = 865; continue;
        }
        865 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 525; continue;
        }
        866 => {
            let Some(t878) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897bd1); };
            c.w64(1, t878);
            let Some(t879) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000897bd8); };
            c.w64(6, t879);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            let Some(t880) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897bdf); };
            c.w64(8, t880);
            let Some(t881) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000897be6); };
            c.cmp(t881, c.r64(7), 64);
            if c.cc_a() { blk = 869; continue; }
            blk = 867; continue;
        }
        867 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000897bf8, 0x7000897bf3);
        }
        868 => {
            c.w64(1, c.r64(0));
            let Some(t882) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000897bfb); };
            c.w64(11, t882);
            let Some(t883) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000897bff); };
            c.w64(9, t883);
            let Some(t884) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897c03); };
            c.w64(12, t884);
            let Some(t885) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897c07); };
            c.w64(0, t885);
            let Some(t886) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000897c0e); };
            c.w64(2, t886);
            let Some(t887) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000897c15); };
            c.w64(3, t887);
            let Some(t888) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000897c1c); };
            c.w64(15, t888);
            blk = 870; continue;
        }
        869 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000897c25); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 870; continue;
        }
        870 => {
            let Some(t889) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897c2c); };
            c.w64(6, t889);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000897c33); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0xc0)) { return RegfileExit::fault(0x7000897c36); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897c3e); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            blk = 525; continue;
        }
        871 => {
            let Some(t890) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000897c56); };
            c.cmp(t890, 0x0u64, 32);
            if c.cc_ne() { blk = 873; continue; }
            blk = 872; continue;
        }
        872 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897c5c); }
            blk = 874; continue;
        }
        873 => {
            c.w64(6, c.r64(11));
            let r891 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r891);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(6) as u32 as i32 as i64));
            blk = 874; continue;
        }
        874 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 876; continue; }
            blk = 875; continue;
        }
        875 => {
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897c76); }
            blk = 877; continue;
        }
        876 => {
            c.w64(6, c.r64(0));
            let r892 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r892);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(6) as u32 as i32 as i64));
            blk = 877; continue;
        }
        877 => {
            c.arith_sd(0, 1, c.xmm_u64(2));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(6, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(6) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0xc0), 1) { return RegfileExit::fault(0x7000897c9a); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 883; continue; }
            blk = 878; continue;
        }
        878 => {
            if c.cc_ne() { blk = 883; continue; }
            blk = 879; continue;
        }
        879 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 881; continue; }
            blk = 880; continue;
        }
        880 => {
            c.movq_rx(7, 1);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 883; continue; }
            blk = 881; continue;
        }
        881 => {
            c.w64(7, c.r64(6));
            let r893 = c.add(c.r32(7), c.r32(6), 32);
            c.w32(7, r893);
            if c.cc_o() { blk = 883; continue; }
            blk = 882; continue;
        }
        882 => {
            c.w64(6, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 529; continue;
        }
        883 => {
            let Some(t894) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897ccc); };
            c.w64(6, t894);
            let Some(t895) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000897cd3); };
            c.w64(7, t895);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(7)) { return RegfileExit::fault(0x7000897cda); }
            let Some(t896) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897ce1); };
            c.w64(7, t896);
            let Some(t897) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000897ce8); };
            c.cmp(t897, c.r64(8), 64);
            if c.cc_a() { blk = 886; continue; }
            blk = 884; continue;
        }
        884 => {
            c.w32(2, 0xcu64);
            c.w64(6, c.r64(2));
            return c.call(0x70007febf4, 0x7000897cfa, 0x7000897cf5);
        }
        885 => {
            c.w64(6, c.r64(0));
            let Some(t898) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000897cfd); };
            c.w64(11, t898);
            let Some(t899) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000897d01); };
            c.w64(9, t899);
            let Some(t900) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897d05); };
            c.w64(12, t900);
            let Some(t901) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897d09); };
            c.w64(0, t901);
            let Some(t902) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000897d10); };
            c.w64(2, t902);
            let Some(t903) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000897d17); };
            c.w64(3, t903);
            let Some(t904) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000897d1e); };
            c.w64(15, t904);
            let Some(t905) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000897d25); };
            c.w64(1, t905);
            blk = 887; continue;
        }
        886 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x7000897d2e); }
            let Some(t906) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000897d31); };
            c.w64(6, t906);
            let r907 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r907);
            blk = 887; continue;
        }
        887 => {
            let Some(t908) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897d3c); };
            c.w64(7, t908);
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000897d43); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0xc0)) { return RegfileExit::fault(0x7000897d46); }
            if !c.movsd_mr(c.r64(6).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897d4e); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            blk = 529; continue;
        }
        888 => {
            let Some(t909) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000897d66); };
            c.cmp(t909, 0x0u64, 32);
            if c.cc_ne() { blk = 890; continue; }
            blk = 889; continue;
        }
        889 => {
            let Some(t910) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897d6f); };
            c.w64(0, t910);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897d76); }
            blk = 891; continue;
        }
        890 => {
            let Some(t911) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897d7d); };
            c.w64(0, t911);
            let r912 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r912);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 891; continue;
        }
        891 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x90), 0) { return RegfileExit::fault(0x7000897dae); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 897; continue; }
            blk = 892; continue;
        }
        892 => {
            if c.cc_ne() { blk = 897; continue; }
            blk = 893; continue;
        }
        893 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 895; continue; }
            blk = 894; continue;
        }
        894 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 897; continue; }
            blk = 895; continue;
        }
        895 => {
            c.w64(11, c.r64(9));
            let r913 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r913);
            if c.cc_o() { blk = 897; continue; }
            blk = 896; continue;
        }
        896 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 507; continue;
        }
        897 => {
            let Some(t914) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897de3); };
            c.w64(9, t914);
            let Some(t915) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000897dea); };
            c.w64(11, t915);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t916) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897df1); };
            c.w64(15, t916);
            let Some(t917) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000897df8); };
            c.cmp(t917, c.r64(12), 64);
            if c.cc_a() { blk = 900; continue; }
            blk = 898; continue;
        }
        898 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000897e07, 0x7000897e02);
        }
        899 => {
            blk = 901; continue;
        }
        900 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000897e09); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 901; continue;
        }
        901 => {
            let Some(t918) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897e13); };
            c.w64(9, t918);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000897e1a); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x90)) { return RegfileExit::fault(0x7000897e1e); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897e26); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            blk = 507; continue;
        }
        902 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x7000897e3e); }
            let Some(t919) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000897e45); };
            c.cmp(t919, 0x0u64, 32);
            if c.cc_ne() { blk = 904; continue; }
            blk = 903; continue;
        }
        903 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897e4b); }
            blk = 905; continue;
        }
        904 => {
            c.w64(0, c.r64(12));
            let r920 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r920);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 905; continue;
        }
        905 => {
            let Some(t921) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000897e61); };
            c.cmp(t921, 0x0u64, 32);
            if c.cc_ne() { blk = 907; continue; }
            blk = 906; continue;
        }
        906 => {
            if !c.movsd_rm(2, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897e67); }
            blk = 908; continue;
        }
        907 => {
            c.w64(0, c.r64(11));
            let r922 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r922);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 908; continue;
        }
        908 => {
            c.arith_sd(1, 2, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(2)); c.w32(0, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x98), 2) { return RegfileExit::fault(0x7000897e8c); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(2));
            if c.cc_p() { blk = 914; continue; }
            blk = 909; continue;
        }
        909 => {
            if c.cc_ne() { blk = 914; continue; }
            blk = 910; continue;
        }
        910 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 912; continue; }
            blk = 911; continue;
        }
        911 => {
            c.movq_rx(3, 2);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 914; continue; }
            blk = 912; continue;
        }
        912 => {
            c.w64(3, c.r64(0));
            let r923 = c.add(c.r32(3), c.r32(0), 32);
            c.w32(3, r923);
            if c.cc_o() { blk = 914; continue; }
            blk = 913; continue;
        }
        913 => {
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 544; continue;
        }
        914 => {
            let Some(t924) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897ebe); };
            c.w64(0, t924);
            let Some(t925) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000897ec5); };
            c.w64(3, t925);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            let Some(t926) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897ecc); };
            c.w64(1, t926);
            let Some(t927) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000897ed3); };
            c.cmp(t927, c.r64(2), 64);
            if c.cc_a() { blk = 917; continue; }
            blk = 915; continue;
        }
        915 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000897ee2, 0x7000897edd);
        }
        916 => {
            blk = 918; continue;
        }
        917 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000897ee4); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 918; continue;
        }
        918 => {
            let Some(t928) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897eeb); };
            c.w64(9, t928);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000897ef2); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x98)) { return RegfileExit::fault(0x7000897ef6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897efe); }
            let Some(t929) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897f03); };
            c.w64(9, t929);
            let Some(t930) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000897f0a); };
            c.w64(15, t930);
            let Some(t931) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000897f0e); };
            c.w64(11, t931);
            let Some(t932) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897f12); };
            c.w64(12, t932);
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            blk = 544; continue;
        }
        919 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x7000897f29); }
            let Some(t933) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000897f30); };
            c.cmp(t933, 0x0u64, 32);
            if c.cc_ne() { blk = 921; continue; }
            blk = 920; continue;
        }
        920 => {
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897f36); }
            blk = 922; continue;
        }
        921 => {
            c.w64(3, c.r64(15));
            let r934 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r934);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 922; continue;
        }
        922 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 924; continue; }
            blk = 923; continue;
        }
        923 => {
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000897f50); }
            blk = 925; continue;
        }
        924 => {
            let r935 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r935);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 925; continue;
        }
        925 => {
            c.arith_sd(0, 1, c.xmm_u64(2));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(0, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x98), 1) { return RegfileExit::fault(0x7000897f71); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 931; continue; }
            blk = 926; continue;
        }
        926 => {
            if c.cc_ne() { blk = 931; continue; }
            blk = 927; continue;
        }
        927 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 929; continue; }
            blk = 928; continue;
        }
        928 => {
            c.movq_rx(3, 1);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 931; continue; }
            blk = 929; continue;
        }
        929 => {
            c.w64(3, c.r64(0));
            let r936 = c.add(c.r32(3), c.r32(0), 32);
            c.w32(3, r936);
            if c.cc_o() { blk = 931; continue; }
            blk = 930; continue;
        }
        930 => {
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 548; continue;
        }
        931 => {
            let Some(t937) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000897fa3); };
            c.w64(0, t937);
            let Some(t938) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000897faa); };
            c.w64(3, t938);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            let Some(t939) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000897fb1); };
            c.w64(1, t939);
            let Some(t940) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000897fb8); };
            c.cmp(t940, c.r64(2), 64);
            if c.cc_a() { blk = 934; continue; }
            blk = 932; continue;
        }
        932 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000897fc7, 0x7000897fc2);
        }
        933 => {
            let Some(t941) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000897fc7); };
            c.w64(9, t941);
            let Some(t942) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000897fce); };
            c.w64(15, t942);
            let Some(t943) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000897fd2); };
            c.w64(11, t943);
            let Some(t944) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000897fd6); };
            c.w64(12, t944);
            blk = 935; continue;
        }
        934 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000897fdc); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 935; continue;
        }
        935 => {
            let Some(t945) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000897fe3); };
            c.w64(3, t945);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000897fea); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x98)) { return RegfileExit::fault(0x7000897fed); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000897ff5); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            blk = 548; continue;
        }
        936 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 938; continue; }
            blk = 937; continue;
        }
        937 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898012); }
            blk = 939; continue;
        }
        938 => {
            c.w64(1, c.r64(9));
            let r946 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r946);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 939; continue;
        }
        939 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(1, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0xc0), 1) { return RegfileExit::fault(0x7000898037); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 945; continue; }
            blk = 940; continue;
        }
        940 => {
            if c.cc_ne() { blk = 945; continue; }
            blk = 941; continue;
        }
        941 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 943; continue; }
            blk = 942; continue;
        }
        942 => {
            c.movq_rx(6, 1);
            if let Some(sr) = c.shr(c.r64(6), 0x20u64, 64) { c.w64(6, sr); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_l() { blk = 945; continue; }
            blk = 943; continue;
        }
        943 => {
            c.w64(6, c.r64(1));
            let r947 = c.add(c.r32(6), c.r32(1), 32);
            c.w32(6, r947);
            if c.cc_o() { blk = 945; continue; }
            blk = 944; continue;
        }
        944 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 560; continue;
        }
        945 => {
            let Some(t948) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000898069); };
            c.w64(1, t948);
            let Some(t949) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000898070); };
            c.w64(6, t949);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            let Some(t950) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000898077); };
            c.w64(8, t950);
            let Some(t951) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089807e); };
            c.cmp(t951, c.r64(7), 64);
            if c.cc_a() { blk = 948; continue; }
            blk = 946; continue;
        }
        946 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000898090, 0x700089808b);
        }
        947 => {
            blk = 949; continue;
        }
        948 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000898092); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 949; continue;
        }
        949 => {
            let Some(t952) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089809c); };
            c.w64(9, t952);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008980a3); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0xc0)) { return RegfileExit::fault(0x70008980a7); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008980af); }
            c.w64(1, c.r64(0));
            let Some(t953) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008980b7); };
            c.w64(15, t953);
            let Some(t954) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008980bb); };
            c.w64(11, t954);
            let Some(t955) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008980bf); };
            c.w64(12, t955);
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t956) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008980d1); };
            c.w64(9, t956);
            let Some(t957) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008980d8); };
            c.w64(2, t957);
            let Some(t958) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008980df); };
            c.w64(3, t958);
            let Some(t959) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x70008980e6); };
            c.w64(0, t959);
            blk = 560; continue;
        }
        950 => {
            let Some(t960) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008980f2); };
            c.cmp(t960, 0x0u64, 32);
            if c.cc_ne() { blk = 952; continue; }
            blk = 951; continue;
        }
        951 => {
            let Some(t961) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008980fb); };
            c.w64(9, t961);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898102); }
            blk = 953; continue;
        }
        952 => {
            let Some(t962) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700089810a); };
            c.w64(9, t962);
            let r963 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r963);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 953; continue;
        }
        953 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x90), 0) { return RegfileExit::fault(0x700089813d); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 959; continue; }
            blk = 954; continue;
        }
        954 => {
            if c.cc_ne() { blk = 959; continue; }
            blk = 955; continue;
        }
        955 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 957; continue; }
            blk = 956; continue;
        }
        956 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 959; continue; }
            blk = 957; continue;
        }
        957 => {
            c.w64(11, c.r64(9));
            let r964 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r964);
            if c.cc_o() { blk = 959; continue; }
            blk = 958; continue;
        }
        958 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 964; continue;
        }
        959 => {
            let Some(t965) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000898172); };
            c.w64(9, t965);
            let Some(t966) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000898179); };
            c.w64(11, t966);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t967) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000898180); };
            c.w64(15, t967);
            let Some(t968) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000898187); };
            c.cmp(t968, c.r64(12), 64);
            if c.cc_a() { blk = 962; continue; }
            blk = 960; continue;
        }
        960 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000898196, 0x7000898191);
        }
        961 => {
            blk = 963; continue;
        }
        962 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000898198); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 963; continue;
        }
        963 => {
            let Some(t969) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008981a2); };
            c.w64(9, t969);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008981a9); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x90)) { return RegfileExit::fault(0x70008981ad); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008981b5); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            blk = 964; continue;
        }
        964 => {
            c.w64(9, c.r64(0));
            blk = 539; continue;
        }
        965 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 967; continue; }
            blk = 966; continue;
        }
        966 => {
            let Some(t970) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008981d6); };
            c.w64(0, t970);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008981dd); }
            blk = 968; continue;
        }
        967 => {
            let Some(t971) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008981e4); };
            c.w64(0, t971);
            let r972 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r972);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 968; continue;
        }
        968 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(9, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x98), 1) { return RegfileExit::fault(0x7000898215); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 974; continue; }
            blk = 969; continue;
        }
        969 => {
            if c.cc_ne() { blk = 974; continue; }
            blk = 970; continue;
        }
        970 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 972; continue; }
            blk = 971; continue;
        }
        971 => {
            c.movq_rx(11, 1);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 974; continue; }
            blk = 972; continue;
        }
        972 => {
            c.w64(11, c.r64(9));
            let r973 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r973);
            if c.cc_o() { blk = 974; continue; }
            blk = 973; continue;
        }
        973 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 568; continue;
        }
        974 => {
            let Some(t974) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089824a); };
            c.w64(9, t974);
            let Some(t975) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000898251); };
            c.w64(11, t975);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t976) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000898258); };
            c.w64(15, t976);
            let Some(t977) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089825f); };
            c.cmp(t977, c.r64(12), 64);
            if c.cc_a() { blk = 977; continue; }
            blk = 975; continue;
        }
        975 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089826e, 0x7000898269);
        }
        976 => {
            blk = 978; continue;
        }
        977 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000898270); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 978; continue;
        }
        978 => {
            let Some(t978) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089827a); };
            c.w64(9, t978);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000898281); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x98)) { return RegfileExit::fault(0x7000898285); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089828d); }
            blk = 568; continue;
        }
        979 => {
            let Some(t979) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000898297); };
            c.cmp(t979, 0x0u64, 32);
            if c.cc_ne() { blk = 981; continue; }
            blk = 980; continue;
        }
        980 => {
            let Some(t980) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089829d); };
            c.w64(12, t980);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008982a1); }
            blk = 982; continue;
        }
        981 => {
            let Some(t981) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008982aa); };
            c.w64(12, t981);
            let r982 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r982);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 982; continue;
        }
        982 => {
            let Some(t983) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008982ba); };
            c.cmp(t983, 0x0u64, 32);
            if c.cc_ne() { blk = 984; continue; }
            blk = 983; continue;
        }
        983 => {
            let Some(t984) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008982c0); };
            c.w64(9, t984);
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008982c4); }
            blk = 985; continue;
        }
        984 => {
            let Some(t985) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008982cc); };
            c.w64(9, t985);
            let r986 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r986);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 985; continue;
        }
        985 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(9, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 1) { return RegfileExit::fault(0x70008982ee); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 991; continue; }
            blk = 986; continue;
        }
        986 => {
            if c.cc_ne() { blk = 991; continue; }
            blk = 987; continue;
        }
        987 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 989; continue; }
            blk = 988; continue;
        }
        988 => {
            c.movq_rx(11, 1);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 991; continue; }
            blk = 989; continue;
        }
        989 => {
            c.w64(11, c.r64(9));
            let r987 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r987);
            if c.cc_o() { blk = 991; continue; }
            blk = 990; continue;
        }
        990 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 996; continue;
        }
        991 => {
            let Some(t988) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000898320); };
            c.w64(9, t988);
            let Some(t989) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000898327); };
            c.w64(11, t989);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t990) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089832e); };
            c.w64(0, t990);
            let Some(t991) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000898335); };
            c.cmp(t991, c.r64(12), 64);
            if c.cc_a() { blk = 994; continue; }
            blk = 992; continue;
        }
        992 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000898344, 0x700089833f);
        }
        993 => {
            blk = 995; continue;
        }
        994 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000898346); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 995; continue;
        }
        995 => {
            let Some(t992) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000898350); };
            c.w64(9, t992);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000898357); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700089835b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000898360); }
            let Some(t993) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898365); };
            c.w64(15, t993);
            blk = 996; continue;
        }
        996 => {
            c.w64(9, c.r64(0));
            blk = 581; continue;
        }
        997 => {
            let Some(t994) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000898371); };
            c.cmp(t994, 0x0u64, 32);
            if c.cc_ne() { blk = 999; continue; }
            blk = 998; continue;
        }
        998 => {
            let Some(t995) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000898377); };
            c.w64(11, t995);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700089837b); }
            blk = 1000; continue;
        }
        999 => {
            let Some(t996) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000898383); };
            c.w64(11, t996);
            let r997 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r997);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 1000; continue;
        }
        1000 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 1002; continue; }
            blk = 1001; continue;
        }
        1001 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898399); }
            blk = 1003; continue;
        }
        1002 => {
            let r998 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r998);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 1003; continue;
        }
        1003 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008983bf); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1009; continue; }
            blk = 1004; continue;
        }
        1004 => {
            if c.cc_ne() { blk = 1009; continue; }
            blk = 1005; continue;
        }
        1005 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 1007; continue; }
            blk = 1006; continue;
        }
        1006 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 1009; continue; }
            blk = 1007; continue;
        }
        1007 => {
            c.w64(11, c.r64(9));
            let r999 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r999);
            if c.cc_o() { blk = 1009; continue; }
            blk = 1008; continue;
        }
        1008 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 1014; continue;
        }
        1009 => {
            let Some(t1000) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008983f1); };
            c.w64(9, t1000);
            let Some(t1001) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008983f8); };
            c.w64(11, t1001);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t1002) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008983ff); };
            c.w64(0, t1002);
            let Some(t1003) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000898406); };
            c.cmp(t1003, c.r64(12), 64);
            if c.cc_a() { blk = 1012; continue; }
            blk = 1010; continue;
        }
        1010 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000898415, 0x7000898410);
        }
        1011 => {
            blk = 1013; continue;
        }
        1012 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000898417); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1013; continue;
        }
        1013 => {
            let Some(t1004) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000898421); };
            c.w64(9, t1004);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000898428); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700089842c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000898431); }
            let Some(t1005) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898436); };
            c.w64(15, t1005);
            blk = 1014; continue;
        }
        1014 => {
            c.w64(9, c.r64(0));
            blk = 585; continue;
        }
        1015 => {
            c.w32(2, c.r32(2));
            let r1006 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1006);
            c.w64(6, 0xfffffffffffc0000u64);
            let r1007 = c.r64(6) & c.r64(2);
            c.w64(6, r1007);
            c.logic(r1007, 64);
            let Some(t1008) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000898451); };
            c.logic(t1008 & 0x2u64, 8);
            if c.cc_e() { blk = 410; continue; }
            blk = 1016; continue;
        }
        1016 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089845f); }
            c.w64(7, c.r64(11));
            return c.call(0x70007f0e54, 0x7000898468, 0x7000898463);
        }
        1017 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000898468); }
            blk = 410; continue;
        }
        1018 => {
            c.w32(6, c.r32(6));
            let r1009 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r1009);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1010 = c.r64(8) & c.r64(6);
            c.w64(8, r1010);
            c.logic(r1010, 64);
            let Some(t1011) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089847d); };
            c.logic(t1011 & 0x2u64, 8);
            if c.cc_e() { blk = 395; continue; }
            blk = 1019; continue;
        }
        1019 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089848c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089848d); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089849a, 0x7000898495);
        }
        1020 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089849a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089849c); }
            blk = 395; continue;
        }
        1021 => {
            c.w32(0, c.r32(0));
            let r1012 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1012);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1013 = c.r64(7) & c.r64(0);
            c.w64(7, r1013);
            c.logic(r1013, 64);
            let Some(t1014) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008984b1); };
            c.logic(t1014 & 0x2u64, 8);
            if c.cc_e() { blk = 388; continue; }
            blk = 1022; continue;
        }
        1022 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008984bf); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008984c0); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008984cd, 0x70008984c8);
        }
        1023 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008984cd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008984cf); }
            blk = 388; continue;
        }
        1024 => {
            c.w32(0, c.r32(0));
            let r1015 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1015);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1016 = c.r64(7) & c.r64(0);
            c.w64(7, r1016);
            c.logic(r1016, 64);
            let Some(t1017) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008984e4); };
            c.logic(t1017 & 0x2u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 1025; continue;
        }
        1025 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008984f2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008984f3); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000898500, 0x70008984fb);
        }
        1026 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000898500); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000898502); }
            blk = 377; continue;
        }
        1027 => {
            c.w32(0, c.r32(0));
            let r1018 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1018);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1019 = c.r64(7) & c.r64(0);
            c.w64(7, r1019);
            c.logic(r1019, 64);
            let Some(t1020) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000898517); };
            c.logic(t1020 & 0x2u64, 8);
            if c.cc_e() { blk = 371; continue; }
            blk = 1028; continue;
        }
        1028 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000898525); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000898526); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000898533, 0x700089852e);
        }
        1029 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000898533); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000898535); }
            blk = 371; continue;
        }
        1030 => {
            c.w32(0, c.r32(0));
            let r1021 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1021);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1022 = c.r64(9) & c.r64(0);
            c.w64(9, r1022);
            c.logic(r1022, 64);
            let Some(t1023) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089854a); };
            c.logic(t1023 & 0x2u64, 8);
            if c.cc_e() { blk = 360; continue; }
            blk = 1031; continue;
        }
        1031 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000898559); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089855a); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000898567, 0x7000898562);
        }
        1032 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000898567); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000898569); }
            blk = 360; continue;
        }
        1033 => {
            c.w32(2, c.r32(2));
            let r1024 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1024);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1025 = c.r64(1) & c.r64(2);
            c.w64(1, r1025);
            c.logic(r1025, 64);
            let Some(t1026) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089857e); };
            c.logic(t1026 & 0x2u64, 8);
            if c.cc_e() { blk = 354; continue; }
            blk = 1034; continue;
        }
        1034 => {
            c.w64(6, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089858c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089858d); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x700089859a, 0x7000898595);
        }
        1035 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089859a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089859c); }
            blk = 354; continue;
        }
        1036 => {
            c.w32(0, c.r32(0));
            let r1027 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1027);
            c.w64(12, 0xfffffffffffc0000u64);
            let r1028 = c.r64(12) & c.r64(0);
            c.w64(12, r1028);
            c.logic(r1028, 64);
            let Some(t1029) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008985b1); };
            c.logic(t1029 & 0x2u64, 8);
            if c.cc_e() { blk = 323; continue; }
            blk = 1037; continue;
        }
        1037 => {
            c.w64(15, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008985c1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008985c2); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008985cf, 0x70008985ca);
        }
        1038 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008985cf); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008985d1); }
            blk = 323; continue;
        }
        1039 => {
            c.w32(0, c.r32(0));
            let r1030 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1030);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1031 = c.r64(11) & c.r64(0);
            c.w64(11, r1031);
            c.logic(r1031, 64);
            let Some(t1032) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008985e6); };
            c.logic(t1032 & 0x2u64, 8);
            if c.cc_e() { blk = 271; continue; }
            blk = 1040; continue;
        }
        1040 => {
            c.w64(15, c.r64(9).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008985f5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008985f6); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000898603, 0x70008985fe);
        }
        1041 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000898603); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000898605); }
            blk = 271; continue;
        }
        1042 => {
            c.w32(0, c.r32(0));
            let r1033 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1033);
            c.w64(12, 0xfffffffffffc0000u64);
            let r1034 = c.r64(12) & c.r64(0);
            c.w64(12, r1034);
            c.logic(r1034, 64);
            let Some(t1035) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089861a); };
            c.logic(t1035 & 0x2u64, 8);
            if c.cc_e() { blk = 226; continue; }
            blk = 1043; continue;
        }
        1043 => {
            c.w64(15, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089862a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089862b); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000898638, 0x7000898633);
        }
        1044 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000898638); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089863a); }
            blk = 226; continue;
        }
        1045 => {
            return RegfileExit::jump(0x7000898644);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000898654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000898654); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000898658); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000898659); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089865a); }
            let r0 = c.sub(c.r64(4), 0x90u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000898662); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000898666); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089866a); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000898682); };
            c.w64(11, t2);
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000898693); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x7000898697); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700089869b); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008986a9); };
            c.w32(15, t3);
            let r4 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r4);
            let Some(t5) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008986b0); };
            c.cmp(t5, 0x834u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, 0x1u64);
            blk = 5; continue;
        }
        4 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x1u64);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008986d0); };
            c.w32(2, t6);
            let r7 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r7);
            let r8 = c.r32(2) & 0x3fu64;
            c.w32(2, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_a() { blk = 47; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008986e4); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008986ea); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008986f0); };
            c.w32(1, t13);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4cu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008986ff);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000898700); };
            c.w32(2, t14);
            let Some(t15) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000898706); };
            c.cmp(t15, c.r32(2), 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000898713); };
            c.w64(2, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089871a); };
            c.cmp(t17, c.r32(15), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000898724); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            let Some(t20) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089872b); };
            c.w32(1, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700089872e); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089873b); };
            c.w32(1, t22);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000898748); };
            c.w32(7, t23);
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ae() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089875e); };
            c.w64(2, t24);
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000898765); };
            c.w32(2, t25);
            let r26 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r26);
            return c.call(0x7000824234, 0x7000898773, 0x700089876e);
        }
        16 => {
            return RegfileExit::fault(0x7000898773);
        }
        17 => {
            return RegfileExit::fault(0x7000898774);
        }
        18 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 133; continue; }
            blk = 20; continue;
        }
        20 => {
            let r27 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r27);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w32(0, c.r32(0));
            let r28 = c.imul2(c.r32(0), 0x4u64, 32);
            c.w32(0, r28);
            if c.cc_o() { blk = 33; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(6, 0x8u64);
            let r29 = c.add(c.r64(0), c.r64(6), 64);
            c.w64(0, r29);
            if c.cc_o() { blk = 32; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008987b3); };
            c.w64(3, t30);
            let Some(t31) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008987ba); };
            c.w64(3, t31);
            c.w64(12, c.r64(3).wrapping_add(c.r64(0)));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008987c1); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008987c8); };
            c.cmp(t33, c.r64(12), 64);
            if c.cc_be() { blk = 131; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 131; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008987dd); };
            c.w64(8, t34);
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008987e4); }
            c.w64(8, c.r64(3).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008987eb); };
            c.w64(12, t35);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008987f2); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008987f6); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(12, c.r64(6));
            blk = 27; continue;
        }
        26 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000898807); }
            let r36 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r36);
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r64(9), c.r64(6), 64);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(0, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(0), c.r64(12), 64);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(c.r64(12)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000898822); };
            c.w32(0, t37);
            let r38 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r38);
            blk = 26; continue;
        }
        30 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089882d); };
            c.w64(0, t39);
            blk = 26; continue;
        }
        31 => {
            c.w64(2, c.r64(8));
            c.w32(3, 0x1u64);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089883e); };
            c.w64(6, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898842); };
            c.w64(0, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000898846); };
            c.w64(9, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089884a); };
            c.w64(12, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089884e); };
            c.w64(8, t44);
            blk = 35; continue;
        }
        32 => {
            return RegfileExit::fault(0x7000898854);
        }
        33 => {
            return RegfileExit::fault(0x7000898855);
        }
        34 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000898856); };
            c.w64(2, t45);
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700089885d); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r46 = c.r64(1) & c.r64(11);
            c.w64(1, r46);
            c.logic(r46, 64);
            let Some(t47) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000898870); };
            c.logic(t47 & 0x4u64, 8);
            if c.cc_ne() { blk = 242; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 39; continue;
        }
        38 => {
            return RegfileExit::fault(0x700089887c);
        }
        39 => {
            let Some(t48) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089887d); };
            c.w32(2, t48);
            let r49 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000898884); };
            c.w32(1, t50);
            c.logic(c.r32(1) & 0x8000000u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t51) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000898897); };
            c.w32(2, t51);
            let r52 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r52);
            let Some(t53) = c.ld16(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089889d); };
            c.w32(1, t53);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            return RegfileExit::fault(0x70008988a7);
        }
        43 => {
            let Some(t54) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008988a8); };
            c.w32(2, t54);
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t55) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008988b0); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008988b4); };
            c.w64(5, t56);
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008988b8); };
            c.w64(2, t57);
            c.w64(7, c.r64(8));
            let r58 = c.add(c.r64(4), 0xb0u64, 64);
            c.w64(4, r58);
            return RegfileExit::jump(0x7000827694);
        }
        45 => {
            blk = 48; continue;
        }
        46 => {
            c.w32(3, 0x1u64);
            blk = 47; continue;
        }
        47 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t59) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008988e2); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008988e9); };
            c.cmp(t61, 0xbau64, 16);
            if c.cc_b() { blk = 135; continue; }
            blk = 50; continue;
        }
        50 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(11)) { return RegfileExit::fault(0x7000898902); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t62) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089890f); };
            c.w32(12, t62);
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            let Some(t64) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000898916); };
            c.cmp(t64, 0x834u64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000898920); };
            c.w32(8, t65);
            let r66 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r66);
            blk = 74; continue;
        }
        53 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 55; continue;
        }
        54 => {
            c.w64(12, c.r64(3));
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t67) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089893a); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            let Some(t69) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000898941); };
            c.w32(12, t69);
            let r70 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000898949); };
            c.w32(2, t71);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t72) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000898951); };
            c.w32(1, t72);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xb0u64, 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            return RegfileExit::fault(0x7000898961);
        }
        58 => {
            let Some(t73) = c.ld32(c.r64(12).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x7000898962); };
            c.cmp(t73, c.r32(1), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 139; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t74) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089897a); };
            c.w32(8, t74);
            let r75 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r75);
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000898981); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000898988); };
            c.w32(12, t78);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t79) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089898f); };
            c.w32(2, t79);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x44u64, 64);
            if c.cc_a() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x700089899c);
        }
        62 => {
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(0x117)) else { return RegfileExit::fault(0x700089899d); };
            c.cmp(t80, c.r32(2), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r64(12), 0xaeu64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x70008989af);
        }
        65 => {
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008989b0); };
            c.cmp(t81, c.r32(2), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(12), 0xafu64, 64);
            if c.cc_a() { blk = 138; continue; }
            blk = 67; continue;
        }
        67 => {
            return RegfileExit::fault(0x70008989c6);
        }
        68 => {
            let Some(t82) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008989c7); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            blk = 69; continue;
        }
        69 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a17f4, 0x70008989dc, 0x70008989d7);
        }
        71 => {
            c.w64(8, c.r64(0));
            c.w32(3, 0x1u64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008989e4); };
            c.w64(6, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008989e8); };
            c.w64(11, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008989ef); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008989f3); };
            c.w64(9, t87);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 74; continue;
        }
        72 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ge() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(8, c.r64(15));
            blk = 74; continue;
        }
        74 => {
            c.w64(12, c.r64(0));
            c.w32(0, c.r64(12).wrapping_add(c.r64(12)));
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.w32(1, c.r32(0));
            c.w32(7, c.r32(8));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(2), 0x0u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000898a30); }
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(0)) { return RegfileExit::fault(0x7000898a34); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000898a3b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000898a3f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000898a43); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000898a47); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000898a4b); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_le() { blk = 121; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(3, c.r64(7));
            let r88 = c.add(c.r32(3), c.r32(1), 32);
            c.w32(3, r88);
            if c.cc_o() { blk = 141; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000898a85); };
            c.w64(12, t89);
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(0x15f)) else { return RegfileExit::fault(0x7000898a8c); };
            c.w32(12, t90);
            let r91 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r91);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 80; continue;
        }
        80 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898a9d); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898aaa); }
            blk = 85; continue;
        }
        82 => {
            let r92 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r92);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 85; continue;
        }
        83 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 84; continue;
        }
        84 => {
            let r93 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r93);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898ac6); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 85; continue;
        }
        85 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 127; continue; }
            blk = 86; continue;
        }
        86 => {
            blk = 88; continue;
        }
        87 => {
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_l() { blk = 127; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000898ae9); };
            c.w64(8, t94);
            blk = 90; continue;
        }
        89 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000898aef); };
            c.w64(0, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898af6); };
            c.w64(6, t96);
            blk = 90; continue;
        }
        90 => {
            c.w32(3, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x7000898b09); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x7000898b0d); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898b17); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 94; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 113; continue;
        }
        93 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_le() { blk = 113; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000898b42); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(12)) { return RegfileExit::fault(0x7000898b46); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 160; continue; }
            blk = 95; continue;
        }
        95 => {
            let r97 = c.sub(c.r32(3), 0x2u64, 32);
            c.w32(3, r97);
            if c.cc_o() { blk = 160; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(3)) { return RegfileExit::fault(0x7000898b60); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 176; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t98) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000898b71); };
            c.cmp(t98, 0x0u64, 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000898b7b); };
            c.w64(3, t99);
            let Some(t100) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000898b7f); };
            let r101 = c.add(c.r32(3), t100, 32);
            c.w32(3, r101);
            if c.cc_o() { blk = 175; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 101; continue;
        }
        101 => {
            c.w32(8, c.r32(3));
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 102; continue;
        }
        102 => {
            let r102 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r102);
            if c.cc_o() { blk = 194; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 104; continue;
        }
        104 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(8)) { return RegfileExit::fault(0x7000898bad); }
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000898bb4); };
            c.w64(0, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000898bbb); };
            c.w64(3, t104);
            return c.call(0x7000820354, 0x7000898bc7, 0x7000898bc2);
        }
        105 => {
            let Some(t105) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000898bc7); };
            c.cmp(t105, c.r32(0), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000898bd0); };
            c.w64(0, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000898bd7); };
            c.w64(3, t107);
            c.w32(1, 0x2u64);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898be3); };
            c.w64(6, t108);
            return c.call(0x7000820e54, 0x7000898bec, 0x7000898be7);
        }
        107 => {
            blk = 110; continue;
        }
        108 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000898bee); };
            c.w64(0, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000898bf5); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898bfc); };
            c.w64(6, t111);
            return c.call(0x7000870914, 0x7000898c05, 0x7000898c00);
        }
        109 => {
            c.w64(1, c.r64(0));
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000898c08); };
            c.w64(0, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000898c0f); };
            c.w64(3, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898c16); };
            c.w64(6, t114);
            return c.call(0x7000871cb4, 0x7000898c1f, 0x7000898c1a);
        }
        110 => {
            let Some(t115) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000898c1f); };
            c.cmp(t115, 0x0u64, 32);
            if c.cc_ne() { blk = 209; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000898c29); };
            c.w64(8, t116);
            let r117 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r117);
            if c.cc_o() { blk = 209; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 89; continue;
        }
        113 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 116; continue;
        }
        114 => {
            c.w32(11, c.r64(3).wrapping_add(0x2));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x7000898c4a); }
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000898c4e); };
            c.w64(0, t118);
            c.w64(1, c.r64(9));
            return c.call(0x7000871cb4, 0x7000898c5d, 0x7000898c58);
        }
        115 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000898c5d); };
            c.w64(3, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898c61); };
            c.w64(6, t120);
            blk = 116; continue;
        }
        116 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000898c65); };
            c.w64(8, t121);
            let Some(t122) = c.ld32(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000898c69); };
            c.cmp(c.r32(3), t122, 32);
            if c.cc_ge() { blk = 120; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t123) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898c74); };
            c.w64(11, (t123 as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t124) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x7000898c83); };
            c.w64(9, t124);
            blk = 114; continue;
        }
        119 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000898c89); };
            c.w64(9, t125);
            blk = 114; continue;
        }
        120 => {
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000898c92); };
            c.w64(8, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000898c96); };
            c.w64(2, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000898c9a); };
            c.w64(9, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000898c9e); };
            c.w64(1, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000898ca2); };
            c.w64(7, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000898ca6); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898caa); };
            c.w64(12, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000898cae); };
            c.w64(0, t133);
            blk = 121; continue;
        }
        121 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x7000898cb8); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 224; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 224; continue; }
            blk = 123; continue;
        }
        123 => {
            let r134 = c.add(c.r32(1), c.r32(7), 32);
            c.w32(1, r134);
            if c.cc_o() { blk = 224; continue; }
            blk = 124; continue;
        }
        124 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 125; continue;
        }
        125 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000898cdb); };
            c.w64(3, t135);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000898ce2); }
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000898ce6); };
            c.w64(0, t136);
            c.w64(1, c.r64(8));
            return c.call(0x7000871cb4, 0x7000898cf5, 0x7000898cf0);
        }
        126 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898cf5); };
            c.w64(8, t137);
            let r138 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000898cfd); };
            c.w64(0, t139);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000898d04); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000898d05); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000898d0b); }
            return c.ret(0x0, 0x7000898d0d);
        }
        127 => {
            c.w32(8, 0x192u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000898d14); }
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000898d16); };
            c.w64(3, t140);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000898d27, 0x7000898d22);
        }
        128 => {
            return RegfileExit::fault(0x7000898d27);
        }
        129 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000898d2a); };
            c.w64(3, t141);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000898d39, 0x7000898d34);
        }
        130 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898d39); };
            c.w64(6, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898d3d); };
            c.w64(0, t143);
            blk = 1; continue;
        }
        131 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(11)) { return RegfileExit::fault(0x7000898d46); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000898d4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000898d51); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000898d55); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000898d59); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x7000898d65, 0x7000898d60);
        }
        132 => {
            c.w64(8, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000898d6b); };
            c.w64(11, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000898d72); };
            c.w64(2, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000898d76); };
            c.w64(1, t146);
            c.w32(6, 0x8u64);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000898d7f); };
            c.w64(9, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000898d83); };
            c.w64(7, t148);
            blk = 25; continue;
        }
        133 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000898d8c); };
            c.w64(3, t149);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x7000898d9d, 0x7000898d98);
        }
        134 => {
            return RegfileExit::fault(0x7000898d9d);
        }
        135 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x7000898da6, 0x7000898da1);
        }
        136 => {
            c.w64(11, c.r64(0));
            c.w32(3, 0x1u64);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898dae); };
            c.w64(6, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898db2); };
            c.w64(0, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000898db6); };
            c.w64(9, t152);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        137 => {
            let Some(t153) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000898dc2); };
            c.w32(8, t153);
            let r154 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r154);
            blk = 69; continue;
        }
        138 => {
            let Some(t155) = c.ld32(c.r64(8).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000898dce); };
            c.cmp(t155, c.r32(2), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000898ddb); };
            c.w64(3, t156);
            c.w64(0, c.r64(11));
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x7000898ded, 0x7000898de8);
        }
        140 => {
            c.w64(8, c.r64(0));
            c.w32(3, 0x1u64);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898df5); };
            c.w64(6, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000898df9); };
            c.w64(11, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898e00); };
            c.w64(0, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000898e04); };
            c.w64(9, t160);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 69; continue;
        }
        141 => {
            c.w32(3, 0x1u64);
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898e1b); }
            blk = 145; continue;
        }
        144 => {
            let r161 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r161);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898e31); }
            blk = 148; continue;
        }
        147 => {
            c.w64(0, c.r64(8));
            let r162 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r162);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 148; continue;
        }
        148 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(0, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x7000898e56); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 154; continue; }
            blk = 149; continue;
        }
        149 => {
            if c.cc_ne() { blk = 154; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 154; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(8, c.r64(0));
            let r163 = c.add(c.r32(8), c.r32(0), 32);
            c.w32(8, r163);
            if c.cc_o() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 159; continue;
        }
        154 => {
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000898e8a); };
            c.w64(8, t164);
            let Some(t165) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000898e91); };
            c.w64(0, t165);
            c.w64(8, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x7000898e98); }
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000898e9c); };
            c.w64(0, t166);
            let Some(t167) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000898ea3); };
            c.cmp(t167, c.r64(8), 64);
            if c.cc_a() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000898eb5, 0x7000898eb0);
        }
        156 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w32(3, 0x1u64);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898ebd); };
            c.w64(6, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000898ec1); };
            c.w64(2, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000898ec5); };
            c.w64(9, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000898ec9); };
            c.w64(1, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000898ecd); };
            c.w64(7, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000898ed1); };
            c.w64(11, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000898ed5); };
            c.w64(12, t174);
            blk = 158; continue;
        }
        157 => {
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000898edb); };
            c.w64(0, t175);
            if !c.st64(c.r64(0), c.r64(8)) { return RegfileExit::fault(0x7000898ee2); }
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000898ee5); };
            c.w64(8, t176);
            let r177 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r177);
            c.w64(0, c.r64(8));
            blk = 158; continue;
        }
        158 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000898ef0); };
            c.w64(8, t178);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000898ef7); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000898efb); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000898f00); }
            blk = 159; continue;
        }
        159 => {
            c.w64(3, c.r64(0));
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000898f08); };
            c.w64(0, t179);
            blk = 79; continue;
        }
        160 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000898f1a); }
            blk = 163; continue;
        }
        162 => {
            c.w64(3, c.r64(8));
            let r180 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r180);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 163; continue;
        }
        163 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x88), 0) { return RegfileExit::fault(0x7000898f4d); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 169; continue; }
            blk = 164; continue;
        }
        164 => {
            if c.cc_ne() { blk = 169; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 169; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(8, c.r64(3));
            let r181 = c.add(c.r32(8), c.r32(3), 32);
            c.w32(8, r181);
            if c.cc_o() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(3));
            blk = 174; continue;
        }
        169 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000898f81); };
            c.w64(8, t182);
            let Some(t183) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000898f88); };
            c.w64(3, t183);
            c.w64(8, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(3)) { return RegfileExit::fault(0x7000898f8f); }
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000898f96); };
            c.w64(3, t184);
            let Some(t185) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000898f9d); };
            c.cmp(t185, c.r64(8), 64);
            if c.cc_a() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000898faf, 0x7000898faa);
        }
        171 => {
            c.w64(8, c.r64(0));
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000898fb2); };
            c.w64(6, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000898fb6); };
            c.w64(11, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000898fba); };
            c.w64(12, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000898fbe); };
            c.w64(0, t189);
            blk = 173; continue;
        }
        172 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000898fc7); };
            c.w64(3, t190);
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x7000898fce); }
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000898fd1); };
            c.w64(8, t191);
            let r192 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r192);
            blk = 173; continue;
        }
        173 => {
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000898fdc); };
            c.w64(3, t193);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000898fe3); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x88)) { return RegfileExit::fault(0x7000898fe7); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000898fef); }
            blk = 174; continue;
        }
        174 => {
            c.w64(3, c.r64(8));
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000898ff8); };
            c.w64(8, t194);
            blk = 97; continue;
        }
        175 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000899001); };
            c.w64(3, t195);
            blk = 176; continue;
        }
        176 => {
            let Some(t196) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000899008); };
            c.cmp(t196, 0x0u64, 32);
            if c.cc_ne() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089900e); }
            blk = 179; continue;
        }
        178 => {
            let r197 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r197);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000899025); }
            blk = 182; continue;
        }
        181 => {
            c.w64(0, c.r64(8));
            let r198 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r198);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 182; continue;
        }
        182 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(0, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x88), 0) { return RegfileExit::fault(0x700089904a); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 188; continue; }
            blk = 183; continue;
        }
        183 => {
            if c.cc_ne() { blk = 188; continue; }
            blk = 184; continue;
        }
        184 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 188; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w64(8, c.r64(0));
            let r199 = c.add(c.r32(8), c.r32(0), 32);
            c.w32(8, r199);
            if c.cc_o() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 193; continue;
        }
        188 => {
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089907e); };
            c.w64(8, t200);
            let Some(t201) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000899085); };
            c.w64(0, t201);
            c.w64(8, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(0)) { return RegfileExit::fault(0x700089908c); }
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899093); };
            c.w64(0, t202);
            let Some(t203) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089909a); };
            c.cmp(t203, c.r64(8), 64);
            if c.cc_a() { blk = 191; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008990ac, 0x70008990a7);
        }
        190 => {
            blk = 192; continue;
        }
        191 => {
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008990ae); };
            c.w64(0, t204);
            if !c.st64(c.r64(0), c.r64(8)) { return RegfileExit::fault(0x70008990b5); }
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008990b8); };
            c.w64(8, t205);
            let r206 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r206);
            c.w64(0, c.r64(8));
            blk = 192; continue;
        }
        192 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008990c6); };
            c.w64(8, t207);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008990cd); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x88)) { return RegfileExit::fault(0x70008990d1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008990d9); }
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008990de); };
            c.w64(3, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008990e5); };
            c.w64(6, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008990e9); };
            c.w64(11, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008990ed); };
            c.w64(12, t211);
            blk = 193; continue;
        }
        193 => {
            c.w64(3, c.r64(0));
            blk = 101; continue;
        }
        194 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008990fe); }
            blk = 197; continue;
        }
        196 => {
            let r212 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r212);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 197; continue;
        }
        197 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x88), 0) { return RegfileExit::fault(0x700089912f); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 203; continue; }
            blk = 198; continue;
        }
        198 => {
            if c.cc_ne() { blk = 203; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            c.movq_rx(0, 0);
            if let Some(sr) = c.shr(c.r64(0), 0x20u64, 64) { c.w64(0, sr); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 203; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(0, c.r64(8));
            let r213 = c.add(c.r32(0), c.r32(8), 32);
            c.w32(0, r213);
            if c.cc_o() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 208; continue;
        }
        203 => {
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000899163); };
            c.w64(8, t214);
            let Some(t215) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089916a); };
            c.w64(0, t215);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(0)) { return RegfileExit::fault(0x7000899171); }
            let Some(t216) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899178); };
            c.w64(0, t216);
            let Some(t217) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089917f); };
            c.cmp(t217, c.r64(3), 64);
            if c.cc_a() { blk = 206; continue; }
            blk = 204; continue;
        }
        204 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000899191, 0x700089918c);
        }
        205 => {
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000899191); };
            c.w64(12, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899195); };
            c.w64(6, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000899199); };
            c.w64(11, t220);
            blk = 207; continue;
        }
        206 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x700089919f); }
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008991a2); };
            c.w64(8, t221);
            let r222 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r222);
            c.w64(0, c.r64(8));
            blk = 207; continue;
        }
        207 => {
            let Some(t223) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008991b0); };
            c.w64(8, t223);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008991b7); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x88)) { return RegfileExit::fault(0x70008991bb); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008991c3); }
            blk = 208; continue;
        }
        208 => {
            c.w64(8, c.r64(0));
            blk = 104; continue;
        }
        209 => {
            let Some(t224) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008991d0); };
            c.cmp(t224, 0x0u64, 32);
            if c.cc_ne() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008991d6); };
            c.w64(8, t225);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008991da); }
            blk = 212; continue;
        }
        211 => {
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008991e2); };
            c.w64(8, t226);
            let r227 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r227);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 212; continue;
        }
        212 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x7000899212); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 218; continue; }
            blk = 213; continue;
        }
        213 => {
            if c.cc_ne() { blk = 218; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 216; continue; }
            blk = 215; continue;
        }
        215 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w64(9, c.r64(8));
            let r228 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r228);
            if c.cc_o() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 223; continue;
        }
        218 => {
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000899244); };
            c.w64(8, t229);
            let Some(t230) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089924b); };
            c.w64(9, t230);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899252); };
            c.w64(12, t231);
            let Some(t232) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000899259); };
            c.cmp(t232, c.r64(11), 64);
            if c.cc_a() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000899269, 0x7000899264);
        }
        220 => {
            blk = 222; continue;
        }
        221 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700089926b); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 222; continue;
        }
        222 => {
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000899275); };
            c.w64(8, t233);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700089927c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000899280); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000899285); }
            blk = 223; continue;
        }
        223 => {
            c.w64(8, c.r64(0));
            blk = 89; continue;
        }
        224 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w64(9, c.r64(0));
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700089929b); }
            blk = 227; continue;
        }
        226 => {
            let r234 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r234);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008992b2); }
            blk = 230; continue;
        }
        229 => {
            let r235 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r235);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 230; continue;
        }
        230 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008992d8); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 236; continue; }
            blk = 231; continue;
        }
        231 => {
            if c.cc_ne() { blk = 236; continue; }
            blk = 232; continue;
        }
        232 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w64(9, c.r64(8));
            let r236 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r236);
            if c.cc_o() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 241; continue;
        }
        236 => {
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089930a); };
            c.w64(8, t237);
            let Some(t238) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000899311); };
            c.w64(9, t238);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899318); };
            c.w64(15, t239);
            let Some(t240) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089931f); };
            c.cmp(t240, c.r64(11), 64);
            if c.cc_a() { blk = 239; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089932e, 0x7000899329);
        }
        238 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089932e); };
            c.w64(12, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899332); };
            c.w64(6, t242);
            blk = 240; continue;
        }
        239 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000899338); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 240; continue;
        }
        240 => {
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000899342); };
            c.w64(8, t243);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000899349); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700089934d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000899352); }
            blk = 241; continue;
        }
        241 => {
            c.w64(8, c.r64(0));
            blk = 125; continue;
        }
        242 => {
            c.w32(2, c.r32(2));
            let r244 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r244);
            c.w64(1, 0xfffffffffffc0000u64);
            let r245 = c.r64(1) & c.r64(2);
            c.w64(1, r245);
            c.logic(r245, 64);
            let Some(t246) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089936e); };
            c.logic(t246 & 0x2u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 243; continue;
        }
        243 => {
            c.w64(7, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089937c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089937d); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(11));
            return c.call(0x70007f0e54, 0x700089938a, 0x7000899385);
        }
        244 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089938a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089938c); }
            blk = 37; continue;
        }
        245 => {
            return RegfileExit::jump(0x7000899394);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008993b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008993b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008993b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008993b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008993ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008993bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008993c9); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008993d2); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008993d8); };
            c.cmp(t5, 0x835u64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008993e0); };
            c.logic(t6 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008993e6); };
            c.w64(1, t7);
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008993fa); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000899401); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x7000899408);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089940b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000899412); }
            return c.ret(0x0, 0x7000899414);
        }
        9 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089941a); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089941b); };
            c.w64(7, t9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x163)) else { return RegfileExit::fault(0x7000899422); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089942b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089942c); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089942d); };
            c.w64(3, t12);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089943e, 0x7000899439);
        }
        10 => {
            return RegfileExit::fault(0x700089943e);
        }
        11 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000899444); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000899445); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x163)) else { return RegfileExit::fault(0x700089944c); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000899455); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000899456); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000899457); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000899468, 0x7000899463);
        }
        12 => {
            return RegfileExit::fault(0x7000899468);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000899469); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089946d); };
            c.w64(3, t17);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089947b, 0x7000899476);
        }
        14 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089947b); };
            c.w64(6, t18);
            blk = 1; continue;
        }
        15 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000899484); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089948b); };
            c.w64(8, t20);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000899492); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899496); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089949d); };
            c.cmp(t22, c.r64(9), 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008994ac, 0x70008994a7);
        }
        17 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008994ac); };
            c.w64(1, t23);
            blk = 19; continue;
        }
        18 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008994b2); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 19; continue;
        }
        19 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let r24 = c.r64(10) | 0x1u64;
            c.w64(10, r24);
            c.logic(r24, 64);
            blk = 22; continue;
        }
        22 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 23; continue;
        }
        23 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008994e3); };
            c.w64(1, t25);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008994ea); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008994ed); }
            blk = 6; continue;
        }
        24 => {
            return RegfileExit::jump(0x70008994f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000899514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000899514); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000899518); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000899519); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089951a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089951f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000899529); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899536); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089953c); };
            c.cmp(t5, 0x835u64, 16);
            if c.cc_ne() { blk = 13; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000899544); };
            c.w32(7, t6);
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(7) & 0x4u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000899553); };
            c.w64(1, t7);
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 7; continue;
        }
        7 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000899567); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089956e); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x7000899575);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000899578); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089957f); }
            return c.ret(0x0, 0x7000899581);
        }
        10 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 7; continue;
        }
        11 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089958b); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089958c); };
            c.w64(7, t9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x167)) else { return RegfileExit::fault(0x7000899593); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089959c); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089959d); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089959e); };
            c.w64(3, t12);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008995af, 0x70008995aa);
        }
        12 => {
            return RegfileExit::fault(0x70008995af);
        }
        13 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008995b5); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008995b6); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x167)) else { return RegfileExit::fault(0x70008995bd); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008995c6); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008995c7); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008995c8); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008995d9, 0x70008995d4);
        }
        14 => {
            return RegfileExit::fault(0x70008995d9);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008995da); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008995de); };
            c.w64(3, t17);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008995ec, 0x70008995e7);
        }
        16 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008995ec); };
            c.w64(6, t18);
            blk = 1; continue;
        }
        17 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008995f5); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008995fc); };
            c.w64(8, t20);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000899603); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899607); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089960e); };
            c.cmp(t22, c.r64(9), 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089961d, 0x7000899618);
        }
        19 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089961d); };
            c.w64(1, t23);
            blk = 21; continue;
        }
        20 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000899623); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let r24 = c.r64(10) | 0x1u64;
            c.w64(10, r24);
            c.logic(r24, 64);
            blk = 24; continue;
        }
        24 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 25; continue;
        }
        25 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000899654); };
            c.w64(1, t25);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089965b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089965e); }
            blk = 7; continue;
        }
        26 => {
            return RegfileExit::jump(0x700089966c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000899674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000899674); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000899678); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000899679); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089967a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089967f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000899689); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899692); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899698); };
            c.cmp(t5, 0x835u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008996a0); };
            c.w32(1, t6);
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(1) & 0x20u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008996ad); };
            c.w64(0, t7);
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008996b4); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008996bb); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x70008996c2);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008996c5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008996cc); }
            return c.ret(0x0, 0x70008996ce);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008996cf); };
            c.w64(0, t9);
            blk = 6; continue;
        }
        10 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008996dd); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008996de); };
            c.w64(1, t10);
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x16b)) else { return RegfileExit::fault(0x70008996e5); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008996ee); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008996ef); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008996f0); };
            c.w64(3, t13);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000899701, 0x70008996fc);
        }
        11 => {
            return RegfileExit::fault(0x7000899701);
        }
        12 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000899707); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000899708); };
            c.w64(1, t14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x16b)) else { return RegfileExit::fault(0x700089970f); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000899718); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000899719); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089971a); };
            c.w64(3, t17);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089972b, 0x7000899726);
        }
        13 => {
            return RegfileExit::fault(0x700089972b);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089972c); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000899730); };
            c.w64(3, t18);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089973e, 0x7000899739);
        }
        15 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089973e); };
            c.w64(6, t19);
            blk = 1; continue;
        }
        16 => {
            return RegfileExit::jump(0x7000899748);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000899754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000899754); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000899758); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000899759); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089975a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089975f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000899769); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899772); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899778); };
            c.cmp(t5, 0x835u64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000899780); };
            c.logic(t6 & 0x10u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000899786); };
            c.w64(1, t7);
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089979a); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008997a1); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x70008997a8);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008997ab); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008997b2); }
            return c.ret(0x0, 0x70008997b4);
        }
        9 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008997ba); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008997bb); };
            c.w64(7, t9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x16f)) else { return RegfileExit::fault(0x70008997c2); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008997cb); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008997cc); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008997cd); };
            c.w64(3, t12);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008997de, 0x70008997d9);
        }
        10 => {
            return RegfileExit::fault(0x70008997de);
        }
        11 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008997e4); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008997e5); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x16f)) else { return RegfileExit::fault(0x70008997ec); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008997f5); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008997f6); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008997f7); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000899808, 0x7000899803);
        }
        12 => {
            return RegfileExit::fault(0x7000899808);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000899809); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089980d); };
            c.w64(3, t17);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089981b, 0x7000899816);
        }
        14 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089981b); };
            c.w64(6, t18);
            blk = 1; continue;
        }
        15 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000899824); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089982b); };
            c.w64(8, t20);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000899832); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899836); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089983d); };
            c.cmp(t22, c.r64(9), 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089984c, 0x7000899847);
        }
        17 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089984c); };
            c.w64(1, t23);
            blk = 19; continue;
        }
        18 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000899852); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 19; continue;
        }
        19 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let r24 = c.r64(10) | 0x1u64;
            c.w64(10, r24);
            c.logic(r24, 64);
            blk = 22; continue;
        }
        22 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 23; continue;
        }
        23 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000899883); };
            c.w64(1, t25);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089988a); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089988d); }
            blk = 6; continue;
        }
        24 => {
            return RegfileExit::jump(0x7000899898);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008998b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008998b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008998b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008998b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008998ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008998bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008998c9); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008998d2); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008998d8); };
            c.cmp(t5, 0x835u64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008998e0); };
            c.w32(1, t6);
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(1) & 0x20u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008998ed); };
            c.w64(0, t7);
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008998f4); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008998fb); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x7000899902);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000899905); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089990c); }
            return c.ret(0x0, 0x700089990e);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089990f); };
            c.w64(0, t9);
            blk = 6; continue;
        }
        10 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089991d); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089991e); };
            c.w64(1, t10);
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x173)) else { return RegfileExit::fault(0x7000899925); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089992e); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089992f); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000899930); };
            c.w64(3, t13);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000899941, 0x700089993c);
        }
        11 => {
            return RegfileExit::fault(0x7000899941);
        }
        12 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000899947); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000899948); };
            c.w64(1, t14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x173)) else { return RegfileExit::fault(0x700089994f); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000899958); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000899959); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089995a); };
            c.w64(3, t17);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089996b, 0x7000899966);
        }
        13 => {
            return RegfileExit::fault(0x700089996b);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089996c); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000899970); };
            c.w64(3, t18);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089997e, 0x7000899979);
        }
        15 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089997e); };
            c.w64(6, t19);
            blk = 1; continue;
        }
        16 => {
            return RegfileExit::jump(0x7000899988);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000899994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000899994); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000899998); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000899999); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089999a); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089999b); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008999a1); };
            c.w64(2, t1);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008999aa); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008999b0); };
            c.w32(2, t4);
            let r5 = c.sub(c.r32(2), 0x82du64, 32);
            c.w32(2, r5);
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_a() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008999bf); };
            c.w64(0, t6);
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008999c6); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008999cd); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x10, 0x70008999d4);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008999d7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008999de); }
            return c.ret(0x0, 0x70008999e0);
        }
        7 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008999e1); };
            c.w64(0, t8);
            blk = 4; continue;
        }
        8 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008999ea); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008999f8, 0x70008999f3);
        }
        9 => {
            blk = 1; continue;
        }
        10 => {
            return RegfileExit::jump(0x70008999fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000899a14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000899a14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000899a18); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x7000899a1e); }
            blk = 3; continue;
        }
        1 => {
            return c.call(0x70008a1014, 0x7000899a29, 0x7000899a24);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000899a29); };
            c.w64(6, t1);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899a31); };
            c.w32(3, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000899a34); };
            c.cmp(t3, c.r32(3), 32);
            if c.cc_ne() { blk = 1; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 7; continue;
        }
        6 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000899a42); }
            return c.ret(0x0, 0x7000899a43);
        }
        7 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000899a44); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 6; continue;
        }
        10 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 20; continue; }
            blk = 12; continue;
        }
        12 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_be() { blk = 20; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000899a82); };
            c.w64(10, t4);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x7000899a89); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(2, 10);
            c.mov128_rr(3, 0);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(0));
            if c.cc_be() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 3, c.xmm_u64(0));
            blk = 15; continue;
        }
        15 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000899abf); };
            c.w64(10, t5);
            if !c.xorpd_rm(3, c.r64(10)) { return RegfileExit::fault(0x7000899ac6); }
            c.mov128_rr(0, 3);
            blk = 20; continue;
        }
        16 => {
            let Some(t6) = c.ld64(0x7000899a8fu64) else { return RegfileExit::fault(0x7000899acf); };
            c.w64(10, t6);
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
            c.mov128_rr(0, 3);
            blk = 20; continue;
        }
        19 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 3, c.xmm_u64(0));
            c.mov128_rr(0, 3);
            blk = 20; continue;
        }
        20 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            if c.cc_ne() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.movq_rx(1, 0);
            if let Some(sr) = c.shr(c.r64(1), 0x20u64, 64) { c.w64(1, sr); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(1, c.r64(3));
            let r7 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r7);
            if c.cc_o() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 6; continue;
        }
        28 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 6; continue;
        }
        29 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000899b51); };
            c.w64(3, t8);
            let Some(t9) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000899b58); };
            c.w64(1, t9);
            c.w64(7, c.r64(1).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000899b5f); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899b64); };
            c.w64(8, t10);
            let Some(t11) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000899b6b); };
            c.cmp(t11, c.r64(7), 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000899b7a, 0x7000899b75);
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x7000899b7c); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(3));
            blk = 33; continue;
        }
        33 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000899b86); };
            c.w64(3, t12);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000899b8d); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000899b90); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000899b95); }
            blk = 6; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000899ba0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000899bb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000899bb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000899bb8); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899bbe); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000899bc4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000899bc8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000899bcc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000899bd0); }
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899bd4); };
            c.cmp(t3, 0x834u64, 16);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(9, 0x1u64);
            blk = 3; continue;
        }
        2 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000899bf4); };
            c.w32(9, t4);
            c.w64(11, c.r64(9));
            let r5 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r5);
            let r6 = c.r32(11) & 0x3fu64;
            c.w32(11, r6);
            c.logic(r6, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000899c04); }
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_a() { blk = 169; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899c12); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000899c19); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000899c20); };
            c.w32(12, t11);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4cu64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000899c30);
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000899c31); };
            c.w32(11, t12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000899c38); };
            c.cmp(t13, c.r32(11), 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000899c42); };
            c.w64(11, t14);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000899c4c); };
            c.cmp(t15, c.r32(12), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 169; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 169; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000899c68); };
            c.w32(11, t16);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_l() { blk = 169; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(3), c.r32(11), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899c7e); };
            c.w32(7, t17);
            let r18 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r18);
            let Some(t19) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899c84); };
            c.w32(15, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000899c88); };
            c.cmp(t20, c.r32(15), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000899c9f); };
            c.w32(15, t21);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_ae() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000899cb4); };
            c.w64(2, t22);
            let Some(t23) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000899cbb); };
            c.w32(2, t23);
            let r24 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r24);
            return c.call(0x7000824234, 0x7000899cc9, 0x7000899cc4);
        }
        18 => {
            return RegfileExit::fault(0x7000899cc9);
        }
        19 => {
            return RegfileExit::fault(0x7000899cca);
        }
        20 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 174; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(8, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w32(8, c.r32(8));
            let r25 = c.imul2(c.r32(8), 0x4u64, 32);
            c.w32(8, r25);
            if c.cc_o() { blk = 34; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(12, 0x8u64);
            let r26 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r26);
            if c.cc_o() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r27 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r27);
            c.w64(2, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000899d17); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000899d1e); };
            c.w64(6, t29);
            c.w64(1, c.r64(6).wrapping_add(c.r64(8)));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000899d25); };
            c.w64(3, t30);
            let Some(t31) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000899d2c); };
            c.cmp(t31, c.r64(1), 64);
            if c.cc_be() { blk = 172; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r64(8), 0x20000u64, 64);
            if c.cc_ae() { blk = 172; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000899d42); };
            c.w64(8, t32);
            if !c.st64(c.r64(8), c.r64(1)) { return RegfileExit::fault(0x7000899d49); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 27; continue;
        }
        27 => {
            c.w64(8, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000899d58); };
            c.w64(3, t33);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000899d5f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000899d62); }
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        28 => {
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000899d6b); }
            let r34 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r34);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_e() { blk = 36; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t35) = c.ld32(c.r64(7).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899d89); };
            c.w32(3, t35);
            let r36 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r36);
            blk = 28; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000899d94); };
            c.w64(3, t37);
            blk = 28; continue;
        }
        33 => {
            return RegfileExit::fault(0x7000899d9d);
        }
        34 => {
            return RegfileExit::fault(0x7000899d9e);
        }
        35 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000899d9f); };
            c.w64(1, t38);
            c.w64(7, c.r64(1));
            blk = 36; continue;
        }
        36 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000899da9); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r39 = c.r64(7) & c.r64(0);
            c.w64(7, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000899dbb); };
            c.logic(t40 & 0x4u64, 8);
            if c.cc_ne() { blk = 219; continue; }
            blk = 38; continue;
        }
        38 => {
            blk = 40; continue;
        }
        39 => {
            return RegfileExit::fault(0x7000899dc7);
        }
        40 => {
            c.w32(1, c.r8(9));
            let r41 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            let r42 = c.r32(1) & 0x3fu64;
            c.w32(1, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_be() { blk = 61; continue; }
            blk = 41; continue;
        }
        41 => {
            let r43 = c.shr(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r43);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899de2); };
            c.w32(1, t44);
            let r45 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r45);
            let Some(t46) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899de8); };
            c.w32(7, t46);
            let r47 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r47);
            let Some(t48) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899dee); };
            c.w32(7, t48);
            let r49 = c.sub(c.r32(7), 0x77u64, 32);
            c.w32(7, r49);
            c.cmp(c.r32(7), 0xcu64, 32);
            if c.cc_a() { blk = 48; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000899dfa); };
            c.w32(7, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000899dfd); };
            c.w64(15, t51);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_b() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            return RegfileExit::fault(0x7000899e06);
        }
        45 => {
            c.w64(7, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899e12); };
            c.w64(3, t52);
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(3)) { return RegfileExit::fault(0x7000899e16); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r53 = c.r64(8) & c.r64(1);
            c.w64(8, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000899e28); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 216; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 170; continue;
        }
        48 => {
            return RegfileExit::fault(0x7000899e38);
        }
        49 => {
            let Some(t55) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899e39); };
            c.logic(t55 & 0x1u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899e3f); };
            c.w64(1, t56);
            blk = 52; continue;
        }
        51 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899e45); };
            c.w64(1, t57);
            let Some(t58) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899e49); };
            c.w32(7, t58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000899e4c); };
            c.cmp(t59, c.r32(7), 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t60) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899e55); };
            c.w32(7, t60);
            let r61 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r61);
            let Some(t62) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899e5b); };
            c.w32(8, t62);
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000899e5f); };
            c.cmp(t63, c.r32(8), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000899e68); };
            c.w32(8, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000899e6c); };
            c.w64(15, t65);
            c.cmp(c.r32(15), c.r32(8), 32);
            if c.cc_b() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            return RegfileExit::fault(0x7000899e75);
        }
        55 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000899e7b); }
            blk = 58; continue;
        }
        57 => {
            let r66 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r66);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 58; continue;
        }
        58 => {
            c.w64(1, (c.r32(15) as u32 as i32 as i64 as u64));
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000899e99); }
            blk = 170; continue;
        }
        59 => {
            return RegfileExit::fault(0x7000899ea4);
        }
        60 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000899ea5); };
            c.w64(3, t67);
            blk = 169; continue;
        }
        61 => {
            let Some(t68) = c.ld8(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899eae); };
            c.logic(t68 & 0x1u64, 8);
            if c.cc_ne() { blk = 69; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t69) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899eb4); };
            c.w32(1, t69);
            let r70 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r70);
            let Some(t71) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899eba); };
            c.w32(7, t71);
            let r72 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r72);
            let Some(t73) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899ec0); };
            c.w32(7, t73);
            let r74 = c.sub(c.r32(7), 0x77u64, 32);
            c.w32(7, r74);
            c.cmp(c.r32(7), 0xcu64, 32);
            if c.cc_a() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t75) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000899ecc); };
            c.w32(7, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000899ecf); };
            c.w64(15, t76);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_b() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x7000899ed8);
        }
        65 => {
            c.w64(7, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899ee4); };
            c.w64(3, t77);
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(3)) { return RegfileExit::fault(0x7000899ee8); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r78 = c.r64(8) & c.r64(1);
            c.w64(8, r78);
            c.logic(r78, 64);
            let Some(t79) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000899efa); };
            c.logic(t79 & 0x4u64, 8);
            if c.cc_ne() { blk = 213; continue; }
            blk = 67; continue;
        }
        67 => {
            blk = 170; continue;
        }
        68 => {
            return RegfileExit::fault(0x7000899f0a);
        }
        69 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000899f0b); };
            c.w64(3, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000899f0f); };
            c.w64(1, t81);
            blk = 169; continue;
        }
        70 => {
            let Some(t82) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000899f18); };
            c.w32(15, t82);
            c.w64(2, c.r64(15));
            let r83 = c.r32(2) & 0x8100000u64;
            c.w32(2, r83);
            c.logic(r83, 32);
            c.cmp(c.r32(2), 0x8000000u64, 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 71; continue;
        }
        71 => {
            c.logic(c.r32(15) & 0x200000u64, 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000899f3e); };
            c.w32(7, t84);
            let r85 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r85);
            let Some(t86) = c.ld16(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000899f44); };
            c.w32(15, t86);
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            return RegfileExit::fault(0x7000899f4f);
        }
        74 => {
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000899f50); };
            c.w32(7, t87);
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 169; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t88) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000899f5d); };
            c.w32(7, t88);
            let r89 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r89);
            let Some(t90) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000899f63); };
            c.w32(15, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000899f67); };
            c.cmp(t91, c.r32(15), 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 76; continue;
        }
        76 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 102; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t92) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000899f7e); };
            c.w32(15, t92);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_a() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_ae() { blk = 82; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000899f93); };
            c.w64(2, t93);
            let Some(t94) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000899f9a); };
            c.w32(2, t94);
            let r95 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r95);
            return c.call(0x7000824234, 0x7000899fa8, 0x7000899fa3);
        }
        80 => {
            return RegfileExit::fault(0x7000899fa8);
        }
        81 => {
            return RegfileExit::fault(0x7000899fa9);
        }
        82 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 98; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 179; continue; }
            blk = 84; continue;
        }
        84 => {
            let r96 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r96);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w32(15, c.r32(15));
            let r97 = c.imul2(c.r32(15), 0x4u64, 32);
            c.w32(15, r97);
            if c.cc_o() { blk = 97; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(2, 0x8u64);
            let r98 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r98);
            if c.cc_o() { blk = 96; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(6, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r99 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r99);
            c.w64(3, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000899ff3); };
            c.w64(1, t100);
            let Some(t101) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000899ffa); };
            c.w64(1, t101);
            c.w64(9, c.r64(1).wrapping_add(c.r64(15)));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089a009); };
            c.w64(0, t102);
            let Some(t103) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089a010); };
            c.cmp(t103, c.r64(9), 64);
            if c.cc_be() { blk = 177; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 177; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089a026); };
            c.w64(15, t104);
            if !c.st64(c.r64(15), c.r64(9)) { return RegfileExit::fault(0x700089a02d); }
            let r105 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r105);
            blk = 89; continue;
        }
        89 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700089a034); };
            c.w64(9, t106);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089a03b); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x700089a03f); }
            c.w64(9, c.r64(2));
            blk = 91; continue;
        }
        90 => {
            if !c.st32(c.r64(1).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700089a047); }
            let r107 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r107);
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_e() { blk = 95; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_e() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t108) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089a065); };
            c.w32(15, t108);
            let r109 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r109);
            blk = 90; continue;
        }
        94 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089a070); };
            c.w64(15, t110);
            blk = 90; continue;
        }
        95 => {
            c.w64(7, c.r64(1));
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089a07c); };
            c.w64(0, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089a080); };
            c.w64(3, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089a084); };
            c.w64(1, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089a088); };
            c.w64(6, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089a08c); };
            c.w64(9, t115);
            blk = 99; continue;
        }
        96 => {
            return RegfileExit::fault(0x700089a092);
        }
        97 => {
            return RegfileExit::fault(0x700089a093);
        }
        98 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089a094); };
            c.w64(7, t116);
            blk = 99; continue;
        }
        99 => {
            c.w64(11, c.r64(0));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700089a09e); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r117 = c.r64(15) & c.r64(11);
            c.w64(15, r117);
            c.logic(r117, 64);
            let Some(t118) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a0b2); };
            c.logic(t118 & 0x4u64, 8);
            if c.cc_ne() { blk = 210; continue; }
            blk = 101; continue;
        }
        101 => {
            blk = 104; continue;
        }
        102 => {
            return RegfileExit::fault(0x700089a0bf);
        }
        103 => {
            c.w64(11, c.r64(0));
            blk = 104; continue;
        }
        104 => {
            c.w32(7, c.r8(9));
            let r119 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r119);
            let r120 = c.r32(7) & 0x3fu64;
            c.w32(7, r120);
            c.logic(r120, 32);
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_be() { blk = 151; continue; }
            blk = 105; continue;
        }
        105 => {
            let r121 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r121);
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t122) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089a0e1); };
            c.w32(7, t122);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(9));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r123 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r123);
            let Some(t124) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089a0f2); };
            c.w32(0, t124);
            let r125 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r125);
            let Some(t126) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089a0f9); };
            c.w32(2, t126);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(2));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700089a105); }
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_ge() { blk = 119; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(6, c.r64(15));
            if let Some(sr) = c.shr(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(6, c.r64(6).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(6), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 123; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r64(6), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 184; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089a137); };
            c.w64(8, t127);
            let Some(t128) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089a13e); };
            c.w64(12, t128);
            c.w64(3, c.r64(6).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089a14d); };
            c.w64(9, t129);
            let Some(t130) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089a154); };
            c.cmp(t130, c.r64(1), 64);
            if c.cc_be() { blk = 181; continue; }
            blk = 110; continue;
        }
        110 => {
            if !c.st64(c.r64(8), c.r64(1)) { return RegfileExit::fault(0x700089a15d); }
            c.w64(1, c.r64(12).wrapping_add(0x1));
            blk = 111; continue;
        }
        111 => {
            c.w32(8, c.r64(6).wrapping_add(c.r64(6)));
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700089a169); };
            c.w64(9, t131);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089a170); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700089a174); }
            c.w64(8, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            c.w64(9, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_e() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089a18d); };
            c.w64(12, t132);
            blk = 113; continue;
        }
        113 => {
            let r133 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r133);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700089a198); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ne() { blk = 113; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            let r134 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r134);
            let Some(t135) = c.ld32(c.r64(8).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x700089a1ab); };
            c.w32(9, t135);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x700089a1af); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 115; continue; }
            blk = 116; continue;
        }
        116 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700089a1b9); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r136 = c.r64(8) & c.r64(11);
            c.w64(8, r136);
            c.logic(r136, 64);
            let Some(t137) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a1cc); };
            c.logic(t137 & 0x4u64, 8);
            if c.cc_ne() { blk = 207; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(0, c.r64(1));
            blk = 119; continue;
        }
        119 => {
            let Some(t138) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089a1da); };
            c.w32(1, t138);
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_ae() { blk = 183; continue; }
            blk = 120; continue;
        }
        120 => {
            c.w32(1, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089a1e9); };
            c.w64(7, t139);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089a1f5); };
            c.w64(9, t140);
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(9)) { return RegfileExit::fault(0x700089a1f9); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r141 = c.r64(8) & c.r64(0);
            c.w64(8, r141);
            c.logic(r141, 64);
            let Some(t142) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a20d); };
            c.logic(t142 & 0x4u64, 8);
            if c.cc_ne() { blk = 204; continue; }
            blk = 122; continue;
        }
        122 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700089a218); }
            blk = 170; continue;
        }
        123 => {
            c.w64(0, c.r64(11));
            blk = 169; continue;
        }
        124 => {
            let Some(t143) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089a229); };
            c.w32(7, t143);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(9));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r144 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r144);
            let Some(t145) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089a23a); };
            c.w32(0, t145);
            let r146 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r146);
            let Some(t147) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089a241); };
            c.w32(2, t147);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(2));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700089a24d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700089a251); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700089a255); }
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_ge() { blk = 141; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w64(6, c.r64(15));
            if let Some(sr) = c.shr(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(6, c.r64(6).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(6), 0x3fffu64, 64);
            if c.cc_ae() { blk = 150; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r64(6), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 191; continue; }
            blk = 127; continue;
        }
        127 => {
            c.w64(3, c.r64(6).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 186; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089a29c); };
            c.w64(9, t148);
            let Some(t149) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089a2a3); };
            c.w64(15, t149);
            c.w64(7, c.r64(3).wrapping_add(c.r64(15)));
            let Some(t150) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a2aa); };
            c.cmp(t150, c.r64(7), 64);
            if c.cc_be() { blk = 188; continue; }
            blk = 129; continue;
        }
        129 => {
            if !c.st64(c.r64(9), c.r64(7)) { return RegfileExit::fault(0x700089a2b4); }
            c.w64(7, c.r64(15).wrapping_add(0x1));
            blk = 130; continue;
        }
        130 => {
            c.w32(9, c.r64(6).wrapping_add(c.r64(6)));
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700089a2bf); };
            c.w64(12, t151);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089a2c6); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089a2ca); }
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089a2ce); };
            c.w64(9, t152);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700089a2d5); }
            c.w64(9, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.w64(12, c.r64(6).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_e() { blk = 132; continue; }
            blk = 131; continue;
        }
        131 => {
            let r153 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r153);
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(7)), 0) { return RegfileExit::fault(0x700089a2f4); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ne() { blk = 131; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 138; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(12, 0xfff7fffffff7ffffu64);
            blk = 134; continue;
        }
        134 => {
            let r154 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r154);
            let Some(t155) = c.ld64(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x700089a313); };
            c.cmp(t155, c.r64(12), 64);
            if c.cc_e() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(0))) { return RegfileExit::fault(0x700089a319); }
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(7)), 0) { return RegfileExit::fault(0x700089a31f); }
            blk = 137; continue;
        }
        136 => {
            if !c.st64(c.r64(9).wrapping_add(c.r64(7)), c.r64(12)) { return RegfileExit::fault(0x700089a327); }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 134; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700089a331); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r156 = c.r64(9) & c.r64(11);
            c.w64(9, r156);
            c.logic(r156, 64);
            let Some(t157) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a345); };
            c.logic(t157 & 0x4u64, 8);
            if c.cc_ne() { blk = 201; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(0, c.r64(7));
            blk = 141; continue;
        }
        141 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t158) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089a358); };
            c.w32(7, t158);
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089a35b); };
            c.cmp(t159, c.r32(7), 32);
            if c.cc_ne() { blk = 150; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(7, c.r64(8));
            blk = 145; continue;
        }
        144 => {
            c.w32(7, 0x1u64);
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700089a373); }
            blk = 148; continue;
        }
        147 => {
            let r160 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r160);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 148; continue;
        }
        148 => {
            let Some(t161) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089a384); };
            c.w32(1, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089a387); };
            c.w64(7, t162);
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_ae() { blk = 190; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089a393); };
            c.w64(1, t163);
            let r164 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r164);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089a3a3); };
            c.w64(7, t165);
            if !c.movsd_mr(c.r64(0).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700089a3a7); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700089a3ad); }
            blk = 170; continue;
        }
        150 => {
            c.w64(0, c.r64(11));
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089a3b9); };
            c.w64(3, t166);
            blk = 169; continue;
        }
        151 => {
            let Some(t167) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089a3c2); };
            c.w32(7, t167);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(9));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r168 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r168);
            let Some(t169) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089a3d3); };
            c.w32(0, t169);
            let r170 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r170);
            let Some(t171) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089a3da); };
            c.w32(2, t171);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(2));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700089a3e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700089a3ea); }
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_ge() { blk = 164; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(6, c.r64(15));
            if let Some(sr) = c.shr(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(6, c.r64(6).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(6), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 168; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r64(6), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 196; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089a41c); };
            c.w64(8, t172);
            let Some(t173) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089a423); };
            c.w64(12, t173);
            c.w64(3, c.r64(6).wrapping_mul(4).wrapping_add(0x8));
            c.w64(9, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089a432); };
            c.w64(15, t174);
            let Some(t175) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089a439); };
            c.cmp(t175, c.r64(9), 64);
            if c.cc_be() { blk = 193; continue; }
            blk = 155; continue;
        }
        155 => {
            if !c.st64(c.r64(8), c.r64(9)) { return RegfileExit::fault(0x700089a442); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 156; continue;
        }
        156 => {
            c.w32(9, c.r64(6).wrapping_add(c.r64(6)));
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700089a44e); };
            c.w64(12, t176);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089a455); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089a459); }
            c.w64(9, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            c.w64(12, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_e() { blk = 159; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089a472); };
            c.w64(15, t177);
            blk = 158; continue;
        }
        158 => {
            let r178 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r178);
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(15)) { return RegfileExit::fault(0x700089a47d); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ne() { blk = 158; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            let r179 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r179);
            let Some(t180) = c.ld32(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x700089a490); };
            c.w32(12, t180);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x700089a494); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 160; continue; }
            blk = 161; continue;
        }
        161 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700089a49e); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r181 = c.r64(9) & c.r64(11);
            c.w64(9, r181);
            c.logic(r181, 64);
            let Some(t182) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a4b2); };
            c.logic(t182 & 0x4u64, 8);
            if c.cc_ne() { blk = 198; continue; }
            blk = 163; continue;
        }
        163 => {
            c.w64(0, c.r64(8));
            blk = 164; continue;
        }
        164 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 167; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t183) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089a4c5); };
            c.w32(8, t183);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ae() { blk = 195; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089a4d2); };
            c.w64(7, t184);
            let r185 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089a4d8); };
            c.w64(8, t186);
            if !c.st32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700089a4dc); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700089a4e1); }
            blk = 170; continue;
        }
        167 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089a4e7); };
            c.w64(3, t187);
            blk = 168; continue;
        }
        168 => {
            c.w64(0, c.r64(11));
            blk = 169; continue;
        }
        169 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089a4ee); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700089a4ef); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089a4f0); }
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x2340)) else { return RegfileExit::fault(0x700089a4f1); };
            c.w64(3, t188);
            c.w32(0, 0x3u64);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089a4fd); };
            c.w64(6, t189);
            return c.call(0x700086ed74, 0x700089a506, 0x700089a501);
        }
        170 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089a506); };
            c.w64(0, t190);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089a510); }
            return c.ret(0x0, 0x700089a511);
        }
        171 => {
            blk = 17; continue;
        }
        172 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700089a517); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700089a51b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700089a51f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700089a523); }
            c.w64(2, c.r64(8));
            return c.call(0x70007febb4, 0x700089a52f, 0x700089a52a);
        }
        173 => {
            c.w64(1, c.r64(0));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089a532); };
            c.w64(0, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089a536); };
            c.w64(9, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089a53a); };
            c.w64(7, t193);
            c.w32(12, 0x8u64);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089a544); };
            c.w64(2, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089a548); };
            c.w64(15, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089a54c); };
            c.w64(11, t196);
            blk = 27; continue;
        }
        174 => {
            let Some(t197) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700089a555); };
            c.w64(3, t197);
            c.w64(0, c.r64(8));
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700089a567, 0x700089a562);
        }
        175 => {
            return RegfileExit::fault(0x700089a567);
        }
        176 => {
            blk = 79; continue;
        }
        177 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700089a56d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700089a571); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700089a575); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700089a579); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x700089a585, 0x700089a580);
        }
        178 => {
            c.w64(1, c.r64(0));
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089a58b); };
            c.w64(7, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089a58f); };
            c.w64(3, t199);
            c.w32(2, 0x8u64);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089a598); };
            c.w64(6, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089a59c); };
            c.w64(11, t201);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 89; continue;
        }
        179 => {
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700089a5a8); };
            c.w64(3, t202);
            c.w64(0, c.r64(8));
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700089a5ba, 0x700089a5b5);
        }
        180 => {
            return RegfileExit::fault(0x700089a5ba);
        }
        181 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700089a5bb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700089a5bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700089a5c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x700089a5c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700089a5cb); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x700089a5d7, 0x700089a5d2);
        }
        182 => {
            c.w64(1, c.r64(0));
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089a5da); };
            c.w64(11, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089a5de); };
            c.w64(15, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089a5e2); };
            c.w64(0, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089a5e6); };
            c.w64(6, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089a5ea); };
            c.w64(2, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089a5ee); };
            c.w64(7, t208);
            blk = 111; continue;
        }
        183 => {
            return RegfileExit::fault(0x700089a5f7);
        }
        184 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700089a5f8); };
            c.w64(3, t209);
            c.w64(0, c.r64(8));
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700089a60a, 0x700089a605);
        }
        185 => {
            return RegfileExit::fault(0x700089a60a);
        }
        186 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700089a60b); };
            c.w64(3, t210);
            c.w64(0, c.r64(8));
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700089a61d, 0x700089a618);
        }
        187 => {
            return RegfileExit::fault(0x700089a61d);
        }
        188 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a621); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700089a622); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700089a624); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700089a628); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x700089a62c); }
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700089a630); };
            c.w64(3, t211);
            c.w64(6, c.r64(12));
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x700089a647, 0x700089a642);
        }
        189 => {
            c.w64(7, c.r64(0));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089a64a); };
            c.w64(11, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089a64e); };
            c.w64(1, t213);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089a655); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089a659); };
            c.w64(6, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089a65d); };
            c.w64(2, t216);
            blk = 130; continue;
        }
        190 => {
            return RegfileExit::fault(0x700089a666);
        }
        191 => {
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700089a667); };
            c.w64(3, t217);
            c.w64(0, c.r64(8));
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700089a679, 0x700089a674);
        }
        192 => {
            return RegfileExit::fault(0x700089a679);
        }
        193 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700089a67a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700089a67e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x700089a682); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700089a686); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x700089a692, 0x700089a68d);
        }
        194 => {
            c.w64(8, c.r64(0));
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089a695); };
            c.w64(11, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089a699); };
            c.w64(1, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089a69d); };
            c.w64(0, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089a6a1); };
            c.w64(6, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089a6a5); };
            c.w64(2, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089a6a9); };
            c.w64(7, t223);
            blk = 156; continue;
        }
        195 => {
            return RegfileExit::fault(0x700089a6b2);
        }
        196 => {
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700089a6b3); };
            c.w64(3, t224);
            c.w64(0, c.r64(8));
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700089a6c5, 0x700089a6c0);
        }
        197 => {
            return RegfileExit::fault(0x700089a6c5);
        }
        198 => {
            c.w32(8, c.r32(8));
            let r225 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r225);
            c.w64(9, 0xfffffffffffc0000u64);
            let r226 = c.r64(9) & c.r64(8);
            c.w64(9, r226);
            c.logic(r226, 64);
            let Some(t227) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a6d6); };
            c.logic(t227 & 0x2u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(12, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a6e5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089a6e6); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700089a6f3, 0x700089a6ee);
        }
        200 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089a6f3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089a6f5); }
            blk = 163; continue;
        }
        201 => {
            c.w32(7, c.r32(7));
            let r228 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r228);
            c.w64(9, 0xfffffffffffc0000u64);
            let r229 = c.r64(9) & c.r64(7);
            c.w64(9, r229);
            c.logic(r229, 64);
            let Some(t230) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a70a); };
            c.logic(t230 & 0x2u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 202; continue;
        }
        202 => {
            c.w64(12, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a719); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089a71a); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700089a727, 0x700089a722);
        }
        203 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089a727); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089a729); }
            blk = 140; continue;
        }
        204 => {
            c.w32(9, c.r32(9));
            let r231 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r231);
            c.w64(8, 0xfffffffffffc0000u64);
            let r232 = c.r64(8) & c.r64(9);
            c.w64(8, r232);
            c.logic(r232, 64);
            let Some(t233) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a73f); };
            c.logic(t233 & 0x2u64, 8);
            if c.cc_e() { blk = 122; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(12, c.r64(0).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a74e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089a74f); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700089a75c, 0x700089a757);
        }
        206 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089a75c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089a75e); }
            blk = 122; continue;
        }
        207 => {
            c.w32(1, c.r32(1));
            let r234 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r234);
            c.w64(8, 0xfffffffffffc0000u64);
            let r235 = c.r64(8) & c.r64(1);
            c.w64(8, r235);
            c.logic(r235, 64);
            let Some(t236) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a773); };
            c.logic(t236 & 0x2u64, 8);
            if c.cc_e() { blk = 118; continue; }
            blk = 208; continue;
        }
        208 => {
            c.w64(9, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a782); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089a783); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089a790, 0x700089a78b);
        }
        209 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089a790); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089a792); }
            blk = 118; continue;
        }
        210 => {
            c.w32(7, c.r32(7));
            let r237 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r237);
            c.w64(15, 0xfffffffffffc0000u64);
            let r238 = c.r64(15) & c.r64(7);
            c.w64(15, r238);
            c.logic(r238, 64);
            let Some(t239) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a7a7); };
            c.logic(t239 & 0x2u64, 8);
            if c.cc_e() { blk = 101; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(0, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a7b6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089a7b7); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700089a7c4, 0x700089a7bf);
        }
        212 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089a7c4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089a7c6); }
            blk = 101; continue;
        }
        213 => {
            c.w32(3, c.r32(3));
            let r240 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r240);
            c.w64(8, 0xfffffffffffc0000u64);
            let r241 = c.r64(8) & c.r64(3);
            c.w64(8, r241);
            c.logic(r241, 64);
            let Some(t242) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a7db); };
            c.logic(t242 & 0x2u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(9, c.r64(1).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a7ea); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089a7eb); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089a7f8, 0x700089a7f3);
        }
        215 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089a7f8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089a7fa); }
            blk = 67; continue;
        }
        216 => {
            c.w32(3, c.r32(3));
            let r243 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r243);
            c.w64(8, 0xfffffffffffc0000u64);
            let r244 = c.r64(8) & c.r64(3);
            c.w64(8, r244);
            c.logic(r244, 64);
            let Some(t245) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a80f); };
            c.logic(t245 & 0x2u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w64(9, c.r64(1).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a81e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089a81f); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089a82c, 0x700089a827);
        }
        218 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089a82c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089a82e); }
            blk = 47; continue;
        }
        219 => {
            c.w32(1, c.r32(1));
            let r246 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r246);
            c.w64(7, 0xfffffffffffc0000u64);
            let r247 = c.r64(7) & c.r64(1);
            c.w64(7, r247);
            c.logic(r247, 64);
            let Some(t248) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089a843); };
            c.logic(t248 & 0x2u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w64(8, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089a851); }
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x700089a85a, 0x700089a855);
        }
        221 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089a85a); }
            blk = 38; continue;
        }
        222 => {
            return RegfileExit::jump(0x700089a864);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089a874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089a874); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089a878); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700089a888); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700089a88c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700089a890); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700089a894); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089a8a1); };
            c.w32(8, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089a8a5); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089a8b2); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x35);
            c.psrlq(1, 0x1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            if c.cc_e() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x34);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if c.cc_e() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.mov128_rr(2, 0);
            blk = 19; continue;
        }
        10 => {
            c.mov128_rr(2, 0);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x700089a91a); };
            c.w64(10, t3);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x700089a921); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(3, 10);
            c.mov128_rr(4, 2);
            c.arith_sd(0, 4, c.xmm_u64(3));
            c.arith_sd(1, 4, c.xmm_u64(3));
            c.ucomisd(c.xmm_u64(4), c.xmm_u64(2));
            if c.cc_be() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(1, 4, c.xmm_u64(2));
            blk = 12; continue;
        }
        12 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x700089a957); };
            c.w64(10, t4);
            if !c.xorpd_rm(4, c.r64(10)) { return RegfileExit::fault(0x700089a95e); }
            c.mov128_rr(2, 4);
            blk = 19; continue;
        }
        13 => {
            c.mov128_rr(2, 0);
            blk = 19; continue;
        }
        14 => {
            let Some(t5) = c.ld64(0x700089a927u64) else { return RegfileExit::fault(0x700089a96c); };
            c.w64(10, t5);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_ae() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            c.mov128_rr(3, 0);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(0));
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.mov128_rr(2, 3);
            blk = 19; continue;
        }
        17 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.mov128_rr(2, 3);
            blk = 19; continue;
        }
        18 => {
            c.mov128_rr(2, 0);
            blk = 19; continue;
        }
        19 => {
            c.arith_sd(1, 0, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 45; continue; }
            blk = 20; continue;
        }
        20 => {
            if c.cc_ne() { blk = 45; continue; }
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700089a9c7); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089a9d1); }
            blk = 27; continue;
        }
        24 => {
            c.w64(8, c.r64(0));
            let r6 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r6);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 27; continue;
        }
        25 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(8, c.r64(3));
            let r7 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r7);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089a9f4); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 27; continue;
        }
        27 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 30; continue;
        }
        29 => {
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_le() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 32; continue;
        }
        31 => {
            c.w32(9, 0x1u64);
            blk = 32; continue;
        }
        32 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089aa36); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700089aa41); }
            blk = 39; continue;
        }
        36 => {
            c.w64(7, c.r64(1));
            let r8 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r8);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 39; continue;
        }
        37 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, c.r64(0));
            let r9 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r9);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700089aa62); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 39; continue;
        }
        39 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 43; continue;
        }
        41 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_g() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(8, 0x1u64);
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089aa87); };
            c.w64(0, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089aa91); }
            return c.ret(0x0, 0x700089aa92);
        }
        45 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089aa93); };
            c.w64(7, t11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x177)) else { return RegfileExit::fault(0x700089aa9a); };
            c.w32(7, t12);
            let r13 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r13);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089aaa3); }
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700089aaab); };
            c.w64(3, t14);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700089aaba, 0x700089aab5);
        }
        46 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089aaba); };
            if !c.push64(t15) { return RegfileExit::fault(0x700089aaba); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2858)) else { return RegfileExit::fault(0x700089aabd); };
            c.w64(3, t16);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089aad0, 0x700089aacb);
        }
        47 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089aad0); };
            if !c.push64(t17) { return RegfileExit::fault(0x700089aad0); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2858)) else { return RegfileExit::fault(0x700089aad3); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089aae6, 0x700089aae1);
        }
        48 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089aae6); };
            if !c.push64(t19) { return RegfileExit::fault(0x700089aae6); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2858)) else { return RegfileExit::fault(0x700089aae9); };
            c.w64(3, t20);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089aafc, 0x700089aaf7);
        }
        49 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089aafc); };
            if !c.push64(t21) { return RegfileExit::fault(0x700089aafc); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2858)) else { return RegfileExit::fault(0x700089aaff); };
            c.w64(3, t22);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089ab12, 0x700089ab0d);
        }
        50 => {
            return RegfileExit::fault(0x700089ab12);
        }
        51 => {
            return RegfileExit::jump(0x700089ab14);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089ab34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089ab34); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089ab41); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089ab45); }
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ab58); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089ab5e); };
            c.cmp(t3, c.r32(7), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ab67); };
            c.w32(8, t4);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(8, c.r16(8));
            c.cmp(c.r32(8), 0x3fu64, 32);
            if c.cc_be() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700089ab7d); };
            c.cmp(t5, c.r32(7), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089ab86); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700089ab8f); };
            c.cmp(t7, c.r32(0), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089ab98); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700089aba1); };
            c.cmp(t9, c.r32(0), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(7, c.r64(8).wrapping_sub(0xba));
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089abd1); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089abd8); };
            c.cmp(t12, c.r32(8), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089abe1); };
            c.w32(9, t13);
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x3fu64, 32);
            if c.cc_be() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700089abf7); };
            c.cmp(t14, c.r32(8), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089ac00); };
            c.cmp(t15, c.r32(3), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700089ac09); };
            c.cmp(t16, c.r32(3), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089ac12); };
            c.cmp(t17, c.r32(3), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700089ac1b); };
            c.cmp(t18, c.r32(3), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(8, c.r64(9).wrapping_sub(0xba));
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ac52); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089ac58); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t22) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ac65); };
            c.w32(1, t22);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 31; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ac79); };
            c.w32(1, t23);
            let r24 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r24);
            let Some(t25) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ac7f); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089ac8a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700089ac8b); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700089ac8e); };
            c.w64(3, t26);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089ac9f, 0x700089ac9a);
        }
        29 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089ac9f); };
            c.cmp(t27, c.r32(0), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 53; continue;
        }
        31 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089acb1); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089acb7); };
            c.cmp(t30, 0x40u64, 16);
            if c.cc_ae() { blk = 53; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x700089accf, 0x700089acca);
        }
        33 => {
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089accf); };
            c.cmp(t31, c.r32(0), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 53; continue;
        }
        35 => {
            let Some(t32) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ace1); };
            c.w32(1, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089ace4); };
            c.cmp(t33, c.r32(1), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089acf1); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700089acf6); }
            blk = 42; continue;
        }
        37 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089acfd); };
            c.w32(1, t34);
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089ad00); };
            c.cmp(t35, c.r32(1), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 38; continue;
        }
        38 => {
            let r36 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r36);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089ad0f); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 42; continue;
        }
        39 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t37) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ad27); };
            c.w32(1, t37);
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089ad2a); };
            c.cmp(t38, c.r32(1), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 41; continue;
        }
        41 => {
            let r39 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x700089ad35); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 42; continue;
        }
        42 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            if c.cc_e() { blk = 49; continue; }
            blk = 44; continue;
        }
        44 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            if c.cc_e() { blk = 53; continue; }
            blk = 46; continue;
        }
        46 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 50; continue; }
            blk = 47; continue;
        }
        47 => {
            if c.cc_e() { blk = 53; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 50; continue;
        }
        49 => {
            c.movq_rx(1, 1);
            if let Some(sr) = c.shr(c.r64(1), 0x20u64, 64) { c.w64(1, sr); }
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089ad72); };
            c.w64(0, t40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089ad7c); }
            return c.ret(0x0, 0x700089ad7d);
        }
        51 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089ad7e); };
            c.cmp(t41, c.r32(3), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089ad87); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089ad8b); }
            blk = 53; continue;
        }
        53 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089ad8d); };
            c.w64(1, t42);
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x17b)) else { return RegfileExit::fault(0x700089ad94); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089ad9d); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700089ad9e); };
            c.w64(3, t45);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089adb1, 0x700089adac);
        }
        54 => {
            return RegfileExit::fault(0x700089adb1);
        }
        55 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089adb2); };
            c.w64(0, t46);
            return c.ret(0x0, 0x700089adb9);
        }
        56 => {
            return RegfileExit::jump(0x700089adbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089add4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089add4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089add8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089add9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089adda); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089addf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700089adef); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089adf9); };
            c.w64(1, t2);
            c.w64(9, c.r64(1));
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089ae02); };
            c.w64(9, t3);
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089ae09); };
            c.w64(1, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700089ae10); };
            c.cmp(t5, c.r32(9), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(11, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089ae22); };
            c.cmp(t6, c.r32(9), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ae2b); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700089ae32); };
            c.logic(t9 & 0x10u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089ae39); };
            c.cmp(t10, c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ae46); };
            c.cmp(t11, 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 12; continue;
        }
        11 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089ae5a); };
            c.w64(11, t12);
            blk = 14; continue;
        }
        13 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700089ae63); };
            c.w64(11, t13);
            blk = 14; continue;
        }
        14 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700089ae6a); };
            c.w64(9, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700089ae71); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089ae75); };
            c.cmp(t15, c.r32(2), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ae82); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            let Some(t18) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700089ae89); };
            c.logic(t18 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700089ae91); };
            c.w32(12, t19);
            let r20 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r20);
            let Some(t21) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ae98); };
            c.w32(15, t21);
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700089ae9d); };
            c.cmp(t22, c.r32(15), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089aea6); };
            c.cmp(t23, c.r32(7), 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, c.r64(12));
            blk = 20; continue;
        }
        19 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089aeb2); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089aeb3); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700089aeb4); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2380)) else { return RegfileExit::fault(0x700089aeb6); };
            c.w64(3, t24);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089aec7, 0x700089aec2);
        }
        20 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089aec7); };
            c.logic(t25 & 0x200000u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089aed0); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089aed7); };
            c.w64(7, t27);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089aee0); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089aee7); };
            c.w64(1, t29);
            blk = 34; continue;
        }
        22 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089aef3); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089aefa); };
            c.w64(7, t31);
            c.w64(8, c.r64(7).wrapping_add(0x4c));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089af01); };
            c.w64(9, t32);
            let Some(t33) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089af08); };
            c.cmp(t33, c.r64(8), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700089af11); }
            let r34 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r34);
            blk = 24; continue;
        }
        24 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x700089af18); };
            c.w64(8, t35);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700089af1f); }
            c.w32(8, 0x22u64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700089af29); }
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089af2d); };
            c.w32(8, t36);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700089af43); }
            let Some(t37) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089af46); };
            c.w32(8, t37);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 57; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x700089af5a); }
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089af5d); };
            c.w32(8, t38);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 58; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(8, 0x8u64);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700089af77); }
            let Some(t39) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089af7b); };
            c.w32(8, t39);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(8, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x700089af95); }
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089af99); };
            c.w32(8, t40);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_be() { blk = 60; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x700089afad); }
            c.w64(8, c.r64(7).wrapping_add(0x4b));
            c.w64(11, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 33; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089afbd); };
            c.w64(12, t41);
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(11), c.r32(12)) { return RegfileExit::fault(0x700089afc4); }
            let r42 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r42);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 31; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 34; continue;
        }
        33 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089afd2); };
            c.w64(12, t43);
            blk = 34; continue;
        }
        34 => {
            let Some(t44) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089afd9); };
            c.w32(8, t44);
            c.w64(2, c.r64(8).wrapping_mul(4));
            let Some(t45) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089afe6); };
            c.w64(8, t45);
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t46) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089afed); };
            c.cmp(t46, c.r64(11), 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x700089aff6); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 36; continue;
        }
        36 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700089b001); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089b004); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089b007); };
            c.w64(7, t47);
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700089b00e); }
            let Some(t48) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089b011); };
            c.w32(7, t48);
            c.w64(9, c.r64(7));
            let r49 = c.r32(9) & 0xe0000000u64;
            c.w32(9, r49);
            c.logic(r49, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            let r50 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r50);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700089b02a); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 46; continue;
        }
        40 => {
            let r51 = c.sub(c.r32(7), 0x20000000u64, 32);
            c.w32(7, r51);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700089b03c); }
            let Some(t52) = c.ld8(c.r64(0).wrapping_add(0x5)) else { return RegfileExit::fault(0x700089b03f); };
            c.w32(7, t52);
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x700089b050); };
            c.w64(11, t53);
            blk = 42; continue;
        }
        42 => {
            let r54 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r54);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700089b05b); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 42; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let r55 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r55);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700089b06e); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_ne() { blk = 44; continue; }
            blk = 45; continue;
        }
        45 => {
            let r56 = c.add(c.r32(9), 0xe0000000u64, 32);
            c.w32(9, r56);
            if c.cc_e() { blk = 63; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089b085); };
            c.w64(0, t57);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700089b089); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089b08c); };
            c.w64(7, t58);
            let r59 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r59);
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089b09a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089b09b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089b0a1); }
            return c.ret(0x0, 0x700089b0a3);
        }
        47 => {
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089b0ae); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089b0af); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089b0b5); }
            return c.ret(0x0, 0x700089b0b7);
        }
        48 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700089b0b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700089b0bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700089b0c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089b0c4); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089b0c8); };
            c.w64(3, t60);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700089b0d9, 0x700089b0d4);
        }
        49 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b0d9); };
            c.w64(7, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089b0dd); };
            c.w64(2, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089b0e1); };
            c.w64(6, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089b0e5); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        50 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b0ee); };
            c.logic(t65 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 12; continue;
        }
        52 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700089b101); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 12; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 13; continue;
        }
        54 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700089b11f); }
            return c.call(0x70007febf4, 0x700089b128, 0x700089b123);
        }
        55 => {
            c.w64(7, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089b12b); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b12f); };
            c.w64(1, t67);
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089b136); };
            c.w64(9, t68);
            blk = 24; continue;
        }
        56 => {
            return RegfileExit::fault(0x700089b142);
        }
        57 => {
            return RegfileExit::fault(0x700089b143);
        }
        58 => {
            return RegfileExit::fault(0x700089b144);
        }
        59 => {
            return RegfileExit::fault(0x700089b145);
        }
        60 => {
            return RegfileExit::fault(0x700089b146);
        }
        61 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700089b147); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700089b14b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700089b14f); }
            return c.call(0x70007febf4, 0x700089b158, 0x700089b153);
        }
        62 => {
            c.w64(1, c.r64(0));
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089b15b); };
            c.w64(0, t69);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089b161); };
            c.w64(12, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089b168); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089b16c); };
            c.w64(2, t72);
            blk = 36; continue;
        }
        63 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089b175); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089b176); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x700089b17a); };
            c.w64(3, t73);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089b18b, 0x700089b186);
        }
        64 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b18b); };
            c.w64(1, t74);
            blk = 46; continue;
        }
        65 => {
            return RegfileExit::jump(0x700089b198);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089b1b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089b1b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089b1b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089b1b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089b1ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089b1bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700089b1c5); };
            c.w64(2, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089b1cb); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089b1d6); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089b1dc); };
            c.cmp(t7, 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700089b1e4); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089b1ed); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089b1f3); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089b1fa); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x700089b201);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089b204); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089b20b); }
            return c.ret(0x0, 0x700089b20d);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089b20e); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089b212); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089b220, 0x700089b21b);
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089b220); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        11 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089b22b); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089b22c); };
            c.w64(2, t14);
            let Some(t15) = c.ld32(c.r64(2).wrapping_add(0x17f)) else { return RegfileExit::fault(0x700089b233); };
            c.w32(2, t15);
            let r16 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r16);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089b23c); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x570)) else { return RegfileExit::fault(0x700089b23d); };
            c.w64(2, t17);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089b244); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089b245); };
            c.w64(3, t18);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089b256, 0x700089b251);
        }
        12 => {
            return RegfileExit::fault(0x700089b256);
        }
        13 => {
            return RegfileExit::jump(0x700089b258);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089b274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089b274); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089b278); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089b279); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089b27a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089b27f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700089b285); };
            c.w64(0, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089b28b); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089b295); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089b29b); };
            c.cmp(t7, 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x700089b2a3); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089b2ac); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089b2b3); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x700089b2ba);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089b2bd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089b2c4); }
            return c.ret(0x0, 0x700089b2c6);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089b2c7); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089b2cb); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089b2d9, 0x700089b2d4);
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089b2d9); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        11 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089b2e4); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089b2e5); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_add(0x183)) else { return RegfileExit::fault(0x700089b2ec); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089b2f5); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x570)) else { return RegfileExit::fault(0x700089b2f6); };
            c.w64(2, t15);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089b2fd); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089b2fe); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089b30f, 0x700089b30a);
        }
        12 => {
            return RegfileExit::fault(0x700089b30f);
        }
        13 => {
            return RegfileExit::jump(0x700089b314);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089b334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089b334); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089b338); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b33e); };
            c.w32(7, t1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b341); };
            c.w32(9, t2);
            c.w64(11, c.r64(7));
            let r3 = c.r32(11) & 0x1u64;
            c.w32(11, r3);
            c.logic(r3, 32);
            c.w64(12, c.r64(9));
            let r4 = c.r32(12) & 0x1u64;
            c.w32(12, r4);
            c.logic(r4, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089b353); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700089b357); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700089b35b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700089b35f); }
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(10, c.r64(4));
            let r5 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r5);
            let r6 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r6);
            c.logic(r6, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089b377); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(3));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1400)) else { return RegfileExit::fault(0x700089b382); };
            c.w64(0, t7);
            c.w64(10, 0x700089b390u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089b390); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089b394); }
            return c.call(c.r64(0), 0x700089b39a, 0x700089b398);
        }
        2 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089b39a); }
            let Some(t8) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089b3a2); };
            c.w64(4, t8);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 22; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089b3b0); };
            c.cmp(t9, 0x0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 6; continue;
        }
        5 => {
            c.w32(7, 0x1u64);
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089b3bf); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b3c3); };
            c.w32(9, t11);
            c.w64(11, c.r64(9));
            let r12 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r12);
            let r13 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r13);
            c.logic(r13, 32);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b3d4); };
            c.w64(8, t14);
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b3d8); };
            c.w32(12, t15);
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(0, 0x1u64);
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            let r16 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r16);
            let r17 = c.r32(12) & 0x3fffffffu64;
            c.w32(12, r17);
            c.logic(r17, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(15), 0x1000000u64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b412); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089b419); };
            c.w64(9, t19);
            c.w64(12, c.r64(15).wrapping_mul(8).wrapping_add(0x8));
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089b428); };
            c.w64(0, t20);
            let Some(t21) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089b42f); };
            c.cmp(t21, c.r64(15), 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 52; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b445); };
            c.w64(12, t22);
            if !c.st64(c.r64(12), c.r64(15)) { return RegfileExit::fault(0x700089b44c); }
            let r23 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r23);
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089b454); };
            c.w64(12, t24);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089b45b); }
            let r25 = c.r32(7) & 0x1u64;
            c.w32(7, r25);
            c.logic(r25, 32);
            let r26 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r26);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700089b465); }
            let r27 = c.r32(7) | c.r32(11);
            c.w32(7, r27);
            c.logic(r27, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089b46c); }
            c.w64(10, c.r64(4));
            let r28 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r28);
            let r29 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r29);
            c.logic(r29, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089b47b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700089b480); }
            c.w64(1, c.r64(9));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x1408)) else { return RegfileExit::fault(0x700089b487); };
            c.w64(0, t30);
            c.w64(10, 0x700089b495u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089b495); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089b499); }
            return c.call(c.r64(0), 0x700089b49f, 0x700089b49d);
        }
        13 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089b49f); }
            let Some(t31) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089b4a7); };
            c.w64(4, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b4ac); };
            c.w64(0, t32);
            blk = 14; continue;
        }
        14 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089b4b3); }
            return c.ret(0x0, 0x700089b4b4);
        }
        15 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089b4ba); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089b4bb); };
            c.w64(3, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b4c2); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x700089b4cb, 0x700089b4c6);
        }
        16 => {
            return RegfileExit::fault(0x700089b4cb);
        }
        17 => {
            let r35 = c.r32(9) & 0x1u64;
            c.w32(9, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, c.r64(2));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b4d8); };
            c.w64(6, t36);
            return c.call(0x700089c674, 0x700089b4e1, 0x700089b4dc);
        }
        19 => {
            blk = 14; continue;
        }
        20 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        21 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        22 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b4ed); };
            c.w64(8, t37);
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b4f1); };
            c.w32(7, t38);
            c.w64(9, c.r64(7));
            let r39 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r39);
            let r40 = c.r32(9) & 0x3fffffffu64;
            c.w32(9, r40);
            c.logic(r40, 32);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089b502); };
            c.w64(2, t41);
            let Some(t42) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b506); };
            c.w32(11, t42);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            let r43 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r43);
            let r44 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r44);
            c.logic(r44, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r64(12), 0x1000000u64, 64);
            if c.cc_g() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b53b); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089b542); };
            c.w64(7, t46);
            c.w64(11, c.r64(12).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089b551); };
            c.w64(15, t47);
            let Some(t48) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089b558); };
            c.cmp(t48, c.r64(12), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b56e); };
            c.w64(11, t49);
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x700089b575); }
            let r50 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r50);
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089b57c); };
            c.w64(11, t51);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700089b583); }
            let r52 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r52);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089b58a); }
            let Some(t53) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089b58e); };
            let r54 = c.r32(9) | t53;
            c.w32(9, r54);
            c.logic(r54, 32);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089b592); }
            c.w64(10, c.r64(4));
            let r55 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r55);
            let r56 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r56);
            c.logic(r56, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089b5a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700089b5a6); }
            c.w64(1, c.r64(7));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(8));
            c.w64(8, c.r64(10));
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x1408)) else { return RegfileExit::fault(0x700089b5b6); };
            c.w64(0, t57);
            c.w64(10, 0x700089b5c4u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089b5c4); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089b5c8); }
            return c.call(c.r64(0), 0x700089b5ce, 0x700089b5cc);
        }
        29 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089b5ce); }
            let Some(t58) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089b5d6); };
            c.w64(4, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b5db); };
            c.w64(0, t59);
            blk = 14; continue;
        }
        30 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089b5e9); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089b5ea); };
            c.w64(3, t60);
            c.w32(0, 0x1u64);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b5f6); };
            c.w64(6, t61);
            return c.call(0x700086ed74, 0x700089b5ff, 0x700089b5fa);
        }
        31 => {
            return RegfileExit::fault(0x700089b5ff);
        }
        32 => {
            let r62 = c.r32(7) & 0x1u64;
            c.w32(7, r62);
            c.logic(r62, 32);
            let Some(t63) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089b603); };
            c.cmp(c.r32(7), t63, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(0, c.r64(8));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b60b); };
            c.w64(6, t64);
            return c.call(0x700089c674, 0x700089b614, 0x700089b60f);
        }
        34 => {
            blk = 14; continue;
        }
        35 => {
            c.w64(0, c.r64(8));
            blk = 14; continue;
        }
        36 => {
            c.w64(0, c.r64(8));
            blk = 14; continue;
        }
        37 => {
            let r65 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r65);
            let r66 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r66);
            let r67 = c.r32(9) & 0x3fffffffu64;
            c.w32(9, r67);
            c.logic(r67, 32);
            let r68 = c.r32(7) & 0x3fffffffu64;
            c.w32(7, r68);
            c.logic(r68, 32);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ge() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 51; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(7), 0x1000000u64, 64);
            if c.cc_g() { blk = 47; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b67d); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089b684); };
            c.w64(9, t70);
            c.w64(2, c.r64(7).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089b693); };
            c.w64(15, t71);
            let Some(t72) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089b69a); };
            c.cmp(t72, c.r64(12), 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b6b0); };
            c.w64(15, t73);
            if !c.st64(c.r64(15), c.r64(12)) { return RegfileExit::fault(0x700089b6b7); }
            let r74 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r74);
            blk = 45; continue;
        }
        45 => {
            c.w32(7, c.r32(7));
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089b6c0); };
            c.w64(12, t75);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089b6c7); }
            let r76 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r76);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089b6cd); }
            let r77 = c.r32(7) | c.r32(11);
            c.w32(7, r77);
            c.logic(r77, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089b6d4); }
            c.w64(10, c.r64(4));
            let r78 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r78);
            let r79 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r79);
            c.logic(r79, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089b6e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700089b6e8); }
            c.w64(1, c.r64(9));
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(3));
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x13f8)) else { return RegfileExit::fault(0x700089b6f5); };
            c.w64(0, t80);
            c.w64(10, 0x700089b703u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089b703); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089b707); }
            return c.call(c.r64(0), 0x700089b70d, 0x700089b70b);
        }
        46 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089b70d); }
            let Some(t81) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089b715); };
            c.w64(4, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b71a); };
            c.w64(0, t82);
            blk = 14; continue;
        }
        47 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 14; continue;
        }
        48 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b72a); };
            c.w32(7, t83);
            let r84 = c.r32(7) & 0x1u64;
            c.w32(7, r84);
            c.logic(r84, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            return c.call(0x700089c674, 0x700089b73a, 0x700089b735);
        }
        50 => {
            blk = 14; continue;
        }
        51 => {
            blk = 14; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700089b744); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700089b748); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700089b754, 0x700089b74f);
        }
        53 => {
            c.w64(9, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089b757); };
            c.w64(2, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b75b); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089b75f); };
            c.w64(7, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089b763); };
            c.w64(11, t88);
            blk = 12; continue;
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700089b76c); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x700089b778, 0x700089b773);
        }
        55 => {
            c.w64(7, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089b77b); };
            c.w64(2, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b77f); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089b783); };
            c.w64(9, t91);
            blk = 28; continue;
        }
        56 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700089b78c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089b790); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700089b794); }
            return c.call(0x70007febb4, 0x700089b79d, 0x700089b798);
        }
        57 => {
            c.w64(9, c.r64(0));
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b7a0); };
            c.w64(0, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089b7a4); };
            c.w64(11, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089b7a8); };
            c.w64(3, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089b7ac); };
            c.w64(7, t95);
            blk = 45; continue;
        }
        58 => {
            return RegfileExit::jump(0x700089b7b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089b7d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089b7d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089b7d8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089b7de); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700089b7e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700089b7e6); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089b7f2); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089b7f8); };
            c.cmp(t3, 0x41u64, 16);
            if c.cc_ne() { blk = 57; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089b80c); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089b812); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_ne() { blk = 57; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b81d); };
            c.w32(7, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b820); };
            c.w32(9, t8);
            c.w64(11, c.r64(7));
            let r9 = c.r32(11) & 0x1u64;
            c.w32(11, r9);
            c.logic(r9, 32);
            c.w64(12, c.r64(9));
            let r10 = c.r32(12) & 0x1u64;
            c.w32(12, r10);
            c.logic(r10, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700089b832); }
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(10, c.r64(4));
            let r11 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            let r12 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r12);
            c.logic(r12, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089b84a); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(3));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1400)) else { return RegfileExit::fault(0x700089b855); };
            c.w64(0, t13);
            c.w64(10, 0x700089b863u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089b863); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089b867); }
            return c.call(c.r64(0), 0x700089b86d, 0x700089b86b);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089b86d); }
            let Some(t14) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089b875); };
            c.w64(4, t14);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 26; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089b883); };
            c.cmp(t15, 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            c.w32(7, 0x1u64);
            blk = 10; continue;
        }
        10 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089b892); };
            c.w64(2, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b896); };
            c.w32(9, t17);
            c.w64(11, c.r64(9));
            let r18 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r18);
            let r19 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r19);
            c.logic(r19, 32);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b8a7); };
            c.w64(8, t20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b8ab); };
            c.w32(12, t21);
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(0, 0x1u64);
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 11; continue;
        }
        11 => {
            let r22 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r22);
            let r23 = c.r32(12) & 0x3fffffffu64;
            c.w32(12, r23);
            c.logic(r23, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(15), 0x1000000u64, 64);
            if c.cc_g() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b8e5); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089b8ec); };
            c.w64(9, t25);
            c.w64(12, c.r64(15).wrapping_mul(8).wrapping_add(0x8));
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089b8fb); };
            c.w64(0, t26);
            let Some(t27) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089b902); };
            c.cmp(t27, c.r64(15), 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 59; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089b918); };
            c.w64(12, t28);
            if !c.st64(c.r64(12), c.r64(15)) { return RegfileExit::fault(0x700089b91f); }
            let r29 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r29);
            blk = 16; continue;
        }
        16 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089b927); };
            c.w64(12, t30);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089b92e); }
            let r31 = c.r32(7) & 0x1u64;
            c.w32(7, r31);
            c.logic(r31, 32);
            let r32 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700089b938); }
            let r33 = c.r32(7) | c.r32(11);
            c.w32(7, r33);
            c.logic(r33, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089b93f); }
            c.w64(10, c.r64(4));
            let r34 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            let r35 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r35);
            c.logic(r35, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089b94e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700089b953); }
            c.w64(1, c.r64(9));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1408)) else { return RegfileExit::fault(0x700089b95a); };
            c.w64(0, t36);
            c.w64(10, 0x700089b968u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089b968); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089b96c); }
            return c.call(c.r64(0), 0x700089b972, 0x700089b970);
        }
        17 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089b972); }
            let Some(t37) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089b97a); };
            c.w64(4, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b97f); };
            c.w64(0, t38);
            blk = 18; continue;
        }
        18 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089b986); }
            return c.ret(0x0, 0x700089b987);
        }
        19 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089b98d); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089b98e); };
            c.w64(3, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b995); };
            c.w64(6, t40);
            return c.call(0x700086ed74, 0x700089b99e, 0x700089b999);
        }
        20 => {
            return RegfileExit::fault(0x700089b99e);
        }
        21 => {
            let r41 = c.r32(9) & 0x1u64;
            c.w32(9, r41);
            c.logic(r41, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(0, c.r64(2));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089b9ab); };
            c.w64(6, t42);
            return c.call(0x700089c674, 0x700089b9b4, 0x700089b9af);
        }
        23 => {
            blk = 18; continue;
        }
        24 => {
            c.w64(0, c.r64(2));
            blk = 18; continue;
        }
        25 => {
            c.w64(0, c.r64(2));
            blk = 18; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089b9c0); };
            c.w64(8, t43);
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b9c4); };
            c.w32(7, t44);
            c.w64(9, c.r64(7));
            let r45 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r45);
            let r46 = c.r32(9) & 0x3fffffffu64;
            c.w32(9, r46);
            c.logic(r46, 32);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089b9d5); };
            c.w64(2, t47);
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089b9d9); };
            c.w32(11, t48);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 40; continue; }
            blk = 27; continue;
        }
        27 => {
            let r49 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r49);
            let r50 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r50);
            c.logic(r50, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r64(12), 0x1000000u64, 64);
            if c.cc_g() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089ba0e); };
            c.w64(7, t51);
            let Some(t52) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089ba15); };
            c.w64(7, t52);
            c.w64(11, c.r64(12).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089ba24); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089ba2b); };
            c.cmp(t54, c.r64(12), 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 61; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089ba41); };
            c.w64(11, t55);
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x700089ba48); }
            let r56 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r56);
            blk = 32; continue;
        }
        32 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089ba4f); };
            c.w64(11, t57);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700089ba56); }
            let r58 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r58);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089ba5d); }
            let Some(t59) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089ba61); };
            let r60 = c.r32(9) | t59;
            c.w32(9, r60);
            c.logic(r60, 32);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089ba65); }
            c.w64(10, c.r64(4));
            let r61 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r61);
            let r62 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r62);
            c.logic(r62, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089ba74); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700089ba79); }
            c.w64(1, c.r64(7));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(8));
            c.w64(8, c.r64(10));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1408)) else { return RegfileExit::fault(0x700089ba89); };
            c.w64(0, t63);
            c.w64(10, 0x700089ba97u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089ba97); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089ba9b); }
            return c.call(c.r64(0), 0x700089baa1, 0x700089ba9f);
        }
        33 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089baa1); }
            let Some(t64) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089baa9); };
            c.w64(4, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089baae); };
            c.w64(0, t65);
            blk = 18; continue;
        }
        34 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089babc); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089babd); };
            c.w64(3, t66);
            c.w32(0, 0x1u64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089bac9); };
            c.w64(6, t67);
            return c.call(0x700086ed74, 0x700089bad2, 0x700089bacd);
        }
        35 => {
            return RegfileExit::fault(0x700089bad2);
        }
        36 => {
            let r68 = c.r32(7) & 0x1u64;
            c.w32(7, r68);
            c.logic(r68, 32);
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089bad6); };
            c.cmp(c.r32(7), t69, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(0, c.r64(8));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089bade); };
            c.w64(6, t70);
            return c.call(0x700089c674, 0x700089bae7, 0x700089bae2);
        }
        38 => {
            blk = 18; continue;
        }
        39 => {
            c.w64(0, c.r64(8));
            blk = 18; continue;
        }
        40 => {
            c.w64(0, c.r64(8));
            blk = 18; continue;
        }
        41 => {
            let r71 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r71);
            let r72 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r72);
            let r73 = c.r32(9) & 0x3fffffffu64;
            c.w32(9, r73);
            c.logic(r73, 32);
            let r74 = c.r32(7) & 0x3fffffffu64;
            c.w32(7, r74);
            c.logic(r74, 32);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ge() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(7), 0x1000000u64, 64);
            if c.cc_g() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089bb50); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089bb57); };
            c.w64(9, t76);
            c.w64(2, c.r64(7).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089bb66); };
            c.w64(15, t77);
            let Some(t78) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089bb6d); };
            c.cmp(t78, c.r64(12), 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 63; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089bb83); };
            c.w64(15, t79);
            if !c.st64(c.r64(15), c.r64(12)) { return RegfileExit::fault(0x700089bb8a); }
            let r80 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r80);
            blk = 49; continue;
        }
        49 => {
            c.w32(7, c.r32(7));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089bb93); };
            c.w64(12, t81);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089bb9a); }
            let r82 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r82);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089bba0); }
            let r83 = c.r32(7) | c.r32(11);
            c.w32(7, r83);
            c.logic(r83, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089bba7); }
            c.w64(10, c.r64(4));
            let r84 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r84);
            let r85 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r85);
            c.logic(r85, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089bbb6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700089bbbb); }
            c.w64(1, c.r64(9));
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(3));
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x13f8)) else { return RegfileExit::fault(0x700089bbc8); };
            c.w64(0, t86);
            c.w64(10, 0x700089bbd6u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089bbd6); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089bbda); }
            return c.call(c.r64(0), 0x700089bbe0, 0x700089bbde);
        }
        50 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089bbe0); }
            let Some(t87) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089bbe8); };
            c.w64(4, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089bbed); };
            c.w64(0, t88);
            blk = 18; continue;
        }
        51 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089bbfb); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089bbfc); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089bc0d, 0x700089bc08);
        }
        52 => {
            return RegfileExit::fault(0x700089bc0d);
        }
        53 => {
            let Some(t90) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089bc0e); };
            c.w32(7, t90);
            let r91 = c.r32(7) & 0x1u64;
            c.w32(7, r91);
            c.logic(r91, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            return c.call(0x700089c674, 0x700089bc1e, 0x700089bc19);
        }
        55 => {
            blk = 18; continue;
        }
        56 => {
            blk = 18; continue;
        }
        57 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089bc2d); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089bc2e); };
            c.w64(3, t92);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089bc3f, 0x700089bc3a);
        }
        58 => {
            return RegfileExit::fault(0x700089bc3f);
        }
        59 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700089bc40); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700089bc44); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700089bc50, 0x700089bc4b);
        }
        60 => {
            c.w64(9, c.r64(0));
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089bc53); };
            c.w64(2, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089bc57); };
            c.w64(8, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089bc5b); };
            c.w64(7, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089bc5f); };
            c.w64(11, t96);
            blk = 16; continue;
        }
        61 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700089bc68); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x700089bc74, 0x700089bc6f);
        }
        62 => {
            c.w64(7, c.r64(0));
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089bc77); };
            c.w64(2, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089bc7b); };
            c.w64(8, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089bc7f); };
            c.w64(9, t99);
            blk = 32; continue;
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700089bc88); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089bc8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700089bc90); }
            return c.call(0x70007febb4, 0x700089bc99, 0x700089bc94);
        }
        64 => {
            c.w64(9, c.r64(0));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089bc9c); };
            c.w64(0, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089bca0); };
            c.w64(11, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089bca4); };
            c.w64(3, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089bca8); };
            c.w64(7, t103);
            blk = 49; continue;
        }
        65 => {
            return RegfileExit::jump(0x700089bcb4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089bcd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089bcd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089bcd8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089bcde); };
            c.w32(7, t1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089bce1); };
            c.w32(9, t2);
            c.w64(11, c.r64(7));
            let r3 = c.r32(11) & 0x1u64;
            c.w32(11, r3);
            c.logic(r3, 32);
            c.w64(12, c.r64(9));
            let r4 = c.r32(12) & 0x1u64;
            c.w32(12, r4);
            c.logic(r4, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089bcf3); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700089bcf7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700089bcfb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700089bcff); }
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(10, c.r64(4));
            let r5 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r5);
            let r6 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r6);
            c.logic(r6, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089bd17); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(3));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1400)) else { return RegfileExit::fault(0x700089bd22); };
            c.w64(0, t7);
            c.w64(10, 0x700089bd30u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089bd30); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089bd34); }
            return c.call(c.r64(0), 0x700089bd3a, 0x700089bd38);
        }
        2 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089bd3a); }
            let Some(t8) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089bd42); };
            c.w64(4, t8);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 22; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089bd50); };
            c.cmp(t9, 0x0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 6; continue;
        }
        5 => {
            c.w32(7, 0x1u64);
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089bd5f); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089bd63); };
            c.w32(9, t11);
            c.w64(11, c.r64(9));
            let r12 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r12);
            let r13 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r13);
            c.logic(r13, 32);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089bd74); };
            c.w64(8, t14);
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089bd78); };
            c.w32(12, t15);
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(0, 0x1u64);
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            let r16 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r16);
            let r17 = c.r32(12) & 0x3fffffffu64;
            c.w32(12, r17);
            c.logic(r17, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(15), 0x1000000u64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089bdb2); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089bdb9); };
            c.w64(9, t19);
            c.w64(12, c.r64(15).wrapping_mul(8).wrapping_add(0x8));
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089bdc8); };
            c.w64(0, t20);
            let Some(t21) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089bdcf); };
            c.cmp(t21, c.r64(15), 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 52; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089bde5); };
            c.w64(12, t22);
            if !c.st64(c.r64(12), c.r64(15)) { return RegfileExit::fault(0x700089bdec); }
            let r23 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r23);
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089bdf4); };
            c.w64(12, t24);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089bdfb); }
            let r25 = c.r32(7) & 0x1u64;
            c.w32(7, r25);
            c.logic(r25, 32);
            let r26 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r26);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700089be05); }
            let r27 = c.r32(7) | c.r32(11);
            c.w32(7, r27);
            c.logic(r27, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089be0c); }
            c.w64(10, c.r64(4));
            let r28 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r28);
            let r29 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r29);
            c.logic(r29, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089be1b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700089be20); }
            c.w64(1, c.r64(9));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x1408)) else { return RegfileExit::fault(0x700089be27); };
            c.w64(0, t30);
            c.w64(10, 0x700089be35u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089be35); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089be39); }
            return c.call(c.r64(0), 0x700089be3f, 0x700089be3d);
        }
        13 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089be3f); }
            let Some(t31) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089be47); };
            c.w64(4, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089be4c); };
            c.w64(0, t32);
            blk = 14; continue;
        }
        14 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089be53); }
            return c.ret(0x0, 0x700089be54);
        }
        15 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089be5a); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089be5b); };
            c.w64(3, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089be62); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x700089be6b, 0x700089be66);
        }
        16 => {
            return RegfileExit::fault(0x700089be6b);
        }
        17 => {
            let r35 = c.r32(9) & 0x1u64;
            c.w32(9, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, c.r64(2));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089be78); };
            c.w64(6, t36);
            return c.call(0x700089c674, 0x700089be81, 0x700089be7c);
        }
        19 => {
            blk = 14; continue;
        }
        20 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        21 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        22 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089be8d); };
            c.w64(8, t37);
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089be91); };
            c.w32(7, t38);
            c.w64(9, c.r64(7));
            let r39 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r39);
            let r40 = c.r32(9) & 0x3fffffffu64;
            c.w32(9, r40);
            c.logic(r40, 32);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089bea2); };
            c.w64(2, t41);
            let Some(t42) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089bea6); };
            c.w32(11, t42);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            let r43 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r43);
            let r44 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r44);
            c.logic(r44, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r64(12), 0x1000000u64, 64);
            if c.cc_g() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089bedb); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089bee2); };
            c.w64(7, t46);
            c.w64(11, c.r64(12).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089bef1); };
            c.w64(15, t47);
            let Some(t48) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089bef8); };
            c.cmp(t48, c.r64(12), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 54; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089bf0e); };
            c.w64(11, t49);
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x700089bf15); }
            let r50 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r50);
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089bf1c); };
            c.w64(11, t51);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700089bf23); }
            let r52 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r52);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089bf2a); }
            let Some(t53) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089bf2e); };
            let r54 = c.r32(9) | t53;
            c.w32(9, r54);
            c.logic(r54, 32);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089bf32); }
            c.w64(10, c.r64(4));
            let r55 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r55);
            let r56 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r56);
            c.logic(r56, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089bf41); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700089bf46); }
            c.w64(1, c.r64(7));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(8));
            c.w64(8, c.r64(10));
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x1408)) else { return RegfileExit::fault(0x700089bf56); };
            c.w64(0, t57);
            c.w64(10, 0x700089bf64u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089bf64); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089bf68); }
            return c.call(c.r64(0), 0x700089bf6e, 0x700089bf6c);
        }
        29 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089bf6e); }
            let Some(t58) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089bf76); };
            c.w64(4, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089bf7b); };
            c.w64(0, t59);
            blk = 14; continue;
        }
        30 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089bf89); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089bf8a); };
            c.w64(3, t60);
            c.w32(0, 0x1u64);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089bf96); };
            c.w64(6, t61);
            return c.call(0x700086ed74, 0x700089bf9f, 0x700089bf9a);
        }
        31 => {
            return RegfileExit::fault(0x700089bf9f);
        }
        32 => {
            let r62 = c.r32(7) & 0x1u64;
            c.w32(7, r62);
            c.logic(r62, 32);
            let Some(t63) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089bfa3); };
            c.cmp(c.r32(7), t63, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(0, c.r64(8));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089bfab); };
            c.w64(6, t64);
            return c.call(0x700089c674, 0x700089bfb4, 0x700089bfaf);
        }
        34 => {
            blk = 14; continue;
        }
        35 => {
            c.w64(0, c.r64(8));
            blk = 14; continue;
        }
        36 => {
            c.w64(0, c.r64(8));
            blk = 14; continue;
        }
        37 => {
            let r65 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r65);
            let r66 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r66);
            let r67 = c.r32(9) & 0x3fffffffu64;
            c.w32(9, r67);
            c.logic(r67, 32);
            let r68 = c.r32(7) & 0x3fffffffu64;
            c.w32(7, r68);
            c.logic(r68, 32);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ge() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 51; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(7), 0x1000000u64, 64);
            if c.cc_g() { blk = 47; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c01d); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089c024); };
            c.w64(9, t70);
            c.w64(2, c.r64(7).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089c033); };
            c.w64(15, t71);
            let Some(t72) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089c03a); };
            c.cmp(t72, c.r64(12), 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c050); };
            c.w64(15, t73);
            if !c.st64(c.r64(15), c.r64(12)) { return RegfileExit::fault(0x700089c057); }
            let r74 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r74);
            blk = 45; continue;
        }
        45 => {
            c.w32(7, c.r32(7));
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089c060); };
            c.w64(12, t75);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089c067); }
            let r76 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r76);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089c06d); }
            let r77 = c.r32(7) | c.r32(11);
            c.w32(7, r77);
            c.logic(r77, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089c074); }
            c.w64(10, c.r64(4));
            let r78 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r78);
            let r79 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r79);
            c.logic(r79, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089c083); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700089c088); }
            c.w64(1, c.r64(9));
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(3));
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x13f8)) else { return RegfileExit::fault(0x700089c095); };
            c.w64(0, t80);
            c.w64(10, 0x700089c0a3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089c0a3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089c0a7); }
            return c.call(c.r64(0), 0x700089c0ad, 0x700089c0ab);
        }
        46 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089c0ad); }
            let Some(t81) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089c0b5); };
            c.w64(4, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c0ba); };
            c.w64(0, t82);
            blk = 14; continue;
        }
        47 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 14; continue;
        }
        48 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c0ca); };
            c.w32(7, t83);
            let r84 = c.r32(7) & 0x1u64;
            c.w32(7, r84);
            c.logic(r84, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            return c.call(0x700089c674, 0x700089c0da, 0x700089c0d5);
        }
        50 => {
            blk = 14; continue;
        }
        51 => {
            blk = 14; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700089c0e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700089c0e8); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700089c0f4, 0x700089c0ef);
        }
        53 => {
            c.w64(9, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c0f7); };
            c.w64(2, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089c0fb); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c0ff); };
            c.w64(7, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089c103); };
            c.w64(11, t88);
            blk = 12; continue;
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700089c10c); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x700089c118, 0x700089c113);
        }
        55 => {
            c.w64(7, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c11b); };
            c.w64(2, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089c11f); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089c123); };
            c.w64(9, t91);
            blk = 28; continue;
        }
        56 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700089c12c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089c130); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700089c134); }
            return c.call(0x70007febb4, 0x700089c13d, 0x700089c138);
        }
        57 => {
            c.w64(9, c.r64(0));
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089c140); };
            c.w64(0, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c144); };
            c.w64(11, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c148); };
            c.w64(3, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089c14c); };
            c.w64(7, t95);
            blk = 45; continue;
        }
        58 => {
            return RegfileExit::jump(0x700089c158);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089c174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089c174); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089c178); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089c17e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700089c182); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700089c186); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089c192); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089c198); };
            c.cmp(t3, 0x41u64, 16);
            if c.cc_ne() { blk = 57; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089c1ac); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089c1b2); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_ne() { blk = 57; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c1bd); };
            c.w32(7, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c1c0); };
            c.w32(9, t8);
            c.w64(11, c.r64(7));
            let r9 = c.r32(11) & 0x1u64;
            c.w32(11, r9);
            c.logic(r9, 32);
            c.w64(12, c.r64(9));
            let r10 = c.r32(12) & 0x1u64;
            c.w32(12, r10);
            c.logic(r10, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700089c1d2); }
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(10, c.r64(4));
            let r11 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            let r12 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r12);
            c.logic(r12, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089c1ea); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(3));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1400)) else { return RegfileExit::fault(0x700089c1f5); };
            c.w64(0, t13);
            c.w64(10, 0x700089c203u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089c203); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089c207); }
            return c.call(c.r64(0), 0x700089c20d, 0x700089c20b);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089c20d); }
            let Some(t14) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089c215); };
            c.w64(4, t14);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 26; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c223); };
            c.cmp(t15, 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            c.w32(7, 0x1u64);
            blk = 10; continue;
        }
        10 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c232); };
            c.w64(2, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c236); };
            c.w32(9, t17);
            c.w64(11, c.r64(9));
            let r18 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r18);
            let r19 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r19);
            c.logic(r19, 32);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089c247); };
            c.w64(8, t20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c24b); };
            c.w32(12, t21);
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(0, 0x1u64);
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 11; continue;
        }
        11 => {
            let r22 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r22);
            let r23 = c.r32(12) & 0x3fffffffu64;
            c.w32(12, r23);
            c.logic(r23, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(15), 0x1000000u64, 64);
            if c.cc_g() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c285); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089c28c); };
            c.w64(9, t25);
            c.w64(12, c.r64(15).wrapping_mul(8).wrapping_add(0x8));
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089c29b); };
            c.w64(0, t26);
            let Some(t27) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089c2a2); };
            c.cmp(t27, c.r64(15), 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 59; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c2b8); };
            c.w64(12, t28);
            if !c.st64(c.r64(12), c.r64(15)) { return RegfileExit::fault(0x700089c2bf); }
            let r29 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r29);
            blk = 16; continue;
        }
        16 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089c2c7); };
            c.w64(12, t30);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089c2ce); }
            let r31 = c.r32(7) & 0x1u64;
            c.w32(7, r31);
            c.logic(r31, 32);
            let r32 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700089c2d8); }
            let r33 = c.r32(7) | c.r32(11);
            c.w32(7, r33);
            c.logic(r33, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089c2df); }
            c.w64(10, c.r64(4));
            let r34 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            let r35 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r35);
            c.logic(r35, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089c2ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700089c2f3); }
            c.w64(1, c.r64(9));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1408)) else { return RegfileExit::fault(0x700089c2fa); };
            c.w64(0, t36);
            c.w64(10, 0x700089c308u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089c308); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089c30c); }
            return c.call(c.r64(0), 0x700089c312, 0x700089c310);
        }
        17 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089c312); }
            let Some(t37) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089c31a); };
            c.w64(4, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c31f); };
            c.w64(0, t38);
            blk = 18; continue;
        }
        18 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089c326); }
            return c.ret(0x0, 0x700089c327);
        }
        19 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089c32d); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089c32e); };
            c.w64(3, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c335); };
            c.w64(6, t40);
            return c.call(0x700086ed74, 0x700089c33e, 0x700089c339);
        }
        20 => {
            return RegfileExit::fault(0x700089c33e);
        }
        21 => {
            let r41 = c.r32(9) & 0x1u64;
            c.w32(9, r41);
            c.logic(r41, 32);
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(0, c.r64(2));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c34b); };
            c.w64(6, t42);
            return c.call(0x700089c674, 0x700089c354, 0x700089c34f);
        }
        23 => {
            blk = 18; continue;
        }
        24 => {
            c.w64(0, c.r64(2));
            blk = 18; continue;
        }
        25 => {
            c.w64(0, c.r64(2));
            blk = 18; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089c360); };
            c.w64(8, t43);
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c364); };
            c.w32(7, t44);
            c.w64(9, c.r64(7));
            let r45 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r45);
            let r46 = c.r32(9) & 0x3fffffffu64;
            c.w32(9, r46);
            c.logic(r46, 32);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c375); };
            c.w64(2, t47);
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c379); };
            c.w32(11, t48);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 40; continue; }
            blk = 27; continue;
        }
        27 => {
            let r49 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r49);
            let r50 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r50);
            c.logic(r50, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r64(12), 0x1000000u64, 64);
            if c.cc_g() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c3ae); };
            c.w64(7, t51);
            let Some(t52) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089c3b5); };
            c.w64(7, t52);
            c.w64(11, c.r64(12).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089c3c4); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089c3cb); };
            c.cmp(t54, c.r64(12), 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 61; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c3e1); };
            c.w64(11, t55);
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x700089c3e8); }
            let r56 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r56);
            blk = 32; continue;
        }
        32 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089c3ef); };
            c.w64(11, t57);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700089c3f6); }
            let r58 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r58);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089c3fd); }
            let Some(t59) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c401); };
            let r60 = c.r32(9) | t59;
            c.w32(9, r60);
            c.logic(r60, 32);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089c405); }
            c.w64(10, c.r64(4));
            let r61 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r61);
            let r62 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r62);
            c.logic(r62, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089c414); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700089c419); }
            c.w64(1, c.r64(7));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(8));
            c.w64(8, c.r64(10));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1408)) else { return RegfileExit::fault(0x700089c429); };
            c.w64(0, t63);
            c.w64(10, 0x700089c437u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089c437); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089c43b); }
            return c.call(c.r64(0), 0x700089c441, 0x700089c43f);
        }
        33 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089c441); }
            let Some(t64) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089c449); };
            c.w64(4, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c44e); };
            c.w64(0, t65);
            blk = 18; continue;
        }
        34 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089c45c); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089c45d); };
            c.w64(3, t66);
            c.w32(0, 0x1u64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c469); };
            c.w64(6, t67);
            return c.call(0x700086ed74, 0x700089c472, 0x700089c46d);
        }
        35 => {
            return RegfileExit::fault(0x700089c472);
        }
        36 => {
            let r68 = c.r32(7) & 0x1u64;
            c.w32(7, r68);
            c.logic(r68, 32);
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c476); };
            c.cmp(c.r32(7), t69, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(0, c.r64(8));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c47e); };
            c.w64(6, t70);
            return c.call(0x700089c674, 0x700089c487, 0x700089c482);
        }
        38 => {
            blk = 18; continue;
        }
        39 => {
            c.w64(0, c.r64(8));
            blk = 18; continue;
        }
        40 => {
            c.w64(0, c.r64(8));
            blk = 18; continue;
        }
        41 => {
            let r71 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r71);
            let r72 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r72);
            let r73 = c.r32(9) & 0x3fffffffu64;
            c.w32(9, r73);
            c.logic(r73, 32);
            let r74 = c.r32(7) & 0x3fffffffu64;
            c.w32(7, r74);
            c.logic(r74, 32);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ge() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(7), 0x1000000u64, 64);
            if c.cc_g() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c4f0); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089c4f7); };
            c.w64(9, t76);
            c.w64(2, c.r64(7).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089c506); };
            c.w64(15, t77);
            let Some(t78) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089c50d); };
            c.cmp(t78, c.r64(12), 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 63; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c523); };
            c.w64(15, t79);
            if !c.st64(c.r64(15), c.r64(12)) { return RegfileExit::fault(0x700089c52a); }
            let r80 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r80);
            blk = 49; continue;
        }
        49 => {
            c.w32(7, c.r32(7));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089c533); };
            c.w64(12, t81);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700089c53a); }
            let r82 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r82);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089c540); }
            let r83 = c.r32(7) | c.r32(11);
            c.w32(7, r83);
            c.logic(r83, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089c547); }
            c.w64(10, c.r64(4));
            let r84 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r84);
            let r85 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r85);
            c.logic(r85, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089c556); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x700089c55b); }
            c.w64(1, c.r64(9));
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(3));
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x13f8)) else { return RegfileExit::fault(0x700089c568); };
            c.w64(0, t86);
            c.w64(10, 0x700089c576u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089c576); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089c57a); }
            return c.call(c.r64(0), 0x700089c580, 0x700089c57e);
        }
        50 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089c580); }
            let Some(t87) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089c588); };
            c.w64(4, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c58d); };
            c.w64(0, t88);
            blk = 18; continue;
        }
        51 => {
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089c59b); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089c59c); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089c5ad, 0x700089c5a8);
        }
        52 => {
            return RegfileExit::fault(0x700089c5ad);
        }
        53 => {
            let Some(t90) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c5ae); };
            c.w32(7, t90);
            let r91 = c.r32(7) & 0x1u64;
            c.w32(7, r91);
            c.logic(r91, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            return c.call(0x700089c674, 0x700089c5be, 0x700089c5b9);
        }
        55 => {
            blk = 18; continue;
        }
        56 => {
            blk = 18; continue;
        }
        57 => {
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089c5cd); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089c5ce); };
            c.w64(3, t92);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089c5df, 0x700089c5da);
        }
        58 => {
            return RegfileExit::fault(0x700089c5df);
        }
        59 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700089c5e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700089c5e4); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x700089c5f0, 0x700089c5eb);
        }
        60 => {
            c.w64(9, c.r64(0));
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c5f3); };
            c.w64(2, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089c5f7); };
            c.w64(8, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c5fb); };
            c.w64(7, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089c5ff); };
            c.w64(11, t96);
            blk = 16; continue;
        }
        61 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700089c608); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x700089c614, 0x700089c60f);
        }
        62 => {
            c.w64(7, c.r64(0));
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c617); };
            c.w64(2, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089c61b); };
            c.w64(8, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089c61f); };
            c.w64(9, t99);
            blk = 32; continue;
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700089c628); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089c62c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700089c630); }
            return c.call(0x70007febb4, 0x700089c639, 0x700089c634);
        }
        64 => {
            c.w64(9, c.r64(0));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089c63c); };
            c.w64(0, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c640); };
            c.w64(11, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c644); };
            c.w64(3, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089c648); };
            c.w64(7, t103);
            blk = 49; continue;
        }
        65 => {
            return RegfileExit::jump(0x700089c654);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089c674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c674); };
            c.w32(1, t0);
            c.w64(7, c.r64(1));
            let r1 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r1);
            let r2 = c.r32(7) & 0x3fffffffu64;
            c.w32(7, r2);
            c.logic(r2, 32);
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 4; continue;
        }
        3 => {
            c.w32(9, 0x1u64);
            blk = 4; continue;
        }
        4 => {
            c.w32(1, 0x1u64);
            c.cmp(c.r64(8), 0x1000000u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c6ad); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089c6b4); };
            c.w64(11, t4);
            c.w64(2, c.r64(8).wrapping_mul(8).wrapping_add(0x8));
            c.w64(12, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089c6c3); };
            c.w64(15, t5);
            let Some(t6) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089c6ca); };
            c.cmp(t6, c.r64(12), 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 16; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(1), c.r64(12)) { return RegfileExit::fault(0x700089c6d8); }
            c.w64(1, c.r64(11).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700089c6df); };
            c.w64(11, t7);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700089c6e6); }
            let r8 = c.r32(9) & 0x1u64;
            c.w32(9, r8);
            c.logic(r8, 32);
            let r9 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r9);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089c6f0); }
            let r10 = c.r32(9) | c.r32(7);
            c.w32(9, r10);
            c.logic(r10, 32);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089c6f6); }
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            c.w64(9, c.r64(7).wrapping_add(0x1));
            let Some(t11) = c.ld64(c.r64(0).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089c702); };
            c.w64(11, t11);
            if !c.st64(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x700089c707); }
            c.w64(7, c.r64(9));
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_l() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700089c717);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089c718); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089c71c); }
            let r12 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r12);
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089c727); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700089c728); };
            c.w64(3, t13);
            c.w64(0, c.r64(1));
            return c.call(0x700086ed74, 0x700089c737, 0x700089c732);
        }
        13 => {
            return RegfileExit::fault(0x700089c737);
        }
        14 => {
            return c.ret(0x0, 0x700089c738);
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089c739); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089c73d); }
            let r14 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r14);
            blk = 17; continue;
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089c745); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089c749); }
            let r15 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r15);
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700089c74f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x700089c753); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700089c757); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700089c75b); }
            return c.call(0x70007febb4, 0x700089c764, 0x700089c75f);
        }
        18 => {
            c.w64(1, c.r64(0));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089c767); };
            c.w64(0, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c76b); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089c76f); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089c773); };
            c.w64(7, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089c77a); }
            blk = 8; continue;
        }
        19 => {
            return RegfileExit::jump(0x700089c784);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089c794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089c794); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089c798); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x700089c79e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700089c7a2); }
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a2574, 0x700089c7b0, 0x700089c7ab);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089c7b0); };
            c.w64(6, t1);
            blk = 3; continue;
        }
        3 => {
            c.w64(1, c.r64(0));
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089c7c7); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089c7ce); };
            c.cmp(t4, c.r32(8), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089c7d7); };
            c.w32(7, t5);
            c.cmp(c.r16(7), 0x3fu64, 16);
            if c.cc_be() { blk = 57; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(7, c.r16(7));
            c.cmp(c.r32(7), 0x43u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 7; continue;
        }
        7 => {
            let r6 = c.sub(c.r32(7), 0xbau64, 32);
            c.w32(7, r6);
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 59; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 1; continue;
        }
        9 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x10f0)) else { return RegfileExit::fault(0x700089c806); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089c80d); };
            c.w32(9, t8);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            let r9 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r9);
            let r10 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r10);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700089c826); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 64; continue; }
            blk = 11; continue;
        }
        11 => {
            if c.cc_ne() { blk = 64; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.movq_rx(12, 0);
            if let Some(sr) = c.shr(c.r64(12), 0x20u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 64; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(12, c.r64(7));
            let r11 = c.add(c.r32(12), c.r32(7), 32);
            c.w32(12, r11);
            if c.cc_o() { blk = 64; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 17; continue;
        }
        16 => {
            c.w64(7, c.r64(1));
            blk = 17; continue;
        }
        17 => {
            c.w64(12, c.r64(7));
            let r12 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r12);
            c.w32(15, c.r32(7));
            let r13 = c.r32(11) & c.r32(12);
            c.w32(11, r13);
            c.logic(r13, 32);
            let r14 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r14);
            c.w64(3, c.r64(11).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700089c88a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700089c88e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700089c892); }
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089c896); };
            c.cmp(t15, c.r32(7), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089c89d); };
            c.w32(0, t16);
            let r17 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r17);
            blk = 19; continue;
        }
        19 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089c8a8); }
            return c.ret(0x0, 0x700089c8a9);
        }
        20 => {
            c.cmp(c.r64(9), 0x8000u64, 64);
            if c.cc_l() { blk = 87; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            c.w8(9, (c.cc_l() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(0, 0x80000000u64);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 23; continue;
        }
        23 => {
            let r18 = c.neg(c.r32(12), 32);
            c.w32(12, r18);
            blk = 25; continue;
        }
        24 => {
            c.cmp(c.r32(12), 0xau64, 32);
            if c.cc_l() { blk = 50; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            blk = 28; continue;
        }
        27 => {
            c.w32(0, 0x1u64);
            blk = 28; continue;
        }
        28 => {
            c.w64(6, c.r64(12));
            c.w64(1, c.r64(0));
            c.w32(0, 0x66666667u64);
            blk = 30; continue;
        }
        29 => {
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(6));
            c.imul1(c.r32(7), 32);
            let r19 = c.shr(c.r32(6), 0x1fu64, 32).unwrap_or(c.r32(6));
            c.w32(6, r19);
            let r20 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r20);
            let r21 = c.add(c.r32(6), c.r32(2), 32);
            c.w32(6, r21);
            let r22 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r22);
            c.w64(0, c.r64(7));
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_g() { blk = 29; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, c.r64(0));
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(2, 0xcu64);
            let r23 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r23);
            if c.cc_o() { blk = 49; continue; }
            blk = 32; continue;
        }
        32 => {
            let r24 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r24);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c938); };
            c.w64(6, t25);
            let Some(t26) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700089c93f); };
            c.w64(6, t26);
            let r27 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r27);
            c.logic(r27, 64);
            c.w64(3, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089c94a); };
            c.w64(11, t28);
            let Some(t29) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089c951); };
            c.cmp(t29, c.r64(3), 64);
            if c.cc_be() { blk = 69; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 69; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089c967); };
            c.w64(11, t30);
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700089c96e); }
            c.w64(11, c.r64(6).wrapping_add(0x1));
            blk = 35; continue;
        }
        35 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089c975); };
            c.w64(3, t31);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700089c97c); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089c980); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700089c988); }
            let r32 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r32);
            c.w64(3, c.r64(12));
            c.w64(12, c.r64(0));
            blk = 37; continue;
        }
        36 => {
            if !c.st8(c.r64(11).wrapping_add(c.r64(12)).wrapping_add(0xb), c.r8(3)) { return RegfileExit::fault(0x700089c998); }
            let r33 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r33);
            c.w64(3, c.r64(0));
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_le() { blk = 42; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089c9a9); };
            c.w64(0, (t34 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), c.r64(0), 64);
            if c.cc_ae() { blk = 48; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(7));
            c.imul1(c.r32(3), 32);
            c.w64(0, c.r64(3));
            let r35 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r35);
            let r36 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r36);
            let r37 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r37);
            let r38 = c.imul2(c.r32(0), 0xau64, 32);
            c.w32(2, r38);
            let r39 = c.sub(c.r32(3), c.r32(2), 32);
            c.w32(3, r39);
            c.cmp(c.r32(3), 0xau64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            let r40 = c.add(c.r32(3), 0x57u64, 32);
            c.w32(3, r40);
            blk = 36; continue;
        }
        41 => {
            let r41 = c.add(c.r32(3), 0x30u64, 32);
            c.w32(3, r41);
            blk = 36; continue;
        }
        42 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            let r42 = c.shl(c.r32(1), 0x1au64, 32).unwrap_or(c.r32(1));
            c.w32(1, r42);
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let r43 = c.r32(1) | c.r32(7);
            c.w32(1, r43);
            c.logic(r43, 32);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700089c9e5); }
            blk = 47; continue;
        }
        44 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089c9eb); };
            c.w64(1, (t44 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700089c9f5);
        }
        46 => {
            if !c.st8(c.r64(11).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x700089c9f6); }
            blk = 47; continue;
        }
        47 => {
            c.w64(0, c.r64(11));
            blk = 52; continue;
        }
        48 => {
            return RegfileExit::fault(0x700089ca00);
        }
        49 => {
            return RegfileExit::fault(0x700089ca01);
        }
        50 => {
            c.w32(1, c.r64(12).wrapping_add(0x30));
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 71; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(9, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089ca1b); };
            c.w64(12, t45);
            let Some(t46) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ca22); };
            c.w32(0, t46);
            let r47 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r47);
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089ca2a); };
            c.cmp(t48, c.r32(0), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t49) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089ca37); };
            c.w32(1, t49);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089ca41); };
            c.w64(7, t50);
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ae() { blk = 85; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089ca4e); };
            c.w64(1, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089ca52); };
            c.w64(9, t52);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700089ca56); }
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089ca5b); };
            c.w32(1, t53);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r54 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r54);
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ae() { blk = 86; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(1, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700089ca7a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r55 = c.r64(7) & c.r64(8);
            c.w64(7, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089ca8c); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 19; continue;
        }
        57 => {
            c.w64(0, c.r64(1));
            blk = 19; continue;
        }
        58 => {
            let Some(t57) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089caa3); };
            c.w32(0, t57);
            let r58 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r58);
            blk = 19; continue;
        }
        59 => {
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700089caae); };
            c.cmp(t59, c.r32(8), 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(1, 0x168u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089cabc); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089cabd); };
            c.w64(3, t60);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089cace, 0x700089cac9);
        }
        61 => {
            return RegfileExit::fault(0x700089cace);
        }
        62 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089cacf); };
            if !c.push64(t61) { return RegfileExit::fault(0x700089cacf); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x24d0)) else { return RegfileExit::fault(0x700089cad2); };
            c.w64(3, t62);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089cae3, 0x700089cade);
        }
        63 => {
            blk = 19; continue;
        }
        64 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089cae8); };
            c.w32(7, t63);
            let Some(t64) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089caeb); };
            c.w32(9, t64);
            c.w64(12, c.r64(7));
            let r65 = c.r32(12) ^ c.r32(9);
            c.w32(12, r65);
            c.logic(r65, 32);
            let r66 = c.r32(12) & c.r32(11);
            c.w32(12, r66);
            c.logic(r66, 32);
            c.w32(11, c.r64(12).wrapping_add(c.r64(12)));
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089cb00); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 87; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t69) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089cb12); };
            c.w32(15, t69);
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089cb17); };
            c.cmp(t70, c.r32(15), 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089cb24); };
            c.w32(15, t71);
            let Some(t72) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089cb29); };
            c.cmp(t72, c.r32(7), 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089cb3d); };
            c.w32(0, t73);
            let r74 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r74);
            blk = 19; continue;
        }
        69 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700089cb4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x700089cb4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089cb52); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700089cb56); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700089cb5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700089cb5e); }
            return c.call(0x70007febb4, 0x700089cb67, 0x700089cb62);
        }
        70 => {
            c.w64(11, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089cb6a); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089cb6e); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089cb72); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089cb76); };
            c.w64(9, t78);
            c.w32(7, 0x66666667u64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089cb7f); };
            c.w64(12, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089cb83); };
            c.w64(0, t80);
            blk = 35; continue;
        }
        71 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089cb8c); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089cb93); };
            c.w64(12, t82);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700089cb9b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089cb9f); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089cba3); };
            c.w64(0, t83);
            let Some(t84) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089cbaa); };
            c.cmp(t84, c.r64(15), 64);
            if c.cc_a() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089cbb9, 0x700089cbb4);
        }
        73 => {
            blk = 75; continue;
        }
        74 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700089cbbb); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 75; continue;
        }
        75 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700089cbc6); };
            c.w64(1, t85);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089cbcd); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089cbd0); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089cbd7); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089cbde); };
            c.w64(1, t86);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x700089cbe2); }
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089cbe6); };
            c.w64(11, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089cbea); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089cbee); };
            c.w64(3, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089cbf2); };
            c.w64(7, t90);
            blk = 52; continue;
        }
        76 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089cbfb); };
            c.w64(15, t91);
            let Some(t92) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089cc02); };
            c.w64(0, t92);
            c.w64(2, c.r64(0).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700089cc09); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700089cc0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089cc11); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089cc15); };
            c.w64(6, t93);
            let Some(t94) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700089cc1c); };
            c.cmp(t94, c.r64(2), 64);
            if c.cc_a() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089cc2b, 0x700089cc26);
        }
        78 => {
            blk = 80; continue;
        }
        79 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700089cc2d); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 80; continue;
        }
        80 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089cc37); };
            c.w64(1, t95);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089cc3e); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089cc41); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089cc48); }
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089cc4f); };
            c.w64(1, t96);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x700089cc53); }
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089cc56); };
            c.w64(12, t97);
            let Some(t98) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089cc5d); };
            c.w32(7, t98);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            return RegfileExit::fault(0x700089cc6d);
        }
        82 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089cc6e); };
            c.w64(9, t99);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x700089cc72); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r100 = c.r64(1) & c.r64(12);
            c.w64(1, r100);
            c.logic(r100, 64);
            let Some(t101) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089cc84); };
            c.logic(t101 & 0x4u64, 8);
            if c.cc_ne() { blk = 89; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089cc8a); };
            c.w64(11, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089cc8e); };
            c.w64(8, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089cc92); };
            c.w64(3, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089cc96); };
            c.w64(7, t105);
            blk = 52; continue;
        }
        85 => {
            return RegfileExit::fault(0x700089cc9f);
        }
        86 => {
            return RegfileExit::fault(0x700089cca0);
        }
        87 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089cca1); }
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x22d8)) else { return RegfileExit::fault(0x700089cca2); };
            c.w64(3, t106);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089ccb5, 0x700089ccb0);
        }
        88 => {
            blk = 19; continue;
        }
        89 => {
            c.w32(0, c.r32(0));
            let r107 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r107);
            c.w64(1, 0xfffffffffffc0000u64);
            let r108 = c.r64(1) & c.r64(0);
            c.w64(1, r108);
            c.logic(r108, 64);
            let Some(t109) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089ccc9); };
            c.logic(t109 & 0x2u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089ccd3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089ccd4); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x700089cce1, 0x700089ccdc);
        }
        91 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089cce1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089cce3); }
            blk = 84; continue;
        }
        92 => {
            c.w32(0, c.r32(0));
            let r110 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r110);
            c.w64(7, 0xfffffffffffc0000u64);
            let r111 = c.r64(7) & c.r64(0);
            c.w64(7, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089ccf5); };
            c.logic(t112 & 0x2u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089cd03); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089cd04); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089cd11, 0x700089cd0c);
        }
        94 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089cd11); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089cd13); }
            blk = 56; continue;
        }
        95 => {
            return RegfileExit::jump(0x700089cd1c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089cd34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089cd34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089cd38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089cd39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089cd3a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089cd3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700089cd45); };
            c.w64(0, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089cd4b); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089cd55); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089cd5b); };
            c.w32(2, t7);
            c.cmp(c.r16(2), 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_ae() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089cd6c); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089cd73); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x700089cd7a);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089cd7d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089cd84); }
            return c.ret(0x0, 0x700089cd86);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089cd87); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089cd8b); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089cd99, 0x700089cd94);
        }
        10 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089cd99); };
            c.w64(6, t10);
            blk = 1; continue;
        }
        11 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089cda4); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089cda5); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x187)) else { return RegfileExit::fault(0x700089cdac); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089cdb5); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa80)) else { return RegfileExit::fault(0x700089cdb6); };
            c.w64(2, t14);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089cdbd); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089cdbe); };
            c.w64(3, t15);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089cdcf, 0x700089cdca);
        }
        12 => {
            return RegfileExit::fault(0x700089cdcf);
        }
        13 => {
            return RegfileExit::jump(0x700089cdd4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089cdf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089cdf4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089cdf8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089cdf9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089cdfa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089cdff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700089ce05); };
            c.w64(0, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089ce0b); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089ce15); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ce1b); };
            c.w32(2, t7);
            c.cmp(c.r16(2), 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_ae() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089ce2c); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089ce33); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x700089ce3a);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089ce3d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089ce44); }
            return c.ret(0x0, 0x700089ce46);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089ce47); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089ce4b); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089ce59, 0x700089ce54);
        }
        10 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089ce59); };
            c.w64(6, t10);
            blk = 1; continue;
        }
        11 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089ce64); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089ce65); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x18b)) else { return RegfileExit::fault(0x700089ce6c); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089ce75); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa80)) else { return RegfileExit::fault(0x700089ce76); };
            c.w64(2, t14);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089ce7d); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089ce7e); };
            c.w64(3, t15);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089ce8f, 0x700089ce8a);
        }
        12 => {
            return RegfileExit::fault(0x700089ce8f);
        }
        13 => {
            return RegfileExit::jump(0x700089ce94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089ceb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089ceb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089ceb8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089cebe); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089cec4); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089ceca); };
            c.w32(9, t5);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ced1); };
            c.w32(11, t6);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700089ced8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700089cedc); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700089cee6);
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x700089cee7); };
            c.w32(7, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700089ceed); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(2, c.r64(11).wrapping_mul(4).wrapping_add(0x18));
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(11), 0x2000000u64, 64);
            if c.cc_a() { blk = 88; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 93; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(8, c.r64(11).wrapping_mul(4));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089cf32); };
            c.w64(9, t8);
            let Some(t9) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089cf39); };
            c.w64(12, t9);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089cf44); };
            c.w64(3, t10);
            let Some(t11) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700089cf4b); };
            c.cmp(t11, c.r64(15), 64);
            if c.cc_be() { blk = 89; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 89; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700089cf61); }
            let r12 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r12);
            blk = 9; continue;
        }
        9 => {
            c.w32(15, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700089cf6c); };
            c.w64(2, t13);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700089cf73); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700089cf78); }
            c.w64(1, c.r64(12).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r14 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r14);
            let r15 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089cf8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x700089cf92); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x700089cf96); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x700089cf9c); };
            c.w64(0, t16);
            c.w64(10, 0x700089cfaau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089cfaa); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089cfae); }
            return c.call(c.r64(0), 0x700089cfb4, 0x700089cfb2);
        }
        10 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089cfb4); }
            let Some(t17) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089cfbc); };
            c.w64(4, t17);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089cfc1); };
            c.w64(7, t18);
            let Some(t19) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089cfc8); };
            c.w64(9, t19);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089cfcf); };
            c.w64(12, t20);
            let Some(t21) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700089cfd6); };
            c.cmp(t21, c.r64(11), 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700089cfe0); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089cfe7); };
            c.w64(9, t22);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089cfeb); }
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089cfef); };
            c.w64(9, t23);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700089cff3); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089cff7); };
            c.w64(9, t24);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700089cffe); }
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089d002); };
            c.w64(0, t25);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700089d006); }
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089d009); };
            c.w64(9, t26);
            c.w64(11, c.r64(0));
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d013); };
            c.w64(0, t27);
            blk = 19; continue;
        }
        13 => {
            c.w64(9, c.r64(11));
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089d023); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700089d02a); };
            c.w64(15, t29);
            c.w64(3, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089d032); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089d039); };
            c.cmp(t31, c.r64(3), 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x700089d042); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089d04a); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700089d04f); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089d054); };
            c.w64(7, t32);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089d05b); }
            c.w64(7, c.r64(12).wrapping_add(0x10));
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700089d065); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700089d06a); };
            c.w64(15, t33);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700089d071); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700089d075); }
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(12));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        16 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089d084); };
            c.w64(9, t34);
            let Some(t35) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089d08b); };
            c.w64(12, t35);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089d097); };
            c.w64(2, t36);
            let Some(t37) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700089d09e); };
            c.cmp(t37, c.r64(15), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700089d0a7); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089d0af); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x700089d0b3); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089d0b7); };
            c.w64(7, t38);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089d0be); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700089d0c2); }
            c.w32(12, 0x1u64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 19; continue;
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700089d0de); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(12, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089d0f6); };
            c.w64(15, t39);
            blk = 22; continue;
        }
        22 => {
            let r40 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r40);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(15)) { return RegfileExit::fault(0x700089d101); }
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_ne() { blk = 22; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d10b); };
            c.w32(11, t41);
            let r42 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r42);
            c.w32(12, 0);
            c.logic(0, 32);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(12));
            blk = 27; continue;
        }
        24 => {
            c.w32(7, c.r64(12).wrapping_add(0x2));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x700089d12f); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r43 = c.r64(12) & c.r64(11);
            c.w64(12, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089d142); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 155; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d14e); };
            c.w32(9, t45);
            let r46 = c.add(c.r64(15), c.r64(9), 64);
            c.w64(15, r46);
            c.w64(12, c.r64(7));
            blk = 27; continue;
        }
        27 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089d158); };
            c.cmp(c.r64(15), t47, 64);
            if c.cc_ge() { blk = 84; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d162); };
            c.w32(2, t48);
            let r49 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r49);
            let Some(t50) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d168); };
            c.w32(2, t50);
            c.logic(c.r8(2) & 0x7u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(6, c.r64(0));
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 30; continue;
        }
        30 => {
            c.w64(8, c.r64(2));
            let r51 = c.r32(8) & 0x7u64;
            c.w32(8, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 99; continue;
        }
        35 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 99; continue;
        }
        38 => {
            let Some(t52) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089d1b3); };
            c.w32(2, t52);
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089d1b6); };
            c.cmp(t53, c.r32(2), 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t54) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d1c3); };
            c.w32(6, t54);
            let r55 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r55);
            let Some(t56) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d1c9); };
            c.w32(2, t56);
            let r57 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r57);
            let Some(t58) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d1cf); };
            c.w32(2, t58);
            blk = 30; continue;
        }
        40 => {
            let Some(t59) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089d1d5); };
            c.w32(2, t59);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r60 = c.add(c.r64(1), c.r64(2), 64);
            c.w64(1, r60);
            let Some(t61) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d1e1); };
            c.w32(6, t61);
            let r62 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r62);
            let Some(t63) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d1e7); };
            c.w32(2, t63);
            let r64 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r64);
            let Some(t65) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d1ed); };
            c.w32(2, t65);
            blk = 30; continue;
        }
        41 => {
            let Some(t66) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d1f3); };
            c.w32(6, t66);
            let r67 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r67);
            let Some(t68) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d1f9); };
            c.w32(2, t68);
            let r69 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r69);
            let Some(t70) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d1ff); };
            c.w32(2, t70);
            blk = 30; continue;
        }
        42 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 45; continue;
        }
        43 => {
            c.w32(8, 0x1u64);
            blk = 45; continue;
        }
        44 => {
            c.w64(8, c.r64(3));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(0));
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 46; continue;
        }
        46 => {
            let r71 = c.add(c.r64(6), 0xbu64, 64);
            c.w64(6, r71);
            blk = 47; continue;
        }
        47 => {
            let r72 = c.r32(2) & 0x8u64;
            c.w32(2, r72);
            c.logic(r72, 32);
            let r73 = c.add(c.r64(1), c.r64(15), 64);
            c.w64(1, r73);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t74) = c.ld16(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x700089d236); };
            c.w32(2, t74);
            blk = 50; continue;
        }
        49 => {
            let Some(t75) = c.ld8(c.r64(1).wrapping_add(c.r64(6))) else { return RegfileExit::fault(0x700089d23c); };
            c.w32(2, t75);
            blk = 50; continue;
        }
        50 => {
            c.w64(1, c.r64(2));
            let r76 = c.r32(1) & 0xfc00u64;
            c.w32(1, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(1), 0xd800u64, 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(1, c.r64(15).wrapping_add(0x1));
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089d259); };
            c.cmp(c.r64(1), t77, 64);
            if c.cc_ge() { blk = 76; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t78) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d263); };
            c.w32(6, t78);
            let r79 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r79);
            let Some(t80) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d269); };
            c.w32(6, t80);
            c.logic(c.r8(6) & 0x7u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(0));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 54; continue;
        }
        54 => {
            c.w64(9, c.r64(6));
            let r81 = c.r32(9) & 0x7u64;
            c.w32(9, r81);
            c.logic(r81, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 116; continue;
        }
        59 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 116; continue;
        }
        62 => {
            let Some(t82) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089d2b6); };
            c.w32(9, t82);
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089d2ba); };
            c.cmp(t83, c.r32(9), 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d2c7); };
            c.w32(7, t84);
            let r85 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r85);
            let Some(t86) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d2cd); };
            c.w32(9, t86);
            let r87 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r87);
            let Some(t88) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d2d4); };
            c.w32(6, t88);
            blk = 54; continue;
        }
        64 => {
            let Some(t89) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089d2db); };
            c.w32(9, t89);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r90 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r90);
            let Some(t91) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d2e8); };
            c.w32(7, t91);
            let r92 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r92);
            let Some(t93) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d2ee); };
            c.w32(9, t93);
            let r94 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r94);
            let Some(t95) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d2f5); };
            c.w32(6, t95);
            blk = 54; continue;
        }
        65 => {
            let Some(t96) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d2fc); };
            c.w32(7, t96);
            let r97 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r97);
            let Some(t98) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d302); };
            c.w32(9, t98);
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            let Some(t100) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d309); };
            c.w32(6, t100);
            blk = 54; continue;
        }
        66 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 69; continue;
        }
        67 => {
            c.w32(9, 0x1u64);
            blk = 69; continue;
        }
        68 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 70; continue;
        }
        70 => {
            let r101 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r101);
            blk = 71; continue;
        }
        71 => {
            let r102 = c.r32(6) & 0x8u64;
            c.w32(6, r102);
            c.logic(r102, 32);
            c.w64(9, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r32(6), 0x8u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t103) = c.ld16(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x700089d343); };
            c.w32(7, t103);
            blk = 74; continue;
        }
        73 => {
            let Some(t104) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700089d34a); };
            c.w32(7, t104);
            blk = 74; continue;
        }
        74 => {
            c.w64(9, c.r64(7));
            let r105 = c.r32(9) & 0xfc00u64;
            c.w32(9, r105);
            c.logic(r105, 32);
            c.cmp(c.r32(9), 0xdc00u64, 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            let r106 = c.shl(c.r32(7), 0x10u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r106);
            let r107 = c.r32(7) | c.r32(2);
            c.w32(7, r107);
            c.logic(r107, 32);
            c.w64(2, c.r64(7));
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(2), 0x10000u64, 32);
            if c.cc_b() { blk = 80; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089d372); };
            c.w64(7, t108);
            let Some(t109) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089d379); };
            c.w64(9, t109);
            c.w64(1, c.r64(9).wrapping_add(0x10));
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089d380); };
            c.w64(6, t110);
            let Some(t111) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700089d387); };
            c.cmp(t111, c.r64(1), 64);
            if c.cc_be() { blk = 133; continue; }
            blk = 78; continue;
        }
        78 => {
            if !c.st64(c.r64(7), c.r64(1)) { return RegfileExit::fault(0x700089d390); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 79; continue;
        }
        79 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700089d397); };
            c.w64(9, t112);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089d39e); }
            if !c.st32(c.r64(7).wrapping_add(0x7), 0x2u64) { return RegfileExit::fault(0x700089d3a2); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089d3a9); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x700089d3b0); }
            c.w64(1, c.r64(7));
            blk = 82; continue;
        }
        80 => {
            c.cmp(c.r32(2), 0xffu64, 32);
            if c.cc_g() { blk = 135; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(7, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089d3cc); };
            c.w64(9, t113);
            let Some(t114) = c.ld32(c.r64(9).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d3d3); };
            c.w32(1, t114);
            let r115 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r115);
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089d3db); };
            c.cmp(t116, c.r32(1), 32);
            if c.cc_e() { blk = 140; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t117) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089d3e8); };
            c.w32(7, t117);
            c.cmp(c.r32(12), c.r32(7), 32);
            if c.cc_b() { blk = 24; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 87; continue;
        }
        84 => {
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089d3f7); };
            c.w64(7, t118);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700089d3fb); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r119 = c.r64(9) & c.r64(7);
            c.w64(9, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089d40f); };
            c.logic(t120 & 0x4u64, 8);
            if c.cc_ne() { blk = 152; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089d420); }
            return c.ret(0x0, 0x700089d421);
        }
        87 => {
            return RegfileExit::fault(0x700089d422);
        }
        88 => {
            return RegfileExit::fault(0x700089d423);
        }
        89 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700089d424); }
            return c.call(0x70007febb4, 0x700089d42d, 0x700089d428);
        }
        90 => {
            c.w64(12, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d430); };
            c.w64(8, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089d434); };
            c.w64(11, t122);
            blk = 9; continue;
        }
        91 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089d447, 0x700089d442);
        }
        92 => {
            c.w64(7, c.r64(0));
            blk = 12; continue;
        }
        93 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700089d44f); };
            c.w64(3, t123);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089d45f, 0x700089d45a);
        }
        94 => {
            return RegfileExit::fault(0x700089d45f);
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x700089d460); }
            return c.call(0x70007febf4, 0x700089d469, 0x700089d464);
        }
        96 => {
            c.w64(12, c.r64(0));
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d46c); };
            c.w64(0, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d470); };
            c.w64(7, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089d474); };
            c.w64(11, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089d478); };
            c.w64(9, t127);
            blk = 15; continue;
        }
        97 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700089d486); }
            return c.call(0x70007febf4, 0x700089d48f, 0x700089d48a);
        }
        98 => {
            c.w64(9, c.r64(0));
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d492); };
            c.w64(0, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d496); };
            c.w64(7, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089d49a); };
            c.w64(3, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089d49e); };
            c.w64(11, t131);
            blk = 18; continue;
        }
        99 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d4a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d4ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d4af); }
            blk = 103; continue;
        }
        100 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d4b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d4b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d4bd); }
            blk = 103; continue;
        }
        101 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d4c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d4c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d4cb); }
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t132) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089d4d4); };
            c.w64(6, t132);
            blk = 47; continue;
        }
        103 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w32(2, c.r64(15).wrapping_add(c.r64(15)));
            blk = 114; continue;
        }
        105 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089d4ec); };
            c.w64(2, t133);
            let Some(t134) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700089d4f3); };
            c.w64(1, t134);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089d4fa); };
            c.w64(8, t135);
            let Some(t136) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089d501); };
            c.cmp(t136, c.r64(6), 64);
            if c.cc_a() { blk = 108; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089d510, 0x700089d50b);
        }
        107 => {
            c.w64(2, c.r64(0));
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d513); };
            c.w64(12, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d517); };
            c.w64(15, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089d51b); };
            c.w64(11, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d51f); };
            c.w64(0, t140);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 109; continue;
        }
        108 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x700089d527); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 109; continue;
        }
        109 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 113; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let r141 = c.r64(10) | 0x1u64;
            c.w64(10, r141);
            c.logic(r141, 64);
            blk = 112; continue;
        }
        112 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 113; continue;
        }
        113 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089d555); };
            c.w64(1, t142);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089d55c); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089d55f); }
            blk = 114; continue;
        }
        114 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089d564); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089d565); }
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700089d566); };
            c.w64(2, t143);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700089d57f, 0x700089d57a);
        }
        115 => {
            let r144 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r144);
            c.w64(2, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d584); };
            c.w64(12, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d588); };
            c.w64(15, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089d58c); };
            c.w64(11, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d590); };
            c.w64(0, t148);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        116 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d59b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d59f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d5a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700089d5a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089d5ab); }
            blk = 120; continue;
        }
        117 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d5b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d5b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d5b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700089d5bd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089d5c1); }
            blk = 120; continue;
        }
        118 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d5c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d5cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d5cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700089d5d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089d5d7); }
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t149) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089d5e1); };
            c.w64(7, t149);
            blk = 71; continue;
        }
        120 => {
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            blk = 131; continue;
        }
        122 => {
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089d5fb); };
            c.w64(7, t150);
            let Some(t151) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089d602); };
            c.w64(9, t151);
            c.w64(6, c.r64(9).wrapping_add(0xc));
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089d609); };
            c.w64(8, t152);
            let Some(t153) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089d610); };
            c.cmp(t153, c.r64(6), 64);
            if c.cc_a() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700089d622, 0x700089d61d);
        }
        124 => {
            c.w64(7, c.r64(0));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d625); };
            c.w64(12, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d629); };
            c.w64(15, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089d62d); };
            c.w64(11, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089d631); };
            c.w64(2, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d635); };
            c.w64(0, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089d639); };
            c.w64(1, t159);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 126; continue;
        }
        125 => {
            if !c.st64(c.r64(7), c.r64(6)) { return RegfileExit::fault(0x700089d641); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 126; continue;
        }
        126 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089d648); };
            c.w64(9, t160);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089d64f); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 130; continue; }
            blk = 127; continue;
        }
        127 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            let r161 = c.r64(10) | 0x1u64;
            c.w64(10, r161);
            c.logic(r161, 64);
            blk = 129; continue;
        }
        129 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 130; continue;
        }
        130 => {
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089d67a); }
            blk = 131; continue;
        }
        131 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089d67f); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089d680); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700089d688); };
            c.w64(3, t162);
            c.w64(7, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700089d69a, 0x700089d695);
        }
        132 => {
            let r163 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r163);
            c.w64(7, c.r64(0));
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d69f); };
            c.w64(12, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d6a3); };
            c.w64(15, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089d6a7); };
            c.w64(11, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089d6ab); };
            c.w64(2, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d6af); };
            c.w64(0, t168);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 74; continue;
        }
        133 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700089d6ba); }
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d6c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d6c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d6cb); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700089d6d7, 0x700089d6d2);
        }
        134 => {
            c.w64(7, c.r64(0));
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d6da); };
            c.w64(12, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d6de); };
            c.w64(15, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089d6e2); };
            c.w64(11, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089d6e6); };
            c.w64(2, t172);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d6ec); };
            c.w64(0, t173);
            blk = 79; continue;
        }
        135 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089d6f5); };
            c.w64(7, t174);
            let Some(t175) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089d6fc); };
            c.w64(9, t175);
            c.w64(1, c.r64(9).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d703); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d707); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d70b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700089d70f); }
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089d713); };
            c.w64(6, t176);
            let Some(t177) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700089d71a); };
            c.cmp(t177, c.r64(1), 64);
            if c.cc_a() { blk = 138; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w32(2, 0x10u64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700089d72c, 0x700089d727);
        }
        137 => {
            blk = 139; continue;
        }
        138 => {
            if !c.st64(c.r64(7), c.r64(1)) { return RegfileExit::fault(0x700089d72e); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 139; continue;
        }
        139 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700089d738); };
            c.w64(7, t178);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089d73f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089d742); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089d749); }
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089d750); };
            c.w64(2, t179);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(2)) { return RegfileExit::fault(0x700089d754); }
            c.w64(1, c.r64(0));
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d75b); };
            c.w64(12, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d75f); };
            c.w64(15, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089d763); };
            c.w64(11, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d767); };
            c.w64(0, t183);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 82; continue;
        }
        140 => {
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089d772); };
            c.w64(1, t184);
            let Some(t185) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089d779); };
            c.w64(6, t185);
            c.w64(8, c.r64(6).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700089d780); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700089d784); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700089d788); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700089d78c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700089d790); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x700089d794); }
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089d798); };
            c.w64(6, t186);
            let Some(t187) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700089d79f); };
            c.cmp(t187, c.r64(8), 64);
            if c.cc_a() { blk = 143; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w32(1, 0x10u64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x700089d7b1, 0x700089d7ac);
        }
        142 => {
            blk = 144; continue;
        }
        143 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700089d7b3); }
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089d7b6); };
            c.w64(1, t188);
            let r189 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r189);
            c.w64(0, c.r64(1));
            blk = 144; continue;
        }
        144 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089d7c1); };
            c.w64(7, t190);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089d7c8); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089d7cb); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089d7d2); }
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089d7d9); };
            c.w64(2, t191);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(2)) { return RegfileExit::fault(0x700089d7dd); }
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089d7e0); };
            c.w64(9, t192);
            let Some(t193) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089d7e7); };
            c.w32(7, t193);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ae() { blk = 148; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089d7f6); };
            c.w64(7, t194);
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700089d7fa); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r195 = c.r64(11) & c.r64(9);
            c.w64(11, r195);
            c.logic(r195, 64);
            let Some(t196) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089d80c); };
            c.logic(t196 & 0x4u64, 8);
            if c.cc_ne() { blk = 149; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(1, c.r64(0));
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089d816); };
            c.w64(12, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d81a); };
            c.w64(15, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089d81e); };
            c.w64(11, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089d822); };
            c.w64(0, t200);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 82; continue;
        }
        148 => {
            return RegfileExit::fault(0x700089d82d);
        }
        149 => {
            c.w32(0, c.r32(0));
            let r201 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r201);
            c.w64(11, 0xfffffffffffc0000u64);
            let r202 = c.r64(11) & c.r64(0);
            c.w64(11, r202);
            c.logic(r202, 64);
            let Some(t203) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089d83d); };
            c.logic(t203 & 0x2u64, 8);
            if c.cc_e() { blk = 147; continue; }
            blk = 150; continue;
        }
        150 => {
            c.w64(12, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089d848); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089d849); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700089d856, 0x700089d851);
        }
        151 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089d856); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089d858); }
            blk = 147; continue;
        }
        152 => {
            c.w32(12, c.r32(12));
            let r204 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r204);
            c.w64(9, 0xfffffffffffc0000u64);
            let r205 = c.r64(9) & c.r64(12);
            c.w64(9, r205);
            c.logic(r205, 64);
            let Some(t206) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089d86b); };
            c.logic(t206 & 0x2u64, 8);
            if c.cc_e() { blk = 86; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(11, c.r64(7).wrapping_add(0xb));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089d87a); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700089d884, 0x700089d87f);
        }
        154 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089d884); }
            blk = 86; continue;
        }
        155 => {
            c.w32(1, c.r32(1));
            let r207 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r207);
            c.w64(12, 0xfffffffffffc0000u64);
            let r208 = c.r64(12) & c.r64(1);
            c.w64(12, r208);
            c.logic(r208, 64);
            let Some(t209) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089d89a); };
            c.logic(t209 & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089d8aa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089d8ab); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x700089d8b8, 0x700089d8b3);
        }
        157 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089d8b8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089d8ba); }
            blk = 26; continue;
        }
        158 => {
            return RegfileExit::jump(0x700089d8c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089d8d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089d8d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089d8d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089d8d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089d8da); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089d8df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700089d8e9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d8f5); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d8fb); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 38; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089d906); };
            c.w64(1, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089d90a); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700089d91b, 0x700089d916);
        }
        5 => {
            c.w64(1, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089d91e); };
            c.w64(0, t7);
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d92e); };
            c.w32(7, t8);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d93d); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d943); };
            c.w32(7, t11);
            c.logic(c.r8(7) & 0x7u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(6));
            blk = 10; continue;
        }
        10 => {
            c.w64(11, c.r64(9));
            let r12 = c.r32(11) & 0x7u64;
            c.w32(11, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 45; continue;
        }
        15 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 45; continue;
        }
        18 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089d995); };
            c.w32(9, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089d999); };
            c.cmp(t14, c.r32(9), 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d9a6); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d9ad); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d9b4); };
            c.w32(9, t19);
            blk = 10; continue;
        }
        20 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089d9bb); };
            c.w32(9, t20);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r21 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r21);
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d9c8); };
            c.w32(8, t22);
            let r23 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r23);
            let Some(t24) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d9cf); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            let Some(t26) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d9d6); };
            c.w32(9, t26);
            blk = 10; continue;
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089d9e0); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089d9e7); };
            c.w32(9, t29);
            let r30 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r30);
            let Some(t31) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089d9ee); };
            c.w32(9, t31);
            blk = 10; continue;
        }
        22 => {
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(10));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        23 => {
            c.w32(11, 0x1u64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 25; continue;
        }
        24 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(6));
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 26; continue;
        }
        26 => {
            let r32 = c.add(c.r64(9), 0xbu64, 64);
            c.w64(9, r32);
            blk = 27; continue;
        }
        27 => {
            let r33 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r33);
            let r34 = c.r32(7) & 0x8u64;
            c.w32(7, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t35) = c.ld16(c.r64(9).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x700089da4d); };
            c.w32(1, t35);
            blk = 30; continue;
        }
        29 => {
            let Some(t36) = c.ld8(c.r64(1).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700089da54); };
            c.w32(1, t36);
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 62; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089da6d); };
            c.w64(8, t37);
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089da74); };
            c.w32(0, t38);
            let r39 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r39);
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089da7c); };
            c.cmp(t40, c.r32(0), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089da89); };
            c.w64(1, t41);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089da90); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return c.ret(0x10, 0x700089da97);
        }
        34 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089da9a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089daa1); }
            return c.ret(0x0, 0x700089daa3);
        }
        35 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089daa4); };
            c.w64(0, t42);
            blk = 32; continue;
        }
        36 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700089daad); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089dab3); };
            c.w64(3, t43);
            return c.call(0x700086ed74, 0x700089dabf, 0x700089daba);
        }
        37 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089dabf); };
            c.w64(6, t44);
            blk = 1; continue;
        }
        38 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700089dac8); };
            c.cmp(t45, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089dad5); };
            c.cmp(t46, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700089dae4); }
            let r47 = c.r32(1) | c.r32(7);
            c.w32(1, r47);
            c.logic(r47, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            return c.call(0x700089c794, 0x700089daf1, 0x700089daec);
        }
        40 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089daf1); };
            c.w64(6, t48);
            blk = 3; continue;
        }
        41 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089daff); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089db00); };
            c.w64(1, t49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x18f)) else { return RegfileExit::fault(0x700089db07); };
            c.w32(1, t50);
            let r51 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r51);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089db10); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089db11); };
            c.w64(3, t52);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089db22, 0x700089db1d);
        }
        42 => {
            return RegfileExit::fault(0x700089db22);
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700089db23); }
            return c.call(0x70008a1214, 0x700089db2c, 0x700089db27);
        }
        44 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089db2c); };
            c.w64(6, t53);
            blk = 3; continue;
        }
        45 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089db35); }
            blk = 49; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089db3b); }
            blk = 49; continue;
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089db41); }
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t54) = c.ld64(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089db4b); };
            c.w64(9, t54);
            blk = 27; continue;
        }
        49 => {
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let r55 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r55);
            blk = 60; continue;
        }
        51 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089db61); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089db68); };
            c.w64(8, t57);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089db6f); };
            c.w64(11, t58);
            let Some(t59) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089db76); };
            c.cmp(t59, c.r64(9), 64);
            if c.cc_a() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089db85, 0x700089db80);
        }
        53 => {
            c.w64(7, c.r64(0));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089db88); };
            c.w64(0, t60);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089db8e); };
            c.w64(1, t61);
            blk = 55; continue;
        }
        54 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700089db94); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 59; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let r62 = c.r64(10) | 0x1u64;
            c.w64(10, r62);
            c.logic(r62, 64);
            blk = 58; continue;
        }
        58 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 59; continue;
        }
        59 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089dbc2); };
            c.w64(1, t63);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089dbc9); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089dbcc); }
            c.w64(1, c.r64(7));
            blk = 60; continue;
        }
        60 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089dbd4); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089dbd5); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700089dbd6); };
            c.w64(3, t64);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089dbe7, 0x700089dbe2);
        }
        61 => {
            let r65 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r65);
            c.w64(1, c.r64(0));
            blk = 30; continue;
        }
        62 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089dbf1); };
            c.w64(7, t66);
            let Some(t67) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089dbf8); };
            c.w64(8, t67);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089dbff); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089dc03); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089dc0a); };
            c.cmp(t69, c.r64(9), 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089dc19, 0x700089dc14);
        }
        64 => {
            blk = 66; continue;
        }
        65 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700089dc1b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 66; continue;
        }
        66 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700089dc25); };
            c.w64(1, t70);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089dc2c); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089dc2f); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089dc36); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089dc3d); };
            c.w64(1, t71);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x700089dc41); }
            blk = 32; continue;
        }
        67 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089dc4a); };
            c.w64(9, t72);
            let Some(t73) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089dc51); };
            c.w64(11, t73);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700089dc58); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089dc5c); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089dc60); };
            c.w64(15, t74);
            let Some(t75) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089dc67); };
            c.cmp(t75, c.r64(12), 64);
            if c.cc_a() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089dc76, 0x700089dc71);
        }
        69 => {
            blk = 71; continue;
        }
        70 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700089dc78); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 71; continue;
        }
        71 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089dc82); };
            c.w64(1, t76);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089dc89); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089dc8c); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089dc93); }
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089dc9a); };
            c.w64(1, t77);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x700089dc9e); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089dca1); };
            c.w64(8, t78);
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089dca8); };
            c.w32(7, t79);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            return RegfileExit::fault(0x700089dcb7);
        }
        73 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089dcb8); };
            c.w64(7, t80);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700089dcbc); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r81 = c.r64(1) & c.r64(8);
            c.w64(1, r81);
            c.logic(r81, 64);
            let Some(t82) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089dcce); };
            c.logic(t82 & 0x4u64, 8);
            if c.cc_ne() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 32; continue;
        }
        76 => {
            c.w32(0, c.r32(0));
            let r83 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r83);
            c.w64(1, 0xfffffffffffc0000u64);
            let r84 = c.r64(1) & c.r64(0);
            c.w64(1, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089dce8); };
            c.logic(t85 & 0x2u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089dcf2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089dcf3); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089dd00, 0x700089dcfb);
        }
        78 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089dd00); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089dd02); }
            blk = 75; continue;
        }
        79 => {
            return RegfileExit::jump(0x700089dd08);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089dd14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089dd14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089dd18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089dd19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089dd1a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089dd1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700089dd29); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089dd35); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089dd3b); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089dd46); };
            c.w64(1, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089dd4a); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700089dd5b, 0x700089dd56);
        }
        5 => {
            c.w64(1, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089dd5e); };
            c.w64(0, t7);
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089dd6e); };
            c.w32(7, t8);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089dd7d); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089dd83); };
            c.w32(7, t11);
            c.logic(c.r8(7) & 0x7u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(6));
            blk = 10; continue;
        }
        10 => {
            c.w64(11, c.r64(9));
            let r12 = c.r32(11) & 0x7u64;
            c.w32(11, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 44; continue;
        }
        15 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 44; continue;
        }
        18 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089ddd5); };
            c.w32(9, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089ddd9); };
            c.cmp(t14, c.r32(9), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089dde6); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089dded); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ddf4); };
            c.w32(9, t19);
            blk = 10; continue;
        }
        20 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089ddfb); };
            c.w32(9, t20);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r21 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r21);
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089de08); };
            c.w32(8, t22);
            let r23 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r23);
            let Some(t24) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089de0f); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            let Some(t26) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089de16); };
            c.w32(9, t26);
            blk = 10; continue;
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089de20); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089de27); };
            c.w32(9, t29);
            let r30 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r30);
            let Some(t31) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089de2e); };
            c.w32(9, t31);
            blk = 10; continue;
        }
        22 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 25; continue;
        }
        23 => {
            c.w32(11, 0x1u64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 25; continue;
        }
        24 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w64(8, c.r64(7));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(6));
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 26; continue;
        }
        26 => {
            let r32 = c.add(c.r64(9), 0xbu64, 64);
            c.w64(9, r32);
            blk = 27; continue;
        }
        27 => {
            let r33 = c.r32(8) & 0x8u64;
            c.w32(8, r33);
            c.logic(r33, 32);
            let r34 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r34);
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t35) = c.ld16(c.r64(9).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x700089de80); };
            c.w32(1, t35);
            blk = 30; continue;
        }
        29 => {
            let Some(t36) = c.ld8(c.r64(1).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700089de87); };
            c.w32(1, t36);
            blk = 30; continue;
        }
        30 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 31; continue;
        }
        31 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089de8f); };
            c.w64(1, t37);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089de96); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return c.ret(0x10, 0x700089de9d);
        }
        33 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089dea0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089dea7); }
            return c.ret(0x0, 0x700089dea9);
        }
        34 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x700089deaa); };
            c.w64(0, t38);
            blk = 31; continue;
        }
        35 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700089deb3); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089deb9); };
            c.w64(3, t39);
            return c.call(0x700086ed74, 0x700089dec5, 0x700089dec0);
        }
        36 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089dec5); };
            c.w64(6, t40);
            blk = 1; continue;
        }
        37 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700089dece); };
            c.cmp(t41, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089dedb); };
            c.cmp(t42, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700089deea); }
            let r43 = c.r32(1) | c.r32(7);
            c.w32(1, r43);
            c.logic(r43, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            return c.call(0x700089c794, 0x700089def7, 0x700089def2);
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089def7); };
            c.w64(6, t44);
            blk = 3; continue;
        }
        40 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089df05); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089df06); };
            c.w64(1, t45);
            let Some(t46) = c.ld32(c.r64(1).wrapping_add(0x193)) else { return RegfileExit::fault(0x700089df0d); };
            c.w32(1, t46);
            let r47 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r47);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089df16); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089df17); };
            c.w64(3, t48);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089df28, 0x700089df23);
        }
        41 => {
            return RegfileExit::fault(0x700089df28);
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700089df29); }
            return c.call(0x70008a1214, 0x700089df32, 0x700089df2d);
        }
        43 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089df32); };
            c.w64(6, t49);
            blk = 3; continue;
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089df3b); }
            blk = 48; continue;
        }
        45 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089df41); }
            blk = 48; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089df47); }
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t50) = c.ld64(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089df51); };
            c.w64(9, t50);
            blk = 27; continue;
        }
        48 => {
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let r51 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r51);
            blk = 59; continue;
        }
        50 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089df67); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089df6e); };
            c.w64(8, t53);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089df75); };
            c.w64(11, t54);
            let Some(t55) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089df7c); };
            c.cmp(t55, c.r64(9), 64);
            if c.cc_a() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089df8b, 0x700089df86);
        }
        52 => {
            c.w64(7, c.r64(0));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089df8e); };
            c.w64(0, t56);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089df94); };
            c.w64(1, t57);
            blk = 54; continue;
        }
        53 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700089df9a); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 54; continue;
        }
        54 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 58; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let r58 = c.r64(10) | 0x1u64;
            c.w64(10, r58);
            c.logic(r58, 64);
            blk = 57; continue;
        }
        57 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 58; continue;
        }
        58 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089dfc8); };
            c.w64(1, t59);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089dfcf); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089dfd2); }
            c.w64(1, c.r64(7));
            blk = 59; continue;
        }
        59 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089dfda); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089dfdb); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700089dfdc); };
            c.w64(3, t60);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089dfed, 0x700089dfe8);
        }
        60 => {
            let r61 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            c.w64(1, c.r64(0));
            blk = 30; continue;
        }
        61 => {
            return RegfileExit::jump(0x700089dff8);
        }
            _ => unreachable!(),
        }
    }
}
