#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000907054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000907054); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000907058); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090705e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907062); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000907066); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090706a); };
            c.w32(0, t4);
            let r5 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r5);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000907071); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000907075); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000907079); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700090707d); }
            let Some(t6) = c.ld16(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000907081); };
            c.logic(t6 & 0x800u64, 16);
            if c.cc_ne() { blk = 97; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090708d); };
            c.w64(0, t7);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 82; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000907099); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090709f); };
            c.w32(0, t10);
            let r11 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r11);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 82; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009070b1); };
            c.w64(0, t12);
            let Some(t13) = c.ld64(c.r64(7).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009070b5); };
            c.w64(3, t13);
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009070b9); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.cmp(c.r64(3), 0x0u64, 64);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t16) = c.ld16(c.r64(0).wrapping_add(0x15)) else { return RegfileExit::fault(0x70009070c9); };
            c.w32(0, t16);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70009070cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70009070d1); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 30; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(0), c.r64(3), 64);
            if c.cc_le() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(1, c.r64(3));
            blk = 7; continue;
        }
        6 => {
            c.w64(1, c.r64(0));
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 85; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(2, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(2, c.r32(2));
            let r17 = c.imul2(c.r32(2), 0x4u64, 32);
            c.w32(2, r17);
            if c.cc_o() { blk = 27; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(6, 0x8u64);
            let r18 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r18);
            if c.cc_o() { blk = 26; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(12, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090712e); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000907135); };
            c.w64(8, t20);
            c.w64(11, c.r64(8).wrapping_add(c.r64(2)));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090713c); };
            c.w64(0, t21);
            let Some(t22) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000907143); };
            c.cmp(t22, c.r64(11), 64);
            if c.cc_be() { blk = 83; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 83; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000907159); };
            c.w64(0, t23);
            if !c.st64(c.r64(0), c.r64(11)) { return RegfileExit::fault(0x7000907160); }
            let r24 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r24);
            blk = 14; continue;
        }
        14 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000907167); };
            c.w64(11, t25);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700090716e); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000907172); }
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(1));
            blk = 16; continue;
        }
        15 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000907183); }
            let r26 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r26);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(11), c.r64(6), 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ne() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_be() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(2, c.r64(7).wrapping_add(0x18));
            let Some(t27) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70009071a5); };
            c.w64(2, t27);
            blk = 22; continue;
        }
        21 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009071ab); };
            c.w64(2, t28);
            blk = 22; continue;
        }
        22 => {
            let r29 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r29);
            blk = 15; continue;
        }
        23 => {
            let r30 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r30);
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009071bc); };
            c.w64(2, t31);
            blk = 15; continue;
        }
        24 => {
            return RegfileExit::fault(0x70009071c5);
        }
        25 => {
            blk = 29; continue;
        }
        26 => {
            return RegfileExit::fault(0x70009071c8);
        }
        27 => {
            return RegfileExit::fault(0x70009071c9);
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009071ca); };
            c.w64(2, t32);
            c.w64(8, c.r64(2));
            blk = 29; continue;
        }
        29 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        30 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 89; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(2, c.r32(2));
            let r33 = c.imul2(c.r32(2), 0x4u64, 32);
            c.w32(2, r33);
            if c.cc_o() { blk = 47; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, 0x8u64);
            let r34 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r34);
            if c.cc_o() { blk = 46; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(6, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(12, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090721d); };
            c.w64(15, t35);
            let Some(t36) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000907224); };
            c.w64(15, t36);
            c.w64(8, c.r64(15).wrapping_add(c.r64(2)));
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000907233); };
            c.w64(11, t37);
            let Some(t38) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090723a); };
            c.cmp(t38, c.r64(8), 64);
            if c.cc_be() { blk = 87; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 87; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000907250); };
            c.w64(11, t39);
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x7000907257); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 37; continue;
        }
        37 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700090725e); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000907265); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000907269); }
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 39; continue;
        }
        38 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000907271); }
            let r41 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r41);
            let r42 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r42);
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_e() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_e() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(3), c.r64(6), 64);
            if c.cc_be() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(11, c.r64(7).wrapping_add(0x18));
            let Some(t43) = c.ld64(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907291); };
            c.w64(11, t43);
            blk = 38; continue;
        }
        43 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000907297); };
            c.w64(11, t44);
            blk = 38; continue;
        }
        44 => {
            return RegfileExit::fault(0x70009072a0);
        }
        45 => {
            blk = 49; continue;
        }
        46 => {
            return RegfileExit::fault(0x70009072a3);
        }
        47 => {
            return RegfileExit::fault(0x70009072a4);
        }
        48 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009072a5); };
            c.w64(2, t45);
            c.w64(8, c.r64(2));
            blk = 49; continue;
        }
        49 => {
            c.w32(11, 0x1u64);
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70009072bf); };
            c.w64(11, t46);
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_le() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(0, c.r64(3));
            blk = 54; continue;
        }
        53 => {
            c.w64(0, c.r64(11));
            blk = 54; continue;
        }
        54 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009072d0); };
            c.w64(6, t47);
            let Some(t48) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009072d4); };
            c.w32(2, t48);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70009072dd); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_a() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            return RegfileExit::fault(0x70009072e7);
        }
        56 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009072e8); };
            c.w32(2, t49);
            let r50 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r50);
            let Some(t51) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009072ee); };
            c.w32(2, t51);
            c.logic(c.r32(2) & 0x8000000u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w32(2, 0x2u64);
            blk = 59; continue;
        }
        58 => {
            c.w32(2, 0x3u64);
            blk = 59; continue;
        }
        59 => {
            c.w32(1, c.r64(0).wrapping_add(c.r64(0)));
            c.w64(12, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w32(15, c.r32(15));
            let r52 = c.imul2(c.r32(15), 0x4u64, 32);
            c.w32(15, r52);
            if c.cc_o() { blk = 75; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(7, 0x10u64);
            let r53 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r53);
            if c.cc_o() { blk = 74; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090732f); };
            c.w64(3, t54);
            let Some(t55) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000907336); };
            c.w64(3, t55);
            c.w64(0, c.r64(3).wrapping_add(c.r64(15)));
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090733d); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000907344); };
            c.cmp(t57, c.r64(0), 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 91; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090735a); };
            c.w64(7, t58);
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x7000907361); }
            c.w64(7, c.r64(3).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000907368); };
            c.w64(0, t59);
            let Some(t60) = c.ld32(c.r64(0).wrapping_add(0x40f)) else { return RegfileExit::fault(0x700090736f); };
            c.w32(0, t60);
            let r61 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r61);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000907378); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700090737b); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700090737e); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000907381); }
            c.w64(8, c.r64(2).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0x10));
            c.w32(0, 0x10u64);
            c.w64(3, c.r64(8));
            blk = 66; continue;
        }
        65 => {
            let r62 = c.sub(c.r64(3), 0x1u64, 64);
            c.w64(3, r62);
            c.w32(2, c.r64(3).wrapping_add(c.r64(3)));
            if !c.st32(c.r64(7).wrapping_add(c.r64(0)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70009073a2); }
            let r63 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r63);
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(2, c.r64(8));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009073b2); };
            let r65 = c.sub(c.r64(2), t64, 64);
            c.w64(2, r65);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 65; continue; }
            blk = 68; continue;
        }
        68 => {
            return RegfileExit::fault(0x70009073bb);
        }
        69 => {
            let Some(t66) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009073bc); };
            c.w32(8, t66);
            let r67 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r67);
            let Some(t68) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009073c3); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009073ca); };
            c.w32(11, t70);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x44u64, 64);
            if c.cc_a() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            return RegfileExit::fault(0x70009073da);
        }
        71 => {
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(0x117)) else { return RegfileExit::fault(0x70009073db); };
            c.w32(8, t71);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009073e2); };
            c.w64(11, t72);
            let Some(t73) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009073e9); };
            c.w64(0, t73);
            c.w64(3, c.r64(0).wrapping_add(0x14));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009073f0); };
            c.w64(2, t74);
            let r75 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009073f6); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70009073fd); };
            c.cmp(t77, c.r64(3), 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 72; continue;
        }
        72 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000907406); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 73; continue;
        }
        73 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090740d); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000907411); };
            c.w64(8, t78);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000907418); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700090741c); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x7000907420); }
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000907424); };
            c.w64(7, t79);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x7000907428); }
            c.w64(3, c.r64(11));
            blk = 81; continue;
        }
        74 => {
            return RegfileExit::fault(0x7000907431);
        }
        75 => {
            return RegfileExit::fault(0x7000907432);
        }
        76 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000907433); };
            c.w64(6, t80);
            let Some(t81) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000907437); };
            c.w32(11, t81);
            let r82 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r82);
            let Some(t83) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090743e); };
            c.w32(11, t83);
            let r84 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r84);
            let Some(t85) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000907445); };
            c.w32(0, t85);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xaeu64, 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            return RegfileExit::fault(0x7000907457);
        }
        78 => {
            let Some(t86) = c.ld32(c.r64(11).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x7000907458); };
            c.w32(11, t86);
            let Some(t87) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090745f); };
            c.w32(0, t87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000907463); };
            c.w64(3, t88);
            let Some(t89) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700090746a); };
            c.w64(2, t89);
            c.w64(1, c.r64(2).wrapping_add(0x14));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000907471); };
            c.w64(6, t90);
            let Some(t91) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000907478); };
            c.cmp(t91, c.r64(1), 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 79; continue;
        }
        79 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x7000907481); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 80; continue;
        }
        80 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000907488); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090748c); };
            c.w64(11, t92);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000907493); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000907497); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700090749b); }
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090749e); };
            c.w64(8, t93);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70009074a2); }
            blk = 81; continue;
        }
        81 => {
            c.w64(7, c.r64(9));
            c.w64(9, c.r64(7).wrapping_add(0x1));
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009074ad); };
            c.w64(0, t94);
            let Some(t95) = c.ld64(c.r64(0).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009074b1); };
            c.w64(7, t95);
            let Some(t96) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009074b5); };
            c.w32(8, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009074ba); };
            c.w64(15, t97);
            let Some(t98) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009074be); };
            c.w64(1, t98);
            let Some(t99) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009074c2); };
            c.w64(5, t99);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(3));
            let r100 = c.add(c.r64(4), 0x78u64, 64);
            c.w64(4, r100);
            return RegfileExit::jump(c.r64(1));
        }
        82 => {
            return RegfileExit::fault(0x70009074d2);
        }
        83 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009074d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70009074d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70009074db); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70009074df); }
            return c.call(0x70007febb4, 0x70009074e8, 0x70009074e3);
        }
        84 => {
            c.w64(8, c.r64(0));
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009074eb); };
            c.w64(9, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009074ef); };
            c.w64(7, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009074f3); };
            c.w64(3, t103);
            c.w32(6, 0x8u64);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009074fc); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000907500); };
            c.w64(15, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000907504); };
            c.w64(12, t106);
            blk = 14; continue;
        }
        85 => {
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000907511); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000907519); };
            c.w64(3, t107);
            return c.call(0x700086ed74, 0x7000907525, 0x7000907520);
        }
        86 => {
            return RegfileExit::fault(0x7000907525);
        }
        87 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000907526); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700090752a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700090752e); }
            return c.call(0x70007febb4, 0x7000907537, 0x7000907532);
        }
        88 => {
            c.w64(8, c.r64(0));
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090753a); };
            c.w64(9, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090753e); };
            c.w64(7, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000907542); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000907546); };
            c.w64(0, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090754a); };
            c.w64(12, t112);
            c.w32(1, 0x8u64);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000907553); };
            c.w64(6, t113);
            blk = 37; continue;
        }
        89 => {
            c.w64(8, c.r64(9));
            let r114 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r114);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000907562); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000907566); };
            c.w64(3, t115);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000907576, 0x7000907571);
        }
        90 => {
            return RegfileExit::fault(0x7000907576);
        }
        91 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000907577); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700090757b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700090757f); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x7000907583); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(1)) { return RegfileExit::fault(0x7000907587); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x7000907593, 0x700090758e);
        }
        92 => {
            c.w64(7, c.r64(0));
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907596); };
            c.w64(9, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090759a); };
            c.w64(6, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090759e); };
            c.w64(8, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70009075a2); };
            c.w64(12, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009075a6); };
            c.w64(2, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70009075aa); };
            c.w64(11, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70009075ae); };
            c.w64(1, t122);
            blk = 64; continue;
        }
        93 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70009075b7); }
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009075c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70009075c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70009075c8); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febb4, 0x70009075d4, 0x70009075cf);
        }
        94 => {
            c.w64(11, c.r64(0));
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009075d7); };
            c.w64(9, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009075db); };
            c.w64(7, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009075df); };
            c.w64(8, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009075e3); };
            c.w64(2, t126);
            blk = 73; continue;
        }
        95 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009075f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009075f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70009075f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70009075fd); }
            return c.call(0x70007febb4, 0x7000907606, 0x7000907601);
        }
        96 => {
            c.w64(3, c.r64(0));
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907609); };
            c.w64(9, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090760d); };
            c.w64(7, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000907611); };
            c.w64(0, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000907615); };
            c.w64(8, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000907619); };
            c.w64(11, t131);
            blk = 80; continue;
        }
        97 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000907626); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090762a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090762c); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x26f8)) else { return RegfileExit::fault(0x7000907630); };
            c.w64(3, t132);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000907644, 0x700090763f);
        }
        98 => {
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907644); };
            c.w64(7, t133);
            let Some(t134) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907648); };
            c.w64(7, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090764c); };
            c.w64(8, t135);
            c.w64(9, c.r64(8).wrapping_add(0x1));
            let Some(t136) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000907654); };
            c.w32(8, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907659); };
            c.w64(15, t137);
            let Some(t138) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090765d); };
            c.w64(1, t138);
            let Some(t139) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000907661); };
            c.w64(5, t139);
            c.w64(12, c.r64(7));
            let r140 = c.add(c.r64(4), 0x78u64, 64);
            c.w64(4, r140);
            return RegfileExit::jump(c.r64(1));
        }
        99 => {
            return RegfileExit::jump(0x7000907670);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000907674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000907678); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090767e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000907682); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000907686); };
            c.w64(11, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090768a); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000907698); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090769f); };
            c.w32(11, t6);
            let r7 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r7);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld64(c.r64(7).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009076b5); };
            c.w64(11, t8);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 29; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(2, c.r32(2));
            let r9 = c.imul2(c.r32(2), 0x4u64, 32);
            c.w32(2, r9);
            if c.cc_o() { blk = 19; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, 0x8u64);
            let r10 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r10);
            if c.cc_o() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009076f9); };
            c.w64(6, t11);
            let Some(t12) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000907700); };
            c.w64(6, t12);
            c.w64(12, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000907707); };
            c.w64(15, t13);
            let Some(t14) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700090770e); };
            c.cmp(t14, c.r64(12), 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000907724); };
            c.w64(2, t15);
            if !c.st64(c.r64(2), c.r64(12)) { return RegfileExit::fault(0x700090772b); }
            c.w64(2, c.r64(6).wrapping_add(0x1));
            blk = 9; continue;
        }
        9 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000907732); };
            c.w64(6, t16);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000907739); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700090773c); }
            c.w64(0, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        10 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700090774b); }
            let r17 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r17);
            let r18 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r18);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(6), c.r64(11), 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(11), c.r64(6), 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(3, c.r64(7).wrapping_add(0x18));
            let Some(t19) = c.ld64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8))) else { return RegfileExit::fault(0x700090776a); };
            c.w64(3, t19);
            blk = 10; continue;
        }
        15 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000907770); };
            c.w64(3, t20);
            blk = 10; continue;
        }
        16 => {
            return RegfileExit::fault(0x7000907779);
        }
        17 => {
            blk = 21; continue;
        }
        18 => {
            return RegfileExit::fault(0x700090777c);
        }
        19 => {
            return RegfileExit::fault(0x700090777d);
        }
        20 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090777e); };
            c.w64(0, t21);
            c.w64(2, c.r64(0));
            blk = 21; continue;
        }
        21 => {
            let Some(t22) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000907788); };
            c.w32(8, t22);
            let r23 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r23);
            let Some(t24) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090778f); };
            c.w32(8, t24);
            let r25 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000907796); };
            c.w32(11, t26);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xb0u64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70009077a9);
        }
        23 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70009077aa); };
            c.w32(8, t27);
            let Some(t28) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009077b1); };
            c.w32(11, t28);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009077b5); };
            c.w64(0, t29);
            let Some(t30) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70009077bc); };
            c.w64(3, t30);
            c.w64(1, c.r64(3).wrapping_add(0x10));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009077c3); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70009077ca); };
            c.cmp(t32, c.r64(1), 64);
            if c.cc_be() { blk = 31; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x70009077d3); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70009077da); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009077de); };
            c.w64(8, t33);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70009077e5); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70009077e9); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70009077ec); }
            let Some(t34) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009077f0); };
            c.w64(7, t34);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x1));
            let Some(t35) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009077fb); };
            c.w32(8, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907800); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907804); };
            c.w64(1, t37);
            let Some(t38) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000907808); };
            c.w64(5, t38);
            c.w64(12, c.r64(7));
            let r39 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r39);
            return RegfileExit::jump(c.r64(1));
        }
        26 => {
            return RegfileExit::fault(0x7000907815);
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000907816); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090781a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700090781e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000907822); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000907826); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700090782a); }
            return c.call(0x70007febb4, 0x7000907833, 0x700090782e);
        }
        28 => {
            c.w64(2, c.r64(0));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907836); };
            c.w64(9, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090783a); };
            c.w64(7, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090783e); };
            c.w64(8, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000907842); };
            c.w64(11, t43);
            c.w32(1, 0x8u64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090784b); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090784f); };
            c.w64(0, t45);
            blk = 9; continue;
        }
        29 => {
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700090785c); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000907860); };
            c.w64(3, t46);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000907870, 0x700090786b);
        }
        30 => {
            return RegfileExit::fault(0x7000907870);
        }
        31 => {
            c.w32(0, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000907876); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090787a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090787e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000907882); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000907886); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x7000907892, 0x700090788d);
        }
        32 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000907892); };
            c.w64(8, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907896); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090789a); };
            c.w64(7, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090789e); };
            c.w64(11, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009078a2); };
            c.w64(2, t51);
            blk = 25; continue;
        }
        33 => {
            return RegfileExit::jump(0x70009078ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009078b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009078b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009078b8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009078be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009078c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009078c6); };
            c.w64(11, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009078ca); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009078d8); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009078df); };
            c.w32(11, t6);
            let r7 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r7);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 29; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009078f5); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009078fc); };
            c.w64(11, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907900); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000907907); };
            c.w32(0, t13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090790b); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907915); };
            c.w64(3, t16);
            let Some(t17) = c.ld16(c.r64(11).wrapping_add(0x15)) else { return RegfileExit::fault(0x7000907919); };
            c.w32(11, t17);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x6bu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x7000907927);
        }
        4 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_le() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(3));
            let r18 = c.sub(c.r64(0), c.r64(11), 64);
            c.w64(0, r18);
            blk = 7; continue;
        }
        6 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 7; continue;
        }
        7 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x700090793a); };
            c.w32(8, t19);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000907943); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 32; continue; }
            blk = 9; continue;
        }
        9 => {
            let r20 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(1, c.r32(1));
            let r21 = c.imul2(c.r32(1), 0x4u64, 32);
            c.w32(1, r21);
            if c.cc_o() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(6, 0x8u64);
            let r22 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r22);
            if c.cc_o() { blk = 23; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000907984); };
            c.w64(12, t23);
            let Some(t24) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700090798b); };
            c.w64(12, t24);
            c.w64(15, c.r64(12).wrapping_add(c.r64(1)));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000907993); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090799a); };
            c.cmp(t26, c.r64(15), 64);
            if c.cc_be() { blk = 30; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 30; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009079b0); };
            c.w64(8, t27);
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x70009079b7); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70009079bf); };
            c.w64(1, t28);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70009079c6); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70009079ca); }
            c.w64(0, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            blk = 16; continue;
        }
        15 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70009079d8); }
            let r29 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r29);
            let r30 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r30);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, c.r64(7).wrapping_add(0x18));
            let Some(t31) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009079f8); };
            c.w64(2, t31);
            blk = 15; continue;
        }
        20 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009079fe); };
            c.w64(2, t32);
            blk = 15; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000907a07);
        }
        22 => {
            blk = 26; continue;
        }
        23 => {
            return RegfileExit::fault(0x7000907a0a);
        }
        24 => {
            return RegfileExit::fault(0x7000907a0b);
        }
        25 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000907a0c); };
            c.w64(0, t33);
            c.w64(8, c.r64(0));
            blk = 26; continue;
        }
        26 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000907a16); };
            c.w32(11, t34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000907a1a); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000907a21); };
            c.w64(3, t36);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000907a28); };
            c.w64(1, t37);
            let Some(t38) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000907a2f); };
            c.cmp(t38, c.r64(2), 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000907a38); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000907a3f); };
            c.w64(3, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000907a43); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000907a46); };
            c.w64(3, t40);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000907a4d); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000907a50); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000907a54); }
            let Some(t41) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907a58); };
            c.w64(7, t41);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x1));
            let Some(t42) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000907a63); };
            c.w32(8, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907a68); };
            c.w64(15, t43);
            let Some(t44) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907a6c); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000907a70); };
            c.w64(5, t45);
            c.w64(12, c.r64(7));
            let r46 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r46);
            return RegfileExit::jump(c.r64(1));
        }
        29 => {
            return RegfileExit::fault(0x7000907a7d);
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000907a7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000907a82); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000907a86); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000907a8a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000907a8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000907a92); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000907a9e, 0x7000907a99);
        }
        31 => {
            c.w64(8, c.r64(0));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907aa1); };
            c.w64(9, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907aa5); };
            c.w64(7, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000907aa9); };
            c.w64(3, t49);
            c.w32(6, 0x8u64);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000907ab2); };
            c.w64(11, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000907ab6); };
            c.w64(2, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000907aba); };
            c.w64(0, t52);
            blk = 14; continue;
        }
        32 => {
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000907ac7); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000907acb); };
            c.w64(3, t53);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000907ad9, 0x7000907ad4);
        }
        33 => {
            return RegfileExit::fault(0x7000907ad9);
        }
        34 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000907adf); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000907ae3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000907ae7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000907aeb); }
            return c.call(0x70007febb4, 0x7000907af4, 0x7000907aef);
        }
        35 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000907af4); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907af8); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907afc); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000907b00); };
            c.w64(11, t57);
            blk = 28; continue;
        }
        36 => {
            return RegfileExit::jump(0x7000907b0c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907b14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000907b17); };
            c.w64(7, t0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907b1b); };
            c.w32(8, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000907b21); };
            c.w32(11, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000907b27); };
            c.cmp(t3, c.r8(11), 8);
            if c.cc_g() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907b2e); };
            c.w64(7, t4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907b32); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            c.w32(3, c.r32(8));
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907b3c); };
            c.w32(1, t7);
            let r8 = c.sub(c.r32(1), 0x3u64, 32);
            c.w32(1, r8);
            let r9 = c.sub(c.r32(1), c.r32(3), 32);
            c.w32(1, r9);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000907b4e); }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907b52); };
            c.w64(2, t10);
            let r11 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000907b59); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907b5e); };
            c.w64(1, t13);
            c.w64(12, c.r64(2));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000907b67); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000907b6b); }
            let r14 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r14);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t15) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907b75); };
            c.w64(3, t15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000907b79); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000907b7f); };
            c.w32(3, t18);
            let r19 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r19);
            let Some(t20) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000907b85); };
            c.w32(3, t20);
            let r21 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000907b8b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000907b8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000907b93); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000907b97); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000907b9b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000907b9f); }
            let Some(t22) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907ba3); };
            c.cmp(t22, 0x5eu64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, c.r32(8));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000907bb1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000907bb5); }
            c.w64(6, c.r64(7));
            return c.call(0x70007fcf54, 0x7000907bc1, 0x7000907bbc);
        }
        6 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907bc1); };
            c.w64(2, t23);
            let Some(t24) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907bc5); };
            c.w64(7, t24);
            let Some(t25) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907bc9); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907bd0); };
            c.w32(11, t27);
            let r28 = c.sub(c.r32(11), 0x3u64, 32);
            c.w32(11, r28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000907bd8); };
            c.w64(9, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000907bdc); };
            let r31 = c.sub(c.r64(9), t30, 64);
            c.w64(9, r31);
            let Some(t32) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000907be0); };
            let r33 = c.sub(c.r32(11), t32, 32);
            c.w32(11, r33);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000907be4); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ge() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t34) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000907bee); };
            c.w64(8, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000907bf2); };
            c.w64(11, t35);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000907bf6); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000907bfa); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2148)) else { return RegfileExit::fault(0x7000907bfb); };
            c.w64(3, t36);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x7000907c0f, 0x7000907c0a);
        }
        8 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907c0f); };
            c.w64(2, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000907c13); };
            c.w64(9, t38);
            blk = 10; continue;
        }
        9 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000907c19); }
            blk = 10; continue;
        }
        10 => {
            let Some(t39) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907c1d); };
            c.w64(2, t39);
            let Some(t40) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000907c21); };
            c.w32(7, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907c26); };
            c.w64(15, t41);
            let Some(t42) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907c2a); };
            c.w64(1, t42);
            let Some(t43) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000907c2e); };
            c.w64(5, t43);
            c.w64(12, c.r64(2));
            let Some(t44) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000907c35); };
            c.w64(0, t44);
            let r45 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r45);
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000907c40); }
            let Some(t46) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000907c44); };
            c.w64(5, t46);
            c.w64(6, c.r64(7));
            let r47 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r47);
            return RegfileExit::jump(0x70007fd494);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000907c54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000907c58); }
            let r48 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r48);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t49) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000907c62); };
            c.w64(3, t49);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000907c66); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000907c6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000907c6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000907c6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000907c73); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000907c77); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000907c7b); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x1u64);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2148)) else { return RegfileExit::fault(0x7000907c87); };
            c.w64(3, t50);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000907c99, 0x7000907c94);
        }
        13 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907c99); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000907c9d); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000907ca1); };
            c.w64(2, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000907ca5); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000907ca9); };
            c.w64(8, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000907cb0); }
            blk = 3; continue;
        }
        14 => {
            return RegfileExit::jump(0x7000907cb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907cd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907cd7); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907cdb); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907ce1); };
            c.w32(2, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907ce7); };
            c.w32(6, t4);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000907cee); }
            let r5 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907cf4); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907cf9); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000907d00);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907d17); };
            c.w32(2, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907d1d); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907d25); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907d29); };
            c.w32(2, t4);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907d30); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907d39); };
            c.w32(6, t7);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000907d40); }
            let r8 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907d46); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907d4b); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000907d54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907d74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907d74); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907d7a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907d82); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000907d86); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907d93); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907d98); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907da1); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907da8); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907db1); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000907db8); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907dbe); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907dc3); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000907dcc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907dd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907dd4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907dda); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907de2); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000907de6); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907df3); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907df8); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907e01); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907e08); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907e11); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000907e18); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907e1e); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907e23); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000907e2c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907e34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907e34); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907e3a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907e42); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000907e46); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907e53); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907e58); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907e61); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907e68); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907e71); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000907e78); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907e7e); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907e83); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000907e8c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907e94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907e94); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907e9a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907ea2); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000907ea6); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907eb3); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907eb8); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907ec1); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907ec8); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907ed1); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000907ed8); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907ede); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907ee3); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000907eec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000907ef4); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000907efd); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907f0a); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907f0f); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            c.w64(3, c.r64(5));
            let Some(t5) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907f18); };
            c.w32(2, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907f1e); };
            c.w32(6, t6);
            let r7 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r7);
            let Some(t8) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907f26); };
            c.w64(3, t8);
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907f2a); };
            c.w32(2, t9);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907f31); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907f3a); };
            c.w32(6, t12);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000907f41); }
            let r13 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907f47); };
            c.w32(3, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907f4c); };
            c.w64(1, t15);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000907f54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907f74); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907f7a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907f82); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000907f86); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907f93); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907f98); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000907fa1); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000907fa8); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907fb1); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000907fb8); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907fbe); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907fc3); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000907fcc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000907fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000907fd4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907fda); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000907fe2); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000907fe6); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000907ff3); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000907ff8); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908001); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908008); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908011); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908018); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090801e); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908023); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700090802c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908034); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090803a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908042); };
            c.w32(3, t3);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090804a); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908050); };
            c.cmp(t6, 0xbau64, 16);
            if c.cc_b() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090805b); };
            c.w64(2, t7);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908062); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090806b); };
            c.w32(6, t10);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908072); }
            let r11 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908078); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090807d); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r14 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r14);
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908087); };
            c.w32(3, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090808c); };
            c.w64(1, t16);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000908094);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009080b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009080b4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009080ba); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009080c2); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70009080c6); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70009080d5); };
            c.cmp(t5, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009080de); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009080e4); };
            c.logic(t8 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009080ea); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009080f3); };
            c.cmp(t10, 0x41u64, 16);
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
            c.w64(2, c.r64(5));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908104); };
            c.w64(2, t11);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090810b); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908114); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700090811b); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908121); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908126); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            let r18 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908130); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908135); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090813b); };
            c.logic(t21 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000908146); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x7000908158);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908174); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090817a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908182); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000908186); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000908195); };
            c.cmp(t5, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090819e); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009081a4); };
            c.logic(t8 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009081aa); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009081b3); };
            c.cmp(t10, 0x41u64, 16);
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
            let r11 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009081c5); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009081ca); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            c.w64(2, c.r64(5));
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009081d3); };
            c.w64(2, t14);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009081da); };
            c.w32(2, t15);
            let r16 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r16);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009081e3); };
            c.w32(6, t17);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009081ea); }
            let r18 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009081f0); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009081f5); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009081fb); };
            c.logic(t21 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000908206); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x7000908218);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908234); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700090823a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000908249); };
            c.cmp(t2, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000908252); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000908258); };
            c.logic(t5 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090825e); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908267); };
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
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908278); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090827c); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908282); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908289); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090828f); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908294); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            let r15 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090829e); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009082a3); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009082a9); };
            c.logic(t18 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009082b4); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x70009082c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009082d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009082d4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70009082da); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70009082e9); };
            c.cmp(t2, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009082f2); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009082f8); };
            c.logic(t5 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009082fe); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908307); };
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
            let r8 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908319); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090831e); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            c.w64(2, c.r64(5));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908327); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090832b); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908331); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908338); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090833e); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908343); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000908349); };
            c.logic(t18 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000908354); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x7000908368);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908374); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700090837a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908387); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090838c); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908395); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908399); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090839f); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009083a6); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009083ac); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009083b1); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70009083b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009083d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009083d4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70009083da); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009083e7); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009083ec); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009083f5); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009083f9); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009083ff); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908406); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090840c); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908411); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000908418);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908434); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700090843a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908447); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090844c); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908455); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908459); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090845f); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908466); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090846c); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908471); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000908478);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908494); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700090849a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009084a7); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009084ac); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009084b5); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009084b9); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009084bf); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009084c6); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009084cc); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009084d1); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70009084d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009084f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009084f4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009084fa); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908507); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090850c); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908515); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908519); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090851f); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908526); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090852c); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908531); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000908538);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908554); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090855a); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908567); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090856c); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908575); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908579); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090857f); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908586); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090858c); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908591); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000908598);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009085b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009085b4); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70009085bd); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009085ca); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009085cf); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            c.w64(3, c.r64(5));
            let Some(t5) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009085d8); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009085dc); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009085e2); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009085e8); };
            c.w32(6, t9);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009085ef); }
            let r10 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009085f5); };
            c.w32(3, t11);
            let Some(t12) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009085fa); };
            c.w64(1, t12);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000908604);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908614); };
            c.w32(3, t0);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090861e); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908624); };
            c.cmp(t3, 0xbau64, 16);
            if c.cc_b() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090862f); };
            c.w64(2, t4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908633); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908639); };
            c.w32(6, t7);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x2));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000908640); }
            let r8 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908646); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090864b); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r11 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908655); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090865a); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000908664);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000908677); };
            c.w64(2, (t0 as u8 as i8 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908680); };
            c.w32(6, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000908686); };
            c.w32(7, t2);
            let r3 = c.sub(c.r64(3), c.r64(2), 64);
            c.w64(3, r3);
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_l() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_le() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r4 = c.add(c.r64(3), c.r64(6), 64);
            c.w64(3, r4);
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009086a0); };
            c.w32(6, t5);
            let r6 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009086a8); };
            c.w64(2, t7);
            let Some(t8) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009086ac); };
            c.w32(3, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009086b3); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009086bc); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x4));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009086c3); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009086c9); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009086ce); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r15 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009086d8); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009086dd); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70009086e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009086f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009086f4); };
            c.w64(2, (t0 as u8 as i8 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009086fd); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000908701); };
            c.w32(0, t2);
            let r3 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908707); };
            c.w64(8, t4);
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090870b); };
            c.w32(11, t5);
            c.w64(3, c.r64(11));
            let r6 = c.r64(3) & 0x3ffu64;
            c.w64(3, r6);
            c.logic(r6, 64);
            c.cmp(c.r64(3), 0x3ffu64, 64);
            if c.cc_e() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(11, c.r64(0));
            c.w64(3, c.r64(2));
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090872c); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090872f); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x7000908738); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908741); };
            c.cmp(t10, 0x834u64, 16);
            if c.cc_ne() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t11) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090874c); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000908751); };
            c.w32(3, t12);
            let r13 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000908758); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000908761); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000908768); };
            c.w32(1, t17);
            c.logic(c.r16(1) & 0x3ffu64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 12; continue;
        }
        9 => {
            let r18 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908779); };
            c.w32(2, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700090877e); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908784); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908788); }
            let r21 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090878e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000908792); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000908796); }
            blk = 26; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090879f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009087a3); }
            let r22 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009087a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009087ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009087b1); }
            blk = 26; continue;
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009087ba); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009087be); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009087c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009087c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009087cc); }
            blk = 26; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009087d5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009087d9); }
            let r24 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009087df); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009087e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009087e7); }
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009087f8); };
            c.w32(11, t25);
            let r26 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r26);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000908805); };
            c.w64(11, t27);
            blk = 16; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090880c); };
            c.w32(3, t28);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x700090881c);
        }
        18 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090881f); };
            c.cmp(t29, c.r32(3), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(2));
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908828); };
            c.w32(3, t30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090882c); };
            c.cmp(t31, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x7000908835); };
            c.cmp(t32, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090883e); };
            c.cmp(t33, 0x834u64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000908848); };
            c.cmp(t34, c.r32(3), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090884e); };
            c.w32(11, t35);
            let r36 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000908855); };
            c.cmp(t37, c.r32(11), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t38) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090885e); };
            c.w32(0, t38);
            let r39 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r39);
            let Some(t40) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000908865); };
            c.w32(3, t40);
            c.logic(c.r16(3) & 0x3ffu64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000908873); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000908877); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x1f50)) else { return RegfileExit::fault(0x7000908878); };
            c.w64(3, t41);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x700090888c, 0x7000908887);
        }
        27 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090888c); };
            c.w64(7, t42);
            let Some(t43) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000908890); };
            c.w64(2, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000908894); };
            c.w64(9, t44);
            let r45 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r45);
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700090889c); };
            c.w32(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009088a1); };
            c.w64(15, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009088a5); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009088a9); };
            c.w64(5, t49);
            c.w64(12, c.r64(2));
            let r50 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        28 => {
            let r51 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r51);
            let Some(t52) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009088ba); };
            c.w32(2, t52);
            let Some(t53) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009088bf); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009088c3); };
            c.w64(5, t54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009088c7); };
            c.w64(0, t55);
            let r56 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r56);
            return RegfileExit::jump(c.r64(1));
        }
        29 => {
            return RegfileExit::jump(0x70009088d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009088f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009088f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009088f8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009088fe); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908902); };
            c.w64(2, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908906); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090890c); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000908912); };
            c.w32(6, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000908918); };
            c.w32(7, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090891b); }
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090891f); };
            c.cmp(t9, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000908928); };
            c.w64(2, t10);
            blk = 2; continue;
        }
        2 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090892f); };
            c.w32(7, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000908932); };
            c.cmp(t12, c.r32(2), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000908941); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090894e); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000908954); };
            c.w32(7, t16);
            let r17 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r17);
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090895a); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000908961); };
            c.w32(11, t20);
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000908965); };
            c.w32(1, t21);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090896b); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            let r24 = c.r64(1) & 0x3ffu64;
            c.w64(1, r24);
            c.logic(r24, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(11, 0x6u64);
            blk = 6; continue;
        }
        5 => {
            c.w32(11, 0x2u64);
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000908994); };
            c.w32(8, t25);
            c.w32(11, c.r32(11));
            c.w32(15, c.r32(8));
            let r26 = c.r32(11) | c.r32(15);
            c.w32(11, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70009089aa); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009089af); }
            blk = 9; continue;
        }
        9 => {
            c.w32(2, c.r64(1).wrapping_add(c.r64(1)));
            blk = 10; continue;
        }
        10 => {
            let Some(t27) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009089b9); };
            c.w64(6, (t27 as u8 as i8 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x70009089bf); }
            let r28 = c.sub(c.r64(6), 0x1u64, 64);
            c.w64(6, r28);
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(7)) { return RegfileExit::fault(0x70009089c7); }
            let r29 = c.sub(c.r64(6), 0x1u64, 64);
            c.w64(6, r29);
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x70009089cf); }
            let r30 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r30);
            let Some(t31) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009089d7); };
            c.w32(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009089dc); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009089e0); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009089e4); };
            c.w64(5, t34);
            let r35 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r35);
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009089f4); };
            c.w32(7, t36);
            c.w32(8, c.r32(7));
            let r37 = c.r32(8) | 0xeu64;
            c.w32(8, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000908a07); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000908a0b); }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000908a12); };
            c.w32(2, t38);
            c.w64(7, c.r64(0));
            blk = 10; continue;
        }
        15 => {
            return RegfileExit::jump(0x7000908a1c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908a34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908a34); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000908a3d); };
            c.w64(6, (t1 as u8 as i8 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908a43); };
            c.w64(3, t2);
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908a47); };
            c.w64(2, t3);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000908a4f); };
            c.w64(3, t4);
            blk = 3; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000908a58); };
            c.w64(3, t5);
            blk = 3; continue;
        }
        3 => {
            let r6 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r6);
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908a63); };
            c.w32(2, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908a68); };
            c.w64(1, t8);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000908a74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908a94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908a94); };
            c.w64(2, (t0 as u8 as i8 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000908a9d); };
            c.w64(8, (t1 as u8 as i8 as i64 as u64));
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000908aa3); };
            c.w64(11, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908aa9); };
            c.w64(2, t3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908aad); };
            c.w64(8, t4);
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908ab5); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908ab9); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908abf); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908ac3); };
            c.w64(11, t9);
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000908ac7); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000908acd); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000908ad3); };
            c.w32(6, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000908ad6); };
            c.cmp(t14, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000908adf); };
            c.w64(3, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000908ae6); };
            c.w32(6, t16);
            c.cmp(c.r32(8), c.r32(6), 32);
            if c.cc_ae() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908af1); };
            c.w32(0, t17);
            let r18 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r18);
            c.w64(8, c.r64(9).wrapping_add(0x5));
            let Some(t19) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000908afd); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t20) = c.ld8(c.r64(8).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908b03); };
            c.w32(2, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908b08); };
            c.w64(1, t21);
            c.w64(9, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            return RegfileExit::fault(0x7000908b11);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908b12); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908b16); }
            let r22 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r22);
            let Some(t23) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908b1c); };
            c.w64(11, t23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000908b20); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000908b24); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000908b28); }
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000908b2c); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000908b35); };
            c.w32(6, t25);
            c.w32(12, c.r32(6));
            let r26 = c.r32(12) | 0xeu64;
            c.w32(12, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(6, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x7000908b48); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000908b4c); }
            blk = 9; continue;
        }
        9 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000908b57); }
            c.w64(6, c.r64(11));
            c.w64(3, c.r64(2));
            return c.call(0x7000857fb4, 0x7000908b66, 0x7000908b61);
        }
        10 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000908b66); };
            c.w64(7, t27);
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000908b6a); };
            c.w64(2, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908b6e); };
            c.w64(8, t29);
            let Some(t30) = c.ld8(c.r64(8).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000908b72); };
            c.w32(7, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000908b77); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908b7b); };
            c.w64(1, t32);
            let Some(t33) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000908b7f); };
            c.w64(5, t33);
            c.w64(12, c.r64(2));
            c.w64(9, c.r64(8));
            let r34 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            return RegfileExit::jump(0x7000908b90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908b94); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908b9d); };
            c.w64(3, t1);
            let r2 = c.add(c.r32(3), 0x2u64, 32);
            c.w32(3, r2);
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            let r3 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908bab); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908bb0); };
            c.w64(1, t5);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000908bb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x3030)) else { return RegfileExit::fault(0x7000908bd4); };
            c.w64(3, t0);
            if !c.st64(c.r64(13).wrapping_add(0x3030), c.r64(0)) { return RegfileExit::fault(0x7000908bdb); }
            let r1 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908be6); };
            c.w32(2, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908beb); };
            c.w64(1, t3);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000908bf8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908c14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908c14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908c18); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000908c1e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908c22); };
            c.w64(1, t2);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000908c2a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000908c2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000908c2f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000908c33); }
            c.w32(0, 0x1u64);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x21b8)) else { return RegfileExit::fault(0x7000908c3c); };
            c.w64(3, t3);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000908c4b, 0x7000908c46);
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908c4b); };
            c.w64(2, t4);
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908c4f); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000908c53); };
            c.w64(7, t6);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000908c57); }
            c.w32(2, 0x62u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000908c60); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000908c61); };
            c.w64(3, t7);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000908c75, 0x7000908c70);
        }
        2 => {
            return RegfileExit::fault(0x7000908c75);
        }
        3 => {
            return RegfileExit::jump(0x7000908c78);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908c94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908c94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908c98); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000908c9e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908ca2); };
            c.w64(1, t2);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000908caa); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000908cae); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000908caf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000908cb3); }
            c.w32(0, 0x1u64);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000908cbc); };
            c.w64(3, t3);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000908ccb, 0x7000908cc6);
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908ccb); };
            c.w64(2, t4);
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908ccf); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000908cd3); };
            c.w64(7, t6);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000908cd7); }
            c.w32(2, 0x62u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000908ce0); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000908ce1); };
            c.w64(3, t7);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000908cf5, 0x7000908cf0);
        }
        2 => {
            return RegfileExit::fault(0x7000908cf5);
        }
        3 => {
            return RegfileExit::jump(0x7000908cf8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(1, c.r64(5));
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000908d1b); }
            let Some(t0) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908d1f); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000908d23); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908d2a); };
            c.w32(11, t3);
            let r4 = c.sub(c.r32(9), 0x21u64, 32);
            c.w32(9, r4);
            let r5 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r5);
            let r6 = c.sub(c.r32(11), c.r32(9), 32);
            c.w32(11, r6);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000908d3f); }
            blk = 2; continue;
        }
        2 => {
            return c.ret(0x0, 0x7000908d43);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908d44); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908d48); }
            let r7 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r7);
            let Some(t8) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908d4e); };
            c.w64(8, t8);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000908d52); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000908d56); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000908d57); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2140)) else { return RegfileExit::fault(0x7000908d5b); };
            c.w64(3, t9);
            c.w64(6, c.r64(8));
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x7000908d72, 0x7000908d6d);
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908d72); };
            c.w64(0, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000908d79); }
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x7000908d80);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908d94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000908d94); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let r1 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908da1); };
            c.w32(2, t2);
            c.w64(7, c.r64(2).wrapping_mul(8));
            c.cmp(c.r32(2), 0xb4u64, 32);
            if c.cc_ae() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908db6); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            c.w64(2, c.r64(5));
            if !c.st64(c.r64(2).wrapping_add(c.r64(7)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x7000908dbf); }
            let r4 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r4);
            let Some(t5) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908dcb); };
            c.w32(2, t5);
            let Some(t6) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908dd0); };
            c.w64(1, t6);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908dd6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908dda); }
            let r7 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908de0); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908de6); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000908dee); };
            c.w64(8, t11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000908df2); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            let Some(t14) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908df9); };
            c.w64(7, t14);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000908e01); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000908e05); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000908e06); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x7000908e0a); }
            c.w32(0, 0x1u64);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2228)) else { return RegfileExit::fault(0x7000908e13); };
            c.w64(3, t15);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000908e22, 0x7000908e1d);
        }
        5 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908e22); };
            c.w64(2, t16);
            let Some(t17) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908e26); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000908e2a); };
            c.w64(8, t18);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000908e2e); }
            c.w32(2, 0x62u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000908e37); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000908e38); };
            c.w64(3, t19);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000908e4c, 0x7000908e47);
        }
        6 => {
            return RegfileExit::fault(0x7000908e4c);
        }
        7 => {
            return RegfileExit::jump(0x7000908e50);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000908e54); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r1 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908e61); };
            c.w32(2, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908e66); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908e6c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908e70); }
            let r4 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r4);
            let Some(t5) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000908e76); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908e7a); };
            c.w64(7, t6);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000908e82); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000908e86); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x7000908e8a); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1dc0)) else { return RegfileExit::fault(0x7000908e8e); };
            c.w64(3, t7);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000908e9f, 0x7000908e9a);
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908e9f); };
            c.w64(2, t8);
            let Some(t9) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908ea3); };
            c.w64(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000908ea7); };
            c.w64(8, t10);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000908eab); }
            c.w32(2, 0x62u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000908eb4); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000908eb5); };
            c.w64(3, t11);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000908ec9, 0x7000908ec4);
        }
        4 => {
            return RegfileExit::fault(0x7000908ec9);
        }
        5 => {
            return RegfileExit::jump(0x7000908ecc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908ed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000908ed4); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r1 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908ee1); };
            c.w32(2, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908ee6); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908eec); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908ef0); }
            let r4 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r4);
            let Some(t5) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000908ef6); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908efa); };
            c.w64(7, t6);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000908f02); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000908f06); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x7000908f0a); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1db8)) else { return RegfileExit::fault(0x7000908f0e); };
            c.w64(3, t7);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000908f1f, 0x7000908f1a);
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908f1f); };
            c.w64(2, t8);
            let Some(t9) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908f23); };
            c.w64(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000908f27); };
            c.w64(8, t10);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000908f2b); }
            c.w32(2, 0x62u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000908f34); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000908f35); };
            c.w64(3, t11);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000908f49, 0x7000908f44);
        }
        4 => {
            return RegfileExit::fault(0x7000908f49);
        }
        5 => {
            return RegfileExit::jump(0x7000908f4c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908f54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908f54); };
            c.w64(2, (t0 as u8 as i8 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908f5d); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908f61); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000908f65); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000908f6c); };
            c.logic(t5 & 0x40u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r6 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r6);
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000908f77); };
            c.w32(2, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908f7c); };
            c.w64(1, t8);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000908f82); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000908f86); }
            let r9 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r9);
            let Some(t10) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908f8c); };
            c.w64(11, t10);
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000908f94); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000908f98); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000908f99); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000908f9b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000908f9f); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1da8)) else { return RegfileExit::fault(0x7000908fa3); };
            c.w64(3, t11);
            c.w64(6, c.r64(8));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x7000908fba, 0x7000908fb5);
        }
        3 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000908fba); };
            c.w64(7, t12);
            let Some(t13) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000908fbe); };
            c.w64(2, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000908fc2); };
            c.w64(8, t14);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000908fc6); }
            c.w32(7, 0x62u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000908fcf); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000908fd0); };
            c.w64(3, t15);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x7000908fe4, 0x7000908fdf);
        }
        4 => {
            return RegfileExit::fault(0x7000908fe4);
        }
        5 => {
            return RegfileExit::jump(0x7000908fe8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000908ff4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000908ff4); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000908ffd); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000909001); };
            c.cmp(t2, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090900a); };
            c.w32(6, t3);
            c.w64(7, 0xfffffffffffffffcu64);
            if !c.st32(c.r64(3).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000909014); }
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000909017); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            if !c.st64(c.r64(2).wrapping_sub(0x8), c.r64(3)) { return RegfileExit::fault(0x700090901d); }
            c.w64(3, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x7000909024); };
            c.w32(6, t6);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r7 = c.add(c.r64(3), c.r64(6), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909030); };
            c.w32(6, t8);
            let r9 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r9);
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000909038); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090903c); };
            c.w32(3, t11);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000909043); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090904c); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x4));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000909053); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909059); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090905e); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            let r18 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909068); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090906d); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000909074);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909094(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909094); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909098); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009090a2); };
            c.w64(1, t1);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009090a6); }
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009090aa); };
            c.w64(7, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009090b0); };
            c.w64(8, t3);
            let Some(t4) = c.ld64(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009090b4); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009090b8); };
            c.w64(11, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009090bc); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009090c3); };
            c.w32(15, t8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70009090c9); };
            c.w32(12, t9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009090cf); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld16(c.r64(8).wrapping_add(0x15)) else { return RegfileExit::fault(0x70009090d5); };
            c.w32(8, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70009090da); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009090de); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009090e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009090e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009090ea); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(0));
            blk = 4; continue;
        }
        1 => {
            c.w64(2, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x7000909101); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r13 = c.r64(7) & c.r64(3);
            c.w64(7, r13);
            c.logic(r13, 64);
            let Some(t14) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000909115); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(0, c.r64(2));
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_ae() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(0).wrapping_add(0x3));
            let Some(t15) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090912b); };
            c.w32(2, t15);
            let Some(t16) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700090912e); };
            c.w64(9, t16);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_b() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 20; continue;
        }
        7 => {
            let r17 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r17);
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(7)), c.r32(2)) { return RegfileExit::fault(0x700090914e); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r18 = c.r64(9) & c.r64(3);
            c.w64(9, r18);
            c.logic(r18, 64);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000909160); };
            c.logic(t19 & 0x4u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_ae() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(2, 0xfffffffffffffffau64);
            let r20 = c.sub(c.r64(2), c.r64(6), 64);
            c.w64(2, r20);
            let Some(t21) = c.ld64(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700090917a); };
            c.w64(2, t21);
            c.w64(7, c.r64(8).wrapping_add(c.r64(6)));
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909182); };
            c.w32(9, t22);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 19; continue;
        }
        12 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090919a); };
            c.w64(7, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090919e); };
            c.w64(8, t24);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70009091a2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r25 = c.r64(9) & c.r64(7);
            c.w64(9, r25);
            c.logic(r25, 64);
            let Some(t26) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009091b6); };
            c.logic(t26 & 0x4u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70009091bd); }
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009091c0); };
            c.w64(2, t27);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x70009091c4); }
            let Some(t28) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009091c7); };
            c.w64(7, t28);
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009091cb); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009091d2); };
            c.w32(9, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009091d6); };
            c.w64(11, t32);
            let r33 = c.sub(c.r32(11), 0x21u64, 32);
            c.w32(11, r33);
            let r34 = c.sub(c.r32(9), 0x5u64, 32);
            c.w32(9, r34);
            let r35 = c.sub(c.r32(9), c.r32(11), 32);
            c.w32(9, r35);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70009091eb); }
            blk = 16; continue;
        }
        16 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009091ef); };
            c.w64(0, t36);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009091f6); }
            return c.ret(0x0, 0x70009091f7);
        }
        17 => {
            let Some(t37) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009091f8); };
            c.w64(8, t37);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009091fc); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000909200); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2140)) else { return RegfileExit::fault(0x7000909201); };
            c.w64(3, t38);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x7000909215, 0x7000909210);
        }
        18 => {
            blk = 16; continue;
        }
        19 => {
            return RegfileExit::fault(0x7000909217);
        }
        20 => {
            return RegfileExit::fault(0x7000909218);
        }
        21 => {
            c.w32(8, c.r32(8));
            let r39 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r39);
            c.w64(9, 0xfffffffffffc0000u64);
            let r40 = c.r64(9) & c.r64(8);
            c.w64(9, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000909229); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, c.r64(7).wrapping_add(0xf));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000909234); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700090923e, 0x7000909239);
        }
        23 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700090923e); }
            blk = 14; continue;
        }
        24 => {
            c.w32(2, c.r32(2));
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            c.w64(9, 0xfffffffffffc0000u64);
            let r43 = c.r64(9) & c.r64(2);
            c.w64(9, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000909254); };
            c.logic(t44 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000909263); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000909264); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000909271, 0x700090926c);
        }
        26 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000909271); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000909273); }
            blk = 9; continue;
        }
        27 => {
            c.w32(9, c.r32(9));
            let r45 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r45);
            c.w64(7, 0xfffffffffffc0000u64);
            let r46 = c.r64(7) & c.r64(9);
            c.w64(7, r46);
            c.logic(r46, 64);
            let Some(t47) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000909289); };
            c.logic(t47 & 0x2u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000909297); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000909298); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70009092a5, 0x70009092a0);
        }
        29 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009092a5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009092a7); }
            blk = 3; continue;
        }
        30 => {
            return RegfileExit::jump(0x70009092b0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009092b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009092b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009092b8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009092be); };
            c.w64(3, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009092c2); };
            c.w64(2, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009092c8); };
            c.w64(6, t3);
            let Some(t4) = c.ld64(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009092cc); };
            c.w64(2, t4);
            let Some(t5) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009092d0); };
            c.w32(6, t5);
            let r6 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r6);
            let Some(t7) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009092d6); };
            c.w32(7, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009092dc); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(6).wrapping_add(0x15)) else { return RegfileExit::fault(0x70009092e3); };
            c.w32(6, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009092e7); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            let r11 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x198)) else { return RegfileExit::fault(0x70009092f4); };
            c.w64(1, t12);
            if !c.st32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70009092fb); }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(0, c.r64(6).wrapping_add(c.r64(11)));
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909309); };
            c.w32(1, t13);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_ae() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909318); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            c.w64(15, 0xfffffffffffffffau64);
            let r16 = c.sub(c.r64(15), c.r64(11), 64);
            c.w64(15, r16);
            if !c.st64(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(8)), c.r64(1)) { return RegfileExit::fault(0x700090932a); }
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090932e); };
            c.w32(1, t17);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_b() { blk = 1; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 8; continue;
        }
        6 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090933f); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            let r20 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909349); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090934e); };
            c.w64(15, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909352); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909356); };
            c.w64(5, t24);
            let r25 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r25);
            return RegfileExit::jump(c.r64(1));
        }
        7 => {
            return RegfileExit::fault(0x7000909360);
        }
        8 => {
            return RegfileExit::fault(0x7000909361);
        }
        9 => {
            return RegfileExit::jump(0x7000909364);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909378); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909382); };
            c.w64(8, t1);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000909386); }
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090938a); };
            c.w64(7, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090938e); };
            c.w64(11, (t3 as u8 as i8 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000909394); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090939a); };
            c.w64(0, t6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090939e); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009093a2); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009093a8); };
            c.w32(3, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70009093ab); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009093af); };
            c.cmp(t11, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009093b8); };
            c.w64(7, t12);
            blk = 2; continue;
        }
        2 => {
            let Some(t13) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009093bf); };
            c.w32(3, t13);
            let Some(t14) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70009093c5); };
            c.w32(2, t14);
            c.w64(1, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(2, c.r64(7));
            return c.call(0x70008a8954, 0x70009093e1, 0x70009093dc);
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009093e1); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009093e5); };
            c.w64(8, t16);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009093ec); };
            c.w64(7, t17);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8).wrapping_add(0x4));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009093f7); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70009093fc); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909403); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909407); };
            c.w64(5, t21);
            c.w64(12, c.r64(7));
            let r22 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000909418);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909434); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909438); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090943e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909442); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700090944a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700090944e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000909452); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000909456); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090945a); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1f10)) else { return RegfileExit::fault(0x700090945e); };
            c.w64(3, t3);
            c.w64(6, c.r64(7));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000909472, 0x700090946d);
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000909472); };
            c.w64(2, t4);
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909476); };
            c.w64(2, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090947a); };
            c.w64(7, t6);
            c.w64(9, c.r64(7).wrapping_add(0x1));
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000909482); };
            c.w32(7, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000909487); };
            c.w64(15, t8);
            let Some(t9) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090948b); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090948f); };
            c.w64(5, t10);
            c.w64(12, c.r64(2));
            let Some(t11) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000909496); };
            c.w64(0, t11);
            let r12 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r12);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009094a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009094b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009094b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009094b8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009094be); };
            c.w64(2, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009094c2); };
            c.w32(7, t2);
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009094c8); };
            c.w64(8, t3);
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009094cc); };
            c.w64(11, t4);
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            let r5 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r5);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009094d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70009094da); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009094de); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70009094e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009094e6); }
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(7));
            c.w64(6, c.r64(11));
            return c.call(0x70008a86d4, 0x70009094f8, 0x70009094f3);
        }
        1 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009094f8); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009094fc); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909500); };
            c.w64(7, t8);
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000909508); };
            c.w32(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090950d); };
            c.w64(15, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909511); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909515); };
            c.w64(5, t12);
            c.w64(12, c.r64(2));
            let Some(t13) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700090951c); };
            c.w64(0, t13);
            let r14 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r14);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000909528);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909534); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909538); }
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090953a); };
            c.w32(2, t0);
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909544); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700090954a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090954e); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x700090954f); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000909562, 0x700090955d);
        }
        1 => {
            return RegfileExit::fault(0x7000909562);
        }
        2 => {
            return RegfileExit::jump(0x7000909564);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000909574); };
            c.w32(3, t0);
            c.w64(2, c.r64(5));
            if !c.st64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x700090957c); }
            let r1 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909588); };
            c.w32(3, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090958d); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000909594);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009095b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009095b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009095b8); }
            let Some(t0) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009095ba); };
            c.w64(2, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009095be); };
            c.w64(1, t1);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70009095c6); }
            c.w32(2, 0x16u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009095cf); }
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x70009095d0); };
            c.w64(3, t2);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x70009095e4, 0x70009095df);
        }
        1 => {
            return RegfileExit::fault(0x70009095e4);
        }
        2 => {
            return RegfileExit::jump(0x70009095e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009095f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009095f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009095f8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909602); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909606); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090960c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000909610); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000909611); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000909615); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000909619); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700090961d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x7000909631, 0x700090962c);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909634); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909638); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090963c); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000909640); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909644); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700090964b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000909658);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909678); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909682); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909686); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090968c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000909690); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000909691); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000909695); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000909699); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700090969d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x70009096b1, 0x70009096ac);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009096b4); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009096b8); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009096bc); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70009096c0); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009096c4); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70009096cb); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009096d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009096f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009096f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009096f8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909702); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909706); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090970c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000909710); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000909711); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000909715); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000909719); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700090971d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x7000909731, 0x700090972c);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909734); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909738); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090973c); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000909740); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909744); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700090974b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000909758);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909774); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909778); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909782); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909786); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090978c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000909790); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000909791); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000909795); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000909799); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700090979d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x70009097b1, 0x70009097ac);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009097b4); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009097b8); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009097bc); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70009097c0); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009097c4); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70009097cb); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009097d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009097f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009097f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009097f8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909802); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909806); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090980c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000909810); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000909811); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000909815); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000909819); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700090981d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x7000909831, 0x700090982c);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909834); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909838); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090983c); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x7000909840); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909844); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700090984b); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000909858);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909874); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909878); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909882); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909886); };
            c.w64(8, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090988c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000909890); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000909891); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000909895); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000909899); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x700090989d); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086f394, 0x70009098b1, 0x70009098ac);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009098b4); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009098b8); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009098bc); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70009098c0); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009098c4); };
            c.w64(5, t9);
            c.w64(12, c.r64(7));
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70009098cb); };
            c.w64(9, t10);
            let r11 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009098d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009098f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009098f4); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009098fd); };
            c.w64(3, t1);
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909905); };
            c.w32(2, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700090990a); };
            c.w64(1, t4);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000909914);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909934(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909934); };
            c.w32(3, (t0 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            let r1 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909941); };
            c.w32(3, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909946); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000909950);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909954); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090995a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909962); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090996d); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909972); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x700090997c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909994); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090999d); };
            c.w64(3, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009099a1); };
            c.w32(2, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009099a7); };
            c.w32(6, t3);
            c.cmp(c.r16(6), 0x0u64, 16);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009099b3); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009099c2); };
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
            return RegfileExit::fault(0x70009099cf);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009099d0); };
            c.w32(6, t8);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70009099de);
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009099df); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009099ea); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009099ef); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            return RegfileExit::jump(0x70009099f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909a14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909a14); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909a1d); };
            c.w64(3, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909a21); };
            c.w32(2, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000909a27); };
            c.w32(6, t3);
            c.cmp(c.r16(6), 0x0u64, 16);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909a33); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000909a42); };
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
            return RegfileExit::fault(0x7000909a4f);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909a50); };
            c.w32(6, t8);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000909a5e);
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909a5f); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909a6a); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909a6f); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            return RegfileExit::jump(0x7000909a78);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909a94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909a97); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909a9b); };
            c.w32(2, t1);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909aa1); };
            c.w32(6, t2);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000909aaf);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909ab0); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909abb); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909ac0); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000909ac8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909ad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909ad7); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909adb); };
            c.w32(2, t1);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909ae1); };
            c.w32(6, t2);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000909aef);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909af0); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909afb); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909b00); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000909b08);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909b14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909b14); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            if !c.st64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x7000909b1d); }
            let r1 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909b25); };
            c.w32(3, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909b2a); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000909b34);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909b54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909b54); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909b5d); };
            c.w64(3, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909b61); };
            c.w64(6, (t2 as u16 as i16 as i64 as u64));
            if !c.st64(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(3)) { return RegfileExit::fault(0x7000909b67); }
            let r3 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909b6f); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909b74); };
            c.w64(1, t5);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000909b7c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909b97); };
            c.w64(2, t0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909b9b); };
            c.w64(6, (t1 as u16 as i16 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x7000909ba1); }
            if !c.st64(c.r64(3).wrapping_sub(0x8), c.r64(0)) { return RegfileExit::fault(0x7000909ba5); }
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909bad); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909bb2); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000909bbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909bd4); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909bdd); };
            c.w64(3, t1);
            if !c.st64(c.r64(2).wrapping_sub(0x8), c.r64(3)) { return RegfileExit::fault(0x7000909be1); }
            let r2 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909be9); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909bee); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000909bf8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909c14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909c14); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909c1d); };
            c.w64(3, t1);
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000909c25); };
            c.w64(3, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000909c2e); };
            c.w64(3, t3);
            blk = 3; continue;
        }
        3 => {
            let r4 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r4);
            let Some(t5) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909c39); };
            c.w32(2, t5);
            let Some(t6) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909c3e); };
            c.w64(1, t6);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000909c48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000909c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000909c57); };
            c.w64(11, t0);
            let Some(t1) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000909c5b); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909c62); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000909c69); };
            c.w32(0, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000909c6d); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000909c7b); };
            c.w64(0, t7);
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000909c82); };
            c.w64(11, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000909c89); };
            c.cmp(t9, c.r32(0), 32);
            if c.cc_e() { blk = 256; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909c96); };
            c.w32(3, t10);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(2, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t11) = c.ld32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000909ca3); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            let r13 = c.r64(1) & 0xfffffffffffffffdu64;
            c.w64(1, r13);
            c.logic(r13, 64);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000909cbc); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000909cc3); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let r17 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r17);
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909cd4); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909cd9); };
            c.w64(1, t19);
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            let Some(t20) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909ce2); };
            c.w64(8, t20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000909ce6); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000909ced); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909cf4); };
            c.w32(11, t25);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x7000909d07);
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x7000909d08); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909d15); };
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
            if c.cc_ae() { blk = 255; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000909d39); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909d41); };
            c.w32(0, t32);
            if let Some(sr) = c.shr(c.r64(11), 0xcu64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r33 = c.r64(11) & 0x3ffffu64;
            c.w64(11, r33);
            c.logic(r33, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000909d5b);
        }
        13 => {
            let r34 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r34);
            let Some(t35) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909d60); };
            c.w32(0, t35);
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909d65); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909d6d); };
            c.w64(1, t38);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909d76); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909d7a); }
            let r39 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000909d80); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000909d84); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000909d88); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000909d8c); }
            blk = 253; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000909d95); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000909d99); }
            let r40 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000909d9f); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000909da6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000909daa); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000909dae); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000909db2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000909db6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000909dba); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000909dbe); };
            c.cmp(t43, c.r32(7), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t44) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000909dcb); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000909dcf); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000909dd5); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909ddb); };
            c.w32(6, t49);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000909de4); }
            c.cmp(c.r64(6), 0x3u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x7000909dee);
        }
        18 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000909def); };
            c.w32(0, t50);
            let r51 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000909df5); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x7000909dff);
        }
        20 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000909e00); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000909e10); };
            c.w32(6, t54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000909e13); };
            c.cmp(t55, c.r32(6), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909e1c); };
            c.w32(6, t56);
            let r57 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r57);
            c.w32(3, 0);
            c.logic(0, 32);
            c.cmp(c.r32(6), c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t58) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909e28); };
            c.cmp(t58, c.r32(3), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000909e31); };
            c.w32(3, t59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000909e34); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000909e40); };
            c.cmp(t62, c.r32(6), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 27; continue;
        }
        27 => {
            let r63 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r63);
            c.logic(r63, 64);
            blk = 29; continue;
        }
        28 => {
            c.w64(6, c.r64(1));
            blk = 29; continue;
        }
        29 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(6));
            c.w64(6, c.r64(10));
            blk = 37; continue;
        }
        30 => {
            let r64 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r64);
            let Some(t65) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000909e6d); };
            c.w32(8, t65);
            let Some(t66) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909e72); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909e76); };
            c.w64(5, t67);
            c.w64(0, c.r64(6));
            let r68 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r68);
            return RegfileExit::jump(c.r64(1));
        }
        31 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r69 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r69);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t70) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909e8e); };
            c.w32(6, t70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909e94); };
            c.w32(9, t71);
            let r72 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r72);
            let Some(t73) = c.ld32(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909e9c); };
            c.w32(6, t73);
            let r74 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r74);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000909ea4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000909ea8); }
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(10));
            c.w64(0, c.r64(3));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000909ec1); };
            c.w64(3, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000909ec5); };
            c.logic(t76 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let r77 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r77);
            blk = 34; continue;
        }
        33 => {
            let Some(t78) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000909ed4); };
            c.w32(7, t78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000909ed7); };
            c.w64(7, t79);
            blk = 34; continue;
        }
        34 => {
            return c.call(c.r64(7), 0x7000909ee1, 0x7000909edf);
        }
        35 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909ee1); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909ee5); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000909ee9); };
            c.w64(9, t82);
            let Some(t83) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000909eed); };
            c.w32(11, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000909ef2); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909ef6); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909efa); };
            c.w64(5, t86);
            c.w64(12, c.r64(8));
            let r87 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r87);
            return RegfileExit::jump(c.r64(1));
        }
        36 => {
            c.w64(3, c.r64(1));
            c.w64(6, c.r64(7));
            blk = 37; continue;
        }
        37 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(2, c.r64(6));
            let r88 = c.r64(2) & 0xfu64;
            c.w64(2, r88);
            c.logic(r88, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000909f1a); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_e() { blk = 183; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(2), 0x5u64, 64);
            if c.cc_e() { blk = 182; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(2), 0xdu64, 64);
            if c.cc_e() { blk = 180; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_e() { blk = 119; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(2), 0x6u64, 64);
            if c.cc_e() { blk = 117; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 112; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(2), 0x8u64, 64);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(2), 0x9u64, 64);
            if c.cc_e() { blk = 101; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r64(2), 0xbu64, 64);
            if c.cc_e() { blk = 115; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(2), 0xcu64, 64);
            if c.cc_e() { blk = 57; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(2), 0xeu64, 64);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r89 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r89);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t90) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000909fa6); };
            c.w32(2, t90);
            let Some(t91) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909fac); };
            c.w32(1, t91);
            let r92 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r92);
            let Some(t93) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000909fb4); };
            c.w32(2, t93);
            let r94 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r94);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000909fbb); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000909fbf); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000909fc0); };
            if !c.push64(t95) { return RegfileExit::fault(0x7000909fc0); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000909fc3); }
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000909fc4); };
            if !c.push64(t96) { return RegfileExit::fault(0x7000909fc4); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000909fc7); };
            if !c.push64(t97) { return RegfileExit::fault(0x7000909fc7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000909fca); }
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000909fce); };
            c.w64(3, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000909fd5); };
            c.w64(6, t99);
            c.w32(0, 0x5u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000909fe6, 0x7000909fe1);
        }
        50 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909fe6); };
            c.w64(8, t100);
            let Some(t101) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000909fea); };
            c.w64(8, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000909fee); };
            c.w64(9, t102);
            let Some(t103) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000909ff2); };
            c.w32(11, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000909ff7); };
            c.w64(15, t104);
            let Some(t105) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000909ffb); };
            c.w64(1, t105);
            let Some(t106) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000909fff); };
            c.w64(5, t106);
            c.w64(12, c.r64(8));
            let r107 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r107);
            return RegfileExit::jump(c.r64(1));
        }
        51 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090a00c); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            let Some(t110) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a013); };
            c.w32(11, t110);
            let r111 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r111);
            let Some(t112) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a01a); };
            c.w32(0, t112);
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r113 = c.r64(6) & 0x1ffffffu64;
            c.w64(6, r113);
            c.logic(r113, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_b() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            return RegfileExit::fault(0x700090a034);
        }
        53 => {
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a035); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a03d); };
            c.w32(0, t116);
            let r117 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r117);
            c.w64(11, c.r64(9).wrapping_add(0x5));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700090a048); }
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090a04c); };
            c.cmp(t118, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t119) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090a055); };
            c.w32(8, t119);
            let Some(t120) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a05a); };
            c.w64(1, t120);
            let Some(t121) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a05e); };
            c.w64(5, t121);
            c.w64(9, c.r64(11));
            let r122 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r122);
            return RegfileExit::jump(c.r64(1));
        }
        55 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r123 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r123);
            let Some(t124) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090a071); };
            c.w32(3, t124);
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a077); };
            c.w32(2, t125);
            let r126 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r126);
            let Some(t127) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a07f); };
            c.w32(3, t127);
            let r128 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r128);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090a086); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090a08f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090a090); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700090a091); };
            c.w64(3, t129);
            c.w32(0, 0x2u64);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a09d); };
            c.w64(6, t130);
            return c.call(0x700086ed74, 0x700090a0a6, 0x700090a0a1);
        }
        56 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a0a6); };
            c.w64(8, t131);
            let Some(t132) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a0aa); };
            c.w64(8, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090a0ae); };
            c.w64(9, t133);
            let Some(t134) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a0b2); };
            c.w32(11, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a0b7); };
            c.w64(15, t135);
            let Some(t136) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a0bb); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a0bf); };
            c.w64(5, t137);
            c.w64(12, c.r64(8));
            let r138 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r138);
            return RegfileExit::jump(c.r64(1));
        }
        57 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r139 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r139);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t140) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090a0d7); };
            c.w32(2, t140);
            let Some(t141) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a0dd); };
            c.w32(1, t141);
            let r142 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r142);
            let Some(t143) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a0e5); };
            c.w32(2, t143);
            let r144 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r144);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090a0ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700090a0f0); }
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a0f4); };
            c.w64(6, t145);
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(2));
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090a0fe); };
            c.w64(1, t146);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(2));
            return c.call(0x70008bc174, 0x700090a10c, 0x700090a107);
        }
        58 => {
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a10c); };
            c.w64(8, t147);
            let Some(t148) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a110); };
            c.w64(8, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090a114); };
            c.w64(9, t149);
            let Some(t150) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a118); };
            c.w32(11, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a11d); };
            c.w64(15, t151);
            let Some(t152) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a121); };
            c.w64(1, t152);
            let Some(t153) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a125); };
            c.w64(5, t153);
            c.w64(12, c.r64(8));
            let r154 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r154);
            return RegfileExit::jump(c.r64(1));
        }
        59 => {
            blk = 115; continue;
        }
        60 => {
            let Some(t155) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090a137); };
            c.w32(0, t155);
            let r156 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r156);
            let Some(t157) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a13d); };
            c.w32(2, t157);
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090a140); };
            c.cmp(t158, c.r32(0), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t159) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090a14d); };
            c.w32(1, t159);
            let Some(t160) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a153); };
            c.w32(6, t160);
            let r161 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r161);
            let Some(t162) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a15b); };
            c.w32(1, t162);
            let r163 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r163);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t164) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a16b); };
            c.w32(2, t164);
            let r165 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r165);
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090a171); };
            c.cmp(t166, c.r32(2), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t167) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a17e); };
            c.w32(2, t167);
            let r168 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r168);
            let Some(t169) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a184); };
            c.w32(6, t169);
            let r170 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r170);
            let Some(t171) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090a18a); };
            c.logic(t171 & 0x2u64, 8);
            if c.cc_ne() { blk = 82; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090a194); };
            c.cmp(t172, c.r32(6), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(0, c.r64(11));
            blk = 99; continue;
        }
        66 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090a1a5); };
            c.w64(11, t173);
            let Some(t174) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090a1ac); };
            c.cmp(t174, 0x20u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t175) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090a1b6); };
            c.w32(11, t175);
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090a1ba); };
            c.cmp(t176, c.r32(11), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t177) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a1c7); };
            c.w32(11, t177);
            let r178 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r178);
            let Some(t179) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a1ce); };
            c.w32(1, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090a1d2); };
            c.cmp(t180, c.r32(1), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t181) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090a1db); };
            c.w32(11, t181);
            let r182 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r182);
            let Some(t183) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a1e2); };
            c.w32(1, t183);
            let Some(t184) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090a1e6); };
            c.cmp(t184, c.r32(1), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 70; continue;
        }
        70 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t185) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a1f9); };
            c.w32(1, t185);
            let r186 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r186);
            let Some(t187) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a200); };
            c.w32(6, t187);
            c.w32(7, c.r64(6).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 76; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w32(11, c.r16(6));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t188) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090a21c); };
            c.cmp(t188, c.r32(1), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 77; continue;
        }
        75 => {
            c.w64(3, c.r64(0));
            blk = 77; continue;
        }
        76 => {
            c.w64(3, c.r64(11));
            blk = 77; continue;
        }
        77 => {
            let Some(t189) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090a233); };
            c.w32(11, t189);
            let r190 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r190);
            let Some(t191) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a23a); };
            c.w32(11, t191);
            let r192 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r192);
            let Some(t193) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a241); };
            c.w32(0, t193);
            let r194 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r194);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 253; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r195 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r195);
            let Some(t196) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090a25a); };
            c.w32(6, t196);
            let r197 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r197);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090a261); }
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090a265); };
            if !c.push64(t198) { return RegfileExit::fault(0x700090a265); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700090a26f, 0x700090a26a);
        }
        79 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a26f); };
            c.w64(8, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a273); };
            c.w64(15, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a277); };
            c.w64(9, t201);
            blk = 99; continue;
        }
        80 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r202 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r202);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090a287); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090a28b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700090a28c); }
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090a28d); };
            if !c.push64(t203) { return RegfileExit::fault(0x700090a28d); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a290); };
            c.w64(6, t204);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090a294); };
            c.w64(3, t205);
            c.w32(0, 0x3u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700090a2a8, 0x700090a2a3);
        }
        81 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a2a8); };
            c.w64(8, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a2ac); };
            c.w64(15, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a2b0); };
            c.w64(9, t208);
            blk = 99; continue;
        }
        82 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r209 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r209);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090a2c2); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090a2c6); };
            if !c.push64(t210) { return RegfileExit::fault(0x700090a2c6); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a2ce); };
            c.w64(6, t211);
            return c.call(0x70007f2274, 0x700090a2d7, 0x700090a2d2);
        }
        83 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a2d7); };
            c.w64(8, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a2db); };
            c.w64(15, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a2df); };
            c.w64(9, t214);
            blk = 99; continue;
        }
        84 => {
            let Some(t215) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a2e8); };
            c.w32(11, t215);
            let r216 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r216);
            let Some(t217) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a2ef); };
            c.w32(1, t217);
            c.cmp(c.r16(1), 0x834u64, 16);
            if c.cc_e() { blk = 98; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w32(6, c.r64(1).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 91; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r16(1), 0x411u64, 16);
            if c.cc_ne() { blk = 253; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t218) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a315); };
            c.w32(11, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090a319); };
            c.cmp(t219, c.r32(11), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 88; continue;
        }
        88 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t220) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a32e); };
            c.cmp(t220, 0x40u64, 16);
            if c.cc_ae() { blk = 253; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t221) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a339); };
            c.w32(11, t221);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 99; continue;
        }
        91 => {
            let Some(t222) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a346); };
            c.w32(2, t222);
            let Some(t223) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090a349); };
            c.cmp(t223, c.r32(2), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t224) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090a356); };
            c.w32(11, t224);
            c.w64(2, c.r64(11));
            let r225 = c.r32(2) & 0xc0u64;
            c.w32(2, r225);
            c.logic(r225, 32);
            c.cmp(c.r32(2), 0xc0u64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t226) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090a36c); };
            c.w32(0, t226);
            let r227 = c.r32(0) & 0x1fu64;
            c.w32(0, r227);
            c.logic(r227, 32);
            let r228 = c.sub(c.r32(0), 0x10u64, 32);
            c.w32(0, r228);
            c.cmp(c.r32(0), 0x4u64, 32);
            if c.cc_a() { blk = 253; continue; }
            blk = 94; continue;
        }
        94 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 253; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t229) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090a388); };
            c.w32(0, t229);
            let r230 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090a38e); };
            c.cmp(t231, c.r32(0), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t232) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a39b); };
            c.w32(11, t232);
            let Some(t233) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090a39f); };
            c.cmp(t233, c.r32(11), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t234) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090a3a8); };
            c.w32(0, t234);
            let r235 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r235);
            blk = 99; continue;
        }
        98 => {
            let Some(t236) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a3b0); };
            c.w32(11, t236);
            let Some(t237) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090a3b4); };
            c.cmp(t237, c.r32(11), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 99; continue;
        }
        99 => {
            let r238 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r238);
            let Some(t239) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a3c5); };
            c.w64(8, t239);
            let Some(t240) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a3c9); };
            c.w32(11, t240);
            let Some(t241) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a3ce); };
            c.w64(1, t241);
            let Some(t242) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a3d2); };
            c.w64(5, t242);
            c.w64(12, c.r64(8));
            let r243 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r243);
            return RegfileExit::jump(c.r64(1));
        }
        100 => {
            blk = 253; continue;
        }
        101 => {
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090a3e4); };
            c.w64(11, t244);
            let Some(t245) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090a3eb); };
            c.cmp(t245, 0x20u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 102; continue;
        }
        102 => {
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t246) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090a3fb); };
            c.w32(11, t246);
            let r247 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r247);
            blk = 105; continue;
        }
        104 => {
            let Some(t248) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090a404); };
            c.w32(11, t248);
            let r249 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r249);
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            return RegfileExit::fault(0x700090a411);
        }
        107 => {
            let r250 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r250);
            c.logic(r250, 64);
            let Some(t251) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a416); };
            c.w32(2, t251);
            let r252 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r252);
            let Some(t253) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090a41c); };
            c.w32(3, t253);
            let r254 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r254);
            let Some(t255) = c.ld64(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a422); };
            c.w64(2, t255);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t256) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a42b); };
            c.w32(0, t256);
            let r257 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r257);
            let Some(t258) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090a431); };
            c.w32(1, t258);
            let r259 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r259);
            blk = 109; continue;
        }
        109 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r260 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r260);
            let r261 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r261);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090a441); }
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090a445); };
            if !c.push64(t262) { return RegfileExit::fault(0x700090a445); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090a448); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(11));
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(0));
            return c.call(0x70007fe8d4, 0x700090a45c, 0x700090a457);
        }
        110 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a45c); };
            c.w64(8, t263);
            let Some(t264) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a460); };
            c.w64(8, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a464); };
            c.w64(9, t265);
            let Some(t266) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a468); };
            c.w32(11, t266);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a46d); };
            c.w64(15, t267);
            let Some(t268) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a471); };
            c.w64(1, t268);
            let Some(t269) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a475); };
            c.w64(5, t269);
            c.w64(12, c.r64(8));
            let r270 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r270);
            return RegfileExit::jump(c.r64(1));
        }
        111 => {
            blk = 115; continue;
        }
        112 => {
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090a484); };
            c.w64(11, t271);
            let Some(t272) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090a48b); };
            c.cmp(t272, 0x20u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            let r273 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r273);
            let r274 = c.r64(6) & 0x3ffu64;
            c.w64(6, r274);
            c.logic(r274, 64);
            let r275 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r275);
            let r276 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r276);
            let Some(t277) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a4ab); };
            c.w32(1, t277);
            let r278 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r278);
            let Some(t279) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090a4b1); };
            c.w32(1, t279);
            let r280 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r280);
            let Some(t281) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090a4b7); };
            c.w32(2, t281);
            let r282 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r282);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090a4be); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090a4c2); }
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(0));
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a4cf); };
            c.w64(6, t283);
            return c.call(0x70007fea14, 0x700090a4d8, 0x700090a4d3);
        }
        114 => {
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a4d8); };
            c.w64(8, t284);
            let Some(t285) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a4dc); };
            c.w64(8, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a4e0); };
            c.w64(9, t286);
            let Some(t287) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a4e4); };
            c.w32(11, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a4e9); };
            c.w64(15, t288);
            let Some(t289) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a4ed); };
            c.w64(1, t289);
            let Some(t290) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a4f1); };
            c.w64(5, t290);
            c.w64(12, c.r64(8));
            let r291 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r291);
            return RegfileExit::jump(c.r64(1));
        }
        115 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r292 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r292);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t293) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090a509); };
            c.w32(3, t293);
            let Some(t294) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a50f); };
            c.w32(2, t294);
            let r295 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r295);
            let Some(t296) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a517); };
            c.w32(3, t296);
            let r297 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r297);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090a51e); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090a522); }
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090a523); };
            if !c.push64(t298) { return RegfileExit::fault(0x700090a523); }
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090a526); };
            if !c.push64(t299) { return RegfileExit::fault(0x700090a526); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090a529); }
            let Some(t300) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x700090a52d); };
            c.w64(3, t300);
            c.w32(0, 0x3u64);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a539); };
            c.w64(6, t301);
            return c.call(0x700086ed74, 0x700090a542, 0x700090a53d);
        }
        116 => {
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a542); };
            c.w64(8, t302);
            let Some(t303) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a546); };
            c.w64(8, t303);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090a54a); };
            c.w64(9, t304);
            let Some(t305) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a54e); };
            c.w32(11, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a553); };
            c.w64(15, t306);
            let Some(t307) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a557); };
            c.w64(1, t307);
            let Some(t308) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a55b); };
            c.w64(5, t308);
            c.w64(12, c.r64(8));
            let r309 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r309);
            return RegfileExit::jump(c.r64(1));
        }
        117 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            let r310 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r310);
            let r311 = c.r64(6) & 0x3ffu64;
            c.w64(6, r311);
            c.logic(r311, 64);
            let r312 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r312);
            let r313 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r313);
            let Some(t314) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a582); };
            c.w32(3, t314);
            let r315 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r315);
            let Some(t316) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090a588); };
            c.w32(3, t316);
            let r317 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r317);
            let Some(t318) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090a58e); };
            c.w32(3, t318);
            let r319 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r319);
            let Some(t320) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a595); };
            c.w32(7, t320);
            let r321 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r321);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090a59b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090a59f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090a5a0); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a5a6); };
            c.w64(6, t322);
            return c.call(0x70007f2274, 0x700090a5af, 0x700090a5aa);
        }
        118 => {
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a5af); };
            c.w64(8, t323);
            let Some(t324) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a5b3); };
            c.w64(8, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a5b7); };
            c.w64(9, t325);
            let Some(t326) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a5bb); };
            c.w32(11, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a5c0); };
            c.w64(15, t327);
            let Some(t328) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a5c4); };
            c.w64(1, t328);
            let Some(t329) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a5c8); };
            c.w64(5, t329);
            c.w64(12, c.r64(8));
            let r330 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r330);
            return RegfileExit::jump(c.r64(1));
        }
        119 => {
            let Some(t331) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a5d5); };
            c.w32(2, t331);
            let r332 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r332);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t333) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700090a5e0); };
            c.w64(2, t333);
            blk = 121; continue;
        }
        121 => {
            let Some(t334) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090a5e7); };
            c.w32(1, t334);
            let Some(t335) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090a5ea); };
            c.w32(6, t335);
            let Some(t336) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a5f0); };
            c.w32(7, t336);
            let r337 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r337);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t338) = c.ld32(c.r64(7).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a5fb); };
            c.w32(6, t338);
            let r339 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r339);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r340 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r340);
            let Some(t341) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a609); };
            c.w32(0, t341);
            let r342 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r342);
            let r343 = c.r64(0) & c.r64(1);
            c.w64(0, r343);
            c.logic(r343, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 122; continue;
        }
        122 => {
            c.w64(8, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(8).wrapping_mul(4));
            let Some(t344) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a622); };
            c.w32(11, t344);
            let Some(t345) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090a627); };
            c.cmp(t345, c.r32(11), 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(6), c.r32(11), 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            let r346 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r346);
            c.w64(8, c.r64(15).wrapping_add(c.r64(0)));
            c.w64(11, c.r64(1));
            let r347 = c.r64(11) & c.r64(8);
            c.w64(11, r347);
            c.logic(r347, 64);
            c.w64(0, c.r64(11));
            blk = 122; continue;
        }
        125 => {
            let Some(t348) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a64c); };
            c.w32(11, t348);
            let Some(t349) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090a650); };
            c.w32(0, t349);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r350 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r350);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            return RegfileExit::fault(0x700090a663);
        }
        127 => {
            let Some(t351) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090a664); };
            c.w32(8, t351);
            let Some(t352) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090a66a); };
            c.w32(11, t352);
            let r353 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r353);
            let Some(t354) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a672); };
            c.w32(8, t354);
            let r355 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r355);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t356) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a682); };
            c.w32(0, t356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090a686); };
            c.cmp(t357, c.r32(0), 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t358) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a693); };
            c.w32(2, t358);
            let r359 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r359);
            let Some(t360) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a69a); };
            c.w32(11, t360);
            let r361 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r361);
            let Some(t362) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090a6a1); };
            c.logic(t362 & 0x2u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t363) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090a6ac); };
            c.cmp(t363, c.r32(11), 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090a6b5); };
            c.w64(11, t364);
            blk = 178; continue;
        }
        132 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090a6c1); };
            c.w64(11, t365);
            let Some(t366) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090a6c8); };
            c.cmp(t366, 0x20u64, 8);
            if c.cc_e() { blk = 149; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t367) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090a6d2); };
            c.w32(8, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090a6d6); };
            c.cmp(t368, c.r32(8), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a6df); };
            c.w64(8, t369);
            blk = 253; continue;
        }
        135 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090a6e8); };
            c.w64(0, t370);
            let Some(t371) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a6ec); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a6f3); };
            c.w32(11, t373);
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090a6f7); };
            c.cmp(t374, c.r32(11), 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t375) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090a700); };
            c.w32(8, t375);
            let r376 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r376);
            let Some(t377) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a707); };
            c.w32(11, t377);
            let Some(t378) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090a70b); };
            c.cmp(t378, c.r32(11), 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 137; continue;
        }
        137 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t379) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a71e); };
            c.w32(11, t379);
            let r380 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r380);
            let Some(t381) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a725); };
            c.w32(3, t381);
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 143; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w32(8, c.r16(3));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t382) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090a742); };
            c.cmp(t382, c.r32(11), 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a74b); };
            c.w64(8, t383);
            blk = 253; continue;
        }
        142 => {
            let Some(t384) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090a754); };
            c.w32(6, t384);
            let r385 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r385);
            c.w64(0, c.r64(6));
            blk = 144; continue;
        }
        143 => {
            c.w64(0, c.r64(8));
            blk = 144; continue;
        }
        144 => {
            let Some(t386) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090a762); };
            c.w32(8, t386);
            let r387 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a769); };
            c.w32(8, t388);
            let r389 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r389);
            let Some(t390) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a770); };
            c.w32(11, t390);
            let r391 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r391);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 147; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r392 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r392);
            let Some(t393) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090a789); };
            c.w32(6, t393);
            let r394 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a790); };
            c.w64(8, t395);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090a794); }
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090a798); };
            if !c.push64(t396) { return RegfileExit::fault(0x700090a798); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700090a7a2, 0x700090a79d);
        }
        146 => {
            c.w64(11, c.r64(0));
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a7a5); };
            c.w64(9, t397);
            blk = 178; continue;
        }
        147 => {
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a7ae); };
            c.w64(8, t398);
            blk = 253; continue;
        }
        148 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a7b7); };
            c.w64(8, t399);
            blk = 253; continue;
        }
        149 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r400 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a7c9); };
            c.w64(3, t401);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090a7cd); }
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090a7d1); };
            if !c.push64(t402) { return RegfileExit::fault(0x700090a7d1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090a7d4); }
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090a7d6); };
            if !c.push64(t403) { return RegfileExit::fault(0x700090a7d6); }
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a7d9); };
            c.w64(6, t404);
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090a7dd); };
            c.w64(3, t405);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700090a7f1, 0x700090a7ec);
        }
        150 => {
            c.w64(11, c.r64(0));
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a7f4); };
            c.w64(9, t406);
            blk = 178; continue;
        }
        151 => {
            c.w64(8, c.r64(9));
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r407 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r407);
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a807); };
            c.w64(0, t408);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090a80b); }
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090a80f); };
            if !c.push64(t409) { return RegfileExit::fault(0x700090a80f); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a817); };
            c.w64(6, t410);
            return c.call(0x70007f2274, 0x700090a820, 0x700090a81b);
        }
        152 => {
            c.w64(11, c.r64(0));
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090a823); };
            c.w64(9, t411);
            blk = 178; continue;
        }
        153 => {
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090a82c); };
            c.w64(0, t412);
            let Some(t413) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a830); };
            c.w32(8, t413);
            let r414 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r414);
            let Some(t415) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a837); };
            c.w32(3, t415);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 175; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w32(2, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 164; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a859); };
            c.w64(8, t416);
            blk = 253; continue;
        }
        157 => {
            let Some(t417) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a862); };
            c.w32(8, t417);
            let Some(t418) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090a866); };
            c.cmp(t418, c.r32(8), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a86f); };
            c.w64(8, t419);
            blk = 253; continue;
        }
        159 => {
            let Some(t420) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090a878); };
            c.w32(8, t420);
            let r421 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r421);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a885); };
            c.w64(8, t422);
            blk = 253; continue;
        }
        161 => {
            let Some(t423) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a88e); };
            c.w32(11, t423);
            let r424 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r424);
            let Some(t425) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a895); };
            c.cmp(t425, 0x40u64, 16);
            if c.cc_b() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a89d); };
            c.w64(8, t426);
            blk = 253; continue;
        }
        163 => {
            let Some(t427) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a8a6); };
            c.w32(8, t427);
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            blk = 178; continue;
        }
        164 => {
            let Some(t428) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a8b3); };
            c.w32(11, t428);
            let Some(t429) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090a8b7); };
            c.cmp(t429, c.r32(11), 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a8c0); };
            c.w64(8, t430);
            blk = 253; continue;
        }
        166 => {
            let Some(t431) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090a8c9); };
            c.w32(8, t431);
            c.w64(11, c.r64(8));
            let r432 = c.r32(11) & 0xc0u64;
            c.w32(11, r432);
            c.logic(r432, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t433) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090a8e1); };
            c.w32(11, t433);
            let r434 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r434);
            let Some(t435) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090a8e8); };
            c.w32(11, t435);
            let r436 = c.r32(11) & 0x1fu64;
            c.w32(11, r436);
            c.logic(r436, 32);
            let r437 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r437);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a8fa); };
            c.w64(8, t438);
            blk = 253; continue;
        }
        169 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 174; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t439) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090a909); };
            c.w32(11, t439);
            let r440 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r440);
            let Some(t441) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090a910); };
            c.cmp(t441, c.r32(11), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t442) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090a919); };
            c.w32(8, t442);
            let Some(t443) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090a91d); };
            c.cmp(t443, c.r32(8), 32);
            if c.cc_ne() { blk = 178; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t444) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090a926); };
            c.w32(11, t444);
            let r445 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r445);
            blk = 178; continue;
        }
        173 => {
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a92f); };
            c.w64(8, t446);
            blk = 253; continue;
        }
        174 => {
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a938); };
            c.w64(8, t447);
            blk = 253; continue;
        }
        175 => {
            let Some(t448) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090a941); };
            c.w32(8, t448);
            let Some(t449) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090a945); };
            c.cmp(t449, c.r32(8), 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a94e); };
            c.w64(8, t450);
            blk = 253; continue;
        }
        177 => {
            let Some(t451) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090a957); };
            c.w32(11, t451);
            let r452 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r452);
            blk = 178; continue;
        }
        178 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a965); };
            c.w64(0, t453);
            let Some(t454) = c.ld64(c.r64(0).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a969); };
            c.w64(8, t454);
            let Some(t455) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a96d); };
            c.w32(0, t455);
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a972); };
            c.w64(15, t456);
            let Some(t457) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a976); };
            c.w64(1, t457);
            let Some(t458) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a97a); };
            c.w64(5, t458);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r459 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r459);
            return RegfileExit::jump(c.r64(1));
        }
        179 => {
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a98a); };
            c.w64(8, t460);
            blk = 253; continue;
        }
        180 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r461 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r461);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t462) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a99e); };
            c.w64(3, t462);
            let Some(t463) = c.ld16(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090a9a2); };
            c.w32(2, t463);
            let Some(t464) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a9a8); };
            c.w32(3, t464);
            let r465 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r465);
            let Some(t466) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090a9ae); };
            c.w32(3, t466);
            let r467 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r467);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090a9b5); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090a9b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090a9ba); }
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090a9be); };
            c.w64(6, t468);
            let Some(t469) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700090a9c2); };
            c.w64(3, t469);
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700090a9d6, 0x700090a9d1);
        }
        181 => {
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a9d6); };
            c.w64(8, t470);
            let Some(t471) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090a9da); };
            c.w64(8, t471);
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090a9de); };
            c.w64(9, t472);
            let Some(t473) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090a9e2); };
            c.w32(11, t473);
            let Some(t474) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090a9e7); };
            c.w64(15, t474);
            let Some(t475) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090a9eb); };
            c.w64(1, t475);
            let Some(t476) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090a9ef); };
            c.w64(5, t476);
            c.w64(12, c.r64(8));
            let r477 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r477);
            return RegfileExit::jump(c.r64(1));
        }
        182 => {
            let r478 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r478);
            let Some(t479) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090aa00); };
            c.w32(8, t479);
            let Some(t480) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090aa05); };
            c.w64(1, t480);
            let Some(t481) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090aa09); };
            c.w64(5, t481);
            c.w64(0, c.r64(3));
            let r482 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r482);
            return RegfileExit::jump(c.r64(1));
        }
        183 => {
            c.logic(c.r8(6) & 0x40u64, 8);
            if c.cc_ne() { blk = 197; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(11, c.r64(6));
            if let Some(sr) = c.shr(c.r64(11), 0x9u64, 64) { c.w64(11, sr); }
            c.logic(c.r16(6) & 0x100u64, 16);
            c.w8(0, (c.cc_ne() as u64));
            c.w32(0, c.r8(0));
            let r483 = c.r64(11) & 0x7ffu64;
            c.w64(11, r483);
            c.logic(r483, 64);
            c.logic(c.r8(6) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 192; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t484) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090aa3f); };
            c.w32(3, t484);
            let r485 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r485);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t486) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090aa4a); };
            c.w64(3, t486);
            blk = 187; continue;
        }
        187 => {
            let Some(t487) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090aa51); };
            c.w32(11, t487);
            let r488 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r488);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            let r489 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r489);
            let Some(t490) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090aa62); };
            c.w32(8, t490);
            let Some(t491) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090aa67); };
            c.w64(1, t491);
            let Some(t492) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090aa6b); };
            c.w64(5, t492);
            c.w64(0, c.r64(11));
            let r493 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r493);
            return RegfileExit::jump(c.r64(1));
        }
        189 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t494) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090aa82); };
            c.w32(0, t494);
            let Some(t495) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090aa86); };
            c.cmp(t495, c.r32(0), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 191; continue;
        }
        191 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700090aa93); }
            blk = 245; continue;
        }
        192 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            let r496 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r496);
            let Some(t497) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090aaa7); };
            c.w32(8, t497);
            let Some(t498) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090aaac); };
            c.w32(0, t498);
            let r499 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r499);
            let Some(t500) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090aab4); };
            c.w64(1, t500);
            let Some(t501) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090aab8); };
            c.w64(5, t501);
            let r502 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r502);
            return RegfileExit::jump(c.r64(1));
        }
        194 => {
            let Some(t503) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090aac2); };
            c.w32(11, t503);
            let r504 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r504);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t505) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090aad4); };
            c.w32(0, t505);
            let Some(t506) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090aad8); };
            c.cmp(t506, c.r32(0), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 196; continue;
        }
        196 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700090aae5); }
            blk = 245; continue;
        }
        197 => {
            c.w64(11, c.r64(6));
            let r507 = c.shr(c.r32(11), 0x7u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r507);
            let r508 = c.r32(11) & 0xfu64;
            c.w32(11, r508);
            c.logic(r508, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_l() { blk = 204; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_l() { blk = 202; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 250; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_e() { blk = 250; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 203; continue;
        }
        203 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 208; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 210; continue;
        }
        210 => {
            return RegfileExit::fault(0x700090ab69);
        }
        211 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r509 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r509);
            let r510 = c.r64(6) & 0xfffffu64;
            c.w64(6, r510);
            c.logic(r510, 64);
            let Some(t511) = c.ld8(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ab79); };
            c.w32(8, (t511 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t512) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ab83); };
            c.w32(8, t512);
            let Some(t513) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ab88); };
            c.w64(1, t513);
            let Some(t514) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ab8c); };
            c.w64(5, t514);
            let r515 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r515);
            return RegfileExit::jump(c.r64(1));
        }
        212 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r516 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r516);
            let r517 = c.r64(6) & 0xfffffu64;
            c.w64(6, r517);
            c.logic(r517, 64);
            let Some(t518) = c.ld16(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090aba5); };
            c.w32(8, (t518 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t519) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090abaf); };
            c.w32(8, t519);
            let Some(t520) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090abb4); };
            c.w64(1, t520);
            let Some(t521) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090abb8); };
            c.w64(5, t521);
            let r522 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r522);
            return RegfileExit::jump(c.r64(1));
        }
        213 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r523 = c.r64(6) & 0xfffffu64;
            c.w64(6, r523);
            c.logic(r523, 64);
            let Some(t524) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090abcd); };
            c.w32(8, t524);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090abd2); }
            c.w64(11, c.r64(8));
            let r525 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r525);
            if c.cc_o() { blk = 215; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 220; continue;
        }
        215 => {
            let Some(t526) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090abe6); };
            c.w64(11, t526);
            let Some(t527) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090abed); };
            c.w64(0, t527);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090abf4); };
            c.w64(2, t528);
            let Some(t529) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090abfb); };
            c.cmp(t529, c.r64(3), 64);
            if c.cc_a() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090ac0a, 0x700090ac05);
        }
        217 => {
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ac0a); };
            c.w64(8, t530);
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ac0e); };
            c.w64(15, t531);
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ac12); };
            c.w64(9, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090ac16); };
            c.w64(12, t533);
            blk = 219; continue;
        }
        218 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090ac1c); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 219; continue;
        }
        219 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090ac2f); };
            c.w64(8, t534);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090ac36); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090ac3a); }
            blk = 220; continue;
        }
        220 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t535) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ac46); };
            c.w32(8, t535);
            let Some(t536) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ac4b); };
            c.w64(1, t536);
            let Some(t537) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ac4f); };
            c.w64(5, t537);
            let r538 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r538);
            return RegfileExit::jump(c.r64(1));
        }
        221 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r539 = c.r64(6) & 0xfffffu64;
            c.w64(6, r539);
            c.logic(r539, 64);
            let Some(t540) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ac64); };
            c.w32(8, t540);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090ac69); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            let r541 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r541);
            blk = 228; continue;
        }
        223 => {
            let Some(t542) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090ac7b); };
            c.w64(11, t542);
            let Some(t543) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090ac82); };
            c.w64(0, t543);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t544) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090ac89); };
            c.w64(2, t544);
            let Some(t545) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090ac90); };
            c.cmp(t545, c.r64(3), 64);
            if c.cc_a() { blk = 226; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090ac9f, 0x700090ac9a);
        }
        225 => {
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ac9f); };
            c.w64(8, t546);
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090aca3); };
            c.w64(15, t547);
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090aca7); };
            c.w64(9, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090acab); };
            c.w64(12, t549);
            blk = 227; continue;
        }
        226 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090acb1); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 227; continue;
        }
        227 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t550) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090acc7); };
            c.w64(8, t550);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090acce); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090acd2); }
            c.w64(8, c.r64(0));
            blk = 228; continue;
        }
        228 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t551) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ace1); };
            c.w32(11, t551);
            let Some(t552) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ace6); };
            c.w64(1, t552);
            let Some(t553) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090acea); };
            c.w64(5, t553);
            c.w64(0, c.r64(8));
            let r554 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r554);
            return RegfileExit::jump(c.r64(1));
        }
        229 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r555 = c.r64(6) & 0xfffffu64;
            c.w64(6, r555);
            c.logic(r555, 64);
            let Some(t556) = c.ld64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ad02); };
            c.w64(8, t556);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090ad07); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 237; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t557) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090ad11); };
            c.w64(11, t557);
            let Some(t558) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090ad18); };
            c.w64(0, t558);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090ad1f); };
            c.w64(2, t559);
            let Some(t560) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090ad26); };
            c.cmp(t560, c.r64(3), 64);
            if c.cc_a() { blk = 233; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700090ad35, 0x700090ad30);
        }
        232 => {
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ad35); };
            c.w64(8, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ad39); };
            c.w64(15, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ad3d); };
            c.w64(9, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090ad41); };
            c.w64(12, t564);
            blk = 234; continue;
        }
        233 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090ad47); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 234; continue;
        }
        234 => {
            let Some(t565) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090ad51); };
            c.w64(3, t565);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700090ad58); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700090ad61); }
            let r566 = c.neg(c.r64(8), 64);
            c.w64(8, r566);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700090ad6b); }
            blk = 242; continue;
        }
        236 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700090ad71); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700090ad78); }
            blk = 242; continue;
        }
        237 => {
            let Some(t567) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090ad7e); };
            c.w64(8, t567);
            let Some(t568) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090ad85); };
            c.w64(11, t568);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t569) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090ad8c); };
            c.w64(0, t569);
            let Some(t570) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700090ad93); };
            c.cmp(t570, c.r64(3), 64);
            if c.cc_a() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700090ada2, 0x700090ad9d);
        }
        239 => {
            blk = 241; continue;
        }
        240 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x700090ada4); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 241; continue;
        }
        241 => {
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090adae); };
            c.w64(8, t571);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090adb5); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700090adb9); }
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090adc0); };
            c.w64(12, t572);
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090adc4); };
            c.w64(15, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090adc8); };
            c.w64(9, t574);
            blk = 242; continue;
        }
        242 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t575) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090add3); };
            c.w32(8, t575);
            let Some(t576) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090add8); };
            c.w64(1, t576);
            let Some(t577) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090addc); };
            c.w64(5, t577);
            let r578 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r578);
            return RegfileExit::jump(c.r64(1));
        }
        243 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r579 = c.r64(6) & 0xfffffu64;
            c.w64(6, r579);
            c.logic(r579, 64);
            if !c.movss_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090adf1); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 245; continue;
        }
        244 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r580 = c.r64(6) & 0xfffffu64;
            c.w64(6, r580);
            c.logic(r580, 64);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090ae08); }
            blk = 245; continue;
        }
        245 => {
            let Some(t581) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090ae0e); };
            c.w64(8, t581);
            let Some(t582) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090ae15); };
            c.w64(11, t582);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700090ae1c); }
            let Some(t583) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090ae21); };
            c.w64(3, t583);
            let Some(t584) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700090ae28); };
            c.cmp(t584, c.r64(0), 64);
            if c.cc_a() { blk = 248; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090ae37, 0x700090ae32);
        }
        247 => {
            blk = 249; continue;
        }
        248 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700090ae39); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 249; continue;
        }
        249 => {
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ae43); };
            c.w64(11, t585);
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t586) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090ae4b); };
            c.w64(8, t586);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090ae52); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700090ae56); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090ae5b); }
            let Some(t587) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090ae60); };
            c.w64(12, t587);
            let Some(t588) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ae64); };
            c.w32(8, t588);
            let Some(t589) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ae69); };
            c.w64(15, t589);
            let Some(t590) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ae6d); };
            c.w64(1, t590);
            let Some(t591) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ae71); };
            c.w64(5, t591);
            let r592 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r592);
            return RegfileExit::jump(c.r64(1));
        }
        250 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r593 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r593);
            let r594 = c.r64(6) & 0xfffffu64;
            c.w64(6, r594);
            c.logic(r594, 64);
            let Some(t595) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ae8a); };
            c.w32(8, t595);
            let Some(t596) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ae8f); };
            c.w32(0, t596);
            let r597 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r597);
            let Some(t598) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ae96); };
            c.w64(1, t598);
            let Some(t599) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ae9a); };
            c.w64(5, t599);
            let r600 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r600);
            return RegfileExit::jump(c.r64(1));
        }
        251 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r601 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r601);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090aeab); }
            let Some(t602) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700090aeaf); };
            c.w64(8, t602);
            let Some(t603) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090aeb6); };
            c.w32(8, t603);
            let r604 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r604);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090aebd); }
            let Some(t605) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700090aebf); };
            c.w64(3, t605);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700090aed2, 0x700090aecd);
        }
        252 => {
            return RegfileExit::fault(0x700090aed2);
        }
        253 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r606 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r606);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t607) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090aede); };
            c.w32(3, t607);
            let Some(t608) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090aee4); };
            c.w32(2, t608);
            let r609 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r609);
            let Some(t610) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090aeec); };
            c.w64(1, t610);
            let Some(t611) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090aef0); };
            c.w32(3, t611);
            let r612 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r612);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090aef7); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090aefb); }
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090aefc); };
            if !c.push64(t613) { return RegfileExit::fault(0x700090aefc); }
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090aeff); };
            if !c.push64(t614) { return RegfileExit::fault(0x700090aeff); }
            c.w32(11, 0x2u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700090af08); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090af0a); }
            let Some(t615) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x700090af0e); };
            c.w64(3, t615);
            c.w64(6, c.r64(1));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700090af25, 0x700090af20);
        }
        254 => {
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090af25); };
            c.w64(8, t616);
            let Some(t617) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090af29); };
            c.w64(8, t617);
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090af2d); };
            c.w64(9, t618);
            let Some(t619) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090af31); };
            c.w32(11, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090af36); };
            c.w64(15, t620);
            let Some(t621) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090af3a); };
            c.w64(1, t621);
            let Some(t622) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090af3e); };
            c.w64(5, t622);
            c.w64(12, c.r64(8));
            let r623 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r623);
            return RegfileExit::jump(c.r64(1));
        }
        255 => {
            return RegfileExit::fault(0x700090af4b);
        }
        256 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090af4c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090af50); }
            let r624 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r624);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r625 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r625);
            let Some(t626) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090af5d); };
            c.w32(0, t626);
            let Some(t627) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090af63); };
            c.w32(3, t627);
            let r628 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r628);
            let Some(t629) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090af6b); };
            c.w64(2, t629);
            let Some(t630) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090af6f); };
            c.w32(1, t630);
            let r631 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r631);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090af76); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090af7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090af7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090af82); }
            c.w32(0, 0xcu64);
            c.w64(6, c.r64(2));
            return c.call(0x7000804df4, 0x700090af93, 0x700090af8e);
        }
        257 => {
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090af93); };
            c.w64(8, t632);
            let Some(t633) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090af97); };
            c.w64(8, t633);
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090af9b); };
            c.w64(11, t634);
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t635) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090afa3); };
            c.w32(11, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090afa8); };
            c.w64(15, t636);
            let Some(t637) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090afac); };
            c.w64(1, t637);
            let Some(t638) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090afb0); };
            c.w64(5, t638);
            c.w64(12, c.r64(8));
            let r639 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r639);
            return RegfileExit::jump(c.r64(1));
        }
        258 => {
            return RegfileExit::jump(0x700090afc0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090afd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090afd7); };
            c.w64(11, t0);
            let Some(t1) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090afdb); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090afe2); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090afe9); };
            c.w32(0, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090afed); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090aff6); };
            c.w64(11, t7);
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090affd); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090b004); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b011); };
            c.w32(3, t10);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(2, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(c.r64(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090b01e); };
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
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090b038); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090b03e); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            let r17 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r17);
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090b04f); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b054); };
            c.w64(1, t19);
            return RegfileExit::jump(c.r64(1));
        }
        7 => {
            let Some(t20) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090b05a); };
            c.w64(8, t20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b05e); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090b065); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b06c); };
            c.w32(11, t25);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x700090b07f);
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x700090b080); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b08d); };
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
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090b0b1); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b0b9); };
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
            return RegfileExit::fault(0x700090b0d3);
        }
        12 => {
            let r34 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r34);
            let Some(t35) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090b0d8); };
            c.w32(0, t35);
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b0dd); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b0e5); };
            c.w64(1, t38);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090b0ee); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090b0f2); }
            let r39 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090b0f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090b0fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700090b100); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700090b104); }
            blk = 251; continue;
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090b10d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090b111); }
            let r40 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(c.r64(2)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090b117); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090b11f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090b123); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090b127); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700090b12b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700090b12f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700090b133); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700090b137); };
            c.cmp(t43, c.r32(7), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t44) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090b144); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b148); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090b14e); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b154); };
            c.w32(6, t49);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x700090b15d); }
            c.cmp(c.r64(6), 0x3u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700090b167);
        }
        17 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090b168); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700090b16f); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700090b179);
        }
        19 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090b17a); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b18a); };
            c.w32(6, t54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700090b18d); };
            c.cmp(t55, c.r32(6), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b196); };
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
            let Some(t58) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b1a2); };
            c.cmp(t58, c.r32(3), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b1ab); };
            c.w32(3, t59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090b1ae); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700090b1ba); };
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
            let r64 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r64);
            let Some(t65) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090b1e7); };
            c.w32(8, t65);
            let Some(t66) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b1ec); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b1f0); };
            c.w64(5, t67);
            c.w64(0, c.r64(6));
            let r68 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r68);
            return RegfileExit::jump(c.r64(1));
        }
        30 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r69 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r69);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t70) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090b208); };
            c.w32(6, t70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b20e); };
            c.w32(9, t71);
            let r72 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r72);
            let Some(t73) = c.ld32(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b216); };
            c.w32(6, t73);
            let r74 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r74);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090b21e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090b222); }
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(10));
            c.w64(0, c.r64(3));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090b23b); };
            c.w64(3, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090b23f); };
            c.logic(t76 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let r77 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r77);
            blk = 33; continue;
        }
        32 => {
            let Some(t78) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090b24e); };
            c.w32(7, t78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700090b251); };
            c.w64(7, t79);
            blk = 33; continue;
        }
        33 => {
            return c.call(c.r64(7), 0x700090b25b, 0x700090b259);
        }
        34 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b25b); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b25f); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090b263); };
            c.w64(9, t82);
            let Some(t83) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b267); };
            c.w32(11, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b26c); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b270); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b274); };
            c.w64(5, t86);
            c.w64(12, c.r64(8));
            let r87 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r87);
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
            let r88 = c.r64(2) & 0xfu64;
            c.w64(2, r88);
            c.logic(r88, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700090b294); }
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
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r89 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r89);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t90) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090b323); };
            c.w32(2, t90);
            let Some(t91) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b329); };
            c.w32(1, t91);
            let r92 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r92);
            let Some(t93) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b331); };
            c.w32(2, t93);
            let r94 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r94);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090b338); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090b33c); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090b33d); };
            if !c.push64(t95) { return RegfileExit::fault(0x700090b33d); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090b340); }
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090b341); };
            if !c.push64(t96) { return RegfileExit::fault(0x700090b341); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090b344); };
            if !c.push64(t97) { return RegfileExit::fault(0x700090b344); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700090b347); }
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700090b34b); };
            c.w64(3, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090b352); };
            c.w64(6, t99);
            c.w32(0, 0x5u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700090b363, 0x700090b35e);
        }
        49 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b363); };
            c.w64(8, t100);
            let Some(t101) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b367); };
            c.w64(8, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700090b36b); };
            c.w64(9, t102);
            let Some(t103) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b36f); };
            c.w32(11, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b374); };
            c.w64(15, t104);
            let Some(t105) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b378); };
            c.w64(1, t105);
            let Some(t106) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b37c); };
            c.w64(5, t106);
            c.w64(12, c.r64(8));
            let r107 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r107);
            return RegfileExit::jump(c.r64(1));
        }
        50 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090b389); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            let Some(t110) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b390); };
            c.w32(11, t110);
            let r111 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r111);
            let Some(t112) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b397); };
            c.w32(0, t112);
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r113 = c.r64(6) & 0x1ffffffu64;
            c.w64(6, r113);
            c.logic(r113, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_b() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700090b3b1);
        }
        52 => {
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b3b2); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b3ba); };
            c.w32(0, t116);
            let r117 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r117);
            c.w64(11, c.r64(9).wrapping_add(0x5));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700090b3c5); }
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090b3c9); };
            c.cmp(t118, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t119) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090b3d2); };
            c.w32(8, t119);
            let Some(t120) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b3d7); };
            c.w64(1, t120);
            let Some(t121) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b3db); };
            c.w64(5, t121);
            c.w64(9, c.r64(11));
            let r122 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r122);
            return RegfileExit::jump(c.r64(1));
        }
        54 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r123 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r123);
            let Some(t124) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090b3ee); };
            c.w32(3, t124);
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b3f4); };
            c.w32(2, t125);
            let r126 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r126);
            let Some(t127) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b3fc); };
            c.w32(3, t127);
            let r128 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r128);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090b403); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090b40c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090b40d); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700090b40e); };
            c.w64(3, t129);
            c.w32(0, 0x2u64);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090b41a); };
            c.w64(6, t130);
            return c.call(0x700086ed74, 0x700090b423, 0x700090b41e);
        }
        55 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b423); };
            c.w64(8, t131);
            let Some(t132) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b427); };
            c.w64(8, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090b42b); };
            c.w64(9, t133);
            let Some(t134) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b42f); };
            c.w32(11, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b434); };
            c.w64(15, t135);
            let Some(t136) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b438); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b43c); };
            c.w64(5, t137);
            c.w64(12, c.r64(8));
            let r138 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r138);
            return RegfileExit::jump(c.r64(1));
        }
        56 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r139 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r139);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t140) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090b454); };
            c.w32(2, t140);
            let Some(t141) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b45a); };
            c.w32(1, t141);
            let r142 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r142);
            let Some(t143) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b462); };
            c.w32(2, t143);
            let r144 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r144);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090b469); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700090b46d); }
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090b471); };
            c.w64(6, t145);
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(2));
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090b47b); };
            c.w64(1, t146);
            c.w32(2, 0x2u64);
            c.w64(11, c.r64(2));
            return c.call(0x70008bc174, 0x700090b48c, 0x700090b487);
        }
        57 => {
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b48c); };
            c.w64(8, t147);
            let Some(t148) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b490); };
            c.w64(8, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090b494); };
            c.w64(9, t149);
            let Some(t150) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b498); };
            c.w32(11, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b49d); };
            c.w64(15, t151);
            let Some(t152) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b4a1); };
            c.w64(1, t152);
            let Some(t153) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b4a5); };
            c.w64(5, t153);
            c.w64(12, c.r64(8));
            let r154 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r154);
            return RegfileExit::jump(c.r64(1));
        }
        58 => {
            blk = 114; continue;
        }
        59 => {
            let Some(t155) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090b4b7); };
            c.w32(11, t155);
            let r156 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r156);
            let Some(t157) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b4be); };
            c.w32(2, t157);
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090b4c1); };
            c.cmp(t158, c.r32(11), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t159) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090b4ce); };
            c.w32(1, t159);
            let Some(t160) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b4d4); };
            c.w32(6, t160);
            let r161 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r161);
            let Some(t162) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b4dc); };
            c.w32(1, t162);
            let r163 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r163);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t164) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b4ec); };
            c.w32(2, t164);
            let r165 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r165);
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090b4f3); };
            c.cmp(t166, c.r32(2), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t167) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b500); };
            c.w32(2, t167);
            let r168 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r168);
            let Some(t169) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b507); };
            c.w32(6, t169);
            let r170 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r170);
            let Some(t171) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090b50d); };
            c.logic(t171 & 0x2u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090b517); };
            c.cmp(t172, c.r32(6), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(11, c.r64(0));
            blk = 98; continue;
        }
        65 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090b528); };
            c.w64(0, t173);
            let Some(t174) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x700090b52f); };
            c.cmp(t174, 0x20u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t175) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090b538); };
            c.w32(0, t175);
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090b53b); };
            c.cmp(t176, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t177) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b548); };
            c.w32(0, t177);
            let r178 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r178);
            let Some(t179) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b54e); };
            c.w32(1, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090b551); };
            c.cmp(t180, c.r32(1), 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t181) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090b55a); };
            c.w32(0, t181);
            let r182 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r182);
            let Some(t183) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b560); };
            c.w32(1, t183);
            let Some(t184) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090b563); };
            c.cmp(t184, c.r32(1), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t185) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b574); };
            c.w32(1, t185);
            let r186 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r186);
            let Some(t187) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b57a); };
            c.w32(6, t187);
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
            let Some(t188) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090b593); };
            c.cmp(t188, c.r32(1), 32);
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
            let Some(t189) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090b5aa); };
            c.w32(11, t189);
            let r190 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r190);
            let Some(t191) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b5b1); };
            c.w32(11, t191);
            let r192 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r192);
            let Some(t193) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b5b8); };
            c.w32(0, t193);
            let r194 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r194);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 251; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r195 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r195);
            let Some(t196) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090b5d1); };
            c.w32(6, t196);
            let r197 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r197);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090b5d8); }
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090b5dc); };
            if !c.push64(t198) { return RegfileExit::fault(0x700090b5dc); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700090b5e6, 0x700090b5e1);
        }
        78 => {
            c.w64(11, c.r64(0));
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b5e9); };
            c.w64(15, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090b5ed); };
            c.w64(9, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b5f1); };
            c.w64(8, t201);
            blk = 98; continue;
        }
        79 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r202 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r202);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090b601); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090b605); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700090b606); }
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090b607); };
            if !c.push64(t203) { return RegfileExit::fault(0x700090b607); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090b60a); };
            c.w64(6, t204);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090b60e); };
            c.w64(3, t205);
            c.w32(0, 0x3u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700090b622, 0x700090b61d);
        }
        80 => {
            c.w64(11, c.r64(0));
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b625); };
            c.w64(15, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090b629); };
            c.w64(9, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b62d); };
            c.w64(8, t208);
            blk = 98; continue;
        }
        81 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r209 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r209);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090b63f); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090b643); };
            if !c.push64(t210) { return RegfileExit::fault(0x700090b643); }
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090b646); };
            c.w64(6, t211);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x700090b657, 0x700090b652);
        }
        82 => {
            c.w64(11, c.r64(0));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b65a); };
            c.w64(15, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090b65e); };
            c.w64(9, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b662); };
            c.w64(8, t214);
            blk = 98; continue;
        }
        83 => {
            let Some(t215) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b66b); };
            c.w32(0, t215);
            let r216 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r216);
            let Some(t217) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b671); };
            c.w32(1, t217);
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
            let Some(t218) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b696); };
            c.w32(0, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090b69a); };
            c.cmp(t219, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 87; continue;
        }
        87 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t220) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b6b1); };
            c.cmp(t220, 0x40u64, 16);
            if c.cc_ae() { blk = 251; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t221) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b6bc); };
            c.w32(11, t221);
            let r222 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r222);
            blk = 98; continue;
        }
        90 => {
            let Some(t223) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b6c8); };
            c.w32(2, t223);
            let Some(t224) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090b6cc); };
            c.cmp(t224, c.r32(2), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t225) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090b6d9); };
            c.w32(0, t225);
            c.w64(2, c.r64(0));
            let r226 = c.r32(2) & 0xc0u64;
            c.w32(2, r226);
            c.logic(r226, 32);
            c.cmp(c.r32(2), 0xc0u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t227) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090b6ee); };
            c.w32(11, t227);
            let r228 = c.r32(11) & 0x1fu64;
            c.w32(11, r228);
            c.logic(r228, 32);
            let r229 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r229);
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
            let Some(t230) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090b70c); };
            c.w32(11, t230);
            let r231 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r231);
            let Some(t232) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090b713); };
            c.cmp(t232, c.r32(11), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t233) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b720); };
            c.w32(0, t233);
            let Some(t234) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090b724); };
            c.cmp(t234, c.r32(0), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t235) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090b72d); };
            c.w32(11, t235);
            let r236 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r236);
            blk = 98; continue;
        }
        97 => {
            let Some(t237) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b736); };
            c.w32(0, t237);
            let Some(t238) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090b73a); };
            c.cmp(t238, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 98; continue;
        }
        98 => {
            let r239 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r239);
            let Some(t240) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b74b); };
            c.w64(8, t240);
            let Some(t241) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b74f); };
            c.w32(0, t241);
            let Some(t242) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b754); };
            c.w64(1, t242);
            let Some(t243) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b758); };
            c.w64(5, t243);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r244 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r244);
            return RegfileExit::jump(c.r64(1));
        }
        99 => {
            blk = 251; continue;
        }
        100 => {
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090b76d); };
            c.w64(0, t245);
            let Some(t246) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x700090b774); };
            c.cmp(t246, 0x20u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 101; continue;
        }
        101 => {
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t247) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090b783); };
            c.w32(0, t247);
            let r248 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r248);
            blk = 104; continue;
        }
        103 => {
            let Some(t249) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090b78b); };
            c.w32(0, t249);
            let r250 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r250);
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            return RegfileExit::fault(0x700090b796);
        }
        106 => {
            let r251 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r251);
            c.logic(r251, 64);
            let Some(t252) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b79b); };
            c.w32(2, t252);
            let r253 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r253);
            let Some(t254) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090b7a1); };
            c.w32(3, t254);
            let r255 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r255);
            let Some(t256) = c.ld64(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b7a7); };
            c.w64(2, t256);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t257) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b7b1); };
            c.w32(11, t257);
            let r258 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r258);
            let Some(t259) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090b7b8); };
            c.w32(1, t259);
            let r260 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r260);
            blk = 108; continue;
        }
        108 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r261 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r261);
            let r262 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r262);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090b7ca); }
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090b7ce); };
            if !c.push64(t263) { return RegfileExit::fault(0x700090b7ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090b7d1); }
            c.w64(6, c.r64(0));
            c.w64(7, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            return c.call(0x70007fe8d4, 0x700090b7e5, 0x700090b7e0);
        }
        109 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b7e5); };
            c.w64(8, t264);
            let Some(t265) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b7e9); };
            c.w64(8, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090b7ed); };
            c.w64(9, t266);
            let Some(t267) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b7f1); };
            c.w32(11, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b7f6); };
            c.w64(15, t268);
            let Some(t269) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b7fa); };
            c.w64(1, t269);
            let Some(t270) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b7fe); };
            c.w64(5, t270);
            c.w64(12, c.r64(8));
            let r271 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r271);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 114; continue;
        }
        111 => {
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090b80d); };
            c.w64(0, t272);
            let Some(t273) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x700090b814); };
            c.cmp(t273, 0x20u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            let r274 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r274);
            let r275 = c.r64(6) & 0x3ffu64;
            c.w64(6, r275);
            c.logic(r275, 64);
            let r276 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r276);
            let r277 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r277);
            let Some(t278) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b832); };
            c.w32(1, t278);
            let r279 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r279);
            let Some(t280) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090b838); };
            c.w32(1, t280);
            let r281 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r281);
            let Some(t282) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090b83e); };
            c.w32(2, t282);
            let r283 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r283);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090b845); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090b849); }
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(11));
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090b856); };
            c.w64(6, t284);
            return c.call(0x70007fea14, 0x700090b85f, 0x700090b85a);
        }
        113 => {
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b85f); };
            c.w64(8, t285);
            let Some(t286) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b863); };
            c.w64(8, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090b867); };
            c.w64(9, t287);
            let Some(t288) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b86b); };
            c.w32(11, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b870); };
            c.w64(15, t289);
            let Some(t290) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b874); };
            c.w64(1, t290);
            let Some(t291) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b878); };
            c.w64(5, t291);
            c.w64(12, c.r64(8));
            let r292 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r292);
            return RegfileExit::jump(c.r64(1));
        }
        114 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r293 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r293);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t294) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090b890); };
            c.w32(3, t294);
            let Some(t295) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b896); };
            c.w32(2, t295);
            let r296 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r296);
            let Some(t297) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b89e); };
            c.w32(3, t297);
            let r298 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r298);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090b8a5); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090b8a9); }
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090b8aa); };
            if !c.push64(t299) { return RegfileExit::fault(0x700090b8aa); }
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090b8ad); };
            if !c.push64(t300) { return RegfileExit::fault(0x700090b8ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090b8b0); }
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x700090b8b4); };
            c.w64(3, t301);
            c.w32(0, 0x3u64);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090b8c0); };
            c.w64(6, t302);
            return c.call(0x700086ed74, 0x700090b8c9, 0x700090b8c4);
        }
        115 => {
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b8c9); };
            c.w64(8, t303);
            let Some(t304) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b8cd); };
            c.w64(8, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090b8d1); };
            c.w64(9, t305);
            let Some(t306) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b8d5); };
            c.w32(11, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b8da); };
            c.w64(15, t307);
            let Some(t308) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b8de); };
            c.w64(1, t308);
            let Some(t309) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b8e2); };
            c.w64(5, t309);
            c.w64(12, c.r64(8));
            let r310 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r310);
            return RegfileExit::jump(c.r64(1));
        }
        116 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            let r311 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r311);
            let r312 = c.r64(6) & 0x3ffu64;
            c.w64(6, r312);
            c.logic(r312, 64);
            let r313 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r313);
            let r314 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r314);
            let Some(t315) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090b908); };
            c.w32(3, t315);
            let r316 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r316);
            let Some(t317) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090b90e); };
            c.w32(3, t317);
            let r318 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r318);
            let Some(t319) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090b914); };
            c.w32(3, t319);
            let r320 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r320);
            let Some(t321) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b91b); };
            c.w32(7, t321);
            let r322 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r322);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090b921); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700090b925); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090b927); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090b92d); };
            c.w64(6, t323);
            return c.call(0x70007f2274, 0x700090b936, 0x700090b931);
        }
        117 => {
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b936); };
            c.w64(8, t324);
            let Some(t325) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090b93a); };
            c.w64(8, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090b93e); };
            c.w64(9, t326);
            let Some(t327) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090b942); };
            c.w32(11, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090b947); };
            c.w64(15, t328);
            let Some(t329) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090b94b); };
            c.w64(1, t329);
            let Some(t330) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090b94f); };
            c.w64(5, t330);
            c.w64(12, c.r64(8));
            let r331 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r331);
            return RegfileExit::jump(c.r64(1));
        }
        118 => {
            let Some(t332) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b95c); };
            c.w32(2, t332);
            let r333 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r333);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700090b967); };
            c.w64(2, t334);
            blk = 120; continue;
        }
        120 => {
            let Some(t335) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090b96e); };
            c.w32(1, t335);
            let Some(t336) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090b971); };
            c.w32(6, t336);
            let Some(t337) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b977); };
            c.w32(7, t337);
            let r338 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r338);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t339) = c.ld32(c.r64(7).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b982); };
            c.w32(6, t339);
            let r340 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r340);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r341 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r341);
            let Some(t342) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b990); };
            c.w32(11, t342);
            let r343 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r343);
            let r344 = c.r64(11) & c.r64(1);
            c.w64(11, r344);
            c.logic(r344, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 121; continue;
        }
        121 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(8).wrapping_mul(4));
            let Some(t345) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b9ab); };
            c.w32(0, t345);
            let Some(t346) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090b9af); };
            c.cmp(t346, c.r32(0), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(6), c.r32(0), 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let r347 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r347);
            c.w64(8, c.r64(15).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(1));
            let r348 = c.r64(11) & c.r64(8);
            c.w64(11, r348);
            c.logic(r348, 64);
            blk = 121; continue;
        }
        124 => {
            let Some(t349) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090b9d0); };
            c.w32(11, t349);
            let Some(t350) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090b9d4); };
            c.w32(0, t350);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r351 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r351);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 126; continue; }
            blk = 125; continue;
        }
        125 => {
            return RegfileExit::fault(0x700090b9e7);
        }
        126 => {
            let Some(t352) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090b9e8); };
            c.w32(8, t352);
            let Some(t353) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090b9ee); };
            c.w32(11, t353);
            let r354 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r354);
            let Some(t355) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090b9f6); };
            c.w32(8, t355);
            let r356 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r356);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 177; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t357) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ba06); };
            c.w32(0, t357);
            let Some(t358) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090ba0a); };
            c.cmp(t358, c.r32(0), 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t359) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090ba17); };
            c.w32(2, t359);
            let r360 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r360);
            let Some(t361) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ba1e); };
            c.w32(11, t361);
            let r362 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r362);
            let Some(t363) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090ba25); };
            c.logic(t363 & 0x2u64, 8);
            if c.cc_ne() { blk = 150; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090ba30); };
            c.cmp(t364, c.r32(11), 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090ba39); };
            c.w64(11, t365);
            blk = 177; continue;
        }
        131 => {
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090ba45); };
            c.w64(11, t366);
            let Some(t367) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090ba4c); };
            c.cmp(t367, 0x20u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t368) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090ba56); };
            c.w32(8, t368);
            let Some(t369) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090ba5a); };
            c.cmp(t369, c.r32(8), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ba63); };
            c.w64(8, t370);
            blk = 251; continue;
        }
        134 => {
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090ba6c); };
            c.w64(0, t371);
            let Some(t372) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ba70); };
            c.w32(8, t372);
            let r373 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r373);
            let Some(t374) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ba77); };
            c.w32(11, t374);
            let Some(t375) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090ba7b); };
            c.cmp(t375, c.r32(11), 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t376) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090ba84); };
            c.w32(8, t376);
            let r377 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r377);
            let Some(t378) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ba8b); };
            c.w32(11, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090ba8f); };
            c.cmp(t379, c.r32(11), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 136; continue;
        }
        136 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 147; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t380) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090baa2); };
            c.w32(11, t380);
            let r381 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r381);
            let Some(t382) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090baa9); };
            c.w32(3, t382);
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
            let Some(t383) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090bac6); };
            c.cmp(t383, c.r32(11), 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bacf); };
            c.w64(8, t384);
            blk = 251; continue;
        }
        141 => {
            let Some(t385) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090bad8); };
            c.w32(6, t385);
            let r386 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r386);
            c.w64(0, c.r64(6));
            blk = 143; continue;
        }
        142 => {
            c.w64(0, c.r64(8));
            blk = 143; continue;
        }
        143 => {
            let Some(t387) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090bae6); };
            c.w32(8, t387);
            let r388 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r388);
            let Some(t389) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090baed); };
            c.w32(8, t389);
            let r390 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r390);
            let Some(t391) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090baf4); };
            c.w32(11, t391);
            let r392 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r392);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 146; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r393 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r393);
            let Some(t394) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090bb0d); };
            c.w32(6, t394);
            let r395 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r395);
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bb14); };
            c.w64(8, t396);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090bb18); }
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090bb1c); };
            if !c.push64(t397) { return RegfileExit::fault(0x700090bb1c); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700090bb26, 0x700090bb21);
        }
        145 => {
            c.w64(11, c.r64(0));
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090bb29); };
            c.w64(9, t398);
            blk = 177; continue;
        }
        146 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bb32); };
            c.w64(8, t399);
            blk = 251; continue;
        }
        147 => {
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bb3b); };
            c.w64(8, t400);
            blk = 251; continue;
        }
        148 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r401 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r401);
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bb4d); };
            c.w64(3, t402);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090bb51); }
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090bb55); };
            if !c.push64(t403) { return RegfileExit::fault(0x700090bb55); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090bb58); }
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090bb5a); };
            if !c.push64(t404) { return RegfileExit::fault(0x700090bb5a); }
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090bb5d); };
            c.w64(6, t405);
            let Some(t406) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090bb61); };
            c.w64(3, t406);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700090bb75, 0x700090bb70);
        }
        149 => {
            c.w64(11, c.r64(0));
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090bb78); };
            c.w64(9, t407);
            blk = 177; continue;
        }
        150 => {
            c.w64(8, c.r64(9));
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r408 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bb8b); };
            c.w64(0, t409);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090bb8f); }
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090bb93); };
            if !c.push64(t410) { return RegfileExit::fault(0x700090bb93); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090bb9b); };
            c.w64(6, t411);
            return c.call(0x70007f2274, 0x700090bba4, 0x700090bb9f);
        }
        151 => {
            c.w64(11, c.r64(0));
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090bba7); };
            c.w64(9, t412);
            blk = 177; continue;
        }
        152 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090bbb0); };
            c.w64(0, t413);
            let Some(t414) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bbb4); };
            c.w32(8, t414);
            let r415 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r415);
            let Some(t416) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090bbbb); };
            c.w32(3, t416);
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
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bbdd); };
            c.w64(8, t417);
            blk = 251; continue;
        }
        156 => {
            let Some(t418) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090bbe6); };
            c.w32(8, t418);
            let Some(t419) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090bbea); };
            c.cmp(t419, c.r32(8), 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bbf3); };
            c.w64(8, t420);
            blk = 251; continue;
        }
        158 => {
            let Some(t421) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090bbfc); };
            c.w32(8, t421);
            let r422 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r422);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bc09); };
            c.w64(8, t423);
            blk = 251; continue;
        }
        160 => {
            let Some(t424) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bc12); };
            c.w32(11, t424);
            let r425 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r425);
            let Some(t426) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090bc19); };
            c.cmp(t426, 0x40u64, 16);
            if c.cc_b() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bc21); };
            c.w64(8, t427);
            blk = 251; continue;
        }
        162 => {
            let Some(t428) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090bc2a); };
            c.w32(8, t428);
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            blk = 177; continue;
        }
        163 => {
            let Some(t429) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090bc37); };
            c.w32(11, t429);
            let Some(t430) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090bc3b); };
            c.cmp(t430, c.r32(11), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bc44); };
            c.w64(8, t431);
            blk = 251; continue;
        }
        165 => {
            let Some(t432) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090bc4d); };
            c.w32(8, t432);
            c.w64(11, c.r64(8));
            let r433 = c.r32(11) & 0xc0u64;
            c.w32(11, r433);
            c.logic(r433, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t434) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090bc65); };
            c.w32(11, t434);
            let r435 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r435);
            let Some(t436) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090bc6c); };
            c.w32(11, t436);
            let r437 = c.r32(11) & 0x1fu64;
            c.w32(11, r437);
            c.logic(r437, 32);
            let r438 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r438);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bc7e); };
            c.w64(8, t439);
            blk = 251; continue;
        }
        168 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 173; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t440) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090bc8d); };
            c.w32(11, t440);
            let r441 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r441);
            let Some(t442) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090bc94); };
            c.cmp(t442, c.r32(11), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t443) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bc9d); };
            c.w32(8, t443);
            let Some(t444) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090bca1); };
            c.cmp(t444, c.r32(8), 32);
            if c.cc_ne() { blk = 177; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t445) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090bcaa); };
            c.w32(11, t445);
            let r446 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r446);
            blk = 177; continue;
        }
        172 => {
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bcb3); };
            c.w64(8, t447);
            blk = 251; continue;
        }
        173 => {
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bcbc); };
            c.w64(8, t448);
            blk = 251; continue;
        }
        174 => {
            let Some(t449) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090bcc5); };
            c.w32(8, t449);
            let Some(t450) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090bcc9); };
            c.cmp(t450, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bcd2); };
            c.w64(8, t451);
            blk = 251; continue;
        }
        176 => {
            let Some(t452) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090bcdb); };
            c.w32(11, t452);
            let r453 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r453);
            blk = 177; continue;
        }
        177 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bce9); };
            c.w64(3, t454);
            let Some(t455) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bced); };
            c.w64(8, t455);
            let Some(t456) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090bcf1); };
            c.w32(0, t456);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090bcf6); };
            c.w64(15, t457);
            let Some(t458) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090bcfa); };
            c.w64(1, t458);
            let Some(t459) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090bcfe); };
            c.w64(5, t459);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r460 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r460);
            return RegfileExit::jump(c.r64(1));
        }
        178 => {
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bd0e); };
            c.w64(8, t461);
            blk = 251; continue;
        }
        179 => {
            let r462 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r462);
            let Some(t463) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bd1b); };
            c.w64(8, t463);
            let Some(t464) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090bd1f); };
            c.w32(11, t464);
            let Some(t465) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090bd24); };
            c.w64(1, t465);
            let Some(t466) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090bd28); };
            c.w64(5, t466);
            c.w64(12, c.r64(8));
            let r467 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r467);
            return RegfileExit::jump(c.r64(1));
        }
        180 => {
            let r468 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r468);
            let Some(t469) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090bd39); };
            c.w32(8, t469);
            let Some(t470) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090bd3e); };
            c.w64(1, t470);
            let Some(t471) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090bd42); };
            c.w64(5, t471);
            c.w64(0, c.r64(3));
            let r472 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r472);
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
            let r473 = c.r64(11) & 0x7ffu64;
            c.w64(11, r473);
            c.logic(r473, 64);
            c.logic(c.r8(6) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 190; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t474) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090bd78); };
            c.w32(3, t474);
            let r475 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r475);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t476) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090bd83); };
            c.w64(3, t476);
            blk = 185; continue;
        }
        185 => {
            let Some(t477) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bd8a); };
            c.w32(11, t477);
            let r478 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r478);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let r479 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r479);
            let Some(t480) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090bd9b); };
            c.w32(8, t480);
            let Some(t481) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090bda0); };
            c.w64(1, t481);
            let Some(t482) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090bda4); };
            c.w64(5, t482);
            c.w64(0, c.r64(11));
            let r483 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r483);
            return RegfileExit::jump(c.r64(1));
        }
        187 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t484) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bdbb); };
            c.w32(0, t484);
            let Some(t485) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090bdbf); };
            c.cmp(t485, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 189; continue;
        }
        189 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700090bdcc); }
            blk = 243; continue;
        }
        190 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            let r486 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r486);
            let Some(t487) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090bde0); };
            c.w32(8, t487);
            let Some(t488) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bde5); };
            c.w32(0, t488);
            let r489 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r489);
            let Some(t490) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090bded); };
            c.w64(1, t490);
            let Some(t491) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090bdf1); };
            c.w64(5, t491);
            let r492 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r492);
            return RegfileExit::jump(c.r64(1));
        }
        192 => {
            let Some(t493) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bdfb); };
            c.w32(11, t493);
            let r494 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r494);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t495) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090be0d); };
            c.w32(0, t495);
            let Some(t496) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090be11); };
            c.cmp(t496, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 194; continue;
        }
        194 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700090be1e); }
            blk = 243; continue;
        }
        195 => {
            c.w64(11, c.r64(6));
            let r497 = c.shr(c.r32(11), 0x7u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r497);
            let r498 = c.r32(11) & 0xfu64;
            c.w32(11, r498);
            c.logic(r498, 32);
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
            return RegfileExit::fault(0x700090bea2);
        }
        209 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r499 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r499);
            let r500 = c.r64(6) & 0xfffffu64;
            c.w64(6, r500);
            c.logic(r500, 64);
            let Some(t501) = c.ld8(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090beb2); };
            c.w32(8, (t501 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t502) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090bebc); };
            c.w32(8, t502);
            let Some(t503) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090bec1); };
            c.w64(1, t503);
            let Some(t504) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090bec5); };
            c.w64(5, t504);
            let r505 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r505);
            return RegfileExit::jump(c.r64(1));
        }
        210 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r506 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r506);
            let r507 = c.r64(6) & 0xfffffu64;
            c.w64(6, r507);
            c.logic(r507, 64);
            let Some(t508) = c.ld16(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bede); };
            c.w32(8, (t508 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t509) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090bee8); };
            c.w32(8, t509);
            let Some(t510) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090beed); };
            c.w64(1, t510);
            let Some(t511) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090bef1); };
            c.w64(5, t511);
            let r512 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r512);
            return RegfileExit::jump(c.r64(1));
        }
        211 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r513 = c.r64(6) & 0xfffffu64;
            c.w64(6, r513);
            c.logic(r513, 64);
            let Some(t514) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bf06); };
            c.w32(8, t514);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090bf0b); }
            c.w64(11, c.r64(8));
            let r515 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r515);
            if c.cc_o() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 218; continue;
        }
        213 => {
            let Some(t516) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090bf1f); };
            c.w64(11, t516);
            let Some(t517) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090bf26); };
            c.w64(0, t517);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t518) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090bf2d); };
            c.w64(2, t518);
            let Some(t519) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090bf34); };
            c.cmp(t519, c.r64(3), 64);
            if c.cc_a() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090bf43, 0x700090bf3e);
        }
        215 => {
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bf43); };
            c.w64(8, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090bf47); };
            c.w64(15, t521);
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090bf4b); };
            c.w64(9, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090bf4f); };
            c.w64(12, t523);
            blk = 217; continue;
        }
        216 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090bf55); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 217; continue;
        }
        217 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t524) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090bf68); };
            c.w64(8, t524);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090bf6f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090bf73); }
            blk = 218; continue;
        }
        218 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t525) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090bf7f); };
            c.w32(8, t525);
            let Some(t526) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090bf84); };
            c.w64(1, t526);
            let Some(t527) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090bf88); };
            c.w64(5, t527);
            let r528 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r528);
            return RegfileExit::jump(c.r64(1));
        }
        219 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r529 = c.r64(6) & 0xfffffu64;
            c.w64(6, r529);
            c.logic(r529, 64);
            let Some(t530) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090bf9d); };
            c.w32(8, t530);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090bfa2); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            let r531 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r531);
            blk = 226; continue;
        }
        221 => {
            let Some(t532) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090bfb4); };
            c.w64(11, t532);
            let Some(t533) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090bfbb); };
            c.w64(0, t533);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090bfc2); };
            c.w64(2, t534);
            let Some(t535) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090bfc9); };
            c.cmp(t535, c.r64(3), 64);
            if c.cc_a() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090bfd8, 0x700090bfd3);
        }
        223 => {
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090bfd8); };
            c.w64(8, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090bfdc); };
            c.w64(15, t537);
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090bfe0); };
            c.w64(9, t538);
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090bfe4); };
            c.w64(12, t539);
            blk = 225; continue;
        }
        224 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090bfea); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 225; continue;
        }
        225 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t540) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090c000); };
            c.w64(8, t540);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090c007); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090c00b); }
            c.w64(8, c.r64(0));
            blk = 226; continue;
        }
        226 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t541) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c01a); };
            c.w32(11, t541);
            let Some(t542) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c01f); };
            c.w64(1, t542);
            let Some(t543) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c023); };
            c.w64(5, t543);
            c.w64(0, c.r64(8));
            let r544 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r544);
            return RegfileExit::jump(c.r64(1));
        }
        227 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r545 = c.r64(6) & 0xfffffu64;
            c.w64(6, r545);
            c.logic(r545, 64);
            let Some(t546) = c.ld64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090c03b); };
            c.w64(8, t546);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090c040); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 235; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t547) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090c04a); };
            c.w64(11, t547);
            let Some(t548) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090c051); };
            c.w64(0, t548);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t549) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090c058); };
            c.w64(2, t549);
            let Some(t550) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090c05f); };
            c.cmp(t550, c.r64(3), 64);
            if c.cc_a() { blk = 231; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700090c06e, 0x700090c069);
        }
        230 => {
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c06e); };
            c.w64(8, t551);
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c072); };
            c.w64(15, t552);
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090c076); };
            c.w64(9, t553);
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090c07a); };
            c.w64(12, t554);
            blk = 232; continue;
        }
        231 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090c080); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 232; continue;
        }
        232 => {
            let Some(t555) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090c08a); };
            c.w64(3, t555);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700090c091); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700090c09a); }
            let r556 = c.neg(c.r64(8), 64);
            c.w64(8, r556);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700090c0a4); }
            blk = 240; continue;
        }
        234 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700090c0aa); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700090c0b1); }
            blk = 240; continue;
        }
        235 => {
            let Some(t557) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090c0b7); };
            c.w64(8, t557);
            let Some(t558) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090c0be); };
            c.w64(11, t558);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090c0c5); };
            c.w64(0, t559);
            let Some(t560) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700090c0cc); };
            c.cmp(t560, c.r64(3), 64);
            if c.cc_a() { blk = 238; continue; }
            blk = 236; continue;
        }
        236 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700090c0db, 0x700090c0d6);
        }
        237 => {
            blk = 239; continue;
        }
        238 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x700090c0dd); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 239; continue;
        }
        239 => {
            let Some(t561) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090c0e7); };
            c.w64(8, t561);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090c0ee); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700090c0f2); }
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090c0f9); };
            c.w64(12, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c0fd); };
            c.w64(15, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090c101); };
            c.w64(9, t564);
            blk = 240; continue;
        }
        240 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t565) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c10c); };
            c.w32(8, t565);
            let Some(t566) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c111); };
            c.w64(1, t566);
            let Some(t567) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c115); };
            c.w64(5, t567);
            let r568 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r568);
            return RegfileExit::jump(c.r64(1));
        }
        241 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r569 = c.r64(6) & 0xfffffu64;
            c.w64(6, r569);
            c.logic(r569, 64);
            if !c.movss_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090c12a); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 243; continue;
        }
        242 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r570 = c.r64(6) & 0xfffffu64;
            c.w64(6, r570);
            c.logic(r570, 64);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090c141); }
            blk = 243; continue;
        }
        243 => {
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090c147); };
            c.w64(8, t571);
            let Some(t572) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090c14e); };
            c.w64(11, t572);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700090c155); }
            let Some(t573) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090c15a); };
            c.w64(3, t573);
            let Some(t574) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700090c161); };
            c.cmp(t574, c.r64(0), 64);
            if c.cc_a() { blk = 246; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090c170, 0x700090c16b);
        }
        245 => {
            blk = 247; continue;
        }
        246 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700090c172); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 247; continue;
        }
        247 => {
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090c17c); };
            c.w64(11, t575);
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t576) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090c184); };
            c.w64(8, t576);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090c18b); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700090c18f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090c194); }
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090c199); };
            c.w64(12, t577);
            let Some(t578) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c19d); };
            c.w32(8, t578);
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c1a2); };
            c.w64(15, t579);
            let Some(t580) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c1a6); };
            c.w64(1, t580);
            let Some(t581) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c1aa); };
            c.w64(5, t581);
            let r582 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r582);
            return RegfileExit::jump(c.r64(1));
        }
        248 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r583 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r583);
            let r584 = c.r64(6) & 0xfffffu64;
            c.w64(6, r584);
            c.logic(r584, 64);
            let Some(t585) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c1c3); };
            c.w32(8, t585);
            let Some(t586) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090c1c8); };
            c.w32(0, t586);
            let r587 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r587);
            let Some(t588) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c1cf); };
            c.w64(1, t588);
            let Some(t589) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c1d3); };
            c.w64(5, t589);
            let r590 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r590);
            return RegfileExit::jump(c.r64(1));
        }
        249 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r591 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r591);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090c1e4); }
            let Some(t592) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700090c1e8); };
            c.w64(8, t592);
            let Some(t593) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c1ef); };
            c.w32(8, t593);
            let r594 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r594);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090c1f6); }
            let Some(t595) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700090c1f8); };
            c.w64(3, t595);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700090c20b, 0x700090c206);
        }
        250 => {
            return RegfileExit::fault(0x700090c20b);
        }
        251 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r596 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r596);
            c.w64(0, c.r64(9).wrapping_add(0x5));
            let Some(t597) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090c217); };
            c.w32(3, t597);
            let Some(t598) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c21d); };
            c.w32(2, t598);
            let r599 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r599);
            let Some(t600) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c225); };
            c.w64(1, t600);
            let Some(t601) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c229); };
            c.w32(3, t601);
            let r602 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r602);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090c230); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090c234); }
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090c235); };
            if !c.push64(t603) { return RegfileExit::fault(0x700090c235); }
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090c238); };
            if !c.push64(t604) { return RegfileExit::fault(0x700090c238); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700090c23e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090c240); }
            let Some(t605) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x700090c244); };
            c.w64(3, t605);
            c.w64(6, c.r64(1));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700090c25b, 0x700090c256);
        }
        252 => {
            let Some(t606) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c25b); };
            c.w64(8, t606);
            let Some(t607) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c25f); };
            c.w64(8, t607);
            let Some(t608) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090c263); };
            c.w64(9, t608);
            let Some(t609) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090c267); };
            c.w32(11, t609);
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c26c); };
            c.w64(15, t610);
            let Some(t611) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c270); };
            c.w64(1, t611);
            let Some(t612) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c274); };
            c.w64(5, t612);
            c.w64(12, c.r64(8));
            let r613 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r613);
            return RegfileExit::jump(c.r64(1));
        }
        253 => {
            return RegfileExit::fault(0x700090c281);
        }
        254 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090c282); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090c286); }
            let r614 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r614);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r615 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r615);
            let Some(t616) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090c293); };
            c.w32(0, t616);
            let Some(t617) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c299); };
            c.w32(3, t617);
            let r618 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r618);
            let Some(t619) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c2a1); };
            c.w64(2, t619);
            let Some(t620) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c2a5); };
            c.w32(1, t620);
            let r621 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r621);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090c2ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090c2b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090c2b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090c2b8); }
            c.w32(0, 0xeu64);
            c.w64(6, c.r64(2));
            return c.call(0x7000804df4, 0x700090c2c9, 0x700090c2c4);
        }
        255 => {
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c2c9); };
            c.w64(8, t622);
            let Some(t623) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c2cd); };
            c.w64(8, t623);
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090c2d1); };
            c.w64(11, t624);
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t625) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090c2d9); };
            c.w32(11, t625);
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c2de); };
            c.w64(15, t626);
            let Some(t627) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c2e2); };
            c.w64(1, t627);
            let Some(t628) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c2e6); };
            c.w64(5, t628);
            c.w64(12, c.r64(8));
            let r629 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r629);
            return RegfileExit::jump(c.r64(1));
        }
        256 => {
            return RegfileExit::jump(0x700090c2f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090c314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090c314); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090c318); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c31e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c322); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090c326); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090c32d); };
            c.w32(11, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c333); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c33b); };
            c.w64(1, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c33f); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c346); };
            c.w32(6, t11);
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c34c); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090c354); };
            c.w32(3, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700090c358); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700090c35c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700090c360); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090c364); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090c368); };
            c.cmp(t15, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090c371); };
            c.w64(8, t16);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r17 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r17);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(6).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700090c385); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(3, c.r64(8));
            return c.call(0x700083bef4, 0x700090c397, 0x700090c392);
        }
        3 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090c397); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c39b); };
            c.w64(2, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c39f); };
            c.w64(8, t20);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700090c3a7); };
            c.w32(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090c3ac); };
            c.w64(15, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c3b0); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c3b4); };
            c.w64(5, t24);
            c.w64(12, c.r64(2));
            let Some(t25) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700090c3bb); };
            c.w64(0, t25);
            let r26 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r26);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700090c3c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090c3d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090c3d4); };
            c.w64(3, (t0 as u16 as i16 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c3dd); };
            c.w64(3, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c3e1); };
            c.w32(2, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090c3e7); };
            c.w32(6, t3);
            c.cmp(c.r16(6), 0x0u64, 16);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c3f3); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090c402); };
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
            return RegfileExit::fault(0x700090c40f);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c410); };
            c.w32(6, t8);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x700090c41e);
        }
        7 => {
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(2)), c.r32(0)) { return RegfileExit::fault(0x700090c427); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r9 = c.r64(6) & c.r64(3);
            c.w64(6, r9);
            c.logic(r9, 64);
            let Some(t10) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090c438); };
            c.logic(t10 & 0x4u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let r11 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c442); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c447); };
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
            let Some(t16) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090c45c); };
            c.logic(t16 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700090c466); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090c467); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f1594, 0x700090c474, 0x700090c46f);
        }
        12 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700090c474); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700090c476); }
            blk = 9; continue;
        }
        13 => {
            return RegfileExit::jump(0x700090c47c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090c494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c497); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c49b); };
            c.w32(2, t1);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090c4a1); };
            c.w32(6, t2);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700090c4af);
        }
        2 => {
            c.w64(2, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(2)), c.r32(0)) { return RegfileExit::fault(0x700090c4b8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r3 = c.r64(6) & c.r64(3);
            c.w64(6, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090c4c9); };
            c.logic(t4 & 0x4u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c4d3); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c4d8); };
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
            let Some(t10) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x700090c4ed); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700090c4f7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090c4f8); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f1594, 0x700090c505, 0x700090c500);
        }
        7 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700090c505); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700090c507); }
            blk = 4; continue;
        }
        8 => {
            return RegfileExit::jump(0x700090c50c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090c514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090c514); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090c518); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090c51e); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c524); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c52c); };
            c.w64(8, t4);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c534); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c53b); };
            c.w64(7, t7);
            let r8 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r8);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090c542); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090c546); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090c547); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090c54b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090c54f); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x700090c553); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700090c567, 0x700090c562);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c567); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c56b); };
            c.w64(2, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090c56f); };
            c.w64(7, t12);
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700090c577); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c57c); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c580); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c584); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700090c594);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090c5b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c5b7); };
            c.w64(7, t0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c5bb); };
            c.w32(8, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090c5c1); };
            c.w32(11, t2);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c5ca); };
            c.w32(3, t3);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c5d9); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c5df); };
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
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090c5f7); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090c5fa); };
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
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090c609); };
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
            return RegfileExit::fault(0x700090c617);
        }
        9 => {
            return RegfileExit::fault(0x700090c618);
        }
        10 => {
            return RegfileExit::fault(0x700090c619);
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c61a); };
            c.w32(2, t12);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700090c628);
        }
        13 => {
            let r13 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c62d); };
            c.w32(2, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c632); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c63a); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090c640); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090c644); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            let Some(t19) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090c64a); };
            c.w32(8, t19);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c654); };
            c.w32(0, t20);
            let r21 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r21);
            let r22 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c65f); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090c667); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090c66b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090c66d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090c671); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700090c675); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x700090c679); };
            c.w64(3, t25);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700090c68d, 0x700090c688);
        }
        15 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c68d); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c691); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090c695); };
            c.w64(9, t28);
            let r29 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r29);
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700090c69d); };
            c.w32(7, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c6a2); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c6a6); };
            c.w64(1, t32);
            let Some(t33) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c6aa); };
            c.w64(5, t33);
            c.w64(12, c.r64(2));
            let r34 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            return RegfileExit::jump(c.r64(1));
        }
        16 => {
            return RegfileExit::jump(0x700090c6b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090c6d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c6d7); };
            c.w64(11, t0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090c6db); };
            c.w32(0, t1);
            c.w64(3, c.r64(11));
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c6e4); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c6f3); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c6f9); };
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
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090c711); };
            c.w32(1, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090c714); };
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
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090c727); };
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
            return RegfileExit::fault(0x700090c734);
        }
        9 => {
            return RegfileExit::fault(0x700090c735);
        }
        10 => {
            return RegfileExit::fault(0x700090c736);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c737); };
            c.w64(11, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090c73b); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c742); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090c749); };
            c.w32(0, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090c74d); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090c756); };
            c.w64(11, t18);
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090c75d); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c76a); };
            c.w32(0, t20);
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(3, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090c777); };
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
            let Some(t24) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090c791); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090c797); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r27);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c7a8); };
            c.w32(8, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c7ad); };
            c.w64(1, t29);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c7b6); };
            c.w64(8, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090c7ba); };
            c.w32(8, t31);
            let r32 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090c7c1); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c7c8); };
            c.w32(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x700090c7db);
        }
        20 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x700090c7dc); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c7e9); };
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
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090c80d); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c815); };
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
            return RegfileExit::fault(0x700090c82f);
        }
        23 => {
            let r44 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c834); };
            c.w32(0, t45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c839); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c841); };
            c.w64(1, t48);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090c84a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090c84e); }
            let r49 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090c854); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090c858); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700090c85c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700090c860); }
            blk = 263; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090c869); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090c86d); }
            let r50 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090c873); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090c87b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090c87f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090c883); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700090c887); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700090c88b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700090c88f); }
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700090c893); };
            c.cmp(t53, c.r32(7), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t54) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090c8a0); };
            c.w64(3, t54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090c8a4); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090c8aa); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c8b0); };
            c.w32(1, t59);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700090c8b9); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x700090c8c3);
        }
        28 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090c8c4); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700090c8ca); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x700090c8d4);
        }
        30 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090c8d5); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090c8e5); };
            c.w32(1, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700090c8e8); };
            c.cmp(t65, c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c8f1); };
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
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c8ff); };
            c.cmp(t68, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090c909); };
            c.w32(1, t69);
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090c90c); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700090c916); };
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
            let r74 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r74);
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090c93d); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c942); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c946); };
            c.w64(5, t77);
            let r78 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r78);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r79 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r79);
            c.w64(6, c.r64(9).wrapping_add(0x7));
            let Some(t80) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090c95a); };
            c.w32(9, t80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c960); };
            c.w32(12, t81);
            let r82 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090c968); };
            c.w32(9, t83);
            let r84 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r84);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700090c970); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700090c974); }
            c.w64(1, c.r64(9));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(11));
            let Some(t85) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090c981); };
            c.logic(t85 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r86 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r86);
            blk = 43; continue;
        }
        42 => {
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090c990); };
            c.w32(7, t87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700090c993); };
            c.w64(7, t88);
            blk = 43; continue;
        }
        43 => {
            return c.call(c.r64(7), 0x700090c99d, 0x700090c99b);
        }
        44 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c99d); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090c9a1); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090c9a5); };
            c.w64(9, t91);
            let Some(t92) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090c9a9); };
            c.w32(11, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090c9ae); };
            c.w64(15, t93);
            let Some(t94) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090c9b2); };
            c.w64(1, t94);
            let Some(t95) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090c9b6); };
            c.w64(5, t95);
            c.w64(12, c.r64(8));
            let r96 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r96);
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
            let r97 = c.r64(3) & 0xfu64;
            c.w64(3, r97);
            c.logic(r97, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090c9d9); }
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
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r98 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r98);
            c.w64(3, c.r64(9).wrapping_add(0x7));
            let Some(t99) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090ca67); };
            c.w32(2, t99);
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ca6d); };
            c.w32(1, t100);
            let r101 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r101);
            let Some(t102) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ca75); };
            c.w32(2, t102);
            let r103 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r103);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090ca7c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090ca80); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090ca81); };
            if !c.push64(t104) { return RegfileExit::fault(0x700090ca81); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090ca84); }
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090ca85); };
            if !c.push64(t105) { return RegfileExit::fault(0x700090ca85); }
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090ca88); };
            if !c.push64(t106) { return RegfileExit::fault(0x700090ca88); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x700090ca8b); }
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090ca8f); };
            c.w64(6, t107);
            c.w32(0, 0x5u64);
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700090ca98); };
            c.w64(3, t108);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700090caa7, 0x700090caa2);
        }
        59 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090caa7); };
            c.w64(8, t109);
            let Some(t110) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090caab); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700090caaf); };
            c.w64(9, t111);
            let Some(t112) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090cab3); };
            c.w32(11, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090cab8); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090cabc); };
            c.w64(1, t114);
            let Some(t115) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090cac0); };
            c.w64(5, t115);
            c.w64(12, c.r64(8));
            let r116 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r116);
            return RegfileExit::jump(c.r64(1));
        }
        60 => {
            let Some(t117) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090cacd); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090cad4); };
            c.w32(11, t119);
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            let Some(t121) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090cadb); };
            c.w32(0, t121);
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r122 = c.r64(1) & 0x1ffffffu64;
            c.w64(1, r122);
            c.logic(r122, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_b() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x700090caf5);
        }
        62 => {
            let Some(t123) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090caf6); };
            c.w32(11, t123);
            let r124 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r124);
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090cafe); };
            c.w32(0, t125);
            let r126 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r126);
            c.w64(11, c.r64(9).wrapping_add(0x7));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700090cb09); }
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090cb0d); };
            c.cmp(t127, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t128) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090cb16); };
            c.w32(8, t128);
            let Some(t129) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090cb1b); };
            c.w64(1, t129);
            let Some(t130) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090cb1f); };
            c.w64(5, t130);
            c.w64(9, c.r64(11));
            let r131 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r131);
            return RegfileExit::jump(c.r64(1));
        }
        64 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r132 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r132);
            let Some(t133) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090cb32); };
            c.w32(3, t133);
            let Some(t134) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cb38); };
            c.w32(2, t134);
            let r135 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r135);
            let Some(t136) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cb40); };
            c.w32(3, t136);
            let r137 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r137);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090cb47); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090cb50); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090cb51); }
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700090cb52); };
            c.w64(3, t138);
            c.w32(0, 0x2u64);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090cb5e); };
            c.w64(6, t139);
            return c.call(0x700086ed74, 0x700090cb67, 0x700090cb62);
        }
        65 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cb67); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cb6b); };
            c.w64(8, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090cb6f); };
            c.w64(9, t142);
            let Some(t143) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090cb73); };
            c.w32(11, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090cb78); };
            c.w64(15, t144);
            let Some(t145) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090cb7c); };
            c.w64(1, t145);
            let Some(t146) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090cb80); };
            c.w64(5, t146);
            c.w64(12, c.r64(8));
            let r147 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r147);
            return RegfileExit::jump(c.r64(1));
        }
        66 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r148 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r148);
            c.w64(2, c.r64(9).wrapping_add(0x7));
            let Some(t149) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090cb97); };
            c.w32(1, t149);
            let Some(t150) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cb9d); };
            c.w32(6, t150);
            let r151 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r151);
            let Some(t152) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cba5); };
            c.w32(1, t152);
            let r153 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r153);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700090cbac); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700090cbb0); }
            c.w64(3, c.r64(1));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090cbb7); };
            c.w64(1, t154);
            c.w64(2, c.r64(11));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090cbbe); };
            c.w64(6, t155);
            return c.call(0x70008bc174, 0x700090cbc7, 0x700090cbc2);
        }
        67 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cbc7); };
            c.w64(8, t156);
            let Some(t157) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cbcb); };
            c.w64(8, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090cbcf); };
            c.w64(9, t158);
            let Some(t159) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090cbd3); };
            c.w32(11, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090cbd8); };
            c.w64(15, t160);
            let Some(t161) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090cbdc); };
            c.w64(1, t161);
            let Some(t162) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090cbe0); };
            c.w64(5, t162);
            c.w64(12, c.r64(8));
            let r163 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r163);
            return RegfileExit::jump(c.r64(1));
        }
        68 => {
            blk = 125; continue;
        }
        69 => {
            let Some(t164) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090cbf2); };
            c.w32(3, t164);
            let r165 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r165);
            let Some(t166) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cbf8); };
            c.w32(2, t166);
            let Some(t167) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090cbfb); };
            c.cmp(t167, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t168) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090cc08); };
            c.w32(1, t168);
            let Some(t169) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cc0e); };
            c.w32(6, t169);
            let r170 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r170);
            let Some(t171) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cc16); };
            c.w32(1, t171);
            let r172 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r172);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t173) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090cc26); };
            c.w32(2, t173);
            let r174 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090cc2c); };
            c.cmp(t175, c.r32(2), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t176) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090cc39); };
            c.w32(2, t176);
            let r177 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r177);
            let Some(t178) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090cc3f); };
            c.w32(6, t178);
            let r179 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r179);
            let Some(t180) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090cc45); };
            c.logic(t180 & 0x2u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090cc4f); };
            c.cmp(t181, c.r32(6), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090cc58); };
            c.w64(3, t182);
            blk = 109; continue;
        }
        75 => {
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090cc64); };
            c.w64(6, t183);
            let Some(t184) = c.ld8(c.r64(6)) else { return RegfileExit::fault(0x700090cc6b); };
            c.cmp(t184, 0x20u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t185) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090cc74); };
            c.w32(1, t185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090cc77); };
            c.cmp(t186, c.r32(1), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t187) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090cc84); };
            c.w32(1, t187);
            let r188 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r188);
            let Some(t189) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090cc8a); };
            c.w32(6, t189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090cc8d); };
            c.cmp(t190, c.r32(6), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t191) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090cc96); };
            c.w32(1, t191);
            let r192 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r192);
            let Some(t193) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090cc9c); };
            c.w32(6, t193);
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090cc9f); };
            c.cmp(t194, c.r32(6), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t195) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ccb1); };
            c.w32(6, t195);
            let r196 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r196);
            let Some(t197) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ccb7); };
            c.w32(7, t197);
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
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090ccd3); };
            c.cmp(t198, c.r32(6), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090ccdc); };
            c.w64(12, t199);
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
            let Some(t200) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090cced); };
            c.w32(0, t200);
            let r201 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r201);
            let Some(t202) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ccf3); };
            c.w32(0, t202);
            let r203 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r203);
            let Some(t204) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ccf9); };
            c.w32(3, t204);
            let r205 = c.sub(c.r32(3), 0x89u64, 32);
            c.w32(3, r205);
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_a() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r206 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r206);
            let Some(t207) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090cd0e); };
            c.w32(6, t207);
            let r208 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r208);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700090cd14); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090cd18); };
            if !c.push64(t209) { return RegfileExit::fault(0x700090cd18); }
            c.w64(1, c.r64(11));
            return c.call(0x70007f6fb4, 0x700090cd23, 0x700090cd1e);
        }
        88 => {
            c.w64(3, c.r64(0));
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090cd26); };
            c.w64(15, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090cd2a); };
            c.w64(9, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cd2e); };
            c.w64(8, t212);
            blk = 109; continue;
        }
        89 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090cd37); };
            c.w64(12, t213);
            blk = 263; continue;
        }
        90 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r214 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r214);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090cd47); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090cd4b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700090cd4c); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090cd4d); };
            if !c.push64(t215) { return RegfileExit::fault(0x700090cd4d); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090cd50); };
            c.w64(6, t216);
            c.w32(0, 0x3u64);
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090cd59); };
            c.w64(3, t217);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700090cd68, 0x700090cd63);
        }
        91 => {
            c.w64(3, c.r64(0));
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090cd6b); };
            c.w64(15, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090cd6f); };
            c.w64(9, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cd73); };
            c.w64(8, t220);
            blk = 109; continue;
        }
        92 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r221 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r221);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090cd85); }
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090cd89); };
            if !c.push64(t222) { return RegfileExit::fault(0x700090cd89); }
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090cd8c); };
            c.w64(6, t223);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x700090cd9a, 0x700090cd95);
        }
        93 => {
            c.w64(3, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090cd9d); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090cda1); };
            c.w64(9, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cda5); };
            c.w64(8, t226);
            blk = 109; continue;
        }
        94 => {
            let Some(t227) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090cdae); };
            c.w32(11, t227);
            let r228 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r228);
            let Some(t229) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cdb5); };
            c.w32(1, t229);
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
            let Some(t230) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090cddb); };
            c.w32(11, t230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090cddf); };
            c.cmp(t231, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t232) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cdf5); };
            c.cmp(t232, 0x40u64, 16);
            if c.cc_ae() { blk = 263; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t233) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ce00); };
            c.w32(11, t233);
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            blk = 109; continue;
        }
        101 => {
            let Some(t234) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090ce0d); };
            c.w32(2, t234);
            let Some(t235) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090ce10); };
            c.cmp(t235, c.r32(2), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t236) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090ce1d); };
            c.w32(11, t236);
            c.w64(2, c.r64(11));
            let r237 = c.r32(2) & 0xc0u64;
            c.w32(2, r237);
            c.logic(r237, 32);
            c.cmp(c.r32(2), 0xc0u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t238) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090ce33); };
            c.w32(3, t238);
            let r239 = c.r32(3) & 0x1fu64;
            c.w32(3, r239);
            c.logic(r239, 32);
            let r240 = c.sub(c.r32(3), 0x10u64, 32);
            c.w32(3, r240);
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
            let Some(t241) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090ce4f); };
            c.w32(3, t241);
            let r242 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r242);
            let Some(t243) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090ce55); };
            c.cmp(t243, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t244) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ce62); };
            c.w32(11, t244);
            let Some(t245) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090ce66); };
            c.cmp(t245, c.r32(11), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t246) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090ce6f); };
            c.w32(3, t246);
            let r247 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r247);
            blk = 109; continue;
        }
        108 => {
            let Some(t248) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090ce77); };
            c.w32(11, t248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090ce7b); };
            c.cmp(t249, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 109; continue;
        }
        109 => {
            let r250 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r250);
            let Some(t251) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ce8c); };
            c.w64(8, t251);
            let Some(t252) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090ce90); };
            c.w32(11, t252);
            let Some(t253) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ce95); };
            c.w64(1, t253);
            let Some(t254) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ce99); };
            c.w64(5, t254);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(3));
            let r255 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r255);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 263; continue;
        }
        111 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090ceae); };
            c.w64(3, t256);
            let Some(t257) = c.ld8(c.r64(3)) else { return RegfileExit::fault(0x700090ceb5); };
            c.cmp(t257, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t258) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090cec3); };
            c.w32(3, t258);
            let r259 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r259);
            blk = 115; continue;
        }
        114 => {
            let Some(t260) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090cecb); };
            c.w32(3, t260);
            let r261 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r261);
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return RegfileExit::fault(0x700090ced6);
        }
        117 => {
            let r262 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r262);
            c.logic(r262, 64);
            let Some(t263) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cedb); };
            c.w32(1, t263);
            let r264 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r264);
            let Some(t265) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090cee1); };
            c.w32(0, t265);
            let r266 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r266);
            let Some(t267) = c.ld64(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090cee7); };
            c.w64(1, t267);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t268) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090cef0); };
            c.w32(2, t268);
            let r269 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r269);
            let Some(t270) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090cef6); };
            c.w32(2, t270);
            let r271 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r271);
            blk = 119; continue;
        }
        119 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r272 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r272);
            let r273 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r273);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700090cf06); }
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090cf0a); };
            if !c.push64(t274) { return RegfileExit::fault(0x700090cf0a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090cf0d); }
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            return c.call(0x70007fe8d4, 0x700090cf25, 0x700090cf20);
        }
        120 => {
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cf25); };
            c.w64(8, t275);
            let Some(t276) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cf29); };
            c.w64(8, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090cf2d); };
            c.w64(9, t277);
            let Some(t278) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090cf31); };
            c.w32(11, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090cf36); };
            c.w64(15, t279);
            let Some(t280) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090cf3a); };
            c.w64(1, t280);
            let Some(t281) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090cf3e); };
            c.w64(5, t281);
            c.w64(12, c.r64(8));
            let r282 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r282);
            return RegfileExit::jump(c.r64(1));
        }
        121 => {
            blk = 125; continue;
        }
        122 => {
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090cf4d); };
            c.w64(11, t283);
            let Some(t284) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090cf54); };
            c.cmp(t284, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            let r285 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r285);
            let r286 = c.r64(1) & 0x3ffu64;
            c.w64(1, r286);
            c.logic(r286, 64);
            let r287 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r287);
            let r288 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r288);
            let Some(t289) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090cf74); };
            c.w32(2, t289);
            let r290 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r290);
            let Some(t291) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090cf7a); };
            c.w32(2, t291);
            let r292 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r292);
            let Some(t293) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090cf80); };
            c.w32(3, t293);
            let r294 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r294);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090cf87); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090cf8b); }
            c.w64(2, c.r64(6));
            c.w64(1, c.r64(0));
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090cf95); };
            c.w64(6, t295);
            return c.call(0x70007fea14, 0x700090cf9e, 0x700090cf99);
        }
        124 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cf9e); };
            c.w64(8, t296);
            let Some(t297) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090cfa2); };
            c.w64(8, t297);
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090cfa6); };
            c.w64(9, t298);
            let Some(t299) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090cfaa); };
            c.w32(11, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090cfaf); };
            c.w64(15, t300);
            let Some(t301) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090cfb3); };
            c.w64(1, t301);
            let Some(t302) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090cfb7); };
            c.w64(5, t302);
            c.w64(12, c.r64(8));
            let r303 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r303);
            return RegfileExit::jump(c.r64(1));
        }
        125 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r304 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r304);
            c.w64(0, c.r64(9).wrapping_add(0x7));
            let Some(t305) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090cfcf); };
            c.w32(3, t305);
            let Some(t306) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cfd5); };
            c.w32(2, t306);
            let r307 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r307);
            let Some(t308) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090cfdd); };
            c.w32(3, t308);
            let r309 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r309);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090cfe4); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090cfe8); }
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090cfe9); };
            if !c.push64(t310) { return RegfileExit::fault(0x700090cfe9); }
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090cfec); };
            if !c.push64(t311) { return RegfileExit::fault(0x700090cfec); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090cfef); }
            let Some(t312) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x700090cff3); };
            c.w64(3, t312);
            c.w32(0, 0x3u64);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090cfff); };
            c.w64(6, t313);
            return c.call(0x700086ed74, 0x700090d008, 0x700090d003);
        }
        126 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d008); };
            c.w64(8, t314);
            let Some(t315) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d00c); };
            c.w64(8, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090d010); };
            c.w64(9, t316);
            let Some(t317) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090d014); };
            c.w32(11, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d019); };
            c.w64(15, t318);
            let Some(t319) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d01d); };
            c.w64(1, t319);
            let Some(t320) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d021); };
            c.w64(5, t320);
            c.w64(12, c.r64(8));
            let r321 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r321);
            return RegfileExit::jump(c.r64(1));
        }
        127 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            let r322 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r322);
            let r323 = c.r64(1) & 0x3ffu64;
            c.w64(1, r323);
            c.logic(r323, 64);
            let r324 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r324);
            let r325 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r325);
            let Some(t326) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d048); };
            c.w32(0, t326);
            let r327 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r327);
            let Some(t328) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090d04e); };
            c.w32(0, t328);
            let r329 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r329);
            let Some(t330) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090d054); };
            c.w32(0, t330);
            let r331 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r331);
            let Some(t332) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d05b); };
            c.w32(7, t332);
            let r333 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r333);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090d061); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700090d065); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090d066); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090d06c); };
            c.w64(6, t334);
            return c.call(0x70007f2274, 0x700090d075, 0x700090d070);
        }
        128 => {
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d075); };
            c.w64(8, t335);
            let Some(t336) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d079); };
            c.w64(8, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d07d); };
            c.w64(9, t337);
            let Some(t338) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090d081); };
            c.w32(11, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d086); };
            c.w64(15, t339);
            let Some(t340) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d08a); };
            c.w64(1, t340);
            let Some(t341) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d08e); };
            c.w64(5, t341);
            c.w64(12, c.r64(8));
            let r342 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r342);
            return RegfileExit::jump(c.r64(1));
        }
        129 => {
            let Some(t343) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d09b); };
            c.w32(3, t343);
            let r344 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r344);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700090d0a6); };
            c.w64(3, t345);
            blk = 131; continue;
        }
        131 => {
            let Some(t346) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090d0ad); };
            c.w32(2, t346);
            let Some(t347) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090d0b0); };
            c.w32(1, t347);
            let Some(t348) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d0b6); };
            c.w32(7, t348);
            let r349 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r349);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t350) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d0c1); };
            c.w32(1, t350);
            let r351 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r351);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r352 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r352);
            let Some(t353) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d0cf); };
            c.w32(6, t353);
            let r354 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r354);
            let r355 = c.r64(6) & c.r64(2);
            c.w64(6, r355);
            c.logic(r355, 64);
            blk = 132; continue;
        }
        132 => {
            c.w64(15, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(8, c.r64(15).wrapping_mul(4));
            let Some(t356) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d0e5); };
            c.w32(0, t356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090d0ea); };
            c.cmp(t357, c.r32(0), 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r358 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r358);
            c.w64(8, c.r64(11).wrapping_add(c.r64(6)));
            c.w64(0, c.r64(2));
            let r359 = c.r64(0) & c.r64(8);
            c.w64(0, r359);
            c.logic(r359, 64);
            c.w64(6, c.r64(0));
            blk = 132; continue;
        }
        135 => {
            let Some(t360) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d10e); };
            c.w32(11, t360);
            let Some(t361) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090d112); };
            c.w32(0, t361);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(2, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_b() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            return RegfileExit::fault(0x700090d126);
        }
        137 => {
            let Some(t362) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090d127); };
            c.w32(11, t362);
            let Some(t363) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090d12d); };
            c.w32(8, t363);
            let r364 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r364);
            let Some(t365) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d135); };
            c.w32(11, t365);
            let r366 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r366);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t367) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d145); };
            c.w32(0, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090d149); };
            c.cmp(t368, c.r32(0), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t369) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d156); };
            c.w32(2, t369);
            let r370 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r370);
            let Some(t371) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d15d); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090d164); };
            c.logic(t373 & 0x2u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090d16f); };
            c.cmp(t374, c.r32(8), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t375) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090d178); };
            c.w64(8, t375);
            blk = 188; continue;
        }
        142 => {
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090d184); };
            c.w64(8, t376);
            let Some(t377) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x700090d18b); };
            c.cmp(t377, 0x20u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t378) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090d195); };
            c.w32(8, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090d199); };
            c.cmp(t379, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d1a2); };
            c.w64(8, t380);
            blk = 263; continue;
        }
        145 => {
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090d1ab); };
            c.w64(0, t381);
            let Some(t382) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d1af); };
            c.w32(8, t382);
            let r383 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r383);
            let Some(t384) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d1b6); };
            c.w32(11, t384);
            let Some(t385) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090d1ba); };
            c.cmp(t385, c.r32(11), 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t386) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090d1c3); };
            c.w32(8, t386);
            let r387 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d1ca); };
            c.w32(11, t388);
            let Some(t389) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090d1ce); };
            c.cmp(t389, c.r32(11), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 147; continue;
        }
        147 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t390) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d1e1); };
            c.w32(11, t390);
            let r391 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r391);
            let Some(t392) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d1e8); };
            c.w32(3, t392);
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
            let Some(t393) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090d205); };
            c.cmp(t393, c.r32(11), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d20e); };
            c.w64(8, t394);
            blk = 263; continue;
        }
        152 => {
            let Some(t395) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090d217); };
            c.w32(0, t395);
            let r396 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r396);
            blk = 154; continue;
        }
        153 => {
            c.w64(0, c.r64(8));
            blk = 154; continue;
        }
        154 => {
            let Some(t397) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090d222); };
            c.w32(8, t397);
            let r398 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r398);
            let Some(t399) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d229); };
            c.w32(8, t399);
            let r400 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r400);
            let Some(t401) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d230); };
            c.w32(11, t401);
            let r402 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r402);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r403 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r403);
            let Some(t404) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090d249); };
            c.w32(6, t404);
            let r405 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d250); };
            c.w64(8, t406);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090d254); }
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090d258); };
            if !c.push64(t407) { return RegfileExit::fault(0x700090d258); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700090d262, 0x700090d25d);
        }
        156 => {
            c.w64(8, c.r64(0));
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d265); };
            c.w64(9, t408);
            blk = 188; continue;
        }
        157 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d26e); };
            c.w64(8, t409);
            blk = 263; continue;
        }
        158 => {
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d277); };
            c.w64(8, t410);
            blk = 263; continue;
        }
        159 => {
            c.w64(8, c.r64(9));
            c.w64(0, c.r64(8).wrapping_sub(0x1));
            let r411 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d289); };
            c.w64(3, t412);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090d28d); }
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090d291); };
            if !c.push64(t413) { return RegfileExit::fault(0x700090d291); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700090d294); }
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090d296); };
            if !c.push64(t414) { return RegfileExit::fault(0x700090d296); }
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090d299); };
            c.w64(6, t415);
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090d29d); };
            c.w64(3, t416);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700090d2b1, 0x700090d2ac);
        }
        160 => {
            c.w64(8, c.r64(0));
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d2b4); };
            c.w64(9, t417);
            blk = 188; continue;
        }
        161 => {
            c.w64(8, c.r64(9));
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r418 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d2c7); };
            c.w64(0, t419);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090d2cb); }
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090d2cf); };
            if !c.push64(t420) { return RegfileExit::fault(0x700090d2cf); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090d2d7); };
            c.w64(6, t421);
            return c.call(0x70007f2274, 0x700090d2e0, 0x700090d2db);
        }
        162 => {
            c.w64(8, c.r64(0));
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d2e3); };
            c.w64(9, t422);
            blk = 188; continue;
        }
        163 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090d2ec); };
            c.w64(0, t423);
            let Some(t424) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d2f0); };
            c.w32(11, t424);
            let r425 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r425);
            let Some(t426) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d2f7); };
            c.w32(3, t426);
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
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d319); };
            c.w64(8, t427);
            blk = 263; continue;
        }
        167 => {
            let Some(t428) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d322); };
            c.w32(8, t428);
            let Some(t429) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090d326); };
            c.cmp(t429, c.r32(8), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d32f); };
            c.w64(8, t430);
            blk = 263; continue;
        }
        169 => {
            let Some(t431) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090d338); };
            c.w32(8, t431);
            let r432 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r432);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d345); };
            c.w64(8, t433);
            blk = 263; continue;
        }
        171 => {
            let Some(t434) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d34e); };
            c.w32(11, t434);
            let r435 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r435);
            let Some(t436) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d355); };
            c.cmp(t436, 0x40u64, 16);
            if c.cc_b() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d35d); };
            c.w64(8, t437);
            blk = 263; continue;
        }
        173 => {
            let Some(t438) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d366); };
            c.w32(8, t438);
            let r439 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r439);
            blk = 188; continue;
        }
        174 => {
            let Some(t440) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d372); };
            c.w32(8, t440);
            let Some(t441) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090d376); };
            c.cmp(t441, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d37f); };
            c.w64(8, t442);
            blk = 263; continue;
        }
        176 => {
            let Some(t443) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090d388); };
            c.w32(8, t443);
            c.w64(11, c.r64(8));
            let r444 = c.r32(11) & 0xc0u64;
            c.w32(11, r444);
            c.logic(r444, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t445) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090d3a0); };
            c.w32(11, t445);
            let r446 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r446);
            let Some(t447) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090d3a7); };
            c.w32(11, t447);
            let r448 = c.r32(11) & 0x1fu64;
            c.w32(11, r448);
            c.logic(r448, 32);
            let r449 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r449);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d3b9); };
            c.w64(8, t450);
            blk = 263; continue;
        }
        179 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 184; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t451) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090d3c8); };
            c.w32(8, t451);
            let r452 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r452);
            let Some(t453) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090d3cf); };
            c.cmp(t453, c.r32(8), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t454) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d3d8); };
            c.w32(11, t454);
            let Some(t455) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090d3dc); };
            c.cmp(t455, c.r32(11), 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t456) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090d3e5); };
            c.w32(8, t456);
            let r457 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r457);
            blk = 188; continue;
        }
        183 => {
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d3ee); };
            c.w64(8, t458);
            blk = 263; continue;
        }
        184 => {
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d3f7); };
            c.w64(8, t459);
            blk = 263; continue;
        }
        185 => {
            let Some(t460) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d400); };
            c.w32(8, t460);
            let Some(t461) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090d404); };
            c.cmp(t461, c.r32(8), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d40d); };
            c.w64(8, t462);
            blk = 263; continue;
        }
        187 => {
            let Some(t463) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090d416); };
            c.w32(8, t463);
            let r464 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r464);
            blk = 188; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d424); };
            c.w64(0, t465);
            let Some(t466) = c.ld64(c.r64(0).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d428); };
            c.w64(11, t466);
            let Some(t467) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700090d42c); };
            c.w32(0, t467);
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d431); };
            c.w64(15, t468);
            let Some(t469) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d435); };
            c.w64(1, t469);
            let Some(t470) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d439); };
            c.w64(5, t470);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            let r471 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r471);
            return RegfileExit::jump(c.r64(1));
        }
        189 => {
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d449); };
            c.w64(8, t472);
            blk = 263; continue;
        }
        190 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r473 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r473);
            c.w64(0, c.r64(9).wrapping_add(0x7));
            let Some(t474) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d45d); };
            c.w64(3, t474);
            let Some(t475) = c.ld16(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090d461); };
            c.w32(2, t475);
            let Some(t476) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d467); };
            c.w32(3, t476);
            let r477 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r477);
            let Some(t478) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d46d); };
            c.w32(3, t478);
            let r479 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r479);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090d474); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090d478); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090d479); }
            let Some(t480) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700090d47d); };
            c.w64(3, t480);
            let Some(t481) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090d484); };
            c.w64(6, t481);
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700090d495, 0x700090d490);
        }
        191 => {
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d495); };
            c.w64(8, t482);
            let Some(t483) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d499); };
            c.w64(8, t483);
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090d49d); };
            c.w64(9, t484);
            let Some(t485) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090d4a1); };
            c.w32(11, t485);
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d4a6); };
            c.w64(15, t486);
            let Some(t487) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d4aa); };
            c.w64(1, t487);
            let Some(t488) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d4ae); };
            c.w64(5, t488);
            c.w64(12, c.r64(8));
            let r489 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r489);
            return RegfileExit::jump(c.r64(1));
        }
        192 => {
            let r490 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r490);
            let Some(t491) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d4bf); };
            c.w32(8, t491);
            let Some(t492) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d4c4); };
            c.w64(1, t492);
            let Some(t493) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d4c8); };
            c.w64(5, t493);
            let r494 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r494);
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
            let r495 = c.r64(11) & 0x7ffu64;
            c.w64(11, r495);
            c.logic(r495, 64);
            c.logic(c.r8(1) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 202; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t496) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090d4f9); };
            c.w32(0, t496);
            let r497 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r497);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t498) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090d503); };
            c.w64(0, t498);
            blk = 197; continue;
        }
        197 => {
            let Some(t499) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d50a); };
            c.w32(0, t499);
            let r500 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r500);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let r501 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r501);
            let Some(t502) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d51b); };
            c.w32(8, t502);
            let Some(t503) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d520); };
            c.w64(1, t503);
            let Some(t504) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d524); };
            c.w64(5, t504);
            let r505 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r505);
            return RegfileExit::jump(c.r64(1));
        }
        199 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t506) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d536); };
            c.w32(11, t506);
            let Some(t507) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090d53a); };
            c.cmp(t507, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 201; continue;
        }
        201 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700090d547); }
            blk = 255; continue;
        }
        202 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            let r508 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r508);
            let Some(t509) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d55a); };
            c.w32(8, t509);
            let Some(t510) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d55f); };
            c.w32(0, t510);
            let r511 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r511);
            let Some(t512) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d567); };
            c.w64(1, t512);
            let Some(t513) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d56b); };
            c.w64(5, t513);
            let r514 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r514);
            return RegfileExit::jump(c.r64(1));
        }
        204 => {
            let Some(t515) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d575); };
            c.w32(11, t515);
            let r516 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r516);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t517) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d587); };
            c.w32(0, t517);
            let Some(t518) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090d58b); };
            c.cmp(t518, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 206; continue;
        }
        206 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700090d598); }
            blk = 255; continue;
        }
        207 => {
            c.w64(3, c.r64(1));
            let r519 = c.shr(c.r32(3), 0x7u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r519);
            let r520 = c.r32(3) & 0xfu64;
            c.w32(3, r520);
            c.logic(r520, 32);
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
            return RegfileExit::fault(0x700090d609);
        }
        221 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r521 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r521);
            let r522 = c.r64(1) & 0xfffffu64;
            c.w64(1, r522);
            c.logic(r522, 64);
            let Some(t523) = c.ld8(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d619); };
            c.w32(8, (t523 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t524) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d623); };
            c.w32(8, t524);
            let Some(t525) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d628); };
            c.w64(1, t525);
            let Some(t526) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d62c); };
            c.w64(5, t526);
            let r527 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r527);
            return RegfileExit::jump(c.r64(1));
        }
        222 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r528 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r528);
            let r529 = c.r64(1) & 0xfffffu64;
            c.w64(1, r529);
            c.logic(r529, 64);
            let Some(t530) = c.ld16(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d645); };
            c.w32(8, (t530 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t531) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d64f); };
            c.w32(8, t531);
            let Some(t532) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d654); };
            c.w64(1, t532);
            let Some(t533) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d658); };
            c.w64(5, t533);
            let r534 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r534);
            return RegfileExit::jump(c.r64(1));
        }
        223 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r535 = c.r64(1) & 0xfffffu64;
            c.w64(1, r535);
            c.logic(r535, 64);
            let Some(t536) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d66d); };
            c.w32(8, t536);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090d672); }
            c.w64(11, c.r64(8));
            let r537 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r537);
            if c.cc_o() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 230; continue;
        }
        225 => {
            let Some(t538) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090d686); };
            c.w64(11, t538);
            let Some(t539) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090d68d); };
            c.w64(0, t539);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t540) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090d694); };
            c.w64(2, t540);
            let Some(t541) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090d69b); };
            c.cmp(t541, c.r64(3), 64);
            if c.cc_a() { blk = 228; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090d6aa, 0x700090d6a5);
        }
        227 => {
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d6aa); };
            c.w64(8, t542);
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d6ae); };
            c.w64(15, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d6b2); };
            c.w64(9, t544);
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090d6b6); };
            c.w64(12, t545);
            blk = 229; continue;
        }
        228 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090d6bc); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 229; continue;
        }
        229 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t546) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090d6cf); };
            c.w64(8, t546);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090d6d6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090d6da); }
            blk = 230; continue;
        }
        230 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x7));
            let Some(t547) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d6e6); };
            c.w32(8, t547);
            let Some(t548) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d6eb); };
            c.w64(1, t548);
            let Some(t549) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d6ef); };
            c.w64(5, t549);
            let r550 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r550);
            return RegfileExit::jump(c.r64(1));
        }
        231 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r551 = c.r64(1) & 0xfffffu64;
            c.w64(1, r551);
            c.logic(r551, 64);
            let Some(t552) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d704); };
            c.w32(8, t552);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090d709); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let r553 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r553);
            blk = 238; continue;
        }
        233 => {
            let Some(t554) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090d71b); };
            c.w64(11, t554);
            let Some(t555) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090d722); };
            c.w64(0, t555);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t556) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090d729); };
            c.w64(2, t556);
            let Some(t557) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090d730); };
            c.cmp(t557, c.r64(3), 64);
            if c.cc_a() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090d73f, 0x700090d73a);
        }
        235 => {
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d73f); };
            c.w64(8, t558);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d743); };
            c.w64(15, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d747); };
            c.w64(9, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090d74b); };
            c.w64(12, t561);
            blk = 237; continue;
        }
        236 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090d751); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 237; continue;
        }
        237 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t562) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090d767); };
            c.w64(8, t562);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090d76e); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090d772); }
            c.w64(8, c.r64(0));
            blk = 238; continue;
        }
        238 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t563) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d781); };
            c.w32(11, t563);
            let Some(t564) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d786); };
            c.w64(1, t564);
            let Some(t565) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d78a); };
            c.w64(5, t565);
            c.w64(0, c.r64(8));
            let r566 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r566);
            return RegfileExit::jump(c.r64(1));
        }
        239 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r567 = c.r64(1) & 0xfffffu64;
            c.w64(1, r567);
            c.logic(r567, 64);
            let Some(t568) = c.ld64(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d7a2); };
            c.w64(8, t568);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090d7a7); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 247; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t569) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090d7b1); };
            c.w64(11, t569);
            let Some(t570) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090d7b8); };
            c.w64(0, t570);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090d7bf); };
            c.w64(2, t571);
            let Some(t572) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090d7c6); };
            c.cmp(t572, c.r64(3), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700090d7d5, 0x700090d7d0);
        }
        242 => {
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d7d5); };
            c.w64(8, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d7d9); };
            c.w64(15, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d7dd); };
            c.w64(9, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090d7e1); };
            c.w64(12, t576);
            blk = 244; continue;
        }
        243 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090d7e7); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 244; continue;
        }
        244 => {
            let Some(t577) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090d7f1); };
            c.w64(3, t577);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700090d7f8); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700090d801); }
            let r578 = c.neg(c.r64(8), 64);
            c.w64(8, r578);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700090d80b); }
            blk = 252; continue;
        }
        246 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700090d811); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700090d818); }
            blk = 252; continue;
        }
        247 => {
            let Some(t579) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090d81e); };
            c.w64(8, t579);
            let Some(t580) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090d825); };
            c.w64(11, t580);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t581) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090d82c); };
            c.w64(0, t581);
            let Some(t582) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700090d833); };
            c.cmp(t582, c.r64(3), 64);
            if c.cc_a() { blk = 250; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700090d842, 0x700090d83d);
        }
        249 => {
            blk = 251; continue;
        }
        250 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x700090d844); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 251; continue;
        }
        251 => {
            let Some(t583) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090d84e); };
            c.w64(8, t583);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090d855); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700090d859); }
            let Some(t584) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090d860); };
            c.w64(12, t584);
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d864); };
            c.w64(15, t585);
            let Some(t586) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d868); };
            c.w64(9, t586);
            blk = 252; continue;
        }
        252 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x7));
            let Some(t587) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d873); };
            c.w32(8, t587);
            let Some(t588) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d878); };
            c.w64(1, t588);
            let Some(t589) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d87c); };
            c.w64(5, t589);
            let r590 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r590);
            return RegfileExit::jump(c.r64(1));
        }
        253 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r591 = c.r64(1) & 0xfffffu64;
            c.w64(1, r591);
            c.logic(r591, 64);
            if !c.movss_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090d891); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 255; continue;
        }
        254 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r592 = c.r64(1) & 0xfffffu64;
            c.w64(1, r592);
            c.logic(r592, 64);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090d8a8); }
            blk = 255; continue;
        }
        255 => {
            let Some(t593) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090d8ae); };
            c.w64(8, t593);
            let Some(t594) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090d8b5); };
            c.w64(11, t594);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700090d8bc); }
            let Some(t595) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090d8c1); };
            c.w64(3, t595);
            let Some(t596) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700090d8c8); };
            c.cmp(t596, c.r64(0), 64);
            if c.cc_a() { blk = 258; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090d8d7, 0x700090d8d2);
        }
        257 => {
            blk = 259; continue;
        }
        258 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700090d8d9); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 259; continue;
        }
        259 => {
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090d8e3); };
            c.w64(3, t597);
            c.w64(9, c.r64(3).wrapping_add(0x7));
            let Some(t598) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090d8eb); };
            c.w64(8, t598);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090d8f2); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700090d8f6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090d8fb); }
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090d900); };
            c.w64(12, t599);
            let Some(t600) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d904); };
            c.w32(8, t600);
            let Some(t601) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d909); };
            c.w64(15, t601);
            let Some(t602) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d90d); };
            c.w64(1, t602);
            let Some(t603) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d911); };
            c.w64(5, t603);
            let r604 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r604);
            return RegfileExit::jump(c.r64(1));
        }
        260 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r605 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r605);
            let r606 = c.r64(1) & 0xfffffu64;
            c.w64(1, r606);
            c.logic(r606, 64);
            let Some(t607) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090d92a); };
            c.w32(8, t607);
            let Some(t608) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090d92f); };
            c.w32(0, t608);
            let r609 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r609);
            let Some(t610) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d936); };
            c.w64(1, t610);
            let Some(t611) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d93a); };
            c.w64(5, t611);
            let r612 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r612);
            return RegfileExit::jump(c.r64(1));
        }
        261 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r613 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r613);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090d94a); }
            let Some(t614) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700090d94e); };
            c.w64(8, t614);
            let Some(t615) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d955); };
            c.w32(8, t615);
            let r616 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r616);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090d95c); }
            let Some(t617) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700090d95e); };
            c.w64(3, t617);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700090d972, 0x700090d96d);
        }
        262 => {
            return RegfileExit::fault(0x700090d972);
        }
        263 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r618 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r618);
            c.w64(0, c.r64(9).wrapping_add(0x7));
            let Some(t619) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090d97e); };
            c.w32(3, t619);
            let Some(t620) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d984); };
            c.w32(2, t620);
            let r621 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r621);
            let Some(t622) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090d98c); };
            c.w64(1, t622);
            let Some(t623) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090d990); };
            c.w32(3, t623);
            let r624 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r624);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090d997); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090d99b); }
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090d99c); };
            if !c.push64(t625) { return RegfileExit::fault(0x700090d99c); }
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090d99f); };
            if !c.push64(t626) { return RegfileExit::fault(0x700090d99f); }
            c.w32(11, 0x2u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700090d9a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090d9aa); }
            let Some(t627) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x700090d9ae); };
            c.w64(3, t627);
            c.w64(6, c.r64(1));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700090d9c5, 0x700090d9c0);
        }
        264 => {
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d9c5); };
            c.w64(8, t628);
            let Some(t629) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090d9c9); };
            c.w64(8, t629);
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090d9cd); };
            c.w64(9, t630);
            let Some(t631) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090d9d1); };
            c.w32(11, t631);
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090d9d6); };
            c.w64(15, t632);
            let Some(t633) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090d9da); };
            c.w64(1, t633);
            let Some(t634) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090d9de); };
            c.w64(5, t634);
            c.w64(12, c.r64(8));
            let r635 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r635);
            return RegfileExit::jump(c.r64(1));
        }
        265 => {
            return RegfileExit::fault(0x700090d9eb);
        }
        266 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090d9ec); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090d9f0); }
            let r636 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r636);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r637 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r637);
            c.w64(0, c.r64(9).wrapping_add(0x7));
            let Some(t638) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090da01); };
            c.w32(3, t638);
            let Some(t639) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090da07); };
            c.w32(2, t639);
            let r640 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r640);
            let Some(t641) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090da0f); };
            c.w64(1, t641);
            let Some(t642) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090da13); };
            c.w32(3, t642);
            let r643 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r643);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090da1a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090da1e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090da22); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090da26); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            c.w32(0, 0xcu64);
            c.w64(11, c.r64(0));
            return c.call(0x7000804df4, 0x700090da3d, 0x700090da38);
        }
        267 => {
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090da3d); };
            c.w64(8, t644);
            let Some(t645) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090da41); };
            c.w64(8, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090da45); };
            c.w64(9, t646);
            let Some(t647) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090da49); };
            c.w32(11, t647);
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090da4e); };
            c.w64(15, t648);
            let Some(t649) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090da52); };
            c.w64(1, t649);
            let Some(t650) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090da56); };
            c.w64(5, t650);
            c.w64(12, c.r64(8));
            let r651 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r651);
            return RegfileExit::jump(c.r64(1));
        }
        268 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090da63); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090da67); }
            let r652 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r652);
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r653 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r653);
            let Some(t654) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090da73); };
            c.w32(3, t654);
            let Some(t655) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090da79); };
            c.w32(2, t655);
            let r656 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r656);
            let Some(t657) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090da81); };
            c.w32(3, t657);
            let r658 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r658);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090da88); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090da8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090da8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090da91); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090da95); }
            let Some(t659) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x700090da99); };
            c.w64(3, t659);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700090daad, 0x700090daa8);
        }
        269 => {
            let Some(t660) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090daad); };
            c.w64(8, t660);
            let Some(t661) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090dab1); };
            c.w64(8, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090dab5); };
            c.w64(11, t662);
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t663) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090dabd); };
            c.w32(11, t663);
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090dac2); };
            c.w64(15, t664);
            let Some(t665) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090dac6); };
            c.w64(1, t665);
            let Some(t666) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090daca); };
            c.w64(5, t666);
            c.w64(12, c.r64(8));
            let r667 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r667);
            return RegfileExit::jump(c.r64(1));
        }
        270 => {
            return RegfileExit::jump(0x700090dad8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090daf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090daf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090daf8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090dafe); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090db04); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090db0c); };
            c.w64(8, t4);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090db14); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090db1b); };
            c.w64(7, t7);
            let r8 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r8);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090db22); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090db26); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090db27); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090db2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090db2f); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x700090db33); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700090db47, 0x700090db42);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090db47); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090db4b); };
            c.w64(2, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090db4f); };
            c.w64(7, t12);
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700090db57); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090db5c); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090db60); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090db64); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x700090db74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090db94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090db97); };
            c.w64(7, t0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090db9b); };
            c.w32(8, t1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090dba1); };
            c.w32(11, t2);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dbaa); };
            c.w32(3, t3);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090dbb9); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dbbf); };
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
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090dbd7); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090dbda); };
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
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090dbe9); };
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
            return RegfileExit::fault(0x700090dbf7);
        }
        9 => {
            return RegfileExit::fault(0x700090dbf8);
        }
        10 => {
            return RegfileExit::fault(0x700090dbf9);
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dbfa); };
            c.w32(2, t12);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700090dc08);
        }
        13 => {
            let r13 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090dc0d); };
            c.w32(2, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090dc12); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700090dc1a); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090dc20); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090dc24); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            let Some(t19) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090dc2a); };
            c.w32(8, t19);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090dc34); };
            c.w32(0, t20);
            let r21 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r21);
            let r22 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090dc3f); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090dc47); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090dc4b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090dc4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090dc51); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700090dc55); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x700090dc59); };
            c.w64(3, t25);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700090dc6d, 0x700090dc68);
        }
        15 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090dc6d); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090dc71); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090dc75); };
            c.w64(9, t28);
            let r29 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r29);
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700090dc7d); };
            c.w32(7, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090dc82); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090dc86); };
            c.w64(1, t32);
            let Some(t33) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090dc8a); };
            c.w64(5, t33);
            c.w64(12, c.r64(2));
            let r34 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            return RegfileExit::jump(c.r64(1));
        }
        16 => {
            return RegfileExit::jump(0x700090dc98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090dcb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090dcb7); };
            c.w64(11, t0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090dcbb); };
            c.w32(0, t1);
            c.w64(3, c.r64(11));
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dcc4); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090dcd3); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dcd9); };
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
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090dcf1); };
            c.w32(1, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090dcf4); };
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
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090dd07); };
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
            return RegfileExit::fault(0x700090dd14);
        }
        9 => {
            return RegfileExit::fault(0x700090dd15);
        }
        10 => {
            return RegfileExit::fault(0x700090dd16);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090dd17); };
            c.w64(11, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090dd1b); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dd22); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090dd29); };
            c.w32(0, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090dd2d); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090dd36); };
            c.w64(11, t18);
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090dd3d); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dd4a); };
            c.w32(0, t20);
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(3, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090dd57); };
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
            let Some(t24) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090dd71); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090dd77); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r27);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090dd88); };
            c.w32(8, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090dd8d); };
            c.w64(1, t29);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090dd96); };
            c.w64(8, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090dd9a); };
            c.w32(8, t31);
            let r32 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090dda1); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dda8); };
            c.w32(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x700090ddbb);
        }
        20 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x700090ddbc); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090ddc9); };
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
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090dded); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090ddf5); };
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
            return RegfileExit::fault(0x700090de0f);
        }
        23 => {
            let r44 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090de14); };
            c.w32(0, t45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090de19); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090de21); };
            c.w64(1, t48);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090de2a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090de2e); }
            let r49 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090de34); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090de38); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700090de3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700090de40); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090de4c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090de50); }
            let r50 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090de56); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090de5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090de62); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090de66); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700090de6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700090de6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700090de72); }
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700090de76); };
            c.cmp(t53, c.r32(7), 32);
            if c.cc_e() { blk = 262; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t54) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090de83); };
            c.w64(3, t54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090de87); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090de8d); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090de93); };
            c.w32(1, t59);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x700090de9c); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x700090dea6);
        }
        28 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090dea7); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700090dead); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x700090deb7);
        }
        30 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090deb8); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090dec8); };
            c.w32(1, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700090decb); };
            c.cmp(t65, c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ded4); };
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
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090dee2); };
            c.cmp(t68, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090deec); };
            c.w32(1, t69);
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090deef); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700090def9); };
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
            let r74 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r74);
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090df20); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090df25); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090df29); };
            c.w64(5, t77);
            let r78 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r78);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r79 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r79);
            c.w64(6, c.r64(9).wrapping_add(0x7));
            let Some(t80) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090df3d); };
            c.w32(9, t80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090df43); };
            c.w32(12, t81);
            let r82 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090df4b); };
            c.w32(9, t83);
            let r84 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r84);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700090df53); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700090df57); }
            c.w64(1, c.r64(9));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(11));
            let Some(t85) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090df64); };
            c.logic(t85 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r86 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r86);
            blk = 43; continue;
        }
        42 => {
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090df73); };
            c.w32(7, t87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700090df76); };
            c.w64(7, t88);
            blk = 43; continue;
        }
        43 => {
            return c.call(c.r64(7), 0x700090df80, 0x700090df7e);
        }
        44 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090df80); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090df84); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090df88); };
            c.w64(9, t91);
            let Some(t92) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090df8c); };
            c.w32(11, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090df91); };
            c.w64(15, t93);
            let Some(t94) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090df95); };
            c.w64(1, t94);
            let Some(t95) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090df99); };
            c.w64(5, t95);
            c.w64(12, c.r64(8));
            let r96 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r96);
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
            let r97 = c.r64(3) & 0xfu64;
            c.w64(3, r97);
            c.logic(r97, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090dfbc); }
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
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r98 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r98);
            c.w64(3, c.r64(9).wrapping_add(0x7));
            let Some(t99) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090e04a); };
            c.w32(2, t99);
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e050); };
            c.w32(1, t100);
            let r101 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r101);
            let Some(t102) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e058); };
            c.w32(2, t102);
            let r103 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r103);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090e05f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090e063); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e064); };
            if !c.push64(t104) { return RegfileExit::fault(0x700090e064); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090e067); }
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090e068); };
            if !c.push64(t105) { return RegfileExit::fault(0x700090e068); }
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090e06b); };
            if !c.push64(t106) { return RegfileExit::fault(0x700090e06b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x700090e06e); }
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e072); };
            c.w64(6, t107);
            c.w32(0, 0x5u64);
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700090e07b); };
            c.w64(3, t108);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700090e08a, 0x700090e085);
        }
        59 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e08a); };
            c.w64(8, t109);
            let Some(t110) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e08e); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700090e092); };
            c.w64(9, t111);
            let Some(t112) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090e096); };
            c.w32(11, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e09b); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e09f); };
            c.w64(1, t114);
            let Some(t115) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e0a3); };
            c.w64(5, t115);
            c.w64(12, c.r64(8));
            let r116 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r116);
            return RegfileExit::jump(c.r64(1));
        }
        60 => {
            let Some(t117) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090e0b0); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e0b7); };
            c.w32(11, t119);
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            let Some(t121) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e0be); };
            c.w32(0, t121);
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r122 = c.r64(1) & 0x1ffffffu64;
            c.w64(1, r122);
            c.logic(r122, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_b() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x700090e0d8);
        }
        62 => {
            let Some(t123) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e0d9); };
            c.w32(11, t123);
            let r124 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r124);
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e0e1); };
            c.w32(0, t125);
            let r126 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r126);
            c.w64(11, c.r64(9).wrapping_add(0x7));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700090e0ec); }
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090e0f0); };
            c.cmp(t127, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t128) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090e0f9); };
            c.w32(8, t128);
            let Some(t129) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e0fe); };
            c.w64(1, t129);
            let Some(t130) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e102); };
            c.w64(5, t130);
            c.w64(9, c.r64(11));
            let r131 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r131);
            return RegfileExit::jump(c.r64(1));
        }
        64 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r132 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r132);
            let Some(t133) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090e115); };
            c.w32(3, t133);
            let Some(t134) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e11b); };
            c.w32(2, t134);
            let r135 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r135);
            let Some(t136) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e123); };
            c.w32(3, t136);
            let r137 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r137);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090e12a); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090e133); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090e134); }
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700090e135); };
            c.w64(3, t138);
            c.w32(0, 0x2u64);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e141); };
            c.w64(6, t139);
            return c.call(0x700086ed74, 0x700090e14a, 0x700090e145);
        }
        65 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e14a); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e14e); };
            c.w64(8, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090e152); };
            c.w64(9, t142);
            let Some(t143) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090e156); };
            c.w32(11, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e15b); };
            c.w64(15, t144);
            let Some(t145) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e15f); };
            c.w64(1, t145);
            let Some(t146) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e163); };
            c.w64(5, t146);
            c.w64(12, c.r64(8));
            let r147 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r147);
            return RegfileExit::jump(c.r64(1));
        }
        66 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r148 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r148);
            c.w64(3, c.r64(9).wrapping_add(0x7));
            let Some(t149) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090e17b); };
            c.w32(2, t149);
            let Some(t150) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e181); };
            c.w32(1, t150);
            let r151 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r151);
            let Some(t152) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e189); };
            c.w32(2, t152);
            let r153 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r153);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090e190); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700090e194); }
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e198); };
            c.w64(6, t154);
            c.w64(3, c.r64(2));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e19f); };
            c.w64(1, t155);
            c.w32(2, 0x2u64);
            c.w64(11, c.r64(2));
            return c.call(0x70008bc174, 0x700090e1b0, 0x700090e1ab);
        }
        67 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e1b0); };
            c.w64(8, t156);
            let Some(t157) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e1b4); };
            c.w64(8, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090e1b8); };
            c.w64(9, t158);
            let Some(t159) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090e1bc); };
            c.w32(11, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e1c1); };
            c.w64(15, t160);
            let Some(t161) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e1c5); };
            c.w64(1, t161);
            let Some(t162) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e1c9); };
            c.w64(5, t162);
            c.w64(12, c.r64(8));
            let r163 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r163);
            return RegfileExit::jump(c.r64(1));
        }
        68 => {
            blk = 125; continue;
        }
        69 => {
            let Some(t164) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090e1db); };
            c.w32(3, t164);
            let r165 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r165);
            let Some(t166) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e1e1); };
            c.w32(2, t166);
            let Some(t167) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090e1e4); };
            c.cmp(t167, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t168) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090e1f1); };
            c.w32(1, t168);
            let Some(t169) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e1f7); };
            c.w32(6, t169);
            let r170 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r170);
            let Some(t171) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e1ff); };
            c.w32(1, t171);
            let r172 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r172);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t173) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e20f); };
            c.w32(2, t173);
            let r174 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090e215); };
            c.cmp(t175, c.r32(2), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t176) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e222); };
            c.w32(2, t176);
            let r177 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r177);
            let Some(t178) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e228); };
            c.w32(6, t178);
            let r179 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r179);
            let Some(t180) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090e22e); };
            c.logic(t180 & 0x2u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090e238); };
            c.cmp(t181, c.r32(6), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090e241); };
            c.w64(3, t182);
            blk = 109; continue;
        }
        75 => {
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090e24d); };
            c.w64(6, t183);
            let Some(t184) = c.ld8(c.r64(6)) else { return RegfileExit::fault(0x700090e254); };
            c.cmp(t184, 0x20u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t185) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090e25d); };
            c.w32(1, t185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090e260); };
            c.cmp(t186, c.r32(1), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t187) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e26d); };
            c.w32(1, t187);
            let r188 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r188);
            let Some(t189) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e273); };
            c.w32(6, t189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090e276); };
            c.cmp(t190, c.r32(6), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t191) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090e27f); };
            c.w32(1, t191);
            let r192 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r192);
            let Some(t193) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e285); };
            c.w32(6, t193);
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090e288); };
            c.cmp(t194, c.r32(6), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t195) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e29a); };
            c.w32(6, t195);
            let r196 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r196);
            let Some(t197) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e2a0); };
            c.w32(7, t197);
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
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090e2bc); };
            c.cmp(t198, c.r32(6), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090e2c5); };
            c.w64(12, t199);
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
            let Some(t200) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090e2d6); };
            c.w32(0, t200);
            let r201 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r201);
            let Some(t202) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e2dc); };
            c.w32(0, t202);
            let r203 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r203);
            let Some(t204) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e2e2); };
            c.w32(3, t204);
            let r205 = c.sub(c.r32(3), 0x89u64, 32);
            c.w32(3, r205);
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_a() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r206 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r206);
            let Some(t207) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090e2f7); };
            c.w32(6, t207);
            let r208 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r208);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700090e2fd); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e301); };
            if !c.push64(t209) { return RegfileExit::fault(0x700090e301); }
            c.w64(1, c.r64(11));
            return c.call(0x70007f6fb4, 0x700090e30c, 0x700090e307);
        }
        88 => {
            c.w64(3, c.r64(0));
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e30f); };
            c.w64(15, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e313); };
            c.w64(9, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e317); };
            c.w64(8, t212);
            blk = 109; continue;
        }
        89 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090e320); };
            c.w64(12, t213);
            blk = 263; continue;
        }
        90 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r214 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r214);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090e330); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090e334); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700090e335); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e336); };
            if !c.push64(t215) { return RegfileExit::fault(0x700090e336); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e339); };
            c.w64(6, t216);
            c.w32(0, 0x3u64);
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090e342); };
            c.w64(3, t217);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700090e351, 0x700090e34c);
        }
        91 => {
            c.w64(3, c.r64(0));
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e354); };
            c.w64(15, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e358); };
            c.w64(9, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e35c); };
            c.w64(8, t220);
            blk = 109; continue;
        }
        92 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r221 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r221);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090e36e); }
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e372); };
            if !c.push64(t222) { return RegfileExit::fault(0x700090e372); }
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e375); };
            c.w64(6, t223);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x700090e383, 0x700090e37e);
        }
        93 => {
            c.w64(3, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e386); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e38a); };
            c.w64(9, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e38e); };
            c.w64(8, t226);
            blk = 109; continue;
        }
        94 => {
            let Some(t227) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e397); };
            c.w32(1, t227);
            let r228 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r228);
            let Some(t229) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e39d); };
            c.w32(6, t229);
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
            let Some(t230) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e3c2); };
            c.w32(0, t230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090e3c5); };
            c.cmp(t231, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t232) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e3db); };
            c.cmp(t232, 0x40u64, 16);
            if c.cc_ae() { blk = 263; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t233) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e3e6); };
            c.w32(11, t233);
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            blk = 109; continue;
        }
        101 => {
            let Some(t234) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e3f3); };
            c.w32(2, t234);
            let Some(t235) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090e3f6); };
            c.cmp(t235, c.r32(2), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t236) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090e403); };
            c.w32(2, t236);
            c.w64(1, c.r64(2));
            let r237 = c.r32(1) & 0xc0u64;
            c.w32(1, r237);
            c.logic(r237, 32);
            c.cmp(c.r32(1), 0xc0u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t238) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090e418); };
            c.w32(3, t238);
            let r239 = c.r32(3) & 0x1fu64;
            c.w32(3, r239);
            c.logic(r239, 32);
            let r240 = c.sub(c.r32(3), 0x10u64, 32);
            c.w32(3, r240);
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
            let Some(t241) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090e433); };
            c.w32(3, t241);
            let r242 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r242);
            let Some(t243) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090e439); };
            c.cmp(t243, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t244) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e446); };
            c.w32(11, t244);
            let Some(t245) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090e44a); };
            c.cmp(t245, c.r32(11), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t246) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090e453); };
            c.w32(3, t246);
            let r247 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r247);
            blk = 109; continue;
        }
        108 => {
            let Some(t248) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e45b); };
            c.w32(0, t248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090e45e); };
            c.cmp(t249, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 109; continue;
        }
        109 => {
            let r250 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r250);
            let Some(t251) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e46f); };
            c.w64(8, t251);
            let Some(t252) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090e473); };
            c.w32(11, t252);
            let Some(t253) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e478); };
            c.w64(1, t253);
            let Some(t254) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e47c); };
            c.w64(5, t254);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(3));
            let r255 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r255);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 263; continue;
        }
        111 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090e491); };
            c.w64(3, t256);
            let Some(t257) = c.ld8(c.r64(3)) else { return RegfileExit::fault(0x700090e498); };
            c.cmp(t257, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t258) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090e4a6); };
            c.w32(3, t258);
            let r259 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r259);
            blk = 115; continue;
        }
        114 => {
            let Some(t260) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090e4ae); };
            c.w32(3, t260);
            let r261 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r261);
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return RegfileExit::fault(0x700090e4b9);
        }
        117 => {
            let r262 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r262);
            c.logic(r262, 64);
            let Some(t263) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e4be); };
            c.w32(1, t263);
            let r264 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r264);
            let Some(t265) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090e4c4); };
            c.w32(0, t265);
            let r266 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r266);
            let Some(t267) = c.ld64(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e4ca); };
            c.w64(1, t267);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t268) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e4d3); };
            c.w32(2, t268);
            let r269 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r269);
            let Some(t270) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090e4d9); };
            c.w32(2, t270);
            let r271 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r271);
            blk = 119; continue;
        }
        119 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r272 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r272);
            let r273 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r273);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700090e4e9); }
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e4ed); };
            if !c.push64(t274) { return RegfileExit::fault(0x700090e4ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090e4f0); }
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            return c.call(0x70007fe8d4, 0x700090e508, 0x700090e503);
        }
        120 => {
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e508); };
            c.w64(8, t275);
            let Some(t276) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e50c); };
            c.w64(8, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e510); };
            c.w64(9, t277);
            let Some(t278) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090e514); };
            c.w32(11, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e519); };
            c.w64(15, t279);
            let Some(t280) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e51d); };
            c.w64(1, t280);
            let Some(t281) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e521); };
            c.w64(5, t281);
            c.w64(12, c.r64(8));
            let r282 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r282);
            return RegfileExit::jump(c.r64(1));
        }
        121 => {
            blk = 125; continue;
        }
        122 => {
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090e530); };
            c.w64(11, t283);
            let Some(t284) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090e537); };
            c.cmp(t284, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            let r285 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r285);
            let r286 = c.r64(1) & 0x3ffu64;
            c.w64(1, r286);
            c.logic(r286, 64);
            let r287 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r287);
            let r288 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r288);
            let Some(t289) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e557); };
            c.w32(2, t289);
            let r290 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r290);
            let Some(t291) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090e55d); };
            c.w32(2, t291);
            let r292 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r292);
            let Some(t293) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090e563); };
            c.w32(3, t293);
            let r294 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r294);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090e56a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090e56e); }
            c.w64(2, c.r64(6));
            c.w64(1, c.r64(0));
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e578); };
            c.w64(6, t295);
            return c.call(0x70007fea14, 0x700090e581, 0x700090e57c);
        }
        124 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e581); };
            c.w64(8, t296);
            let Some(t297) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e585); };
            c.w64(8, t297);
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e589); };
            c.w64(9, t298);
            let Some(t299) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090e58d); };
            c.w32(11, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e592); };
            c.w64(15, t300);
            let Some(t301) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e596); };
            c.w64(1, t301);
            let Some(t302) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e59a); };
            c.w64(5, t302);
            c.w64(12, c.r64(8));
            let r303 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r303);
            return RegfileExit::jump(c.r64(1));
        }
        125 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r304 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r304);
            c.w64(0, c.r64(9).wrapping_add(0x7));
            let Some(t305) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090e5b2); };
            c.w32(3, t305);
            let Some(t306) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e5b8); };
            c.w32(2, t306);
            let r307 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r307);
            let Some(t308) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e5c0); };
            c.w32(3, t308);
            let r309 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r309);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090e5c7); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090e5cb); }
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090e5cc); };
            if !c.push64(t310) { return RegfileExit::fault(0x700090e5cc); }
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090e5cf); };
            if !c.push64(t311) { return RegfileExit::fault(0x700090e5cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090e5d2); }
            let Some(t312) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x700090e5d6); };
            c.w64(3, t312);
            c.w32(0, 0x3u64);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e5e2); };
            c.w64(6, t313);
            return c.call(0x700086ed74, 0x700090e5eb, 0x700090e5e6);
        }
        126 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e5eb); };
            c.w64(8, t314);
            let Some(t315) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e5ef); };
            c.w64(8, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090e5f3); };
            c.w64(9, t316);
            let Some(t317) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090e5f7); };
            c.w32(11, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e5fc); };
            c.w64(15, t318);
            let Some(t319) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e600); };
            c.w64(1, t319);
            let Some(t320) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e604); };
            c.w64(5, t320);
            c.w64(12, c.r64(8));
            let r321 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r321);
            return RegfileExit::jump(c.r64(1));
        }
        127 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            let r322 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r322);
            let r323 = c.r64(1) & 0x3ffu64;
            c.w64(1, r323);
            c.logic(r323, 64);
            let r324 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r324);
            let r325 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r325);
            let Some(t326) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e62b); };
            c.w32(0, t326);
            let r327 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r327);
            let Some(t328) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090e631); };
            c.w32(0, t328);
            let r329 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r329);
            let Some(t330) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700090e637); };
            c.w32(0, t330);
            let r331 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r331);
            let Some(t332) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e63e); };
            c.w32(7, t332);
            let r333 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r333);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090e644); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700090e648); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090e649); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e64f); };
            c.w64(6, t334);
            return c.call(0x70007f2274, 0x700090e658, 0x700090e653);
        }
        128 => {
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e658); };
            c.w64(8, t335);
            let Some(t336) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e65c); };
            c.w64(8, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e660); };
            c.w64(9, t337);
            let Some(t338) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090e664); };
            c.w32(11, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090e669); };
            c.w64(15, t339);
            let Some(t340) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090e66d); };
            c.w64(1, t340);
            let Some(t341) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090e671); };
            c.w64(5, t341);
            c.w64(12, c.r64(8));
            let r342 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r342);
            return RegfileExit::jump(c.r64(1));
        }
        129 => {
            let Some(t343) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e67e); };
            c.w32(3, t343);
            let r344 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r344);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700090e689); };
            c.w64(3, t345);
            blk = 131; continue;
        }
        131 => {
            let Some(t346) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090e690); };
            c.w32(2, t346);
            let Some(t347) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090e693); };
            c.w32(1, t347);
            let Some(t348) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e699); };
            c.w32(7, t348);
            let r349 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r349);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t350) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e6a4); };
            c.w32(1, t350);
            let r351 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r351);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r352 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r352);
            let Some(t353) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e6b2); };
            c.w32(6, t353);
            let r354 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r354);
            let r355 = c.r64(6) & c.r64(2);
            c.w64(6, r355);
            c.logic(r355, 64);
            blk = 132; continue;
        }
        132 => {
            c.w64(15, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(8, c.r64(15).wrapping_mul(4));
            let Some(t356) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e6c8); };
            c.w32(0, t356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090e6cd); };
            c.cmp(t357, c.r32(0), 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r358 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r358);
            c.w64(8, c.r64(11).wrapping_add(c.r64(6)));
            c.w64(0, c.r64(2));
            let r359 = c.r64(0) & c.r64(8);
            c.w64(0, r359);
            c.logic(r359, 64);
            c.w64(6, c.r64(0));
            blk = 132; continue;
        }
        135 => {
            let Some(t360) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e6f1); };
            c.w32(11, t360);
            let Some(t361) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090e6f5); };
            c.w32(0, t361);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(2, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_b() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            return RegfileExit::fault(0x700090e709);
        }
        137 => {
            let Some(t362) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090e70a); };
            c.w32(11, t362);
            let Some(t363) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090e710); };
            c.w32(8, t363);
            let r364 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r364);
            let Some(t365) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e718); };
            c.w32(11, t365);
            let r366 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r366);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t367) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e728); };
            c.w32(0, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090e72c); };
            c.cmp(t368, c.r32(0), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t369) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e739); };
            c.w32(2, t369);
            let r370 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r370);
            let Some(t371) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e740); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090e747); };
            c.logic(t373 & 0x2u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090e752); };
            c.cmp(t374, c.r32(8), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t375) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090e75b); };
            c.w64(8, t375);
            blk = 188; continue;
        }
        142 => {
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090e767); };
            c.w64(8, t376);
            let Some(t377) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x700090e76e); };
            c.cmp(t377, 0x20u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t378) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090e778); };
            c.w32(8, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090e77c); };
            c.cmp(t379, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e785); };
            c.w64(8, t380);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        145 => {
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090e791); };
            c.w64(0, t381);
            let Some(t382) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e795); };
            c.w32(8, t382);
            let r383 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r383);
            let Some(t384) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e79c); };
            c.w32(11, t384);
            let Some(t385) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090e7a0); };
            c.cmp(t385, c.r32(11), 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t386) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090e7a9); };
            c.w32(8, t386);
            let r387 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e7b0); };
            c.w32(11, t388);
            let Some(t389) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090e7b4); };
            c.cmp(t389, c.r32(11), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 147; continue;
        }
        147 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t390) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e7c7); };
            c.w32(11, t390);
            let r391 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r391);
            let Some(t392) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e7ce); };
            c.w32(3, t392);
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
            let Some(t393) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090e7eb); };
            c.cmp(t393, c.r32(11), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e7f4); };
            c.w64(8, t394);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        152 => {
            let Some(t395) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090e800); };
            c.w32(0, t395);
            let r396 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r396);
            blk = 154; continue;
        }
        153 => {
            c.w64(0, c.r64(8));
            blk = 154; continue;
        }
        154 => {
            let Some(t397) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090e80b); };
            c.w32(8, t397);
            let r398 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r398);
            let Some(t399) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e812); };
            c.w32(8, t399);
            let r400 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r400);
            let Some(t401) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e819); };
            c.w32(11, t401);
            let r402 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r402);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r403 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r403);
            let Some(t404) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090e832); };
            c.w32(6, t404);
            let r405 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e839); };
            c.w64(8, t406);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090e83d); }
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e841); };
            if !c.push64(t407) { return RegfileExit::fault(0x700090e841); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700090e84b, 0x700090e846);
        }
        156 => {
            c.w64(8, c.r64(0));
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e84e); };
            c.w64(9, t408);
            blk = 188; continue;
        }
        157 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e857); };
            c.w64(8, t409);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        158 => {
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e863); };
            c.w64(8, t410);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        159 => {
            c.w64(8, c.r64(9));
            c.w64(0, c.r64(8).wrapping_sub(0x1));
            let r411 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e878); };
            c.w64(3, t412);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090e87c); }
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090e880); };
            if !c.push64(t413) { return RegfileExit::fault(0x700090e880); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700090e883); }
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e885); };
            if !c.push64(t414) { return RegfileExit::fault(0x700090e885); }
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e888); };
            c.w64(6, t415);
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090e88c); };
            c.w64(3, t416);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700090e8a0, 0x700090e89b);
        }
        160 => {
            c.w64(8, c.r64(0));
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e8a3); };
            c.w64(9, t417);
            blk = 188; continue;
        }
        161 => {
            c.w64(8, c.r64(9));
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r418 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e8b6); };
            c.w64(0, t419);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090e8ba); }
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090e8be); };
            if !c.push64(t420) { return RegfileExit::fault(0x700090e8be); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700090e8c6); };
            c.w64(6, t421);
            return c.call(0x70007f2274, 0x700090e8cf, 0x700090e8ca);
        }
        162 => {
            c.w64(8, c.r64(0));
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090e8d2); };
            c.w64(9, t422);
            blk = 188; continue;
        }
        163 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090e8db); };
            c.w64(0, t423);
            let Some(t424) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e8df); };
            c.w32(11, t424);
            let r425 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r425);
            let Some(t426) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e8e6); };
            c.w32(3, t426);
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
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e908); };
            c.w64(8, t427);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        167 => {
            let Some(t428) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e914); };
            c.w32(8, t428);
            let Some(t429) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090e918); };
            c.cmp(t429, c.r32(8), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e921); };
            c.w64(8, t430);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        169 => {
            let Some(t431) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090e92d); };
            c.w32(8, t431);
            let r432 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r432);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e93a); };
            c.w64(8, t433);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        171 => {
            let Some(t434) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e946); };
            c.w32(11, t434);
            let r435 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r435);
            let Some(t436) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e94d); };
            c.cmp(t436, 0x40u64, 16);
            if c.cc_b() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e955); };
            c.w64(8, t437);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        173 => {
            let Some(t438) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090e961); };
            c.w32(8, t438);
            let r439 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r439);
            blk = 188; continue;
        }
        174 => {
            let Some(t440) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090e96d); };
            c.w32(8, t440);
            let Some(t441) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090e971); };
            c.cmp(t441, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e97a); };
            c.w64(8, t442);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        176 => {
            let Some(t443) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090e986); };
            c.w32(8, t443);
            c.w64(11, c.r64(8));
            let r444 = c.r32(11) & 0xc0u64;
            c.w32(11, r444);
            c.logic(r444, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t445) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090e99e); };
            c.w32(11, t445);
            let r446 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r446);
            let Some(t447) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090e9a5); };
            c.w32(11, t447);
            let r448 = c.r32(11) & 0x1fu64;
            c.w32(11, r448);
            c.logic(r448, 32);
            let r449 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r449);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e9b7); };
            c.w64(8, t450);
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
            let Some(t451) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090e9c9); };
            c.w32(8, t451);
            let r452 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r452);
            let Some(t453) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090e9d0); };
            c.cmp(t453, c.r32(8), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t454) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090e9d9); };
            c.w32(11, t454);
            let Some(t455) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090e9dd); };
            c.cmp(t455, c.r32(11), 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t456) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090e9e6); };
            c.w32(8, t456);
            let r457 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r457);
            blk = 188; continue;
        }
        183 => {
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e9ef); };
            c.w64(8, t458);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        184 => {
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090e9fb); };
            c.w64(8, t459);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        185 => {
            let Some(t460) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090ea07); };
            c.w32(8, t460);
            let Some(t461) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090ea0b); };
            c.cmp(t461, c.r32(8), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ea14); };
            c.w64(8, t462);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        187 => {
            let Some(t463) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090ea20); };
            c.w32(8, t463);
            let r464 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r464);
            blk = 188; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ea2e); };
            c.w64(3, t465);
            let Some(t466) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ea32); };
            c.w64(11, t466);
            let Some(t467) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700090ea36); };
            c.w32(0, t467);
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ea3b); };
            c.w64(15, t468);
            let Some(t469) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ea3f); };
            c.w64(1, t469);
            let Some(t470) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ea43); };
            c.w64(5, t470);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            let r471 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r471);
            return RegfileExit::jump(c.r64(1));
        }
        189 => {
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ea53); };
            c.w64(8, t472);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        190 => {
            let r473 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r473);
            let Some(t474) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ea63); };
            c.w64(8, t474);
            let Some(t475) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090ea67); };
            c.w32(11, t475);
            let Some(t476) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ea6c); };
            c.w64(1, t476);
            let Some(t477) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ea70); };
            c.w64(5, t477);
            let Some(t478) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090ea74); };
            c.w64(0, t478);
            c.w64(12, c.r64(8));
            let r479 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r479);
            return RegfileExit::jump(c.r64(1));
        }
        191 => {
            let r480 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r480);
            let Some(t481) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ea88); };
            c.w32(8, t481);
            let Some(t482) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ea8d); };
            c.w64(1, t482);
            let Some(t483) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ea91); };
            c.w64(5, t483);
            let r484 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r484);
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
            let r485 = c.r64(3) & 0x7ffu64;
            c.w64(3, r485);
            c.logic(r485, 64);
            c.logic(c.r8(1) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 201; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t486) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090eac2); };
            c.w32(0, t486);
            let r487 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r487);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t488) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090eacc); };
            c.w64(0, t488);
            blk = 196; continue;
        }
        196 => {
            let Some(t489) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ead3); };
            c.w32(0, t489);
            let r490 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r490);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            let r491 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r491);
            let Some(t492) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090eae3); };
            c.w32(8, t492);
            let Some(t493) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090eae8); };
            c.w64(1, t493);
            let Some(t494) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090eaec); };
            c.w64(5, t494);
            let r495 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r495);
            return RegfileExit::jump(c.r64(1));
        }
        198 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t496) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090eafe); };
            c.w32(3, t496);
            let Some(t497) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090eb01); };
            c.cmp(t497, c.r32(3), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 200; continue;
        }
        200 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700090eb0e); }
            blk = 254; continue;
        }
        201 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let r498 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r498);
            let Some(t499) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090eb21); };
            c.w32(8, t499);
            let Some(t500) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090eb26); };
            c.w32(0, t500);
            let r501 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r501);
            let Some(t502) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090eb2d); };
            c.w64(1, t502);
            let Some(t503) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090eb31); };
            c.w64(5, t503);
            let r504 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r504);
            return RegfileExit::jump(c.r64(1));
        }
        203 => {
            let Some(t505) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090eb3b); };
            c.w32(0, t505);
            let r506 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r506);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t507) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090eb4a); };
            c.w32(3, t507);
            let Some(t508) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090eb4d); };
            c.cmp(t508, c.r32(3), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 205; continue;
        }
        205 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700090eb5a); }
            blk = 254; continue;
        }
        206 => {
            c.w64(3, c.r64(1));
            let r509 = c.shr(c.r32(3), 0x7u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r509);
            let r510 = c.r32(3) & 0xfu64;
            c.w32(3, r510);
            c.logic(r510, 32);
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
            return RegfileExit::fault(0x700090ebca);
        }
        220 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r511 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r511);
            let r512 = c.r64(1) & 0xfffffu64;
            c.w64(1, r512);
            c.logic(r512, 64);
            let Some(t513) = c.ld8(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ebda); };
            c.w32(8, (t513 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t514) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ebe4); };
            c.w32(8, t514);
            let Some(t515) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ebe9); };
            c.w64(1, t515);
            let Some(t516) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ebed); };
            c.w64(5, t516);
            let r517 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r517);
            return RegfileExit::jump(c.r64(1));
        }
        221 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r518 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r518);
            let r519 = c.r64(1) & 0xfffffu64;
            c.w64(1, r519);
            c.logic(r519, 64);
            let Some(t520) = c.ld16(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ec06); };
            c.w32(8, (t520 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t521) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ec10); };
            c.w32(8, t521);
            let Some(t522) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ec15); };
            c.w64(1, t522);
            let Some(t523) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ec19); };
            c.w64(5, t523);
            let r524 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r524);
            return RegfileExit::jump(c.r64(1));
        }
        222 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r525 = c.r64(1) & 0xfffffu64;
            c.w64(1, r525);
            c.logic(r525, 64);
            let Some(t526) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ec2e); };
            c.w32(8, t526);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090ec33); }
            c.w64(11, c.r64(8));
            let r527 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r527);
            if c.cc_o() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 229; continue;
        }
        224 => {
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090ec47); };
            c.w64(11, t528);
            let Some(t529) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090ec4e); };
            c.w64(0, t529);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t530) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090ec55); };
            c.w64(2, t530);
            let Some(t531) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090ec5c); };
            c.cmp(t531, c.r64(3), 64);
            if c.cc_a() { blk = 227; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090ec6b, 0x700090ec66);
        }
        226 => {
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ec6b); };
            c.w64(8, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ec6f); };
            c.w64(15, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ec73); };
            c.w64(9, t534);
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090ec77); };
            c.w64(12, t535);
            blk = 228; continue;
        }
        227 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090ec7d); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 228; continue;
        }
        228 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t536) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090ec90); };
            c.w64(8, t536);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090ec97); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090ec9b); }
            blk = 229; continue;
        }
        229 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x7));
            let Some(t537) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090eca7); };
            c.w32(8, t537);
            let Some(t538) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ecac); };
            c.w64(1, t538);
            let Some(t539) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ecb0); };
            c.w64(5, t539);
            let r540 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r540);
            return RegfileExit::jump(c.r64(1));
        }
        230 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r541 = c.r64(1) & 0xfffffu64;
            c.w64(1, r541);
            c.logic(r541, 64);
            let Some(t542) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ecc5); };
            c.w32(8, t542);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090ecca); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            let r543 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r543);
            blk = 237; continue;
        }
        232 => {
            let Some(t544) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090ecdc); };
            c.w64(11, t544);
            let Some(t545) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090ece3); };
            c.w64(0, t545);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t546) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090ecea); };
            c.w64(2, t546);
            let Some(t547) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090ecf1); };
            c.cmp(t547, c.r64(3), 64);
            if c.cc_a() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090ed00, 0x700090ecfb);
        }
        234 => {
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ed00); };
            c.w64(8, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ed04); };
            c.w64(15, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ed08); };
            c.w64(9, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090ed0c); };
            c.w64(12, t551);
            blk = 236; continue;
        }
        235 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090ed12); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 236; continue;
        }
        236 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t552) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090ed28); };
            c.w64(8, t552);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090ed2f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090ed33); }
            c.w64(8, c.r64(0));
            blk = 237; continue;
        }
        237 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t553) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ed42); };
            c.w32(11, t553);
            let Some(t554) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ed47); };
            c.w64(1, t554);
            let Some(t555) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ed4b); };
            c.w64(5, t555);
            c.w64(0, c.r64(8));
            let r556 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r556);
            return RegfileExit::jump(c.r64(1));
        }
        238 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r557 = c.r64(1) & 0xfffffu64;
            c.w64(1, r557);
            c.logic(r557, 64);
            let Some(t558) = c.ld64(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ed63); };
            c.w64(8, t558);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090ed68); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 246; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090ed72); };
            c.w64(11, t559);
            let Some(t560) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090ed79); };
            c.w64(0, t560);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t561) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090ed80); };
            c.w64(2, t561);
            let Some(t562) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090ed87); };
            c.cmp(t562, c.r64(3), 64);
            if c.cc_a() { blk = 242; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700090ed96, 0x700090ed91);
        }
        241 => {
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ed96); };
            c.w64(8, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ed9a); };
            c.w64(15, t564);
            let Some(t565) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ed9e); };
            c.w64(9, t565);
            let Some(t566) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090eda2); };
            c.w64(12, t566);
            blk = 243; continue;
        }
        242 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090eda8); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 243; continue;
        }
        243 => {
            let Some(t567) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090edb2); };
            c.w64(3, t567);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700090edb9); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700090edc2); }
            let r568 = c.neg(c.r64(8), 64);
            c.w64(8, r568);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700090edcc); }
            blk = 251; continue;
        }
        245 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700090edd2); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700090edd9); }
            blk = 251; continue;
        }
        246 => {
            let Some(t569) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090eddf); };
            c.w64(8, t569);
            let Some(t570) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090ede6); };
            c.w64(11, t570);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090eded); };
            c.w64(0, t571);
            let Some(t572) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700090edf4); };
            c.cmp(t572, c.r64(3), 64);
            if c.cc_a() { blk = 249; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700090ee03, 0x700090edfe);
        }
        248 => {
            blk = 250; continue;
        }
        249 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x700090ee05); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 250; continue;
        }
        250 => {
            let Some(t573) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090ee0f); };
            c.w64(8, t573);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090ee16); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700090ee1a); }
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090ee21); };
            c.w64(12, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ee25); };
            c.w64(15, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ee29); };
            c.w64(9, t576);
            blk = 251; continue;
        }
        251 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x7));
            let Some(t577) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090ee34); };
            c.w32(8, t577);
            let Some(t578) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ee39); };
            c.w64(1, t578);
            let Some(t579) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ee3d); };
            c.w64(5, t579);
            let r580 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r580);
            return RegfileExit::jump(c.r64(1));
        }
        252 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r581 = c.r64(1) & 0xfffffu64;
            c.w64(1, r581);
            c.logic(r581, 64);
            if !c.movss_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090ee52); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 254; continue;
        }
        253 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r582 = c.r64(1) & 0xfffffu64;
            c.w64(1, r582);
            c.logic(r582, 64);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090ee69); }
            blk = 254; continue;
        }
        254 => {
            let Some(t583) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090ee6f); };
            c.w64(8, t583);
            let Some(t584) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700090ee76); };
            c.w64(11, t584);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700090ee7d); }
            let Some(t585) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090ee82); };
            c.w64(3, t585);
            let Some(t586) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700090ee89); };
            c.cmp(t586, c.r64(0), 64);
            if c.cc_a() { blk = 257; continue; }
            blk = 255; continue;
        }
        255 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700090ee98, 0x700090ee93);
        }
        256 => {
            blk = 258; continue;
        }
        257 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700090ee9a); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 258; continue;
        }
        258 => {
            let Some(t587) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090eea4); };
            c.w64(3, t587);
            c.w64(9, c.r64(3).wrapping_add(0x7));
            let Some(t588) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090eeac); };
            c.w64(8, t588);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700090eeb3); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700090eeb7); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090eebc); }
            let Some(t589) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090eec1); };
            c.w64(12, t589);
            let Some(t590) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090eec5); };
            c.w32(8, t590);
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090eeca); };
            c.w64(15, t591);
            let Some(t592) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090eece); };
            c.w64(1, t592);
            let Some(t593) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090eed2); };
            c.w64(5, t593);
            let r594 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r594);
            return RegfileExit::jump(c.r64(1));
        }
        259 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r595 = c.add(c.r64(9), 0x7u64, 64);
            c.w64(9, r595);
            let r596 = c.r64(1) & 0xfffffu64;
            c.w64(1, r596);
            c.logic(r596, 64);
            let Some(t597) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700090eeeb); };
            c.w32(8, t597);
            let Some(t598) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090eef0); };
            c.w32(0, t598);
            let r599 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r599);
            let Some(t600) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700090eef7); };
            c.w64(1, t600);
            let Some(t601) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090eefb); };
            c.w64(5, t601);
            let r602 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r602);
            return RegfileExit::jump(c.r64(1));
        }
        260 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r603 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r603);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090ef0b); }
            let Some(t604) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700090ef0f); };
            c.w64(8, t604);
            let Some(t605) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ef16); };
            c.w32(8, t605);
            let r606 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r606);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700090ef1d); }
            let Some(t607) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700090ef1f); };
            c.w64(3, t607);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700090ef33, 0x700090ef2e);
        }
        261 => {
            return RegfileExit::fault(0x700090ef33);
        }
        262 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        263 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r608 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r608);
            c.w64(3, c.r64(9).wrapping_add(0x7));
            let Some(t609) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090ef41); };
            c.w32(2, t609);
            let Some(t610) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ef47); };
            c.w32(1, t610);
            let r611 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r611);
            let Some(t612) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090ef4f); };
            c.w64(6, t612);
            let Some(t613) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ef53); };
            c.w32(2, t613);
            let r614 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r614);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090ef5a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090ef5e); }
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090ef5f); };
            if !c.push64(t615) { return RegfileExit::fault(0x700090ef5f); }
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090ef62); };
            if !c.push64(t616) { return RegfileExit::fault(0x700090ef62); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700090ef65); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700090ef67); }
            let Some(t617) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x700090ef6b); };
            c.w64(3, t617);
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700090ef7f, 0x700090ef7a);
        }
        264 => {
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ef7f); };
            c.w64(8, t618);
            let Some(t619) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ef83); };
            c.w64(8, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090ef87); };
            c.w64(9, t620);
            let Some(t621) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090ef8b); };
            c.w32(11, t621);
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ef90); };
            c.w64(15, t622);
            let Some(t623) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090ef94); };
            c.w64(1, t623);
            let Some(t624) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090ef98); };
            c.w64(5, t624);
            c.w64(12, c.r64(8));
            let r625 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r625);
            return RegfileExit::jump(c.r64(1));
        }
        265 => {
            return RegfileExit::fault(0x700090efa5);
        }
        266 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090efa6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090efaa); }
            let r626 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r626);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r627 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r627);
            c.w64(0, c.r64(9).wrapping_add(0x7));
            let Some(t628) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090efbb); };
            c.w32(3, t628);
            let Some(t629) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090efc1); };
            c.w32(2, t629);
            let r630 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r630);
            let Some(t631) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090efc9); };
            c.w64(1, t631);
            let Some(t632) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090efcd); };
            c.w32(3, t632);
            let r633 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r633);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090efd4); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090efd8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090efdc); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700090efe0); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            c.w32(0, 0xeu64);
            c.w64(11, c.r64(0));
            return c.call(0x7000804df4, 0x700090eff7, 0x700090eff2);
        }
        267 => {
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090eff7); };
            c.w64(8, t634);
            let Some(t635) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090effb); };
            c.w64(8, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090efff); };
            c.w64(9, t636);
            let Some(t637) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090f003); };
            c.w32(11, t637);
            let Some(t638) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090f008); };
            c.w64(15, t638);
            let Some(t639) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090f00c); };
            c.w64(1, t639);
            let Some(t640) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090f010); };
            c.w64(5, t640);
            c.w64(12, c.r64(8));
            let r641 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r641);
            return RegfileExit::jump(c.r64(1));
        }
        268 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090f01d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090f021); }
            let r642 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r642);
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r643 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r643);
            let Some(t644) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090f02d); };
            c.w32(3, t644);
            let Some(t645) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f033); };
            c.w32(2, t645);
            let r646 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r646);
            let Some(t647) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f03b); };
            c.w32(3, t647);
            let r648 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r648);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090f042); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090f046); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090f047); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090f04b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090f04f); }
            let Some(t649) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x700090f053); };
            c.w64(3, t649);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700090f067, 0x700090f062);
        }
        269 => {
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f067); };
            c.w64(8, t650);
            let Some(t651) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f06b); };
            c.w64(8, t651);
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090f06f); };
            c.w64(11, t652);
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t653) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090f077); };
            c.w32(11, t653);
            let Some(t654) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090f07c); };
            c.w64(15, t654);
            let Some(t655) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700090f080); };
            c.w64(1, t655);
            let Some(t656) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090f084); };
            c.w64(5, t656);
            c.w64(12, c.r64(8));
            let r657 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r657);
            return RegfileExit::jump(c.r64(1));
        }
        270 => {
            return RegfileExit::jump(0x700090f094);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700090f0b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090f0b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090f0b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090f0be); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f0c4); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f0cc); };
            c.w32(8, t4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f0d2); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090f0d9); };
            c.w64(7, t7);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t8) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090f0e1); };
            c.w64(3, t8);
            let r9 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r9);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090f0e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090f0ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700090f0f0); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090f100); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090f104); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090f105); }
            c.w64(6, c.r64(3));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2728)) else { return RegfileExit::fault(0x700090f109); };
            c.w64(2, t10);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            c.w64(3, c.r64(2));
            return c.call(0x700086ed74, 0x700090f120, 0x700090f11b);
        }
        3 => {
            blk = 7; continue;
        }
        4 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090f122); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090f126); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090f127); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2730)) else { return RegfileExit::fault(0x700090f128); };
            c.w64(2, t11);
            c.w64(6, c.r64(3));
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(2));
            return c.call(0x700086ed74, 0x700090f13f, 0x700090f13a);
        }
        5 => {
            blk = 7; continue;
        }
        6 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090f141); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090f145); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090f146); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2738)) else { return RegfileExit::fault(0x700090f147); };
            c.w64(2, t12);
            c.w64(6, c.r64(3));
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(2));
            return c.call(0x700086ed74, 0x700090f15e, 0x700090f159);
        }
        7 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f15e); };
            c.w64(2, t13);
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f162); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090f166); };
            c.w64(7, t15);
            c.w64(9, c.r64(7).wrapping_add(0x4));
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700090f16e); };
            c.w32(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090f173); };
            c.w64(15, t17);
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700090f177); };
            c.w64(1, t18);
            let Some(t19) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090f17b); };
            c.w64(5, t19);
            c.w64(12, c.r64(2));
            let r20 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r20);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            return RegfileExit::jump(0x700090f18c);
        }
            _ => unreachable!(),
        }
    }
}
